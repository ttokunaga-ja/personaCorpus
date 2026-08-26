# Operational runbooks

This directory is a local operational copy of the accepted persona-production
guidance from the Kio planning repository. In this standalone project:

- `canonical/` contains create-only canonical planning artifacts.
- `workspace/` is the single shared production workspace.
- `./bin/persona` invokes the pinned Rust `kio-eval persona` interface.
- `runbooks/personas/` contains the twenty creative production briefs.
- `prompts/` contains one ready-to-paste parent-task prompt per persona.
- `FULL_PRODUCTION_PROTOCOL.md` and `FULL_LEDGER_SPEC.md` define resumable
  additions beyond the byte-frozen M1 milestone.
- `CROSS_PC_PERSONA_BUNDLES.md` defines persona-exclusive transfer between
  machines without synchronizing leases or `_control/`.
- `./bin/full-ledger` publishes create-only Full inventories, manifests and M1
  reservations, including an explicit digest-bound legacy reconciliation only
  where accepted; persona-specific `./bin/p01-m1-assignment`,
  `./bin/p02-m1-assignment`, and `./bin/p03-m1-assignment` safely backfill their
  legacy M1 assignment ledgers. Post-Full p02/p03 backfills also use the
  read-only `./bin/m1-home-manifest-guard` so the complete home path set must
  match the accepted root corpus manifest before an M1 ledger is emitted.
- `./bin/full-resume-gate` is the pinned read-only v2 acceptance-chain/live-home
  gate for prepared p04--p20 Full packages. It does not replace the completed,
  closed p01--p03 persona-isolated v1 chains or any lease rule.
- `GIT_MANAGEMENT.md` defines the tracked accepted-artifact boundary, and
  `./bin/corpus-manifest` verifies all Git-managed final artifact bytes.

The canonical plan and Rust owner record override prose if they disagree.
Do not use these Markdown files to regenerate, reinterpret, or replace them.
