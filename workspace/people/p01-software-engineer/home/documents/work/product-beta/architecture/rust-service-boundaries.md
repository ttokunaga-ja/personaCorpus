# Rust service boundaries

| Crate | API boundary | Error policy | Test seam |
|---|---|---|---|
| `beta-domain` | commands, events, transitions | typed invariant errors | pure transition tests |
| `beta-router` | cancellation-aware dispatch | maps validation vs retryable errors | fake command handler |
| `beta-store` | append/read/checkpoint traits | conflict is explicit, never silently retried | in-memory store |
| `beta-projection` | idempotent event application | retryable projection failure retains checkpoint | scripted sink |
| `beta-alpha-adapter` | Alpha R7 event translation | unknown field tolerant; malformed event rejected | fixture adapter |

The Alpha adapter only consumes terminal release-gate decisions produced under ADR-042. It retains `gate_id`, `decision_ref`, and observed p95 **184 ms** as synthetic diagnostics. The service owns no Alpha credentials, customer records, or promotion commands.

Tests divide at trait boundaries: domain state transitions are synchronous; router and projection paths use async trait methods; store conflicts, partial application, cancellation, rollback, and recovery are all deterministic fixtures.
