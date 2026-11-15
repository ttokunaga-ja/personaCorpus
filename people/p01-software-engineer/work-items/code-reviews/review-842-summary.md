# Review 842 - asynchronous release gate

**Aster Loop 株式会社 · Product Alpha R7 · release 2026-07-13 · ADR-042**

## Scope

This review covers the change that turns the Product Alpha release gate into an asynchronous state machine. The proposed API accepts a request, emits a durable pending state, and publishes a terminal decision for compatible polling clients. Synthetic canary evidence records p95 **184 ms**.

## Findings and resolution

| Finding | Resolution | Disposition |
|---|---|---|
| A cancellation could be returned as a generic dependency error | Preserve the cancellation branch in the router | Resolved |
| A stale append could overwrite a newer aggregate version | Require optimistic version matching | Resolved |
| Terminal decisions lacked a stable audit reference in one fixture | Require `decision_ref=ADR-042` | Resolved |

## Approval conditions

1. R6 polling compatibility must be exercised against the R7 terminal state.
2. p95 gate completion evidence must remain at or below 200 ms; observed synthetic p95 is 184 ms.
3. Rollback must retain append-only decision evidence and leave promotion paused until terminal status is verified.

Approved for controlled promotion once the conditions above are attached to the release timeline.
