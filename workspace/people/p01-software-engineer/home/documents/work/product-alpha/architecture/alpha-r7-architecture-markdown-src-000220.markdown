# Product Alpha R7 architecture record ARCH-A7-000220

## Decision context

Aster Loop Product Engineering reviewed `svc-observability` on 2026-07-26 under ADR-022. The asynchronous release gate remains in effect; its approval boundary is between Release Coordinator and Decision Store.

## Evidence

| Measure | Value | Interpretation |
| --- | ---: | --- |
| Observed p95 | 187 ms | bounded architecture observation |
| Alpha SLO ceiling | 200 ms | release target |
| Release | Product Alpha R7 | released 2026-07-13 |

## Design note

`svc-observability` emits a typed architecture event only after the gate decision. Observability confirmation closes the measured path; this entry is `planned`. This is synthetic engineering record `p01-src-000220`.
