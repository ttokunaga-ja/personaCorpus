# Product Alpha R7 architecture review ARCH-A7-008146

Aster Loop Product Engineering recorded `svc-observability` on 2026-07-17 under ADR-187. The asynchronous release gate preserves the approval boundary from Release Coordinator to Decision Store.

| Measure | Value |
| --- | ---: |
| observed p95 | 189 ms |
| SLO ceiling | 200 ms |

The review is `reviewed`. Observability confirmation closes the measured path after Event Bus delivery. Source: `p01-src-008146`.
