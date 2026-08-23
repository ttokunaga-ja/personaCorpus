# Product Alpha R7 post-launch architecture delta

**Aster Loop株式会社（synthetic）** · Release 2026-07-13 · Evidence 2026-07-13..2026-07-20

## Decision retained

The 2026-07-20 review records **keep ADR-042**. The asynchronous release gate remains the control point: approval occurs at the boundary between Release Coordinator and Decision Store, and Event Bus delivery begins only after the Decision Store gate decision.

## Observed path

| Ordered stage | p95 contribution |
| --- | ---: |
| Request intake | 28 ms |
| Edge API | 32 ms |
| Release Coordinator | 46 ms |
| Decision Store | 24 ms |
| Event Bus | 18 ms |
| Observability confirmation | 36 ms |
| **End-to-end** | **184 ms** |

Across 84,200 synthetic requests, 84,149 completed successfully (99.94% rounded); 51 were unsuccessful. Gate evidence sampled 9,000 decisions: 8,883 approved (98.7%) and 117 non-approved.

## Resilience follow-up

The controlled rollback drill on 2026-07-17 recovered in 11m42s (702 seconds). Observability confirmation closes the measured path; no production incident is implied.
