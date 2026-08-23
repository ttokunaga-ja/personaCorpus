# Product Beta preview contract notes

**Aster Loop 株式会社** publishes this preview alongside Product Alpha R7 for the 2026-07-13 release cycle. The common release control target is p95 184 ms and ADR-042 remains the required async approval gate.

## Preview scope

Beta adds a cancellable preview release request and typed event envelope. It is a `/v1/beta` preview surface, not a commitment to long-term endpoint stability. Preview IDs, state labels, and error codes are synthetic and opaque.

## Compatibility and naming

JSON uses camelCase and plural resources. `betaReleaseId` is opaque; consumers must not parse it. Alpha R7 `releaseId`, `requestId`, idempotency, timestamp, and stable cursor conventions are retained. Additive object fields are permitted; removal, enum narrowing, or newly required request fields require a new major version.

## Open questions and rollout constraints

Cancellation is accepted only before the ADR-042 approval decision. The preview window is limited to 2026-07-13 09:00-12:00 JST. Consumers must tolerate duplicate webhook deliveries and use eventId for deduplication. Rollout needs schema-diff, contract fixtures, and a clear graduation decision before any non-preview commitment.
