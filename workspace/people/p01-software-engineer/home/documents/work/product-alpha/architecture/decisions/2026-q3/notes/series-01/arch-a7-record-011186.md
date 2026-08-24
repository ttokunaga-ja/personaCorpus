# Product Alpha R7 architecture review ARCH-A7-011186

Aster Loop Product Engineering recorded `svc-decision-store` on 2026-07-21 under ADR-043. The asynchronous release gate preserves the approval boundary from Release Coordinator to Decision Store.

| Measure | Value |
| --- | ---: |
| observed p95 | 178 ms |
| SLO ceiling | 200 ms |

The review is `reviewed`. Observability confirmation closes the measured path after Event Bus delivery. Source: `p01-src-011186`.
