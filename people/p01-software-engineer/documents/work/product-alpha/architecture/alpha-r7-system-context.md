# Product Alpha R7 system context
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

Clients call Edge API; Edge API validates release intent; Release Coordinator owns ticket state and durable decisions; Event Bus distributes versioned events; Observability records evidence without control.

Flow: Clients -> Edge API -> Release Coordinator -> Event Bus -> Observability. Coordinator -> Decision Store. Edge API -> Observability.

C4 boundaries: clients own intent/idempotency; Edge owns validation; Coordinator owns transitions; Event Bus owns delivery; Observability owns evidence. p95 budget is 184 ms.
