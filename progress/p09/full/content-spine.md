# p09 Full content spine — Beacon Field Research

This is a synthetic, token-free factual contract for p09 Full artifacts. It
extends the accepted M1 environment without altering its files. All studies,
participants, recordings, organisations, documents, and measurements below are
invented test data; no contact details, biometric data, credentials, protected
health information, or real research records may appear in an artifact.

## Shared facts

- `org.beacon-field-research` — Beacon Field Research, a fictional UX research
  practice using only synthetic study data.
- `program.study-alpha` — Study Alpha, a fictional accessibility-focused
  product research programme.
- `program.study-beta` — Study Beta, a fictional comparative prototype study.
- `session.ux-184` — synthetic moderated session UX-184.
- `date.2026-07-13` — recorder/research-drive synthetic export date.
- `measure.sample-n24` — synthetic sample target `n=24`; never represent it as
  a real participant population.
- `measure.task-completion-83` — synthetic task-completion reference of 83%.
- `measure.confidence-08` — synthetic evidence confidence reference of 0.8.
- `term.synthetic-consent` — a template/example only, never a real consent.
- `term.redacted-transcript` — synthetic, non-identifying transcript content.
- `term.accessible-prototype` — accessible interaction and content review.
- `term.research-ops` — synthetic planning, recruitment simulation, and
  research operations.
- `rel.alpha-evidence` — Study Alpha planning, sessions, findings, and design
  evidence use the same date, session alias, and synthetic measures.
- `rel.beta-comparison` — Study Beta compares fictional prototype conditions
  using the same safe measurement vocabulary.
- `rel.design-handoff` — prototype specifications, exports, and journey maps
  must agree on interaction states and accessibility labels.
- `rel.synthetic-archive` — closeout, exports, mail, and archive material is a
  synthetic copy/reference relationship, not real correspondence.
- `rel.scan-source` — every scan PDF is derived from a scope-local synthetic
  ImageGen source image or a deterministic scratch-only source specified by its
  allocation row; it must have no text layer.
- `term.no-kio-claim` — these records make no Kio index, history, chunk,
  search, performance, or readiness claim.

## Scope relationships

| Scope family | Contract IDs |
| --- | --- |
| Study Alpha plans, transcripts, findings | `program.study-alpha`, `session.ux-184`, `rel.alpha-evidence` |
| Study Beta plans, transcripts, findings | `program.study-beta`, `session.ux-184`, `rel.beta-comparison` |
| Surveys and prototype specifications | `measure.sample-n24`, `measure.task-completion-83`, `term.accessible-prototype` |
| Figma exports and journey maps | `term.accessible-prototype`, `rel.design-handoff`, `measure.task-completion-83` |
| Transcript sidecars and synthetic consent | `term.redacted-transcript`, `term.synthetic-consent`, `session.ux-184` |
| Desktop, reference, downloads, cloud, mail, archive | `term.research-ops`, `rel.synthetic-archive`, `date.2026-07-13` |

## Allocation coverage contract

The Full master distributes facts deliberately and verifies the following
minimum references before it can be published. This is an allocation contract,
not a requirement to put every measure into every artifact.

| Scope subjects | Required semantic coverage |
| --- | --- |
| Each Study Alpha and Study Beta plans/transcripts/findings scope | Its study programme and relationship in every row; each of `n=24`, `83%`, and `0.8` in at least 20 rows |
| Surveys/results | `n=24`, `83%`, and `0.8` in every row; accessible-prototype contract present |
| Prototype specs, Figma exports, journey maps | accessible-prototype and design-handoff in every row; `83%` and `0.8` in every row |
| Transcript sidecars and synthetic consent | transcript, consent, UX-184, and `0.8` in every row |
| Desktop/archive/operations scopes | research-operations/archive relationship in every row and one stable distributed shared measure |

## Cross-format rules

Structured or CSV/TSV facts are the source of truth for dependent XLSX, DOCX,
searchable PDF, and PPTX artifacts in the same scope. ImageGen images precede
their derived scan PDFs. All documents that refer to a session use
`session.ux-184`; all synthetic sample figures use `measure.sample-n24`; and
all content retains `term.no-kio-claim`. Study Alpha/Beta material must use
only the matching programme/relationship; survey and design formats reuse the
same `n=24`, `83%`, and `0.8` meanings rather than inventing a competing value.
