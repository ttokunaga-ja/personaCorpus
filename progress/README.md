# Local production checkpoints

Persona parent tasks keep resumable records under `progress/<persona-id>/`.
The allowlisted assignment ledgers, content spines, accepted manifests, and
token-free checkpoints are Git-managed after coordinator acceptance. Local
failure notes, render output, caches, and other machine-specific state remain
ignored. Never write lease release tokens, real personal data, or final corpus
artifacts here.

M1 production freezes `progress/<persona-id>/m1-assignment-ledger.jsonl`.
It is the sole machine-readable M1 allocation authority; the adjacent
`m1-assignments.md` is a derived human display only. Full production uses that
ledger plus the persona's `progress/.../full/` and
`scratch/.../full/` areas for token-free local state:

- deterministic canonical source inventory;
- frozen M1 artifact IDs, paths, scopes, families and physical extensions;
- deterministic 200-row M1 source/path reservations;
- an optional, frozen digest-bound same-scope reconciliation for accepted
  legacy physical-family mismatches (never an ad hoc reservation exception);
- byte-frozen M1 baseline and cumulative accepted manifests;
- immutable Full addition assignment;
- content spine and batch checkpoints;
- recomputed actual-file manifests and validation summaries.

Prepared p04--p20 Full production uses checkpoint schema
`persona-corpus.full-batch-checkpoint/v2` with `bin/full-resume-gate` to prove
one append-only acceptance chain and a byte-identical live-home tip. The v2
record is token-free and includes pinned before/after manifest bytes, exact
family counts, required QA booleans, and final-image scan provenance where
needed. It neither changes lease ownership/recovery rules nor retrofits the
accepted p01--p03 v1/persona-isolated chains mid-wave.

These records coordinate production but do not replace the Rust plan or owner
record. Production tasks never stage or commit them; the wave coordinator does
so only after validation. Never store parent or scope release tokens in them.
