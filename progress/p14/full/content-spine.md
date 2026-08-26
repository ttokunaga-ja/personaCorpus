# p14 Full content spine

All artifacts are synthetic finance operations records for `org.cedar-works`.
They consistently describe the FY2026 Q1 close (`period.fy2026-q1`), the
2026-03 close (`date.2026-03-31`), and the reconciled cash variance
`measure.cash-variance-jpy-12400000` (JPY 12,400,000).

## Shared facts

- `org.cedar-works` — synthetic manufacturer and operating company.
- `project.fy2026-close` — Q1 close and controllership evidence program.
- `measure.cash-variance-jpy-12400000` — the common variance measure.
- `control.dual-review` — preparer and reviewer separation.
- `control.synthetic-only` — no real account, vendor, employee, tax, or bank data.
- `term.no-kio-claim` — no indexing, history, chunk, search, or performance claim.

## Scope relationships

Every Full assignment row carries all five shared facts above and exactly one
scope-local relationship ID. The mapping is fixed and deterministic:

| Scope | Local relationship | Meaning in the finance corpus |
| --- | --- | --- |
| `p14-primary-01`–`p14-primary-03` (`finance/close/2026/q1/*`) | `rel.close-to-forecast` | monthly close evidence reconciles to the Q1 outlook |
| `p14-primary-04`–`p14-primary-06` (`budget/*`, `forecasts/*`) | `rel.assumption-to-outlook` | approved assumptions explain the budget/forecast outlook |
| `p14-primary-07`–`p14-primary-08` (`invoices/*`, `expenses/*`) | `rel.transaction-to-control` | synthetic transactions demonstrate the dual-review control |
| `p14-primary-09`–`p14-primary-10` (`audit/*`, `board/*`) | `rel.evidence-to-decision` | audit evidence supports a recorded finance decision |
| `p14-primary-11`–`p14-primary-12` (`models/*`, `erp/*`) | `rel.model-to-export` | operating-model outputs are reconciled to synthetic ERP exports |
| `p14-secondary-01`–`p14-secondary-08` (desktop/document/download/cloud/mail/archive) | `rel.reference-to-record` | user-facing references point to the relevant synthetic record |

The six relationship IDs are semantic anchors, not extra artifact dependencies;
`dependencies` in the master remains the authoritative same-scope production
ordering.

## Isolated M1 PDF normalization

The frozen raw M1 ledger retains three `pdf_scan` declarations whose actual
files contain searchable text. `bin/p14-full-ledger` snapshots all six fixed
inputs, verifies their raw SHA-256 values, then changes only those three
in-memory/snapshot family values to `pdf_text` before invoking the shared
validator. Therefore the reservation header's M1 digest describes this
normalized validation view, while the raw M1 ledger digest remains the fixed
input pin. This expected distinction is not a reservation or M1 rewrite.
