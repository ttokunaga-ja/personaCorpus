# p95 latency budget
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

|Stage|Target|Observed|Source|
|---|---:|---:|---|
|Edge routing|28 ms|26 ms|edge span|
|API validation|32 ms|31 ms|validation span|
|Durable admission|46 ms|45 ms|transaction|
|Event publish|24 ms|22 ms|broker ack|
|Observability|18 ms|16 ms|telemetry|
|Reserve|36 ms|44 ms|variance|
|Total|184 ms|184 ms|synthetic R7 cohort|

Measure edge acceptance to durable response. Warn at 165 ms for 5m; page at 184 ms for 10m. Durability costs 46 ms but prevents duplicate decisions.
