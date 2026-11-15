# Product Alpha R7 architecture record ARCH-A7-000166

## Decision context

Aster Loop Product Engineering reviewed `svc-observability` on 2026-08-10 under ADR-167. The asynchronous release gate remains in effect; its approval boundary is between Release Coordinator and Decision Store.

## Evidence

| Measure | Value | Interpretation |
| --- | ---: | --- |
| Observed p95 | 190 ms | bounded architecture observation |
| Alpha SLO ceiling | 200 ms | release target |
| Release | Product Alpha R7 | released 2026-07-13 |

## Design note

`svc-observability` emits a typed architecture event only after the gate decision. Observability confirmation closes the measured path; this entry is `reviewed`. This is synthetic engineering record `p01-src-000166`.
