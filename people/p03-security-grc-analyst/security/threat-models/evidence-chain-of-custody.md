# Evidence Chain of Custody Model

## Definition

Chain of custody is the documented sequence from evidence creation through intake, review,
storage, export, and retention. It supports reliability without claiming that a file alone proves
an operational fact.

## Required record

| Event | Required fields |
|---|---|
| Creation | source system class, owner, period |
| Intake | package identifier, manifest hash, classification |
| Review | reviewer role, conclusion, references |
| Transfer | approved recipient, reason, receipt |
| Retention | schedule, hold status, disposition event |

## Threat response

Missing links, unrecorded transfers, or hash mismatches are treated as evidence-quality findings.
The custodian preserves the available history, limits downstream reliance on the affected item,
and initiates a corrective-action assessment. The review is fact-focused and does not speculate
about unauthorized activity without supporting evidence.

## Quality metric

Measure the percentage of sampled items with all five record stages and no unresolved variance.
Trend the result by evidence source to prioritize process improvements.
