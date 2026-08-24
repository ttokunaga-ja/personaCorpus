# MIG-26Q3-W05 migration field note: observability

**Record:** p01-src-003154
**Program:** Aster Loop migration program
**Review window:** 2026-09-13T11:00:00+09:00

## Operating decision

Wave 5 moves media-worker, reporting-api, partner-bridge through a controlled cutover. observability advances only after replay age is at or below the 5m recovery-point objective and a restore rehearsal fits the 45m recovery-time objective. Production cutover is 2026-09-14; closeout review is 2026-09-18.

## Checkpoints

| Checkpoint | Evidence | Owner role | Result rule |
| --- | --- | --- | --- |
| Snapshot | export digest fd0ea563b4af | Migration Lead | source and target counts agree |
| Replay | cursor age | Platform Engineer | no skipped sequence range |
| Switch | routing acknowledgement | Release Coordinator | one active writer path |
| Restore | bounded rehearsal | Observability Lead | elapsed time at or below 45m |

This synthetic record distinguishes scheduled migration work from an incident.
