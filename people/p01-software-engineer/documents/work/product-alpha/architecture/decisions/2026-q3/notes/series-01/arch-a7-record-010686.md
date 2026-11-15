# Product Alpha R7 architecture review ARCH-A7-010686

Aster Loop Product Engineering recorded `svc-edge-api` on 2026-09-11 under ADR-140. The asynchronous release gate preserves the approval boundary from Release Coordinator to Decision Store.

| Measure | Value |
| --- | ---: |
| observed p95 | 191 ms |
| SLO ceiling | 200 ms |

The review is `reviewed`. Observability confirmation closes the measured path after Event Bus delivery. Source: `p01-src-010686`.
