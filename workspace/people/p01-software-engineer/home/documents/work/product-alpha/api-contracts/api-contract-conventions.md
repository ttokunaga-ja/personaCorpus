# Product Alpha R7 API contract conventions

**Owner:** Aster Loop 株式会社 Platform Engineering  
**Release window:** 2026-07-13 09:00-12:00 JST  
**Decision:** ADR-042 async release gate

## Naming and resources

Use plural, lowercase nouns (`/v1/releases`) and opaque resource identifiers (`rel_r7_...`). Actions that create a release are modeled as collection creation, not verbs in a path. JSON fields use `camelCase`; enums use lowercase snake case. A release moves through `requested`, `awaiting_approval`, `approved`, `running`, `completed`, or `failed`.

## Pagination

List operations accept `limit` (1-100, default 25) and an opaque `cursor`. Responses return `items` and `nextCursor`; clients must not construct or interpret cursors. The ordering is stable: `createdAt` ascending, then `releaseId` ascending. A missing `nextCursor` means the end of the collection.

## Errors and request identity

Every response includes `X-Request-Id`. Clients may supply a printable `X-Request-Id` up to 64 characters for correlation; otherwise the service creates one. Error bodies use `{ code, message, requestId, retryable, details? }`. Messages are safe for users and never expose internal policies, hosts, or secrets.

## Idempotency and time

`POST /v1/releases` requires `Idempotency-Key` (8-128 printable characters). The same key and semantically identical body replay the original response; a changed body returns `409 idempotency_conflict`. All timestamps are RFC 3339 UTC strings. A requested release becomes eligible only when ADR-042 records an approval.

## Versioning and deprecation

The URL major version is `/v1`. Additive fields and enum values are announced in release notes; clients must ignore unknown object fields. Removing an endpoint, making a request field required, or narrowing an enum is a breaking change and requires `/v2`. Deprecated fields remain available for at least two release windows and carry `Deprecation` and `Sunset` response headers.

## R7 reliability objective

The create and status endpoints target p95 **184 ms** excluding asynchronous execution time. A `504 upstream_timeout` is retryable only when the caller reuses the same idempotency key.
