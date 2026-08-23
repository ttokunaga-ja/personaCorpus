# Cross-PC persona production bundles

## Purpose

This runbook permits a Windows staging machine to produce a persona exclusively assigned to it, while the Mac remains the integration authority. It is artifact transfer, not a shared live workspace or lease database.

## Ownership boundary

- Assign each persona to exactly one PC for the whole active production batch. Neither PC may create, alter, recover, release, or validate-as-final work for a persona assigned to the other.
- Git synchronizes tracked control-plane material only: runbooks, prompts, scripts, and accepted code. It is not transport for corpus bytes, progress, scratch output, or leases.
- Never use a shared folder or Git merge to make two PCs write the same persona. Transfer ownership only between completed batches, after verified import and explicit coordinator assignment.

## Windows staging bootstrap

1. Clone the tracked repository for control-plane documents. A Kio runtime is
   neither required nor authorized for artifact staging; do not copy or run the
   macOS ARM64 runtime. Install only the document/render dependencies required
   by the assigned formats and record their versions.
2. Create a persona-only staging root outside the Mac production workspace. Configure paths for that machine; do not reuse `/Users/...` paths.
3. Copy only coordinator-approved, token-free inputs for the assigned persona:
   canonical-source inventory excerpt, M1 assignment-ledger/reservation digests,
   content spine, fixed assignment, and brief. Verify their recorded digests
   before production.
4. Do not synchronize or reconstruct `workspace/_control`, any lease record, release token, `canonical/`, or `.runtime/` from the Mac. Windows staging has no authority to mutate the Mac lease topology.

## Required completion bundle

One bundle contains exactly one completed persona batch and has these top-level entries:

```text
final/                         # only this batch's new files, home-relative layout
assignment/                    # token-free fixed assignment / ledger excerpt
manifest/files.jsonl           # relative paths, bytes and SHA-256 for final/
validation/report.json         # batch and artifact QA results
metadata/platform.json         # OS, architecture and generator/render versions
metadata/bundle.json           # persona, batch ID, source ledger digest
```

`final/` contains only paths allocated to that batch and preserves their layout
relative to the target persona's `home/`; it must not contain the pre-existing
M1 or previously accepted Full subtree. `assignment/` contains no mutable
control state. The bundle must not contain `_control`, any lease file, release
token, `canonical/`, `.runtime/`, a whole workspace, another persona, scratch
render output, credentials, or user-private data.

The JSONL manifest uses `persona-corpus.full-manifest/v1`: exactly one header
with the bundle's persona, canonical `plan_digest`, and `file_count`, followed
by file rows sorted by relative path with one entry per transferred file,
byte count, and SHA-256.
The validation report identifies fixed assignment rows,
counts, skill QA status, and blocked items. Platform metadata records OS/WSL
version, CPU architecture, document/render tool versions, and bundle producer
version, without secrets.

## Mac import gate

Before importing a bundle, the Mac coordinator must:

1. Stop all work for the target persona and confirm no target-persona parent or
   scope lease is active. Leases for other personas are irrelevant. Validate
   the bundle and assignment metadata in scratch before touching the final
   tree.
2. Claim the target persona parent lease and every affected Rust scope lease
   with unique Mac import sessions. The Mac parent retains all release tokens;
   Windows never receives them. Keep these leases until the cumulative import
   gate and parent QA have passed.
3. Verify the bundle persona and complete fixed-ledger digest match the
   coordinator assignment; reject path traversal, unexpected top-level
   entries, paths outside the assigned scope leaves, and any path not selected
   by the bundle's batch ID or acceptance-group batch IDs.
4. Create one Mac cumulative before manifest with `./bin/full-ledger manifest`.
   Verify that it contains the immutable 200-file M1 baseline plus only
   previously accepted Full assignment paths. Import must not overwrite,
   delete, rename, or alter any existing path.
5. Copy only the new `final/` entries allowed by the selected fixed assignment
   rows and verify their bytes against `manifest/files.jsonl`. Then create a
   cumulative after manifest from the Mac persona home.
6. Run `./bin/full-ledger verify` against the complete master assignment, the
   immutable M1 baseline, the cumulative before and after manifests, and one
   repeated `--batch-id` for every imported acceptance-group member. The
   selected union must be exactly the new paths, every prior file must be
   byte-for-byte unchanged, and the regenerated actual Mac manifest must match
   the supplied after manifest.
7. Inspect the validation report and skill QA evidence. Only after every gate
   passes, record the token-free import receipt and per-member checkpoints in
   Mac persona-owned progress/scratch, then normally release all scope leases
   followed by the parent lease.

On mismatch, do not write an acceptance receipt or release the work as
accepted. Preserve the received bundle for diagnosis, report the exact Mac
lease owners and imported paths, and do not use lease recovery as an import
workaround. Any cleanup or retry must follow explicit coordinator direction.

## Non-claims

Bundles demonstrate only transferred bytes and stated production QA. They do not establish Kio indexing, search, history replay, chunk counts, or performance, and do not authorize canonical-plan regeneration.
