# MIG-26Q3-W02 migration field note: catalog-index

**Record:** p01-src-003121
**Program:** Aster Loop migration program
**Review window:** 2026-09-10T10:00:00+09:00

## Operating decision

Wave 2 moves event-bus, observability, contract-registry through a controlled cutover. catalog-index advances only after replay age is at or below the 5m recovery-point objective and a restore rehearsal fits the 45m recovery-time objective. Production cutover is 2026-09-14; closeout review is 2026-09-18.

## Checkpoints

| Checkpoint | Evidence | Owner role | Result rule |
| --- | --- | --- | --- |
| Snapshot | export digest fe425a379a06 | Migration Lead | source and target counts agree |
| Replay | cursor age | Platform Engineer | no skipped sequence range |
| Switch | routing acknowledgement | Release Coordinator | one active writer path |
| Restore | bounded rehearsal | Observability Lead | elapsed time at or below 45m |

This synthetic record distinguishes scheduled migration work from an incident.
