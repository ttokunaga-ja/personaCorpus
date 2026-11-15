# Synthetic platform tier evaluation

**Aster Loop株式会社 | Product Alpha R7 | release 2026-07-13 | ADR-042**

This internal evaluation compares invented platform tiers only. Nimbus Tier is the lowest-latency control-plane option, Orbit Tier emphasizes operational automation, and Harbor Tier favors portable deployment packages. The Product Alpha R7 objective is p95 184 ms for control-plane create/status calls; asynchronous execution begins only after the ADR-042 approval event.

| Criterion | Nimbus Tier | Orbit Tier | Harbor Tier |
|---|---:|---:|---:|
| Latency fit | 92 | 84 | 78 |
| Operations fit | 76 | 94 | 81 |
| Portability fit | 72 | 79 | 96 |

Recommendation: select Nimbus Tier for the R7 control plane, while retaining Harbor-compatible adapter boundaries for exit flexibility. Orbit practices inform runbooks, not the initial runtime choice.
