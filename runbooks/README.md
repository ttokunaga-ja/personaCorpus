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
  reservations; `./bin/p01-m1-assignment` backfills the legacy p01 M1
  assignment ledger for the first pilot.

The canonical plan and Rust owner record override prose if they disagree.
Do not use these Markdown files to regenerate, reinterpret, or replace them.
