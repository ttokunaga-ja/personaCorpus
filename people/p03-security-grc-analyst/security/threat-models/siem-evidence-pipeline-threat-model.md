# SIEM Evidence Pipeline Threat Model

## Context

This model covers defensive collection of normalized security events used as control evidence.
It addresses source accountability, parser quality, retention, and analyst access; it does not
describe intrusion methods or live investigation procedures.

## Boundaries and controls

Event sources send approved telemetry through a collection boundary. A parser adds a schema
version and source identifier. A quality gate measures timestamp coverage, field completeness,
and duplicate rate before evidence summaries become available to GRC reviewers.

| Risk | Preventive measure | Detection |
|---|---|---|
| Parser change drops a field | versioned schema approval | coverage trend alert |
| Source stops reporting | source inventory with expected cadence | missing-source report |
| Analyst alters raw evidence | read-only raw tier | hash verification |
| Summary overstates coverage | sample-to-source reconciliation | independent review |

## Governance

Parser changes require documented testing against synthetic samples and a rollback plan. The
control owner signs the monthly completeness statement, while the monitoring owner investigates
gaps. Evidence summaries cite the source period and query version so auditors can reproduce the
reported conclusion using approved tools.
