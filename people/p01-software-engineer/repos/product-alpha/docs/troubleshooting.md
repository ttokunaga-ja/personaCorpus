# Troubleshooting
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

Queue lag: check depth, consumer health, dead-letter count. Schema mismatch: run deterministic v1 guard and block publisher. p95 regression: compare 28/32/46/24/18/36 ms spans against 184 ms budget.
