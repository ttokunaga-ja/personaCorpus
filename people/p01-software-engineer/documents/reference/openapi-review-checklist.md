# OpenAPI review checklist

For Product Alpha R7, check compatibility before style: no endpoint removal, no new required request field, and no enum narrowing within a major version. Confirm resource naming, opaque IDs, pagination ordering, idempotency, request IDs, and RFC 3339 timestamps.

At the security boundary, ensure error messages are safe, examples carry no credentials, and approval is represented by ADR-042 state rather than a client-supplied bypass. Review success, conflict, validation, and timeout examples; make p95 184 ms scope clear. Documentation must say whether a release is awaited, approved, running, or completed and name deprecation/sunset behavior.
