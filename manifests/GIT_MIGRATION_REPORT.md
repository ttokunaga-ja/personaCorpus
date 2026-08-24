# Accepted corpus Git migration report

Date: 2026-08-24 (Asia/Tokyo)

Artifact tip before this report/manifest commit:
`d16de9cbfec6cb2434be1509b9524db3e4f69760`.

## Concurrent-production boundary

This migration intentionally ran while p02 and p03 Full production remained
active. Their already tracked M1 baselines (200 artifacts each) remain managed,
but every untracked p02/p03 Full artifact, checkpoint, manifest, allocation
record, helper, and prompt was excluded from staging.

The coordinator used `bin/corpus-manifest --tracked-only`. That mode binds the
canonical persona plan and selected artifacts to stage-0 Git-index bytes,
rejects index conflicts and worktree/index drift, and ignores unrelated
untracked production output. Generation and an independent verify both returned
`result: ok` for the final managed snapshot.

No lease was recovered or released and `workspace/_control/` was not edited.
Stale Full-allocation leases for other personas were not treated as completed
production. p15 M1 was included only after its 200-file artifact and ledger
digest remained identical across two checks 30 seconds apart; p15 Full records
remain local.

## Git-managed artifact snapshot

| Persona | Files | Bytes |
| --- | ---: | ---: |
| p01 | 12,000 | 784,708,930 |
| p02 | 200 | 5,987,337 |
| p03 | 200 | 20,472,981 |
| p04 | 200 | 11,929,120 |
| p05 | 200 | 25,381,289 |
| p06 | 200 | 58,297,415 |
| p07 | 200 | 135,635,749 |
| p08 | 200 | 52,666,315 |
| p09 | 200 | 61,354,069 |
| p10 | 200 | 43,923,771 |
| p11 | 200 | 24,863,187 |
| p12 | 200 | 24,526,384 |
| p13 | 200 | 66,739,166 |
| p14 | 200 | 43,921,563 |
| p15 | 200 | 44,928,380 |
| p16 | 200 | 51,442,769 |
| p17 | 200 | 81,214,417 |
| p18 | 200 | 42,396,104 |
| p19 | 200 | 72,848,754 |
| p20 | 200 | 48,920,271 |
| **Total** | **15,800** | **1,702,157,971** |

The root manifest has 15,801 JSONL lines (one header plus 15,800 artifacts),
SHA-256 `94c0e7da8d7bfbb18288c1bf378c5dd18cfb772148498318483370ffbcf0f115`,
and a largest managed artifact of 9,138,385 bytes. The largest blob anywhere in
reachable Git history is 14,540,322 bytes. Both are below the ordinary GitHub
100 MiB object ceiling.

## Accepted additions in this migration

- p01 Full allocation authority, 91 token-free checkpoints, six cumulative
  acceptance-group manifest pairs, and all 11,788 Full additions. The six
  artifact waves were committed and pushed separately.
- p11 through p20 M1 corpora, one persona per commit: 200 physical artifacts,
  one 200-row machine-readable ledger, and one portable display per persona.
- A fail-closed Git-index manifest mode and its regression tests, allowing an
  accepted subset to be committed while another persona remains untracked and
  active.

p01 has 12,000 regular artifacts, no symlink or nonregular entry, a complete
11,800-row Full assignment ledger, and 91 checkpoints matching all 91 assigned
batches. Each cumulative manifest before-state is byte-identical to the prior
after-state, and each wave's manifest additions exactly match its assigned
batch paths.

## Safety and validation

- No selected artifact is a symlink or nonregular entry.
- No selected blob is 50 MiB or larger; none approaches 100 MiB.
- Commit-boundary scans found no confirmed private key, access key, credential,
  release token, or other secret in the selected paths.
- M1 home path sets for p11 through p20 exactly match their frozen ledgers.
- `tests/corpus-manifest.sh`, Python compilation, staged diff checks, and
  `corpus-manifest verify --tracked-only` passed.
- There are no repository GitHub Actions workflows and no active custom Git
  hooks, so no remote CI job was expected from these pushes.

Kio indexing, history, search, chunking, and performance evaluation were not
run and are not claimed by this migration.

## Intentionally local or deferred

- all active p02/p03 Full production output and portable progress;
- Full allocation/start packages not yet safe to publish as READY, including
  packages affected by stale leases or the pending resume-gate review;
- `.runtime/`, `workspace/_control/`, workspace owner/scaffold receipts,
  release and lock state, scratch renders/temp/QA output, and machine-specific
  absolute-path receipts;
- canonical render/materialization JSON files above 100 MiB; and
- exact-ignore root renderer/debug spill files (`-.jpg`, `-.png`, `-.ppm`,
  `scratch_rows.jsonl`, and `scratch_rows.tmp`).

The pending resume-gate changes are deliberately not part of this migration:
review found missing enforcement for the 200-file M1 baseline, assignment-ledger
digest binding, and general dependency acceptance order. The production-state
dependent `tests/full-ledger.sh` is also not valid against completed p01 Full.
Those changes require repair and review after active p02/p03 work no longer
depends on the shared checkout.

## GitHub operational envelope

GitHub blocks ordinary Git objects above 100 MiB and pushes above 2 GiB. This
migration used an initial baseline push followed by small coherent incremental
pushes. At report time, `.git` contains about 968 MiB of packs plus about 499 MiB
of loose objects; repository maintenance can compact the loose objects later,
after production I/O is idle. No force-add, history rewrite, Git LFS migration,
or push outside `main` was used.
