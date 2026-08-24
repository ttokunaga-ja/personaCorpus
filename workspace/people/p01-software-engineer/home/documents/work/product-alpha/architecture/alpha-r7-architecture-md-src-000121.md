# Product Alpha R7 architecture record ARCH-A7-000121

## Decision context

Aster Loop Product Engineering reviewed `svc-release-coordinator` on 2026-09-03 under ADR-122. The asynchronous release gate remains in effect; its approval boundary is between Release Coordinator and Decision Store.

## Evidence

| Measure | Value | Interpretation |
| --- | ---: | --- |
| Observed p95 | 183 ms | bounded architecture observation |
| Alpha SLO ceiling | 200 ms | release target |
| Release | Product Alpha R7 | released 2026-07-13 |

## Design note

`svc-release-coordinator` emits a typed architecture event only after the gate decision. Observability confirmation closes the measured path; this entry is `reviewed`. This is synthetic engineering record `p01-src-000121`.
