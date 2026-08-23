# Beta compatibility notes
Aster Loop 株式会社 | Product Alpha R7 | release 2026-07-13 | p95 184 ms | ADR-042

| Surface | Direction | Impact | Detection | Mitigation | Rollback | Evidence |
|---|---|---|---|---|---|---|
| API | Alpha v1 -> Beta additive | optional projection fields | transformer validation | map known fields | keep Alpha read path | dry-run result |
| Event | v1 dual-read | ordering/duplicate risk | checkpoint ledger | stable event id | disable Beta consumer | consumer lag |
| Storage | source -> derived Beta | count divergence | record-count verifier | rebuildable projection | discard derived projection | count report |
| Config | synthetic flags -> Beta | cohort drift | desired vs observed routing | typed validation | freeze flag | flag snapshot |
| Operations | p95 184 ms SLO | release latency | p50/p95 and lag | staged windows | route to Alpha | dashboard export |

Compatibility is intentionally one-way for preview: Alpha remains authoritative and Beta is derived.
