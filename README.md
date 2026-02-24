# galed — גל-עד

**A requirements definition language and traceability engine for human-AI hybrid authorship in regulated systems.**

Named from the Hebrew גל-עד (*gal-ed*), meaning "heap of testimony" — a cairn built stone by stone to permanently mark a covenant between parties.

---

## What is Galed?

Galed is a structured requirements language (`.gal` files) with a CLI, graph engine, and MCP server designed for regulated industries where audit trails are non-negotiable — clinical genomics, pharma, aerospace, and any system requiring CAP/CLIA/FDA-style validation.

Existing requirements tools bolt AI onto legacy RM systems. Galed is designed from the ground up for human-AI hybrid authorship — with explicit permission boundaries, discourse trails, quorum enforcement, and semantic conflict detection built into the language itself.

## Core Concepts

- **`.gal` files** — declarative requirement definitions with four native types: `deterministic`, `probabilistic`, `temporal`, `adversarial`
- **Field-level permissions** — `locked`, `human_approved`, `ai_autonomous`, `computed`
- **Proposal lifecycle** — AI proposes, humans modify/approve, full discourse trail is immutable
- **Requirement graph** — declared edges (authoritative) + inferred edges (advisory via embeddings)
- **Quorum enforcement** — domain-defaulted, role-based, with segregation of duties support
- **Semantic versioning per requirement** — `REQ-{id} v{MAJOR}.{MINOR}.{PATCH}` with mandatory change classification

## Architecture

```
galed/               # Rust — CLI core (parser, validator, graph engine)
galed-py/            # Python — embedding pipeline + MCP server
```

The CLI is git-agnostic. Git integration is an optional connector, not a dependency.

## Build Order

1. **JSON Schema** — `.gal` file format definition (language-agnostic foundation)
2. **Rust CLI MVP** — `galed validate`, `galed graph`, `galed propose`, `galed status`
3. **Python MCP server** — AI-native graph query interface
4. **Hosted platform** — web UI, multi-user quorum workflows, audit export

## CLI (planned)

```bash
galed init              # scaffold .galed/ directory in a project
galed validate          # schema + field lock enforcement
galed graph             # output requirement graph as JSON or render locally
galed propose           # open a proposal on a field
galed status            # show open proposals, conflicts, impact flags
galed import --source jira --issue PROJ-123 --domain clinical
```

## Status

**Pre-alpha.** Namespace claimed, RFC complete, implementation starting.

- [x] RFC v0.4 — language design, permission model, versioning, graph model, governance
- [x] crates.io namespace — `galed`
- [x] PyPI namespace — `galed`
- [ ] JSON Schema for `.gal` format
- [ ] Rust CLI MVP
- [ ] Python MCP server
- [ ] VS Code extension

## License

MIT
