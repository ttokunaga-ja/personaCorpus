# Product Alpha R7 architecture review ARCH-A7-010335

Aster Loop Product Engineering recorded `svc-event-bus` on 2026-09-05 under ADR-187. The asynchronous release gate preserves the approval boundary from Release Coordinator to Decision Store.

| Measure | Value |
| --- | ---: |
| observed p95 | 191 ms |
| SLO ceiling | 200 ms |

The review is `planned`. Observability confirmation closes the measured path after Event Bus delivery. Source: `p01-src-010335`.
