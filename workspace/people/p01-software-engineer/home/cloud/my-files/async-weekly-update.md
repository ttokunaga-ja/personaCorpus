# Async weekly update - 2026-07-13

**Aster Loop株式会社 | Product Alpha R7 | ADR-042**

## Outcomes

The async release gate contract, event lifecycle, and status fixtures are aligned. The control-plane target remains p95 184 ms; execution remains outside that timing boundary.

## Metrics and decisions

Synthetic staging fixtures use stable request IDs and report state transitions. ADR-042 remains accepted: requests await approval before a worker starts. No release execution is authorized by a preview flag.

## Risks and next work

Approval delay and duplicate event delivery remain monitored risks. Next work is to run compatibility fixtures, review p95 evidence, and summarize the 2026-07-13 release window for the Platform Release Owner.
