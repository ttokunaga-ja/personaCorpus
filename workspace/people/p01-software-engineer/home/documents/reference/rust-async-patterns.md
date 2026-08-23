# Rust async patterns - internal reference

**Aster Loop株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042**

Use cancellation-aware task boundaries: propagate a cancellation token to workers, finish idempotent cleanup, and return a named cancelled state. Apply timeouts at I/O boundaries rather than around whole workflows; record elapsed control-plane time separately from asynchronous execution.

Retry only classified transient failures, with bounded attempts and jitter. Backpressure uses a bounded queue and an explicit rejected/awaiting state instead of unbounded spawning. Tests use deterministic clocks, fake transports, cancellation races, timeout assertions, and queue-saturation fixtures. ADR-042 approval remains a state transition, not a background side effect.
