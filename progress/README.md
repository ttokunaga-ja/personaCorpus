# Local production checkpoints

Persona parent tasks may keep resumable notes under `progress/<persona-id>/`.
Those persona-owned folders are intentionally ignored by Git. Never write
lease release tokens, real personal data, or final corpus artifacts here.

M1 production freezes `progress/<persona-id>/m1-assignment-ledger.jsonl`.
It is the sole machine-readable M1 allocation authority; the adjacent
`m1-assignments.md` is a derived human display only. Full production uses that
ledger plus the persona's `progress/.../full/` and
`scratch/.../full/` areas for token-free local state:

- deterministic canonical source inventory;
- frozen M1 artifact IDs, paths, scopes, families and physical extensions;
- deterministic 200-row M1 source/path reservations;
- byte-frozen M1 baseline manifest;
- immutable Full addition assignment;
- content spine and batch checkpoints;
- recomputed actual-file manifests and validation summaries.

These records coordinate production but do not replace the Rust plan or owner
record. Never store parent or scope release tokens in them.
