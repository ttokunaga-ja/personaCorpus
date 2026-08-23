# Rollback strategy
Aster Loop 株式会社 | Product Alpha R7 | 2026-07-13 | p95 184 ms | ADR-042

|Trigger|Action|Verification|Stop|
|---|---|---|---|
|p95 above 184 ms / 10m|disable async_release_gate|legacy canary|new async admissions|
|error above 0.2%|halt and drain queue|decision invariant|error persists|
|schema guard failure|block publisher|consumer check|payload incompatible|
|duplicate decision|freeze release id|ledger comparison|unclassified cause|

Flags: async_release_gate, async_gate_shadow, sync_release_gate. Rollback changes routing only; data is forward-read/backward-write for one R7 window.
