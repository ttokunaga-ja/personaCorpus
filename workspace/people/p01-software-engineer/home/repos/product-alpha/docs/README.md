# Product Alpha R7 developer docs
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

## Purpose
Synthetic async release gate docs.
## Architecture
Client -> Edge API -> Release Coordinator -> Event Bus -> Observability.
## Setup
Synthetic fixture only.
## Test
Run smoke_test.sh and latency notebook.
## Release flow
Shadow, 10%, 50%, 100%; stop at p95 184 ms.
