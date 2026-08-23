# p01 Full pilot content spine

This is the fixed synthetic-facts contract for batch
`p01-primary-01-full-pilot-b001`. It is operational input, not canonical Rust
plan evidence and not a Kio/history/chunk claim.

## Identity and decision anchors

| ID | Fixed fact |
| --- | --- |
| `org` | Aster Loop株式会社（synthetic） |
| `product` | Product Alpha R7 |
| `release_date` | 2026-07-13 |
| `decision` | ADR-042 keeps the asynchronous release gate |
| `review_date` | 2026-07-20 |
| `evidence_window` | 2026-07-13 through 2026-07-20, inclusive |

All twelve artifacts must use these exact facts when the subject is mentioned.
Do not invent a second company, product version, decision ID, release date, or
review disposition.

## Shared measurements

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

Do not present the rounded percentages as independent raw counts. XLSX formulas,
CSV rows, JSON values, PDF tables, DOCX prose, slide charts, and image labels
must reconcile to the same numerator and denominator.

## Latency stages

| Order | Stage | p95 contribution |
| ---: | --- | ---: |
| 1 | Request intake | 28 ms |
| 2 | Edge API | 32 ms |
| 3 | Release Coordinator | 46 ms |
| 4 | Decision Store | 24 ms |
| 5 | Event Bus | 18 ms |
| 6 | Observability confirmation | 36 ms |
|  | **Total** | **184 ms** |

Visuals must preserve this order and show the approval boundary between Release
Coordinator and Decision Store. Event Bus delivery occurs only after the gate
decision. Observability confirmation closes the measured path.

## Artifact relationships

- `alpha-r7-observability-snapshot.json` and
  `release-gate-slo-evidence-2026-q3.csv` are the base factual records.
- The XLSX workbook calculates success, approval, failure, and latency totals
  from typed cells; it must not store formula results as unexplained constants.
- DOCX and searchable PDFs explain the same evidence for different audiences.
- The PPTX uses the same six-stage latency visual and review disposition.
- `release-gate-observability-map.png` is a clean operational topology.
- `release-gate-whiteboard-capture.png` is a realistic synthetic photographed
  whiteboard. `release-gate-whiteboard-scan.pdf` wraps that exact image as an
  image-only scan PDF without an accidental searchable text layer.

Synthetic role labels may include Release Coordinator, Platform Engineer,
Observability Lead, and Decision Owner. Do not include real names, contacts,
customers, credentials, secrets, or copied proprietary material.
