# Galed — Project Context

This document captures the full strategic and design narrative behind Galed. It exists so that any contributor or AI assistant working on this codebase understands not just *what* to build but *why* — the thinking, tradeoffs, and market context that produced the RFC and technical decisions.

**Read this before CONTRIBUTING.md and RFC.md.**

---

## The Problem

Regulated software systems — clinical genomics, pharma, aerospace, medical devices — require formal validation documentation. Requirements must be traceable to tests, tests to code, code to deployment. Every change must be logged. Every threshold relaxation must be justified and approved. Auditors need to answer "why did this change and who agreed to it" for every significant decision in the system's history.

Currently, this answer lives in:
- Email threads
- Confluence pages with no structure
- Excel spreadsheets with informal version history
- Jira tickets with free-text acceptance criteria
- Word documents that get copy-pasted between validation cycles

The cost of reconstructing this trail for a CAP/CLIA/FDA audit: **$50,000–$200,000 per validation cycle**, primarily in manual labor. This is the pain Galed is designed to eliminate.

---

## Why Existing Tools Fail

The requirements management market has incumbent players: **Jama Connect, IBM DOORS, Visure, Copilot4DevOps, Aqua**. All of them bolt AI onto legacy RM systems as a feature. None of them have:

- A structured language designed for human-AI hybrid authorship
- Field-level permission boundaries between human and AI principals
- Probabilistic requirement types for ML/AI components
- Discourse trails as first-class artifacts (proposals with full history)
- Semantic conflict detection across requirements
- An open, file-based format that lives in version control

They are also expensive ($50K–$200K+ enterprise contracts), require dedicated administrators, and create vendor lock-in through proprietary formats.

---

## The Core Insight

Most requirements tools were designed for deterministic software. Clinical genomics, drug discovery, and autonomous systems are increasingly built on ML/AI components. A classifier that achieves 97.2% sensitivity on a validation cohort of 500 samples is not a deterministic system — it has a distribution, it drifts, it needs a drift trigger that mandates revalidation. No existing RM tool has a first-class concept for this.

Galed's four native requirement types (`deterministic`, `probabilistic`, `temporal`, `adversarial`) are the core technical differentiation. The probabilistic type with `drift_trigger` is genuinely novel in the RM space.

The second insight: **nobody has built a requirements language designed for human-AI hybrid authorship with explicit permission tiers.** The field-level lock model (`locked`, `human_approved`, `ai_autonomous`, `computed`) is novel IP. It answers the regulatory question "which parts of this requirement did a human author and which parts did AI suggest?" in a structured, auditable way.

---

## Market Strategy

### Don't fight Jira

Jira and ADO have overwhelming adoption gravity. Fighting them for project management is a losing battle. The strategy is to be a **language and intelligence layer** that sits above whatever ticketing system teams already use — not a replacement.

### Open source language, hosted platform

The HashiCorp/GitLab model:
- **CLI and language spec: MIT license, free forever** — this is the adoption driver
- **Hosted platform: paid** — graph UI, multi-user quorum workflows, audit exports, embedding service

The language becomes infrastructure. The platform is where you monetize. Nobody builds the state layer themselves (Terraform model).

### Wedge market: clinical bioinformatics

Don't go horizontal (competing with Jira/ADO at scale requires capital). The wedge is **clinical bioinformatics and regulated lab software** where:
- CAP/CLIA/FDA validation pain is acute and quantifiable
- The buyer (validation manager, lab director) is not an engineer
- Existing tools are either too expensive (DOORS) or too generic (Jira)
- The ML/AI req types directly address the genomics pipeline validation problem

Long-term expand to pharma, aerospace, medical devices.

---

## Git Strategy

### Git gravity is real

Users will put `.gal` files in their git repos. Fighting this is a losing battle — embrace it.

The question is what layer *above* git you own that git can't do alone. Git stores files. Git cannot:
- Enforce schema or field-level permissions at commit time
- Run quorum-gated merges with role validation
- Detect semantic conflicts across files
- Maintain a proposal lifecycle with discourse trails
- Run the embedding pipeline for inferred conflict detection
- Propagate impact analysis through the dependency graph

### Git-agnostic CLI, connector-based integration

The CLI is completely git-agnostic. Files are just files. Git is an optional connector:

```bash
galed connect --git          # installs pre-commit and pre-merge hooks
galed connect --sharepoint   # for clinical buyers who don't use git
galed connect --confluence   # sync to Confluence pages
```

This is how you reach the validation manager who lives in SharePoint. Same language, same CLI, different connector.

---

## Why Not a SaaS Product First

Early temptation: build a hosted SaaS platform immediately. Rejected because:

1. **No feedback loop** — you don't know if the language design resonates before you've built a $500K platform
2. **Wrong adoption strategy** — engineers adopt CLI tools; the language needs to earn adoption before the platform monetizes it
3. **Capital intensity** — competing with Jira/ADO horizontally requires capital you don't have

The correct sequence: **demo → CLI → MCP server → hosted platform**. Each stage validates the next.

---

## Demo Strategy

The demo was ranked last in the build sequence. This is a mistake.

**Without a demo you have no feedback loop.** The CLI is 2–3 months of real work. The demo is a week. Show it to one engineer, one validation manager, and one compliance lead. What they react to tells you what to actually build.

The existing graph visualization (`reqlang-graph.html`) is the foundation of the demo. Populate it with 10–15 real requirements from a real clinical system, add a mock CLI session, write one-pagers for each audience type. That's a demo.

---

## Build Sequence

### Phase 1 — JSON Schema + Demo (weeks 1–2)
- JSON Schema for `.gal` format — language-agnostic foundation, already started (`schema/requirement.json`)
- 10–15 real example `.gal` files from a carrier variant curation system
- Mock CLI session (scripted, doesn't need to be real)
- Update graph visualization with real data

### Phase 2 — Rust CLI MVP (months 1–3)
Priority order:
1. `galed validate` — schema + field lock enforcement
2. `galed graph` — build graph from `.galed/` directory, output JSON or summary
3. `galed status` — open proposals, conflicts, impact flags
4. `galed propose` — open a proposal on a field
5. `galed init` — scaffold `.galed/` directory
6. `galed import` — Jira/ADO authoring assistant (not a blind importer)

### Phase 3 — Python MCP Server (months 3–5)
Higher leverage than the web platform right now. An MCP server means any AI tool — Claude, Cursor, Copilot — can query the requirement graph natively:

- "What requirements depend on REQ-004?"
- "Are there any open conflicts in the reporting component?"
- "Draft a new requirement for the methylation QC gate"
- "What changed in the last 30 days and what needs impact review?"

This is the core thesis made tangible: Galed becomes the AI-native grounding layer for requirements in any codebase.

### Phase 4 — Hosted Platform (months 6–12)
Web UI for graph, proposal workflow, quorum tracking, audit export. This is where you monetize. The CLI stays free forever.

---

## Technical Architecture Rationale

### Rust for CLI

- **Single binary distribution** — `galed` ships as one executable, no runtime dependencies. Critical for regulated environments where installing Python runtimes requires change control.
- **Memory safety** — parsing and graph traversal are deterministic, no GC pauses
- **Type system** — schema correctness enforced at compile time, philosophically aligned with what Galed does for requirements
- **petgraph** — mature graph library for the requirement graph engine
- **serde** — first-class YAML/JSON serialization
- **clap** — best-in-class CLI argument parsing

### Python for MCP + Embeddings

- Anthropic MCP Python SDK is mature
- sentence-transformers and vector similarity are native Python territory
- The embedding pipeline doesn't need to be a compiled binary

### YAML for file format

- Human-readable and git-diffable (line-level diffs show exactly what changed)
- serde_yaml in Rust, PyYAML in Python — both mature
- Familiar to engineers who work with Kubernetes, GitHub Actions, etc.

### SQLite for local graph storage

- Embedded, no server dependency
- The CLI needs to build and query the graph locally without a running service
- Sufficient for thousands of requirements; the hosted platform can use Postgres

### Hybrid graph: declared + inferred edges

Pure graph is auditable but misses semantic conflicts. Pure embeddings are powerful but not regulatory-defensible. The hybrid approach:
- **Declared edges** — authoritative, human-confirmed, permanent
- **Inferred edges** — advisory, from embedding similarity, with confidence scores
- False positives marked as such become negative training signal; model calibrates to domain over time
- Inferred edges at ≥0.95 confidence auto-promote and block the requirement from locking until resolved

This gives auditors a clean, declared graph for compliance while the embedding layer surfaces conflicts humans would miss.

---

## The Jira Import Decision

**No generic Jira/ADO importer.** Jira data is structurally lossy — no requirement type, no AC fields, no cohort, no lock levels, no rationale. A blind import produces artifacts that appear complete but aren't, creating false confidence in the audit trail.

**Decision: authoring assistant.** The `galed import` command fetches a Jira issue and uses AI to draft a complete `.gal` requirement, with all unmappable fields explicitly flagged `status: incomplete`. The requirement cannot lock until a human resolves all incomplete fields. The discourse trail begins at first authorship, not at a broken migration.

Long-term: once Galed has adoption, native Jira/ADO plugins write `.gal` natively, making the import problem irrelevant.

---

## The Carrier Variant Incident (Real-World Motivation)

The example requirements in this repo are based on a real production incident: embedded newlines in genetic notation fields (ALT column) caused months of silent data failures in a carrier variant curation system. Records were being silently dropped or corrupted at the ingestion boundary. The failure was invisible until a data reconciliation audit revealed months of rollbacks.

This incident is the visceral example of what happens when:
- Requirements don't specify edge cases explicitly ("must reject embedded newlines")
- There's no structured acceptance criterion linking the requirement to a test
- There's no traceability from the failure back to a missing requirement

Galed makes this class of failure traceable before it happens. The carrier variant system is the canonical demo dataset.

---

## Governance Model Summary

The governance model is fully specified in RFC.md. Key decisions for implementers:

**Proposals are first-class artifacts.** When any `human_approved` or `ai_assisted` field is changed, a Proposal is opened. Every action in the review process (modify, approve, reject) is logged as a discourse entry with `actor`, `role`, `action`, `timestamp`. The discourse trail is immutable. This is the structured answer to "why did this threshold change and who agreed to it" — currently that answer lives in email.

**Quorum is domain-defaulted.** Clinical domain requires 2 approvers (`HUMAN_REVIEWER` + `CLINICAL_LEAD`) for `ac_relaxation`, `scope_change`, `type_change`. Users can tighten but never loosen domain defaults. `self_quorum_permitted` is a universal hard floor — you cannot approve your own proposal regardless of roles held.

**`ac_relaxation` is always flagged.** Loosening a clinical threshold (e.g., sensitivity from 0.995 to 0.98) is a distinct change classification that always triggers quorum review. This is the regulatory heart of the system — threshold relaxations are how labs get into trouble with regulators.

---

## Naming

The project is named from the Hebrew גל-עד (*gal-ed*), the modern Hebrew word for "cairn," literally meaning "heap of testimony" or "heap of evidence."

In Genesis 31, Jacob and Laban built a cairn of stones as a permanent covenant marker — a structured, physical record of an agreement between parties. The cairn was built stone by stone, each stone deliberately placed, the whole structure permanent and queryable by anyone who passed by.

Requirements are the same thing: each one a stone deliberately placed, the whole structure a permanent record of what was agreed, why, and by whom.

File extension: `.gal`
CLI binary: `galed`
crates.io: `galed`
PyPI: `galed`
GitHub: `github.com/aetucker91/galed`

---

## What Claude Code Should Do First

1. Read this file, CONTRIBUTING.md, and RFC.md in full
2. Look at `schema/requirement.json` and `examples/REQ-001.gal`
3. Look at `src/main.rs` — the CLI stub is already wired with all 6 subcommands
4. **Implement `galed validate` first** — this is the foundation
   - Parse a `.gal` file or directory of `.gal` files
   - Validate against `schema/requirement.json`
   - Enforce that required AC fields are present for the declared `type`
   - Output clear, colored errors with field paths
   - Exit 0 on success, 1 on validation failure (for CI integration)
5. Write tests as you go — `cargo test` should pass before moving to the next command
