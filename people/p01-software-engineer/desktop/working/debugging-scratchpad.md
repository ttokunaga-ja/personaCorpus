# Debugging scratchpad

Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

Hypothesis: durable admission dominates tail latency because its transaction waits for a decision-store acknowledgement. Experiment: replay 120 synthetic release intents with queue depth below 200 and compare spans. Result: edge 28, validation 32, admission 46, publish 24, telemetry 18, reserve 36 ms; total p95 184 ms. Next: keep the durable boundary, instrument queue backlog, and rehearse timeout recovery.
