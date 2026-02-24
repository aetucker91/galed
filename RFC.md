# Galed RFC v0.4
## A Requirements Definition Language for Human-AI Hybrid Authorship in Regulated Systems

**Status:** Draft — Governance Model Complete  
**Authors:** Avery Tucker (BISD, Baylor Genetics)  
**Date:** 2026-02-24  

---

## 1. Motivation

Existing requirements languages (IEEE 830 SRS, EARS, Gherkin, Planguage) were designed for human authors and human readers. They fail in three ways as AI becomes a co-author in regulated software development:

1. **No permission model** — any author can modify any field; there is no machine-enforceable distinction between what a human must own vs. what an AI may assist with
2. **No probabilistic type system** — deterministic "shall" statements cannot express ML model behavior, drift thresholds, or adversarial robustness criteria
3. **No semantic versioning** — change history is narrative (commit messages) rather than structured, making regulatory audit trails fragile and manual

Galed addresses all three. It is designed to be:
- Human-readable and writable (EARS-influenced natural language templates)
- Machine-parseable and embeddable (strict schema, semantic field types)
- AI-collaborative with enforced permission boundaries
- Audit-ready by construction (versioning, attribution, change classification baked in)

---

## 2. Requirement Types

Galed natively supports four requirement types. The `type` field is mandatory and drives which sub-fields are required.

### 2.1 `deterministic`
Classical exact-output requirements. Output is fully specified and binary pass/fail.

```yaml
type: deterministic
```
Required AC fields: `input`, `expected_output`, `tolerance` (optional, for numeric equality)

### 2.2 `probabilistic`
For ML/LLM components where output is a distribution, not a point value.

```yaml
type: probabilistic
```
Required AC fields: `metric`, `threshold`, `comparison` (≥ / ≤ / within), `cohort`, `n_samples`, `confidence_interval`  
Additional required: `drift_trigger` — defines the condition that mandates revalidation

### 2.3 `temporal`
Time-bounded or sequence-dependent behavior. Covers SLAs, ordering constraints, timeout behavior.

```yaml
type: temporal
```
Required AC fields: `deadline`, `deadline_unit`, `trigger_event`, `failure_behavior`

### 2.4 `adversarial`
Model robustness under distributional shift, out-of-distribution inputs, or deliberate perturbation.

```yaml
type: adversarial
```
Required AC fields: `attack_type`, `perturbation_budget`, `robustness_metric`, `threshold`, `cohort`  
Note: `attack_type` is an enum: `{ood, label_flip, input_perturbation, prompt_injection, data_poisoning}`

---

## 3. Permission Model

Galed uses a **hybrid model**: field-level locking combined with role-based write scope.

### 3.1 Principal Hierarchy

```
HUMAN_AUTHOR      — full write access to all fields
HUMAN_REVIEWER    — approve/reject AI-proposed changes; cannot initiate edits
AI_ASSISTED       — may propose changes to unlocked fields; changes enter PENDING state
AI_AUTONOMOUS     — may commit patch-level changes to explicitly ai_autonomous fields without approval
CI_GATE           — system principal; writes computed fields (hash, test_links) automatically
```

### 3.2 Field Lock Levels

Each field in a requirement carries one of four lock levels:

| Lock Level | Who Can Write | Approval Required |
|---|---|---|
| `locked` | HUMAN_AUTHOR only | N/A |
| `human_approved` | AI_ASSISTED may propose | HUMAN_REVIEWER must approve before commit |
| `ai_autonomous` | AI_AUTONOMOUS may commit | No approval, but change is logged + reversible |
| `computed` | CI_GATE only | N/A — derived from other fields |

### 3.3 Default Field Lock Assignments

| Field | Default Lock Level | Rationale |
|---|---|---|
| `id` | `locked` | Immutable identifier — regulatory traceability anchor |
| `version` | `computed` | Bumped by system on any committed change |
| `type` | `locked` | Changing type is a scope change; always human |
| `domain` | `locked` | Scope classification |
| `rationale` | `locked` | Captures intent; must remain human-authored |
| `description` | `human_approved` | AI may refine wording; human must approve |
| `ac.threshold` | `human_approved` | Threshold changes have clinical significance |
| `ac.metric` | `locked` | Changing the metric changes what is measured |
| `ac.cohort` | `locked` | Cohort changes require revalidation |
| `ac.wording` | `ai_autonomous` | Clarity/grammar improvements only |
| `drift_trigger` | `human_approved` | Operationally significant |
| `linked_tests` | `computed` | Populated by CI from test registry |
| `linked_code` | `computed` | Populated by CI from code traceability scan |
| `change_log` | `computed` | Append-only, system-managed |

---

## 4. Versioning

Galed uses **semantic versioning per requirement** with mandatory change classification.

### 4.1 Version Format

```
REQ-{id} v{MAJOR}.{MINOR}.{PATCH}
```

| Segment | Increment When | Examples |
|---|---|---|
| `MAJOR` | Scope changes — type changes, domain changes, rationale changes, cohort changes | Changing from deterministic → probabilistic; changing the metric |
| `MINOR` | AC changes — threshold changes, drift trigger changes, new AC fields added | Tightening sensitivity from 0.90 → 0.95 |
| `PATCH` | Wording changes — description clarity, grammar, linked_tests updates | Rephrasing description without changing meaning |

### 4.2 Change Classification (mandatory on every commit)

Every version bump requires a `change_class` field drawn from this enum:

```
scope_change      → MAJOR bump required
ac_refinement     → MINOR bump required  
ac_relaxation     → MINOR bump required (flagged separately for audit)
wording_only      → PATCH bump required
test_link_update  → PATCH bump, CI-managed
code_link_update  → PATCH bump, CI-managed
```

`ac_relaxation` is intentionally separated from `ac_refinement` — loosening a clinical threshold is a distinct regulatory event and may trigger revalidation review.

### 4.3 Content Hash

Every committed version also carries a `sha256` hash of the canonical serialized requirement fields (excluding `change_log`). This enables content-addressed lookup and tamper detection — similar to git object storage.

---

## 5. Full Schema

```yaml
# ── Identity ─────────────────────────────────────────────────────────────────
id: REQ-001                          # locked | format: REQ-{NNN} or DOMAIN-{NNN}
version: 1.2.0                       # computed | semver, bumped on every commit
hash: sha256:a3f9...                  # computed | content hash of canonical fields
domain: Carrier Curation             # locked
type: deterministic                  # locked | enum: deterministic | probabilistic | temporal | adversarial

# ── Authorship & Permissions ─────────────────────────────────────────────────
authored_by: human                   # locked | principal who created this req
created_at: 2026-02-23T10:00:00Z     # locked
permissions:                         # locked | override default field locks if needed
  description: human_approved        # example override
  ac.threshold: locked               # example override — tighter than default

# ── Content ──────────────────────────────────────────────────────────────────
rationale: |                         # locked
  Silent curation losses were undetected for ~3 months in production.
  This requirement ensures zero unlogged losses per batch.

description: |                       # human_approved
  When the curation updater processes a batch run, the system shall produce
  a reconciliation report comparing input variant count to successfully
  committed variant count, and shall emit a structured error for any
  non-zero loss delta.

# ── Acceptance Criteria ──────────────────────────────────────────────────────
ac:
  type_fields:                       # fields vary by requirement type
    input: "Batch of N curated review files"
    expected_output: "Reconciliation report with input_count == committed_count or structured error emitted"
    tolerance: 0                     # zero loss tolerance

# ── For probabilistic type, ac looks like: ───────────────────────────────────
# ac:
#   metric: sensitivity              # locked
#   threshold: 0.95                  # human_approved
#   comparison: ">="                 # locked
#   cohort: validation_set_v3        # locked
#   n_samples: 500                   # locked
#   confidence_interval: 0.95        # human_approved
#   drift_trigger:                   # human_approved
#     metric: confidence_distribution
#     condition: shifts_by_sigma
#     sigma: 2.0
#     action: trigger_revalidation

# ── For temporal type: ───────────────────────────────────────────────────────
# ac:
#   trigger_event: batch_job_submitted
#   deadline: 4
#   deadline_unit: hours
#   failure_behavior: emit_alert_and_halt

# ── For adversarial type: ────────────────────────────────────────────────────
# ac:
#   attack_type: ood
#   perturbation_budget: 0.05
#   robustness_metric: accuracy_drop
#   threshold: 0.03
#   cohort: adversarial_test_set_v1

# ── Traceability (CI-managed) ─────────────────────────────────────────────────
linked_tests:                        # computed
  - TEST-047
  - TEST-048
linked_code:                         # computed
  - src/curation/updater.pl:L1410
  - src/curation/report_generator.pm:L323

# ── Change Log (append-only, system-managed) ──────────────────────────────────
change_log:
  - version: 1.0.0
    date: 2026-01-10T09:00:00Z
    author: human
    principal: avery.tucker
    change_class: scope_change
    summary: "Initial authorship post carrier-var incident"
  - version: 1.1.0
    date: 2026-01-28T14:22:00Z
    author: ai_assisted
    principal: claude-sonnet-4
    approved_by: avery.tucker
    change_class: ac_refinement
    summary: "Clarified loss delta condition and structured error requirement"
  - version: 1.2.0
    date: 2026-02-23T10:00:00Z
    author: ai_autonomous
    principal: claude-sonnet-4
    change_class: wording_only
    summary: "Grammar refinement in description field"
```

---

## 6. Embedding Strategy

For AI-native search and similarity, each requirement is serialized to a canonical embedding document:

```
[DOMAIN: Carrier Curation] [TYPE: deterministic] [VERSION: 1.2.0]
RATIONALE: Silent curation losses were undetected for ~3 months...
DESCRIPTION: When the curation updater processes a batch run...
AC: input_count == committed_count OR structured error emitted. tolerance=0.
LINKED_TESTS: TEST-047, TEST-048
```

Fields are ordered by semantic weight: domain > type > rationale > description > ac > links.  
`wording_only` patch bumps do not invalidate the embedding (re-embed only on MINOR or MAJOR).

---

## 7. Resolved Design Decisions

### 7.1 Proposal Lifecycle — Discourse Trail

Proposals are first-class artifacts, not log entries. Every `human_approved` or `ai_assisted` change initiates a Proposal with a full discourse trail before any field is mutated:

```yaml
proposal:
  id: PROP-0042
  req_id: REQ-005
  field: ac.threshold
  proposed_value: 0.98
  proposed_by: claude-sonnet-4
  proposed_by_role: AI_ASSISTED
  proposed_at: 2026-02-23T09:00:00Z
  rationale: "Align with REQ-013 reporting threshold to reduce cross-component divergence"

  discourse:
    - actor: avery.tucker
      role: HUMAN_AUTHOR             # role exercised at time of this action
      action: modified               # human did not simply approve — they changed it
      modified_value: 0.995
      comment: "0.98 insufficient for CAP audit — must match curation cohort baseline"
      at: 2026-02-23T10:14:00Z

    - actor: reviewer.a              # anonymized — second approver
      role: CLINICAL_LEAD
      action: approved
      comment: "Concur — 0.995 aligns with BIRD validation spec"
      at: 2026-02-23T11:02:00Z

  quorum_satisfied:
    required_roles: [HUMAN_REVIEWER, CLINICAL_LEAD]
    fulfilled_by:
      - role: HUMAN_REVIEWER  →  avery.tucker   (action: modified_and_approved)
      - role: CLINICAL_LEAD   →  reviewer.a     (action: approved)
    segregation_of_duties: true      # enforced — different actors required per role
    self_quorum_permitted: false     # enforced — actor cannot approve own proposal
    result: satisfied

  resolution: modified_and_approved  # enum: approved | modified_and_approved | rejected | withdrawn
  final_value: 0.995
  committed_version: "REQ-005 v2.1.0"
```

Both the original AI proposal and all human modifications are immutable records. The discourse trail is the auditable answer to "why did this threshold change and who agreed to it" — currently that answer lives in email threads. In Galed it is structured, queryable, and permanently associated with the requirement.

### 7.2 Deprecation — Version-Implicit vs. Explicit

Non-leading versions are **historical by default** — they exist as audit references but are not deprecated. Explicit deprecation is a distinct, intentional act:

- `v1.1.0 → v1.2.0` — v1.1.0 is historical; both are valid audit references
- `REQ-005 → REQ-016 (SUPERSEDES)` — REQ-005 is deprecated; excluded from active validation sets

```yaml
status: deprecated                   # enum: active | deprecated | superseded
deprecated_at: 2026-02-23T...
deprecated_by: avery.tucker
deprecation_reason: "Replaced by REQ-016 post-pipeline migration"
superseded_by: REQ-016
archived: true                       # excluded from active runs; permanently queryable in audit
```

The `SUPERSEDES` graph edge maintains the ancestry chain. Deprecated requirements are immutable.

### 7.3 Quorum Policy — Domain-Defaulted, Role-Based, User-Tightenable Only

Quorum is domain-defaulted and role-based. Users may tighten defaults but never loosen them — loosening requires a logged justification that itself enters the discourse trail.

```yaml
quorum_policy:
  default:
    min_approvers: 1
    roles_required: [HUMAN_REVIEWER]
    segregation_of_duties: false     # single actor may fulfill multiple roles
    self_quorum_permitted: false     # hard floor — actor cannot approve own proposal, ever

  domain_overrides:
    clinical:
      min_approvers: 2
      roles_required: [HUMAN_REVIEWER, CLINICAL_LEAD]
      triggers: [ac_relaxation, scope_change, type_change]
      segregation_of_duties: true    # different actors required per role
      self_quorum_permitted: false

    aerospace:
      min_approvers: 3
      roles_required: [HUMAN_REVIEWER, SAFETY_ENGINEER, QA_LEAD]
      triggers: [ac_relaxation, scope_change]
      segregation_of_duties: true
      self_quorum_permitted: false

    research:
      min_approvers: 1
      roles_required: [HUMAN_REVIEWER]
      segregation_of_duties: false   # single actor may hold and exercise multiple roles
      self_quorum_permitted: false

  user_overrides:
    allowed: true
    constraint: cannot_reduce_below_domain_default
    loosening_requires: logged_justification_in_discourse

# Distinction:
# self_quorum_permitted: false  — you cannot approve your own proposal, regardless of roles held
# segregation_of_duties: true   — even if you didn't propose it, one actor cannot satisfy
#                                 multiple required roles in the same quorum
```

Rationale: ad hoc per-requirement quorum creates audit ambiguity. Fixed role lists per domain produce a deterministic, auditable answer to "why did this only need one approver."

### 7.4 Language Interop — Authoring Assistant over Generic Importer

No generic Jira/ADO importer. Jira data is structurally lossy — no req type, no AC fields, no cohort, no lock levels. A blind import produces incomplete artifacts that appear done but aren't, creating false confidence.

**Decision:** build a Galed authoring assistant — a CLI or VS Code command that fetches a Jira issue and uses AI to draft a complete Galed requirement, with all unmappable fields explicitly flagged `status: incomplete`. The requirement cannot be locked until a human resolves all incomplete fields. The discourse trail begins at first authorship, not at a broken migration.

```bash
galed import --source jira --issue BISD-412 --domain clinical
# Produces: REQ-draft-BISD-412.gal with incomplete fields flagged
# Human fills gaps → req promoted to active → lock eligible
```

Long-term: once Galed has adoption, native Jira/ADO plugins write Galed directly, making the import problem irrelevant.

---

## 8. Relational Model — Ontology, Graph, and Conflict Detection

### 8.1 Three-Tier Hierarchy

Galed organizes artifacts into three tiers:

```
System
  └── Component
        └── Requirement
```

- **System** — top-level product boundary (e.g., `carrier-var-curation-system`)
- **Component** — functional subsystem with owned behavior and an explicit interface contract (e.g., `ingestion`, `reconciliation`, `reporting`)
- **Requirement** — atomic behavioral statement, always scoped to exactly one component

Component boundaries define isolation: conflicts *within* a component are expected and resolvable during authorship; conflicts *across* components are architectural problems requiring escalation.

### 8.2 Component Schema

```yaml
component:
  id: COMP-ingestion
  version: 2.0.0                     # semver, same rules as requirements
  system: carrier-var-curation-system
  owner: BISD
  owns:
    - REQ-001
    - REQ-002
    - REQ-003
  exposes:                           # interface contract — fields other components may depend on
    - field: batch_reconciliation_report
      type: artifact
      producing_req: REQ-004
    - field: loss_delta
      type: integer
      producing_req: REQ-001
  depends_on:
    - component: COMP-lock-manager
      fields: [lock_directory]
```

The `exposes` contract is the key automation hook: when a `producing_req` changes at MINOR or MAJOR version, the system automatically flags all downstream `DEPENDS_ON` edges for human impact review.

### 8.3 Graph Edge Types

| Edge Type | Direction | Meaning |
|---|---|---|
| `SCOPED_TO` | REQ → COMP | Requirement belongs to component |
| `DEPENDS_ON` | REQ/COMP → REQ/COMP | Cannot be satisfied without target |
| `CONSTRAINS` | REQ → REQ | Limits valid solution space of target |
| `IMPLEMENTS` | REQ → REQ | Concrete realization of abstract requirement |
| `CONFLICTS_WITH` | REQ ↔ REQ | Behavioral contradiction (bidirectional) |
| `SUPERSEDES` | REQ → REQ | Replaces target — deprecation without deletion |

### 8.4 Hybrid Conflict Detection

The graph is **authoritative**. Embeddings are **advisory**. Every edge carries provenance and confidence:

```yaml
graph:
  scoped_to: COMP-ingestion
  edges:
    - type: DEPENDS_ON
      target: REQ-003
      provenance: declared
      confidence: 1.0
      rationale: "Reconciliation report requires lock directory write first"
      status: confirmed

    - type: CONFLICTS_WITH
      target: REQ-011
      provenance: inferred
      confidence: 0.91
      reason: "High semantic similarity with divergent AC threshold on same metric"
      severity_class: major
      status: under_review
```

### 8.5 Confidence Threshold Policy

| Confidence | Behavior |
|---|---|
| ≥ 0.95 | Auto-promote to declared; block requirement lock until resolved |
| 0.80 – 0.94 | Flagged in UI; non-blocking; human must mark resolved or false_positive |
| 0.60 – 0.79 | Advisory; surfaced passively in graph view |
| < 0.60 | Suppressed |

### 8.6 Conflict Severity Scoring

```
severity_score = f(
  edge_type,           # CONFLICTS_WITH > CONSTRAINS overlap > DEPENDS_ON cycle
  component_boundary,  # cross-component higher than intra-component
  req_type,            # probabilistic threshold conflicts > deterministic > wording
  lock_level,          # locked field conflicts → critical; ai_autonomous → low
  change_class         # ac_relaxation → always critical regardless of other factors
)

critical  → score ≥ 0.85  | blocks all downstream work; quorum resolution required
major     → score 0.60–0.84 | blocks requirement lock; single reviewer resolves
minor     → score 0.35–0.59 | non-blocking; tracked in audit log
advisory  → score < 0.35   | informational only
```

### 8.7 False Positive Learning

Inferred edges marked `false_positive` are persisted as negative training signal. The embedding model calibrates to your domain over time. The graph remains authoritative and static; the model evolves beneath it without changing declared structure.

### 8.8 Impact Propagation

When a requirement changes at MINOR or MAJOR version:

1. System traverses all outbound `DEPENDS_ON` and `CONSTRAINS` edges
2. All reachable requirements and components are flagged `impact_review_required`
3. If an `exposes` field's `producing_req` changed, all consuming components are flagged
4. No downstream requirement can be locked or promoted to QA until impact review is cleared

Automated blast radius analysis on every non-trivial change — equivalent to manual impact assessment before a clinical validation run, but traceable and fast.

---

## 9. Next Steps

- [ ] Define formal grammar (EBNF or JSON Schema) covering all four req types
- [ ] Build reference parser in Python (open source, MIT license)
- [ ] Graph storage backend — evaluate RDF triple store vs. property graph (Neo4j) vs. embedded (SQLite + adjacency)
- [ ] VS Code extension: syntax highlighting, field validation, lock enforcement, inline conflict flags
- [ ] CI integration: auto-populate `linked_tests` and `linked_code` via static analysis
- [ ] Embedding pipeline: canonical serialization → embedding → inferred edge generation
- [ ] Interactive graph visualization (force-directed, component clusters, conflict severity coloring)
- [ ] Run syntax experiment results through schema and validate type coverage
- [ ] Publish RFC as open standard — GitHub org, versioned spec, CHANGELOG

---

## 10. Appendix — Decision Log

| # | Decision | Rationale | Status |
|---|---|---|---|
| D-001 | Four native req types: deterministic, probabilistic, temporal, adversarial | LLM/ML products require distribution-level AC; temporal and adversarial are first-class in regulated systems | Resolved |
| D-002 | Hybrid permission model: field-level locks + role-based principals | Field-level gives precision; roles give auditability | Resolved |
| D-003 | Semver per requirement with mandatory change_class | Regulators need structured change history, not narrative commit messages | Resolved |
| D-004 | Proposals as first-class artifacts with full discourse trail | Audit answer to "why did this change" must be structured and queryable, not in email | Resolved |
| D-005 | Non-leading versions are historical; explicit status:deprecated for intentional retirement | Version supersession ≠ deprecation; both must be distinguishable in audit | Resolved |
| D-006 | Domain-defaulted quorum, role-based, user-tightenable only | Prevents audit ambiguity; clinical default = 2 approvers for ac_relaxation/scope_change | Resolved |
| D-007 | Authoring assistant over generic importer for Jira/ADO interop | Lossy import creates false confidence; discourse trail must start at first authorship | Resolved |
| D-008 | Hybrid graph: declared edges authoritative, inferred edges advisory with confidence score | Pure graph is auditable; embeddings surface undeclared conflicts; false positives become training signal | Resolved |
| D-009 | Confidence thresholds: ≥0.95 auto-promote, 0.80-0.94 flagged, 0.60-0.79 advisory, <0.60 suppressed | Human fingerprint required on all confirmed conflict edges | Resolved |
| D-010 | exposes contract on components triggers downstream impact review on MINOR/MAJOR change | Automated blast radius analysis replaces manual impact assessment | Resolved |
| D-011 | actor + role required on every discourse entry | Role at time of action is what quorum validation checks; same actor can hold multiple roles | Resolved |
| D-012 | segregation_of_duties is domain-level config; self_quorum_permitted is a universal hard floor | Clinical/aerospace need actor separation per role; research does not; nobody approves their own proposal | Resolved |
