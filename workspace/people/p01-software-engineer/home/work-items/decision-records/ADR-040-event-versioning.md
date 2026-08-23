# ADR-040: Release event versioning

**Status:** Accepted | **Date:** 2026-07-13

Release events use named event types and versioned payload schemas. Producers add fields compatibly; consumers ignore unknown fields and deduplicate by `eventId`. Removing an event type, making a field required, or narrowing an enum requires a new major event version.

Deprecation requires a notice in the event description, a compatible parallel period of two release windows, and a published sunset date. ADR-042 gate transitions remain correlated by release ID and request ID.
