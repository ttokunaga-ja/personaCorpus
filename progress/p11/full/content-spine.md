# p11 Full content spine

- Persona: `p11` / Account Executive
- Contract: `p11-content-spine-v1`
- State: frozen for `p11-full-master-v1`
- Content language: en-US; dates use ISO 8601 and times use UTC unless labelled otherwise.
- Synthetic-use notice: every organization, account, role, contact alias, commercial term, identifier, and measurement below is fictional.
- Safety boundary: no credentials, secrets, real customer data, real personal data, payment data, private contracts, or Kio readiness claim may occur in derived artifacts.

This contract preserves the accepted M1 anchors **Northstar Cloud**, **Account Alpha**,
**2026-07-17**, **USD 248,000**, and **OPP-260717-042**. Files may select only the
facts relevant to their purpose, but may not contradict this contract.

## Stable commercial facts

| Spine ID | Frozen fact |
| --- | --- |
| `org.northstar` | Northstar Cloud is a fictional business software provider. |
| `account.alpha` | Account Alpha is a synthetic enterprise renewal account. |
| `account.beta` | Account Beta is a synthetic expansion account. |
| `opportunity.opp-260717-042` | OPP-260717-042 is Account Alpha's FY26 renewal opportunity, valued at USD 248,000 before approved discounting. |
| `team.revenue` | Revenue Operations, Deal Desk, and Account Management use role aliases only. |
| `measure.renewal-rate` | Account Alpha renewal confidence is 92%; this is a planning measure, not a revenue forecast guarantee. |
| `measure.coverage` | Pipeline coverage for the synthetic July review is 3.1x against the stated target. |
| `term.approved-discount` | The approved discount is 8.0% and must retain its percentage unit. |
| `term.no-kio-claim` | Corpus production and QA establish no Kio indexing, history, chunks, search quality, or performance claim. |
| `date.2026-07-17` | Weekly commercial review date. |
| `date.2026-07-20` | Follow-up and approval reconciliation date. |
| `period.2026-q3` | Synthetic Q3 FY26 commercial planning period. |

Permitted role aliases use only the `.invalid` domain: `ae@northstar.invalid`,
`deal-desk@northstar.invalid`, and `revops@northstar.invalid`.

## Scope reuse map

Every scope below uses its stable scope subject plus `org.northstar`,
`opportunity.opp-260717-042`, and `term.no-kio-claim`. The listed commercial
subject/measure is mandatory for that scope's Full rows; it is deliberately
limited to the facts meaningful for that scope rather than citing every metric
in every artifact. Derived Office, PDF, presentation, image, and scan artifacts
consume same-scope foundation facts.

| Scope ID | Stable subject | Commercial subject / measure |
| --- | --- | --- |
| `p11-primary-01` | `scope.p11-primary-01` | `account.alpha`, `measure.renewal-rate` |
| `p11-primary-02` | `scope.p11-primary-02` | `account.alpha`, `measure.renewal-rate` |
| `p11-primary-03` | `scope.p11-primary-03` | `account.alpha`, `term.approved-discount` |
| `p11-primary-04` | `scope.p11-primary-04` | `account.beta`, `measure.coverage` |
| `p11-primary-05` | `scope.p11-primary-05` | `account.beta`, `measure.coverage` |
| `p11-primary-06` | `scope.p11-primary-06` | `account.beta`, `measure.coverage` |
| `p11-primary-07` | `scope.p11-primary-07` | `account.alpha`, `account.beta`, `measure.coverage` |
| `p11-primary-08` | `scope.p11-primary-08` | `account.beta`, `measure.coverage` |
| `p11-primary-09` | `scope.p11-primary-09` | `account.alpha`, `term.approved-discount` |
| `p11-primary-10` | `scope.p11-primary-10` | `account.alpha`, `term.approved-discount` |
| `p11-primary-11` | `scope.p11-primary-11` | `account.alpha`, `measure.renewal-rate`, `term.approved-discount` |
| `p11-primary-12` | `scope.p11-primary-12` | `account.alpha`, `measure.renewal-rate` |
| `p11-secondary-01` | `scope.p11-secondary-01` | `account.alpha`, `measure.renewal-rate` |
| `p11-secondary-02` | `scope.p11-secondary-02` | `account.alpha`, `measure.renewal-rate` |
| `p11-secondary-03` | `scope.p11-secondary-03` | `account.beta`, `measure.coverage` |
| `p11-secondary-04` | `scope.p11-secondary-04` | `account.alpha`, `term.approved-discount` |
| `p11-secondary-05` | `scope.p11-secondary-05` | `account.beta`, `measure.coverage` |
| `p11-secondary-06` | `scope.p11-secondary-06` | `account.alpha`, `measure.renewal-rate` |
| `p11-secondary-07` | `scope.p11-secondary-07` | `account.alpha`, `measure.renewal-rate` |
| `p11-secondary-08` | `scope.p11-secondary-08` | `account.alpha`, `term.approved-discount` |

## Allocation and dependency contract

1. `structured_text`, `csv_tsv`, `code`, `txt_log`, and `domain_binary` rows are
   same-scope foundation material and precede all derived rows.
2. XLSX, DOCX, searchable PDF, and PPTX rows depend on a same-scope foundation row.
3. Images use ImageGen from same-scope foundation facts. Scan PDFs use a same-scope
   final image when available; otherwise the row carries an explicit scratch-only
   ImageGen source object. Scratch sources are never final artifacts or master rows.
4. All dependencies point to an earlier batch within the same Rust scope. No row
   crosses scope boundaries and no Full row modifies an M1 artifact.
