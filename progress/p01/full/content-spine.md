# p01 Full content spine

This is the frozen synthetic-facts and relationship contract for all twenty
Rust scopes in the p01 Full physical-artifact profile. It contains and preserves
the accepted pilot contract for `p01-primary-01-full-pilot-b001`, then extends
the same world across the complete p01 allocation. It is operational production
input, not canonical Rust-plan evidence and not a Kio indexing, history, chunk,
search-quality, or performance claim.

## Contract rules

- All organizations, people, products, identifiers, messages, measurements, and
  events are synthetic. Role labels are preferred to personal names. Any email
  address uses the reserved `.invalid` domain.
- The canonical inventory remains authoritative for source, scope, family,
  variant, and extension. This spine supplies reusable content facts only.
- Dates use ISO `YYYY-MM-DD`; timestamps use RFC 3339 with an explicit offset.
  Working time is Asia/Tokyo unless a record explicitly says UTC.
- Counts are primary facts. Displayed percentages are derived and rounded, never
  substituted for their numerators and denominators.
- A scope may omit facts that are irrelevant to its subject, but it must not
  contradict the anchors below or create a second identity for an existing ID.
- Cross-format derivatives stay in the same Rust scope as their base facts.
  Ledger dependencies identify the precise production inputs.

## Spine ID catalog

Every non-pilot assignment row references the applicable IDs from this catalog.
The accepted pilot rows retain their original ID arrays field-for-field.

| Spine ID | Meaning |
| --- | --- |
| `org` | Synthetic organization identity and role aliases |
| `product` | Accepted pilot product identity, Product Alpha R7 |
| `release_date` | Accepted pilot release date, 2026-07-13 |
| `decision` | Accepted pilot decision, ADR-042 |
| `review_date` | Accepted pilot review date, 2026-07-20 |
| `evidence_window` | Accepted pilot observation window |
| `latency` | Accepted pilot six-stage latency reconciliation |
| `quality` | Accepted pilot request and gate-decision reconciliation |
| `rollback` | Accepted pilot rollback-drill facts |
| `portfolio_calendar` | Full-profile dates and project milestones |
| `identity_catalog` | Stable project, decision, review, message, and batch IDs |
| `service_catalog` | Stable synthetic services, environments, and role owners |
| `measure_catalog` | Shared Full-profile counts, durations, ratios, and units |
| `terminology` | Controlled vocabulary and status meanings |
| `cross_format_relationships` | Base-data and derivative rules |
| `visual_system` | Diagram, screenshot, scan, and chart conventions |
| `scope_p01_primary_01` | Product Alpha architecture facts |
| `scope_p01_primary_02` | Product Alpha API-contract facts |
| `scope_p01_primary_03` | Product Alpha release-note facts |
| `scope_p01_primary_04` | Product Beta architecture facts |
| `scope_p01_primary_05` | Product Beta API-contract facts |
| `scope_p01_primary_06` | Product Alpha repository-documentation facts |
| `scope_p01_primary_07` | Product Beta repository-documentation facts |
| `scope_p01_primary_08` | Engineering decision-record facts |
| `scope_p01_primary_09` | Code-review facts |
| `scope_p01_primary_10` | Engineering-meeting facts |
| `scope_p01_primary_11` | Synthetic platform-vendor evaluation facts |
| `scope_p01_primary_12` | Migration-program facts |
| `scope_p01_secondary_01` | Desktop working-copy semantics |
| `scope_p01_secondary_02` | Reference-copy semantics |
| `scope_p01_secondary_03` | Download inbox semantics |
| `scope_p01_secondary_04` | Export-bundle semantics |
| `scope_p01_secondary_05` | Individual cloud working-copy semantics |
| `scope_p01_secondary_06` | Team-shared accepted-copy semantics |
| `scope_p01_secondary_07` | Recent synthetic mail semantics |
| `scope_p01_secondary_08` | Closed-record archive semantics |

## Organization and role anchors (`org`)

| ID | Fixed fact |
| --- | --- |
| Organization | Aster Loop株式会社（synthetic） |
| Short code | `AL` |
| Locale | `ja-JP`, with Japanese/English engineering material |
| Work timezone | `Asia/Tokyo` |
| Engineering unit | Product Engineering |
| Role aliases | Release Coordinator, Platform Engineer, Observability Lead, Decision Owner, API Steward, Migration Lead |
| Mail aliases | role labels at `aster-loop.invalid`; no personal mailbox or real contact |

## Accepted pilot contract

The following facts and relationships are unchanged from
`prompts/p01-full-pilot-content-spine.md`.

### Identity and decision anchors

| ID | Fixed fact |
| --- | --- |
| `org` | Aster Loop株式会社（synthetic） |
| `product` | Product Alpha R7 |
| `release_date` | 2026-07-13 |
| `decision` | ADR-042 keeps the asynchronous release gate |
| `review_date` | 2026-07-20 |
| `evidence_window` | 2026-07-13 through 2026-07-20, inclusive |

All twelve accepted pilot artifacts use these exact facts when the subject is
mentioned. No Full artifact may assign a different company, Product Alpha R7
version, ADR-042 meaning, release date, or pilot review disposition.

### Shared pilot measurements

| Metric | Fixed value | Reconciliation |
| --- | ---: | --- |
| Observed end-to-end p95 | 184 ms | Equals the six stage values below |
| Observed requests | 84,200 | Synthetic requests in the evidence window |
| Successful requests | 84,149 | 99.94% when rounded to two decimals |
| Unsuccessful requests | 51 | `84,200 - 84,149` |
| Gate decisions sampled | 9,000 | Synthetic gate decisions |
| Approved decisions | 8,883 | 98.7% approval rate |
| Non-approved decisions | 117 | `9,000 - 8,883` |
| Rollback drill | 2026-07-17 | Controlled drill, not a production incident |
| Recovery duration | 11m42s | 702 seconds |
| Review disposition | keep ADR-042 | Recorded on 2026-07-20 |

### Pilot latency stages (`latency`)

| Order | Stage | p95 contribution |
| ---: | --- | ---: |
| 1 | Request intake | 28 ms |
| 2 | Edge API | 32 ms |
| 3 | Release Coordinator | 46 ms |
| 4 | Decision Store | 24 ms |
| 5 | Event Bus | 18 ms |
| 6 | Observability confirmation | 36 ms |
|  | **Total** | **184 ms** |

The approval boundary is between Release Coordinator and Decision Store. Event
Bus delivery occurs only after the gate decision, and Observability confirmation
closes the measured path.

### Accepted pilot relationships

- `alpha-r7-observability-snapshot.json` and
  `release-gate-slo-evidence-2026-q3.csv` are the base factual records.
- The accepted XLSX calculates success, approval, failure, and latency totals
  from typed cells rather than unexplained constants.
- The accepted DOCX, searchable PDFs, PPTX, and topology image explain the same
  evidence for different audiences.
- `release-gate-whiteboard-scan.pdf` wraps the exact
  `release-gate-whiteboard-capture.png` image as an image-only scan PDF without
  an accidental searchable text layer.

## Portfolio calendar (`portfolio_calendar`)

| Date or window | Fixed event |
| --- | --- |
| 2026-06-01 through 2026-09-30 | Full-profile evidence horizon |
| 2026-07-13 | Product Alpha R7 release |
| 2026-07-17 | Product Alpha controlled rollback drill |
| 2026-07-20 | ADR-042 post-launch review; disposition is keep |
| 2026-08-17 | Product Beta R4 architecture review |
| 2026-09-04 | Product Beta R4 API schema freeze |
| 2026-09-14 | Migration program production cutover |
| 2026-09-18 | Migration closeout review |
| 2026-09-28 | Product Beta R4 target release |

Dates outside the horizon may appear only as clearly labeled historical
references or future plans. They cannot replace these milestone meanings.

## Identity catalog (`identity_catalog`)

| Entity | Stable form and meaning |
| --- | --- |
| Alpha release | `ALPHA-R7`; display name Product Alpha R7 |
| Beta release | `BETA-R4`; display name Product Beta R4 |
| Alpha architecture decisions | `ADR-001` through `ADR-199`; ADR-042 is reserved as above |
| Beta architecture decisions | `ADR-B4-001` through `ADR-B4-199` |
| API contracts | `API-A7-####` and `API-B4-####` |
| Release records | `REL-A7-YYYYMMDD-NN` and `REL-B4-YYYYMMDD-NN` |
| Code reviews | `CR-A7-######` and `CR-B4-######` |
| Engineering meetings | `MTG-ENG-YYYYMMDD-NN` |
| Vendor evaluations | `VND-26Q3-NN` |
| Migration work | `MIG-26Q3-W01` through `MIG-26Q3-W06` |
| Synthetic mail | `MSG-2026Q3-######` |
| Canonical source | Existing Rust `p01-src-######`; never reassigned |
| Full artifact | Ledger `artifact_id`; never treated as a project or decision ID |

## Service and environment catalog (`service_catalog`)

| ID | Fixed role |
| --- | --- |
| `svc-edge-api` | Product Alpha/Beta request ingress |
| `svc-release-coordinator` | Asynchronous release-gate orchestration |
| `svc-decision-store` | Durable decision state after the approval boundary |
| `svc-event-bus` | Post-decision event delivery |
| `svc-observability` | Confirmation and evidence capture |
| `svc-contract-registry` | Versioned API schema catalog |
| `env-dev` | Synthetic developer environment |
| `env-stg` | Synthetic staging environment |
| `env-prod` | Synthetic production label; contains no real production data or secrets |

Example credentials, bearer values, private keys, customer identifiers, and
real infrastructure addresses are prohibited even in code or logs.

## Shared measures (`measure_catalog`)

| Measure | Fixed value | Use |
| --- | ---: | --- |
| Alpha observed p95 | 184 ms | Reconciles to the accepted six-stage pilot path |
| Alpha SLO ceiling | 200 ms | Target, not an observed value |
| Alpha requests / successful / unsuccessful | 84,200 / 84,149 / 51 | Accepted pilot evidence |
| Alpha gate sampled / approved / non-approved | 9,000 / 8,883 / 117 | Accepted pilot evidence |
| Beta design p95 budget | 225 ms | Design target, not an observed Alpha result |
| Beta contract tests | 1,240 total; 1,231 pass; 9 fail | 99.27% when rounded to two decimals |
| CI median duration | 7m18s | 438 seconds |
| Review median elapsed time | 6h40m | 400 minutes |
| Migration estate | 18 services in 6 waves | Exactly 3 services per wave |
| Migration recovery objectives | RPO 5m; RTO 45m | Targets, not measured incident outcomes |

Every CSV/TSV total, structured record, workbook formula, prose table, PDF,
slide, and chart that cites one of these measures must reconcile to the same
unit and primary counts.

## Controlled terminology (`terminology`)

- **release gate** is the asynchronous decision flow governed by ADR-042.
- **approval boundary** is the transition from Release Coordinator to Decision
  Store; it is not a network perimeter or authentication boundary.
- **non-approved** combines controlled denials and deferrals. It is not an
  outage count.
- **rollback drill** means the controlled 2026-07-17 exercise, not a production
  incident.
- **searchable PDF** means a `pdf_text` artifact with a genuine text layer.
- **scan PDF** means a `pdf_scan` derivative of a declared image dependency,
  with no accidental searchable text layer.
- **working**, **inbox**, and **my-files** copies are drafts or transport copies;
  **team-shared** is the accepted collaboration copy; **archive/closed** is an
  immutable closed snapshot. Location does not create a new project truth.
- `planned`, `draft`, `reviewed`, `approved`, `released`, and `closed` are
  distinct lifecycle states. They must not be used interchangeably.

## Cross-format relationships (`cross_format_relationships`)

1. In every Rust scope, `csv_tsv` and `structured_text` assignment rows are the
   reusable typed base facts for dependent XLSX, DOCX, searchable PDF, and PPTX
   rows. Each such derivative names a same-scope base artifact in
   `dependencies`.
2. XLSX workbooks consume typed cells and use formulas for rates, differences,
   duration totals, and reconciliations. A displayed formula result is never an
   unexplained replacement for its inputs.
3. DOCX and searchable PDF derivatives preserve the same counts, units, dates,
   status, and IDs while changing presentation for their audience.
4. PPTX derivatives use the same factual base. A chart's labels, totals, and
   ordering agree with its input row.
5. Every scan PDF names a same-scope ImageGen PNG dependency. The PNG is
   produced and visually inspected before the image-only PDF wrapper.
6. Dependencies never cross Rust scopes. A batch that supplies a dependency is
   produced before, or atomically with, the dependent batch; the master ledger
   batch order is topological.
7. Ordinary text, code, mail, notebooks, and domain binaries may refer to the
   same scope facts but do not silently become authoritative base data.

## Visual system (`visual_system`)

- Diagrams use a clean engineering palette: navy structure, teal healthy flow,
  amber review state, and red only for explicit failure or blocked state.
- Latency diagrams preserve the six accepted Alpha stages and approval-boundary
  placement whenever they depict the pilot flow.
- Charts label units and denominators. Decorative values that could be mistaken
  for measurements are not used.
- Synthetic photographed whiteboards may include natural perspective, marker
  texture, and role labels, but no real names, contacts, screens, badges, or
  customer material.
- A scan PDF is derived from its exact ledger-named PNG; it is not regenerated
  independently from a prompt.

## Scope contracts

| Spine ID | Canonical scope path | Fixed subject and relationship |
| --- | --- | --- |
| `scope_p01_primary_01` | `documents/work/product-alpha/architecture` | Alpha R7 architecture, ADR-042, accepted pilot evidence, and subsequent same-world architecture reviews |
| `scope_p01_primary_02` | `documents/work/product-alpha/api-contracts` | Alpha R7 API schemas and compatibility evidence under `API-A7-####` |
| `scope_p01_primary_03` | `documents/work/product-alpha/release-notes` | Alpha R7 release and post-release records under `REL-A7-*` |
| `scope_p01_primary_04` | `documents/work/product-beta/architecture` | Beta R4 event-driven architecture, 2026-08-17 review, and 225 ms design budget |
| `scope_p01_primary_05` | `documents/work/product-beta/api-contracts` | Beta R4 contract set, 2026-09-04 schema freeze, and 1,240-test reconciliation |
| `scope_p01_primary_06` | `repos/product-alpha/docs` | Alpha R7 repository documentation for the same services and ADR-042 behavior |
| `scope_p01_primary_07` | `repos/product-beta/docs` | Beta R4 repository documentation aligned to the Beta architecture and API facts |
| `scope_p01_primary_08` | `work-items/decision-records` | Alpha and Beta decision lifecycle records using the catalogued ADR ranges |
| `scope_p01_primary_09` | `work-items/code-reviews` | Synthetic review exports, comments, checks, and elapsed-time evidence under `CR-A7-*` or `CR-B4-*` |
| `scope_p01_primary_10` | `meetings/engineering` | Role-based agendas, minutes, action registers, and decision follow-ups under `MTG-ENG-*` |
| `scope_p01_primary_11` | `vendor-docs/platforms` | Synthetic `VND-26Q3-*` evaluations of Platform Helix, Platform Juniper, Platform Meridian, and Platform Quartz |
| `scope_p01_primary_12` | `operations/migration-notes` | Six-wave, 18-service `MIG-26Q3-*` migration through 2026-09-18 closeout |
| `scope_p01_secondary_01` | `desktop/working` | Ephemeral drafts and working copies; they may cite, but never replace, accepted primary-scope facts |
| `scope_p01_secondary_02` | `documents/reference` | Stable reference copies and synthetic standards snapshots with provenance back to a primary subject |
| `scope_p01_secondary_03` | `downloads/inbox` | Unreviewed incoming synthetic material; status remains inbox until another artifact records acceptance |
| `scope_p01_secondary_04` | `downloads/exports` | Timestamped synthetic tool exports whose rows reconcile to their base scope facts |
| `scope_p01_secondary_05` | `cloud/my-files` | Individual drafts and work-in-progress copies with explicit draft status |
| `scope_p01_secondary_06` | `cloud/team-shared` | Accepted team collaboration copies; role ownership and reviewed status are explicit |
| `scope_p01_secondary_07` | `mail/recent` | Synthetic role-to-role mail using `.invalid` addresses and `MSG-2026Q3-*` IDs |
| `scope_p01_secondary_08` | `archive/closed` | Closed immutable snapshots whose project IDs, facts, and close dates remain consistent with their source subject |

## Allocation invariants

- The complete assignment contains exactly 11,800 Full additions: every one of
  the 12,000 canonical p01 sources except the 200 frozen M1 reservations.
- The accepted pilot's 12 artifact rows, IDs, paths, dependencies, content-spine
  IDs, skill routing, and batch ID are retained unchanged.
- Remaining artifact IDs and paths are deterministic functions of canonical
  source IDs and inventory extensions. No extension is inferred from a family.
- Ordinary scope-local batches contain 100--500 rows. Smaller high-cost image,
  PDF, DOCX, XLSX, and PPTX QA boundaries are allowed.
- The ledger allocates physical files only. It does not authorize production in
  this allocation-only task and does not establish Kio readiness.
