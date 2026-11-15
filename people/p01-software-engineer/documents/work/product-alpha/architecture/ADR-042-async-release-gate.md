---
id: ADR-042
status: accepted
date: 2026-07-13
---
# ADR-042: Asynchronous release gate
## Context
Aster Loop 株式会社 ships Product Alpha R7 on 2026-07-13. The R7 synthetic release cohort measured p95 184 ms.
## Decision
Use a durable asynchronous release coordinator with typed transitions and idempotency keys.
## Alternatives
Synchronous gate keeps tail coupling. Client retry shifts ownership. Fire-and-forget lacks a durable boundary.
## Consequences
Clients tolerate pending state; timeout is never approval.
## Rollout
Shadow, then 10%, 50%, 100%. Stop when p95 exceeds 184 ms for 10 minutes, admission errors exceed 0.2%, or schema compatibility fails.
## p95 evidence
28 edge + 32 validation + 46 admission + 24 publish + 18 observability + 36 reserve = 184 ms.
## Sign-off
2026-07-13 synthetic role sign-off: Software Engineering, Release Engineering, Platform accepted.
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042