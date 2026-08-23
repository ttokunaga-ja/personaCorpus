# ADR-039: R7 p95 latency budget

**Status:** Accepted | **Date:** 2026-07-13

Product Alpha R7 allocates a p95 budget of **184 ms** to create and status control-plane operations. The budget excludes asynchronous worker execution after ADR-042 approval.

Measure server receive-to-response timestamps in a synthetic staging fixture, grouped by operation and minute. Use at least 100 deterministic requests per operation, exclude malformed fixture setup, retain request IDs, and report p50/p95/p99. A p95 breach pauses rollout expansion and requires an owner review; it does not bypass the release gate.
