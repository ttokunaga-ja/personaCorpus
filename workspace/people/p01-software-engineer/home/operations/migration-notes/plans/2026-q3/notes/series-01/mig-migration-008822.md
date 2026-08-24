# MIG-26Q3-W03 migration field note: decision-store

**Record:** p01-src-008822
**Program:** Aster Loop migration program
**Review window:** 2026-09-11T15:00:00+09:00

## Operating decision

Wave 3 moves identity-sync, catalog-index, billing-ledger through a controlled cutover. decision-store advances only after replay age is at or below the 5m recovery-point objective and a restore rehearsal fits the 45m recovery-time objective. Production cutover is 2026-09-14; closeout review is 2026-09-18.

## Checkpoints

| Checkpoint | Evidence | Owner role | Result rule |
| --- | --- | --- | --- |
| Snapshot | export digest 805ee43d00f7 | Migration Lead | source and target counts agree |
| Replay | cursor age | Platform Engineer | no skipped sequence range |
| Switch | routing acknowledgement | Release Coordinator | one active writer path |
| Restore | bounded rehearsal | Observability Lead | elapsed time at or below 45m |

This synthetic record distinguishes scheduled migration work from an incident.
