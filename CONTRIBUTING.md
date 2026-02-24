# Contributing to Galed

This document provides context for contributors and AI assistants working on this codebase. **Read RFC.md first** — it contains the full language specification and all resolved design decisions.

---

## Project Context

Galed is a requirements definition language for regulated systems (clinical, pharma, aerospace). The core thesis: existing tools bolt AI onto legacy RM systems. Galed is designed from the ground up for human-AI hybrid authorship with explicit permission boundaries, immutable discourse trails, and semantic conflict detection.

Target buyer pain: CAP/CLIA/FDA validation audit prep costs $50-200K per cycle. Galed makes that tractable by making requirements structured, traceable, and queryable from day one.

---

## Tech Stack Decisions (Final)

| Component | Language | Rationale |
|---|---|---|
| CLI core (parser, validator, graph) | **Rust** | Single binary distribution, memory safety, petgraph for graph traversal, serde for YAML/JSON |
| Embedding pipeline | **Python** | sentence-transformers, numpy/scipy — native territory |
| MCP server | **Python** | Anthropic MCP Python SDK is mature |
| File format | **YAML** | Human-readable, git-diffable, serde_yaml in Rust |
| Graph storage (local) | **SQLite** | Embedded, no server dependency for CLI |

---

## Key Rust Crates

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }     # CLI argument parsing
serde = { version = "1", features = ["derive"] }    # serialization
serde_yaml = "0.9"                                   # YAML parsing
serde_json = "1"                                     # JSON output
jsonschema = "0.18"                                  # schema validation
petgraph = "0.6"                                     # graph engine
rusqlite = { version = "0.31", features = ["bundled"] } # local graph store
sha2 = "0.10"                                        # content hashing
chrono = { version = "0.4", features = ["serde"] }  # timestamps
anyhow = "1"                                         # error handling
thiserror = "1"                                      # error types
indicatif = "0.17"                                   # progress bars
colored = "2"                                        # terminal output
```

---

## File Format (.gal)

Requirements live in `.gal` files (YAML). The `.galed/` directory in a project contains:

```
.galed/
  system.gal          # system definition
  policy/
    quorum.gal        # domain quorum config
  components/
    ingestion.gal     # component definition + owned req IDs
  requirements/
    REQ-001.gal
    REQ-002.gal
  proposals/
    PROP-0042.gal     # open proposals (pending approval)
```

### Requirement Types

Four native types — each has different required AC fields:

- `deterministic` — exact output, binary pass/fail. Requires: `input`, `expected_output`, `tolerance`
- `probabilistic` — ML/AI components, distribution-based. Requires: `metric`, `threshold`, `comparison`, `cohort`, `n_samples`, `confidence_interval`, `drift_trigger`
- `temporal` — time-bounded behavior, SLAs. Requires: `deadline`, `deadline_unit`, `trigger_event`, `failure_behavior`
- `adversarial` — model robustness. Requires: `attack_type`, `perturbation_budget`, `robustness_metric`, `threshold`, `cohort`

### Permission Model

Field lock levels:
- `locked` — HUMAN_AUTHOR only
- `human_approved` — AI proposes, human approves before commit
- `ai_autonomous` — AI commits directly, logged + reversible
- `computed` — CI_GATE only (derived fields like `linked_tests`, `linked_code`)

Principal hierarchy: `HUMAN_AUTHOR > HUMAN_REVIEWER > CLINICAL_LEAD > AI_ASSISTED > AI_AUTONOMOUS > CI_GATE`

### Versioning

Semver per requirement: `REQ-{id} v{MAJOR}.{MINOR}.{PATCH}`

- MAJOR: scope changes (type, domain, rationale, cohort, metric)
- MINOR: AC changes (threshold, drift_trigger)
- PATCH: wording changes, test/code link updates

Change classification is mandatory on every version bump:
`scope_change | ac_refinement | ac_relaxation | wording_only | test_link_update | code_link_update`

`ac_relaxation` is always flagged — loosening a clinical threshold triggers revalidation review.

---

## Graph Model

Three-tier hierarchy: `System → Component → Requirement`

Edge types:
- `SCOPED_TO` — REQ → COMP
- `DEPENDS_ON` — REQ/COMP → REQ/COMP
- `CONSTRAINS` — REQ → REQ
- `IMPLEMENTS` — REQ → REQ
- `CONFLICTS_WITH` — REQ ↔ REQ (bidirectional)
- `SUPERSEDES` — REQ → REQ (deprecation)

Every edge carries `provenance: declared | inferred` and `confidence: 0.0–1.0`.

Inferred edges come from the Python embedding pipeline. Confidence thresholds:
- ≥0.95 → auto-promote to declared, block req lock until resolved
- 0.80–0.94 → flagged, non-blocking
- 0.60–0.79 → advisory
- <0.60 → suppressed

---

## Governance Model

### Proposals

Every `human_approved` or `ai_assisted` field change opens a Proposal artifact. Proposals have a full discourse trail — actor, role, action, timestamp — before any field is mutated. The discourse trail is immutable.

Resolution enum: `approved | modified_and_approved | rejected | withdrawn`

### Quorum

Domain-defaulted, role-based. Users can tighten but never loosen domain defaults.

- `self_quorum_permitted: false` — universal hard floor, never overridable
- `segregation_of_duties` — domain-configurable (clinical: true, research: false)

Clinical domain default: `min_approvers: 2`, roles: `[HUMAN_REVIEWER, CLINICAL_LEAD]`, triggers on `ac_relaxation | scope_change | type_change`

### Deprecation

```yaml
status: deprecated       # enum: active | deprecated | superseded
deprecated_by: actor
deprecation_reason: "..."
superseded_by: REQ-016
archived: true
```

Deprecated requirements are immutable and permanently queryable in audit.

---

## Build Priorities

1. **JSON Schema** — validate `.gal` files language-agnostically. This is the foundation.
2. **`galed validate`** — parse + schema validate a `.gal` file or directory
3. **`galed graph`** — build and output the requirement graph from a `.galed/` directory
4. **`galed propose`** — open a proposal on a specific field
5. **`galed status`** — show open proposals, unresolved conflicts, impact flags
6. **Python MCP server** — expose graph queries to AI tools via MCP protocol

---

## Design Principles

- **Git-agnostic** — `.gal` files are just files. Git is an optional connector, not a dependency.
- **Single binary** — `galed` CLI ships as one Rust binary, no runtime dependencies
- **Hybrid graph** — declared edges authoritative, inferred edges advisory. False positives become negative training signal.
- **Proposals are first-class** — not log entries. Full discourse trail, queryable, permanent.
- **Honest about incomplete imports** — Jira importer flags unknown fields as `status: incomplete`. Requirements cannot lock until all incomplete fields are resolved.

---

## Naming

The project is named from the Hebrew גל-עד (*gal-ed*), the modern Hebrew word for "cairn," meaning "heap of testimony." In Genesis 31, Jacob built a cairn as a permanent covenant marker between himself and Laban. The metaphor: requirements are stones in a cairn — each one placed deliberately, building a permanent record of what was agreed and why.

File extension: `.gal`
CLI binary: `galed`
crates.io: `galed`
PyPI: `galed`
