# Release gate resilience review

**Product Alpha R7** · Evidence window 2026-07-13..2026-07-20 · Review 2026-07-20

## Conclusion

**keep ADR-042**. The evidence supports retaining the asynchronous release gate while preserving a visible approval boundary between Release Coordinator and Decision Store.

## Evidence reconciliation

- Requests: 84,200; successful: 84,149 (99.94% rounded); unsuccessful: 51.
- Gate decisions: 9,000; approved: 8,883 (98.7%); non-approved: 117.
- End-to-end p95: Request intake 28 ms; Edge API 32 ms; Release Coordinator 46 ms; Decision Store 24 ms; Event Bus 18 ms; Observability confirmation 36 ms; total 184 ms.
- Rollback drill: 2026-07-17; controlled recovery 11m42s (702 seconds).

## Operational guardrails

Event Bus delivery is downstream of the Decision Store decision. Observability confirmation is the closing stage of the measured path. This review covers synthetic evidence only.
