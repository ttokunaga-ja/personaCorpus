# Accepted corpus Git migration report

Date: 2026-08-25 (Asia/Tokyo)

Artifact tip before this report/manifest commit:
`402781920607e84657d25ba96bd848328fa75265`.

## Concurrent-production boundary

This update migrated the accepted p02 Full corpus while p03 and other persona
work remained outside its Git boundary. p02 now contributes all 15,000 accepted
artifacts. Every p03 Full artifact and every unrelated active-persona output,
progress record, helper, and prompt remained unstaged.

The coordinator used `bin/corpus-manifest --tracked-only`. That mode binds the
canonical persona plan and selected artifacts to stage-0 Git-index bytes,
rejects index conflicts and worktree/index drift, and ignores unrelated
untracked production output. Generation and an independent verify both returned
`result: ok` for the final managed snapshot.

p02's parent and 20 scope leases were claimed only to materialize 12,932 final
hard-linked paths onto independent inodes, then released normally. No lease was
recovered and `workspace/_control/` was not edited manually. The seven leases
reported by the final readiness check belong to unrelated ongoing work and were
not treated as completed production.

## Git-managed artifact snapshot

| Persona | Files | Bytes |
| --- | ---: | ---: |
| p01 | 12,000 | 784,708,930 |
| p02 | 15,000 | 729,811,259 |
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
| **Total** | **30,600** | **2,425,981,893** |

The root manifest has 30,601 JSONL lines (one header plus 30,600 artifacts),
SHA-256 `888d11146fe305807b5398eef48a50b261ad1bb97bb5725645596e4b6fa38cb9`,
and a largest managed artifact of 9,138,385 bytes. The largest blob anywhere in
reachable Git history is 14,540,322 bytes. Both are below the ordinary GitHub
100 MiB object ceiling.

## Accepted additions reflected in this snapshot

- p01 Full allocation authority, 91 token-free checkpoints, six cumulative
  acceptance-group manifest pairs, and all 11,788 Full additions. The six
  artifact waves were committed and pushed separately.
- p02 Full allocation authority, 12 portable cumulative wave manifests, the
  structure-v2 current-path overlay and format audit, and all 14,800 Full
  additions. The artifacts were committed in six accepted production waves.
  Raw batch checkpoints remain local because some contain machine-specific
  render-receipt paths.
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

p02 has 15,000 regular artifacts, no symlink or nonregular entry, a complete
14,800-row Full assignment ledger, and no duplicate content hash. Structure v2
preserves all artifact bytes while distributing them across 999 file-bearing
directories with at most 82 direct files. Its overlay translates each
historical wave path to the current Git-managed path, and all 15,000 final
artifacts have link count one.

## Safety and validation

- No selected artifact is a symlink or nonregular entry.
- No selected blob is 50 MiB or larger; none approaches 100 MiB.
- Commit-boundary scans found no confirmed private key, access key, credential,
  release token, or other secret in the selected paths.
- M1 home path sets for p11 through p20 exactly match their frozen ledgers.
- `tests/corpus-manifest.sh`, the p02 structure and format tests, Python
  compilation, staged diff checks, and `corpus-manifest verify --tracked-only`
  passed.
- There are no repository GitHub Actions workflows and no active custom Git
  hooks, so no remote CI job was expected from these pushes.

Kio indexing, history, search, chunking, and performance evaluation were not
run and are not claimed by this migration.

## Intentionally local or deferred

- p03 Full production output and all unrelated in-progress persona output;
- p02's 85 raw batch checkpoints and local render receipts; portable acceptance
  remains represented by the 12 wave manifests and structure-v2 evidence;
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
Those changes require repair and review after active Full work no longer depends
on the shared checkout.

## GitHub operational envelope

GitHub blocks ordinary Git objects above 100 MiB and pushes above 2 GiB. This
migration used an initial baseline push followed by small coherent incremental
pushes. At report time, `.git` contains about 2.04 GiB of packs plus about 16 MiB
of loose objects. This is below GitHub's strongly recommended 5 GiB repository
boundary, but future corpus growth should continue to use measured, coherent
pushes and may eventually require release assets or object storage. No
force-add, history rewrite, Git LFS migration, or push outside `main` was used.
