# MIG-26Q3-W03 migration field note: billing-ledger

**Record:** p01-src-001952
**Program:** Aster Loop migration program
**Review window:** 2026-09-11T09:00:00+09:00

## Operating decision

Wave 3 moves identity-sync, catalog-index, billing-ledger through a controlled cutover. billing-ledger advances only after replay age is at or below the 5m recovery-point objective and a restore rehearsal fits the 45m recovery-time objective. Production cutover is 2026-09-14; closeout review is 2026-09-18.

## Checkpoints

| Checkpoint | Evidence | Owner role | Result rule |
| --- | --- | --- | --- |
| Snapshot | export digest e7dd99c2ad0b | Migration Lead | source and target counts agree |
| Replay | cursor age | Platform Engineer | no skipped sequence range |
| Switch | routing acknowledgement | Release Coordinator | one active writer path |
| Restore | bounded rehearsal | Observability Lead | elapsed time at or below 45m |

This synthetic record distinguishes scheduled migration work from an incident.
