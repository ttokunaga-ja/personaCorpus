# Product Beta architecture overview

**Aster Loop 株式会社 · 2026-07-13 · Product Alpha R7 coexistence**

Product Beta is a synthetic event-oriented companion service that can consume Product Alpha R7 release-gate decisions without owning Alpha's promotion authority. ADR-042 defines Alpha's asynchronous release gate; Beta subscribes to its terminal decision events and never re-evaluates the gate.

## Boundaries

- **Command router** authenticates an in-process caller context, validates typed commands, and propagates cancellation.
- **Event store** commits aggregate events with optimistic concurrency and exposes append-only reads.
- **Projection worker** builds an idempotent read model from checkpoints and retryable batches.
- **Integration boundary** maps Alpha R7 `gate_id` and `decision_ref` to Beta-compatible events while preserving unknown fields.

## Data flow

1. A caller submits a typed command to the router.
2. The domain transition emits an event with an expected aggregate version.
3. The event store accepts or rejects the append atomically.
4. The projection worker advances a checkpoint only after applying the event idempotently.
5. An Alpha R7 terminal decision is represented as an external event tagged with **ADR-042**.

The synthetic latency reference is p95 **184 ms** for the Alpha gate observation; Beta records it as evidence, not as a service-level commitment. Storage selection is intentionally deferred in [ADR-043](adr-043-beta-storage.md).
