# ADR-042: Asynchronous release gate

**Status:** Accepted | **Date:** 2026-07-13 | **Scope:** Aster Loop株式会社, Product Alpha R7

## Context

Product Alpha R7 needs a traceable boundary between release request and execution. The API target is p95 184 ms for control-plane calls; release execution is asynchronous and must not depend on a synchronous approval round trip.

## Decision

Adopt ADR-042: each release enters `awaiting_approval` after request validation. An approval event transitions it to `approved`, after which the worker may run it. Request IDs correlate API responses and events; idempotency prevents duplicate requests. The 2026-07-13 release window is 09:00-12:00 JST.

## Consequences

Consumers receive an accepted state rather than an immediate completion promise. Operators gain an auditable gate and a consistent failure path. Cancellation is permitted only before approval for the Beta preview. This record is implemented with ADR-039 latency measurements, ADR-040 event versioning, and ADR-041 feature-flag controls.
