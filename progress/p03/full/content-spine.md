# p03 Full content spine

- Persona: `p03` / Security GRC Analyst
- Contract: `p03-content-spine-v1`
- State: frozen for `p03-full-master-v1`
- Content language: ja-JP and en-US are permitted; timestamps use UTC unless explicitly labelled otherwise
- Synthetic-use notice: every organization, role, service, control, evidence item, address, identifier, and measurement below is fictional
- Safety boundary: no exploit instructions, live targets, credentials, secrets, real PII/PHI, customer records, incident data, or private source material belongs in any derived artifact

This factual contract applies to all p03 scopes. It preserves the accepted M1
anchors **Cobalt Harbor**, **Aegis Control AC-27**, **2026-07-13**, **evidence set
184**, and **RISK-031**. A derived artifact may show only relevant facts, but may
not contradict this contract.

## Stable entity and governance registry

| Spine ID | Frozen fact |
| --- | --- |
| `org.cobalt-harbor` | Cobalt Harbor Systems (CHS) is a fictional organization operating a managed logistics and harbor-operations platform. |
| `program.aegis` | Aegis Assurance is CHS's fictional control-assurance program, using a synthetic control catalogue and defensive evidence only. |
| `control.ac-27` | Aegis Control AC-27 requires evidence-backed quarterly access review, approval, retention, and exception disposition. |
| `team.grc` | Security GRC owns the AC-27 evidence register, risk treatment tracking, and audit-request coordination. |
| `team.soc` | Security Operations owns bounded SIEM exports, detection review evidence, and incident evidence-chain handoff. |
| `team.privacy` | Privacy Assurance owns privacy-risk assessments and data-use reviews. |
| `team.vendor-risk` | Third-Party Assurance owns supplier questionnaires, assurance follow-up, and exception evidence. |
| `env.cobalt-prod` | `cobalt-prod` is a fictional managed production environment represented only by synthetic summaries and safe documentation IDs. |
| `system.aegis-register` | The Aegis register holds control, evidence, remediation, and disposition IDs; it has no real operational access. |
| `system.sentinel-export` | Sentinel Export is a fictional SIEM export service; its records are minimized, synthetic, and non-routable. |
| `dataset.evidence-184` | Evidence set 184 is the bounded July 2026 AC-27 review package, containing only synthetic evidence IDs and checksums. |

Permitted role aliases are `grc-reviewer@cobalt.invalid`, `soc-duty@cobalt.invalid`,
`privacy-review@cobalt.invalid`, and `vendor-assurance@cobalt.invalid`.
The `.invalid` domain is deliberate and must not be replaced.

## Date, case, and decision registry

| Spine ID | Frozen fact |
| --- | --- |
| `date.2026-07-13` | Primary review date. Evidence intake opens 08:30 UTC; AC-27 review closes at 16:45 UTC. |
| `date.2026-07-14` | Evidence normalization, checksum confirmation, and action-owner acknowledgement. |
| `date.2026-07-15` | Control owner validates the remediation plan and retention disposition. |
| `date.2026-07-20` | Seven-day follow-up validates closure evidence and residual-risk status. |
| `period.2026-q3` | Governance planning period from 2026-07-01 through 2026-09-30. |
| `risk.risk-031` | RISK-031: incomplete quarterly privileged-access review evidence could prevent AC-27 operating-effectiveness confirmation. Inherent High; residual Medium after bounded remediation. |
| `case.case-260713-184` | CASE-260713-184 is the synthetic evidence intake case for evidence set 184, not a real incident. |
| `audit.ar-260713-027` | AR-260713-027 is a fictional audit request for AC-27 review material. |
| `exception.exc-260713-006` | EXC-260713-006 records one synthetic delayed reviewer attestation; compensating review completes on 2026-07-14. |
| `action.act-260713-011` | Publish an AC-27 reviewer-completion register; completed 2026-07-15. |
| `action.act-260713-012` | Add evidence-set checksum reconciliation to intake; completed 2026-07-20. |
| `action.act-260713-013` | Confirm supplier subprocessor attestation mapping; due 2026-07-31. |
| `decision.dec-260713-004` | Retain evidence set 184 under the synthetic GRC retention schedule; no live-source data is included. |

### CASE-260713-184 canonical timeline

| UTC | Event |
| --- | --- |
| 08:30 | GRC opens bounded AC-27 evidence intake for evidence set 184. |
| 09:15 | SOC posts a minimized Sentinel Export summary and synthetic checksum manifest. |
| 10:20 | GRC flags missing reviewer attestation as RISK-031 and opens EXC-260713-006. |
| 11:45 | Privacy and Vendor Risk confirm scope boundaries and no restricted data in the packet. |
| 13:30 | Control owner supplies a synthetic completion register and remediation rationale. |
| 15:10 | GRC verifies evidence IDs, date ranges, and exception linkage. |
| 16:45 | AC-27 review closes with residual Medium and two completed actions. |
| 17:15 | Follow-up package is queued for the 2026-07-20 closure review. |

The causal relationship is fixed: incomplete review evidence creates the AC-27
assurance gap; bounded evidence normalization and compensating review reduce, but
do not erase, the residual governance risk. No file may imply an intrusion,
compromise, exploit, or access to a real system.

## Evidence, measure, and control registry

| Spine ID | Frozen fact |
| --- | --- |
| `evidence.set-184` | Evidence set 184 has 24 synthetic items: 12 access-review attestations, 6 approval records, 4 checksum manifests, and 2 exception records. |
| `evidence.ev-184-01` | EV-184-01 is the AC-27 review population summary: 48 synthetic privileged roles, 48 reviewed, 47 timely attestations. |
| `evidence.ev-184-07` | EV-184-07 is the Sentinel Export minimization statement, reporting only event categories and counts. |
| `evidence.ev-184-19` | EV-184-19 is the evidence-set 184 checksum reconciliation record. |
| `measure.review-population` | Review population: 48 synthetic privileged-role records; not a workforce count. |
| `measure.timely-attestation` | Timely attestation rate before compensating review: 97.9% (47 of 48). |
| `measure.final-completion` | Final completion rate after compensating review: 100.0% (48 of 48). |
| `measure.evidence-completeness` | Evidence-set 184 completeness: 24 of 24 required synthetic items verified. |
| `measure.exception-age` | EXC-260713-006 age at closure: 1 calendar day. |
| `measure.residual-rating` | RISK-031 residual rating is Medium after the compensating review and follow-up actions. |
| `control.retention-18m` | Synthetic evidence retention minimum is 18 months; this is a corpus fact, not legal advice. |
| `control.approval-dual` | AC-27 requires preparer and reviewer role separation for each exception disposition. |

Exact values must be preserved in structured/tabular sources. Presentation files may
round only when they retain direction, denominator, and unit. Workbooks, DOCX,
PDF, decks, images, and scan PDFs cite scope-local foundation facts through ledger
dependencies.

## Terminology registry

| Spine ID | Frozen usage |
| --- | --- |
| `term.control-evidence` | Bounded synthetic record supporting a control design or operating-effectiveness assertion. |
| `term.compensating-review` | Documented review mitigating a missed or delayed primary attestation without rewriting history. |
| `term.residual-risk` | Remaining risk after agreed treatment, distinct from inherent risk and incident severity. |
| `term.evidence-chain` | Stable IDs connecting normalized source facts, derived reports, rendered files, and disposition records. |
| `term.minimized-export` | Export limited to synthetic categories, counts, and timestamps; it has no real raw events or routable addresses. |
| `term.exception` | Time-bounded deviation with owner, rationale, compensating action, and closure evidence. |
| `term.no-kio-claim` | Corpus production and QA establish no Kio indexing, history, chunks, search quality, or performance claim. |

## Cross-format relationship registry

| Spine ID | Required relationship |
| --- | --- |
| `rel.ac27.evidence-184` | Structured/CSV/log evidence facts precede AC-27 workbooks, review DOCX, searchable PDFs, and security-review decks. |
| `rel.risk-031.treatment` | RISK-031, EXC-260713-006, and action IDs retain the same status and residual rating in risk, privacy, vendor, mail, and archive scopes. |
| `rel.siem.minimization` | Sentinel Export category/count facts precede SIEM summaries, detection reviews, diagrams, and rendered evidence. |
| `rel.audit.request-027` | AR-260713-027 connects request, response, approval, and retention documents without implying live audit access. |
| `rel.privacy.vendor` | Privacy and supplier assurance reuse AC-27 terminology while remaining distinct from the privileged-review population. |
| `rel.case.timeline` | CASE-260713-184 timeline events and evidence IDs remain consistent across reports, meeting notes, mail, desktop, and archive records. |
| `rel.scan.source` | Every scan-PDF uses a scope-local ImageGen source and local foundation evidence; scan files have no accidental text layer. When a scope has a canonical image assignment, that final image is the explicit ledger dependency. `p03-secondary-07` (`mail/recent`) has no canonical image source, so its five scan PDFs carry unique scratch-only ImageGen input objects under `scratch/p03/p03-secondary-07/<batch-id>/scan-sources/`. Each source is original-size inspected, legible, pixel/source-to-scan matched, and checked for a zero PDF text layer. It is never a final artifact, workspace file, master artifact row, or cross-scope dependency. |
| `rel.archive.closed` | Closed records preserve synthetic case/control IDs and final disposition, never real source data. |

## Scope reuse map

| Rust scope ID | Primary subject IDs | Relationship ID |
| --- | --- | --- |
| `p03-primary-01` | `control.ac-27`, `risk.risk-031`, `evidence.set-184` | `rel.ac27.evidence-184` |
| `p03-primary-02` | `case.case-260713-184`, `control.ac-27`, `term.compensating-review` | `rel.risk-031.treatment` |
| `p03-primary-03` | `risk.risk-031`, `exception.exc-260713-006`, `action.act-260713-011` | `rel.risk-031.treatment` |
| `p03-primary-04` | `case.case-260713-184`, `evidence.ev-184-07`, `term.evidence-chain` | `rel.case.timeline` |
| `p03-primary-05` | `control.ac-27`, `control.retention-18m`, `decision.dec-260713-004` | `rel.audit.request-027` |
| `p03-primary-06` | `evidence.set-184`, `evidence.ev-184-19`, `control.approval-dual` | `rel.ac27.evidence-184` |
| `p03-primary-07` | `audit.ar-260713-027`, `evidence.set-184`, `term.control-evidence` | `rel.audit.request-027` |
| `p03-primary-08` | `team.vendor-risk`, `risk.risk-031`, `action.act-260713-013` | `rel.privacy.vendor` |
| `p03-primary-09` | `system.sentinel-export`, `evidence.ev-184-07`, `term.minimized-export` | `rel.siem.minimization` |
| `p03-primary-10` | `team.soc`, `control.ac-27`, `measure.evidence-completeness` | `rel.siem.minimization` |
| `p03-primary-11` | `team.privacy`, `risk.risk-031`, `measure.residual-rating` | `rel.privacy.vendor` |
| `p03-primary-12` | `period.2026-q3`, `decision.dec-260713-004`, `action.act-260713-012` | `rel.case.timeline` |
| `p03-secondary-01` | `case.case-260713-184`, `term.evidence-chain`, `team.grc` | `rel.case.timeline` |
| `p03-secondary-02` | `control.ac-27`, `term.compensating-review`, `term.exception` | `rel.ac27.evidence-184` |
| `p03-secondary-03` | `audit.ar-260713-027`, `evidence.ev-184-19`, `term.minimized-export` | `rel.audit.request-027` |
| `p03-secondary-04` | `system.sentinel-export`, `measure.evidence-completeness`, `risk.risk-031` | `rel.siem.minimization` |
| `p03-secondary-05` | `team.grc`, `measure.timely-attestation`, `action.act-260713-011` | `rel.risk-031.treatment` |
| `p03-secondary-06` | `team.grc`, `team.soc`, `team.privacy` | `rel.case.timeline` |
| `p03-secondary-07` | `audit.ar-260713-027`, `exception.exc-260713-006`, `date.2026-07-13` | `rel.case.timeline` |
| `p03-secondary-08` | `case.case-260713-184`, `decision.dec-260713-004`, `date.2026-07-20` | `rel.archive.closed` |

## Allocation and dependency contract

1. Foundation rows are canonical `structured_text`, `csv_tsv`, `code`,
   `txt_log`, and `domain_binary` sources, assigned before visual and Office rows.
   `p03-secondary-08` has exactly 94 unreserved foundation rows; they form its
   distinct first `foundation-small` batch. This bounded exception does not mix
   visual rows and permits no same-batch dependencies; all dependent rows are in
   later batches.
2. Every `xlsx`, `docx`, `pdf_text`, and `pptx` assignment depends on
   scope-local structured or tabular foundation data.
3. Every `image` assignment uses ImageGen and depends on local foundation data.
   Every `pdf_scan` row uses a scope-local ImageGen source. Where the canonical
   scope has an image assignment, it is an explicit ledger dependency; where it
   does not (`p03-secondary-07` only), the batch creates a scratch-only ImageGen
   QA input and retains only its local foundation dependency in the master ledger.
   No dependency crosses a Rust scope boundary and scratch inputs are never final
   artifacts or master rows.
4. Markdown and HTML/EML rows complete a scope after foundation/visual batches
   without violating dependency order.
5. A dependent artifact is always in a strictly later batch than its
   prerequisites; no same-batch dependency or cycle is permitted.
6. This is allocation evidence only. `term.no-kio-claim` applies to every
   planned row and no row represents a created final artifact.
