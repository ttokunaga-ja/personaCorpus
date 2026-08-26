# p15 Full content spine — Orchid People

## Scope and safety

This is a synthetic Windows People Operations corpus. Every candidate, employee,
manager, supplier, contact detail, compensation datum and assessment is fictional.
Artifacts must never represent a real application, résumé, employee record,
compensation record, credential, or protected health information. The corpus is
operationally useful only as synthetic defensive test data and makes no Kio
indexing, history, search, chunk, or performance claim (`term.no-kio-claim`).

## Stable organization and period

- Organization: `org.orchid-people` — Orchid People, a fictional People Ops
  organization.
- Hiring program: `program.requisition-alpha` and
  `program.requisition-beta`.
- Decision date: `date.2026-08-18`.
- Planning period: `period.2026-q3`.
- Hiring capacity: `measure.openings-3` — three synthetic openings.
- Compensation reference: `measure.jpy-band-7.2-8.4m` — JPY 7.2–8.4M, used
  only as a synthetic planning band.
- Review boundary: `control.synthetic-identity` and
  `control.minimized-people-data` require pseudonymous records and minimal
  fields in every format.

## Cross-format rules

All connected artifacts use the same organization, requisition IDs, date,
three-opening capacity and compensation band. Tabular or structured facts are
created before Office documents, searchable PDFs and presentations. ImageGen
source art is created before an image-only scan PDF. A scan PDF has no text
layer and preserves a verifiable pixel relationship with its scope-local source
image. Any temporary source image remains below `scratch/`, is not a final
artifact, and is not counted by the Full ledger.

## Scope profiles

| Scope | Stable subjects | Relationship | Content focus |
| --- | --- | --- | --- |
| p15-primary-01 | `program.requisition-alpha`, `control.synthetic-identity` | `rel.alpha-candidate-pipeline` | synthetic candidate flow and consent metadata |
| p15-primary-02 | `program.requisition-alpha`, `measure.openings-3` | `rel.alpha-interview-loop` | interview calibration and decision records |
| p15-primary-03 | `program.requisition-beta`, `control.synthetic-identity` | `rel.beta-candidate-pipeline` | synthetic beta candidate comparison |
| p15-primary-04 | `program.requisition-beta`, `measure.openings-3` | `rel.beta-interview-loop` | beta interview review and evidence |
| p15-primary-05 | `program.requisition-alpha`, `measure.jpy-band-7.2-8.4m` | `rel.offer-governance` | offer approvals and planning controls |
| p15-primary-06 | `control.minimized-people-data`, `period.2026-q3` | `rel.people-policy` | policy acknowledgement and retention guidance |
| p15-primary-07 | `measure.openings-3`, `measure.jpy-band-7.2-8.4m` | `rel.headcount-plan` | synthetic headcount and compensation planning |
| p15-primary-08 | `control.synthetic-identity`, `period.2026-q3` | `rel.performance-synthetic` | synthetic performance process material |
| p15-primary-09 | `control.minimized-people-data`, `period.2026-q3` | `rel.survey-synthetic` | aggregate synthetic survey analysis |
| p15-primary-10 | `program.requisition-alpha`, `period.2026-q3` | `rel.training-readiness` | interviewer learning and calibration |
| p15-primary-11 | `measure.jpy-band-7.2-8.4m`, `control.minimized-people-data` | `rel.compensation-governance` | band governance and scenario review |
| p15-primary-12 | `control.minimized-people-data`, `date.2026-08-18` | `rel.retention-compliance` | retention and disposition records |
| p15-secondary-01 | `program.requisition-alpha`, `date.2026-08-18` | `rel.desktop-working` | private working drafts |
| p15-secondary-02 | `control.synthetic-identity`, `control.minimized-people-data` | `rel.reference-library` | reusable reference guidance |
| p15-secondary-03 | `program.requisition-beta`, `date.2026-08-18` | `rel.inbox-intake` | controlled incoming synthetic records |
| p15-secondary-04 | `measure.openings-3`, `period.2026-q3` | `rel.export-review` | export and reconciliation material |
| p15-secondary-05 | `program.requisition-alpha`, `measure.jpy-band-7.2-8.4m` | `rel.personal-workspace` | individual planning copies |
| p15-secondary-06 | `control.minimized-people-data`, `period.2026-q3` | `rel.shared-people-ops` | shared operational handoffs |
| p15-secondary-07 | `program.requisition-beta`, `date.2026-08-18` | `rel.recruiting-mail` | synthetic recruiting communications |
| p15-secondary-08 | `control.minimized-people-data`, `period.2026-q3` | `rel.closed-archive` | closed synthetic records and retention indexes |

## Required identifiers and row mapping

Every Full row uses the exact unique identifier set for its scope: the stable
subjects and relationship in the table above, plus `org.orchid-people`,
`date.2026-08-18`, `period.2026-q3`, and `term.no-kio-claim`; an identifier
already listed as a stable subject is included only once. This makes the Alpha
and Beta requisitions mutually exclusive, keeps the shared People Ops scopes
free of requisition leakage, and retains the decision-date/Q3 planning context
without encoding personal data. `bin/p15-full-content-spine` is the frozen
deterministic mapping and fail-closed validation gate for the unaccepted master.
