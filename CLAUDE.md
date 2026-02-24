# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**galed** (גל-עד) — a requirements definition language and traceability engine for human-AI hybrid authorship in regulated systems (clinical genomics, pharma, aerospace). Read `RFC.md` for the full language specification and all resolved design decisions.

## Build & Run

```bash
cargo build              # compile
cargo run -- <subcommand> # run CLI (validate, graph, propose, status, init, import)
cargo test               # run all tests
cargo test <test_name>   # run a single test
```

The binary is `galed`. Cargo.toml declares it as `[[bin]]` at `src/main.rs`.

## Repository Layout

```
src/main.rs              # CLI entry point (clap derive-based subcommands)
schema/requirement.json  # JSON Schema for .gal file format (language-agnostic foundation)
.galed/                  # Example scaffolded project directory
  requirements/          # .gal requirement files (e.g. REQ-001.gal)
  components/            # component definitions
  policy/                # quorum and governance config
  proposals/             # open proposal artifacts
RFC.md                   # Full language spec (v0.4) — types, permissions, versioning, graph model, governance
CONTRIBUTING.md          # Contributor guide — tech stack, file format, build priorities
```

## Architecture

**Two-crate design (planned):**
- `galed` (Rust) — CLI core: parser, validator, graph engine, SQLite-backed graph store
- `galed-py` (Python, future) — embedding pipeline + MCP server

**Key Rust dependencies:** clap 4 (CLI), serde + serde_yaml (YAML parsing), jsonschema (schema validation), petgraph (graph engine), rusqlite with bundled SQLite (local graph store), sha2 (content hashing).

**CLI subcommands:** `validate`, `graph`, `propose`, `status`, `init`, `import` — all currently stubbed, printing placeholder messages.

## .gal File Format

YAML-based. Four requirement types with type-specific required AC fields:

- `deterministic` — requires: `input`, `expected_output`, `tolerance`
- `probabilistic` — requires: `metric`, `threshold`, `comparison`, `cohort`, `n_samples`, `confidence_interval`, `drift_trigger`
- `temporal` — requires: `deadline`, `deadline_unit`, `trigger_event`, `failure_behavior`
- `adversarial` — requires: `attack_type`, `perturbation_budget`, `robustness_metric`, `threshold`, `cohort`

Field-level permission locks: `locked` (human only), `human_approved` (AI proposes, human approves), `ai_autonomous` (AI commits directly), `computed` (CI-managed). See `schema/requirement.json` for the full schema including lock annotations.

## Build Priorities

1. JSON Schema validation for `.gal` files
2. `galed validate` — parse + schema validate
3. `galed graph` — build requirement graph from `.galed/` directory
4. `galed propose` — open a field-change proposal
5. `galed status` — show proposals, conflicts, impact flags
6. Python MCP server

## Key Design Rules

- **Git-agnostic** — `.gal` files are just files; git is an optional connector
- **Declared graph edges are authoritative; inferred edges (from embeddings) are advisory** — every edge carries `provenance` and `confidence`
- **Proposals are first-class artifacts** with full immutable discourse trails, not log entries
- **Semver per requirement** — `REQ-{id} v{MAJOR}.{MINOR}.{PATCH}` with mandatory `change_class`
- **`ac_relaxation` is always flagged** — loosening a threshold is a distinct regulatory event
- **`self_quorum_permitted: false`** is a universal hard floor — no actor approves their own proposal
