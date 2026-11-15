# ADR-043: Product Beta event-storage candidate

- **Status:** proposed
- **Date:** 2026-07-13
- **Context:** Aster Loop 株式会社; Product Beta coexists with Product Alpha R7.

## Decision

Evaluate a transactional append-only event store with optimistic concurrency and checkpoint reads. The initial test fixture uses SQLite only as a portable synthetic benchmark fixture; it is not a production storage decision.

## Rationale

Beta needs ordered aggregate versions, durable projection checkpoints, and explicit conflict behavior. These needs are distinct from **ADR-042**, which governs Product Alpha R7's asynchronous release gate and its observed p95 **184 ms**. ADR-043 does not alter Alpha gate evaluation, release authority, or latency target.

## Options and consequences

| Option | Result |
|---|---|
| Embedded transactional store | Simple local fixture, bounded concurrency semantics |
| Managed relational event store | Candidate for operational durability; requires later sizing review |
| Log-only store | Rejected for now: checkpoint and conflict semantics need additional design |

Adoption requires a follow-up ADR with retention, backup, and recovery evidence. Until then, interfaces remain storage-neutral.
