# Accepted corpus Git migration report

Date: 2026-08-24 (Asia/Tokyo)

Source control-plane HEAD: `60f0962f49af607fc5e66d1776025a32d39bce4c`

## Idle and completion gates

- `./bin/check-ready` reported `active_leases=0` on two checks separated by at
  least 30 seconds.
- The path/size/mtime state digest for `workspace/people` plus `progress`
  remained `3762fd393548f301a0912fa9c286b44a0aeb113e8fa99bc8d001cc075d84c334`.
- No relevant final/progress file had changed in the preceding five minutes.
- p01 Full pilot was reverified with `./bin/full-ledger verify`: 200 immutable
  M1 files, 12 selected additions, 212 after files, `result: ok`.
- The p01 baseline and cumulative after manifest SHA-256 values are
  `6b59e44cd520134928b710303f0ddaa2890f006d58d7c582dc07bc280f429bec`
  and `4dae33fac0543a2f692d55c5159e70d40f5d2708ae1429936ba16272db44affb`.
- p08, p09, and p10 each have 200 artifacts and a frozen 200-row M1 ledger;
  each ledger path set exactly equals its final artifact path set, with no
  exact SHA-256 duplicate.

## Git-managed artifact snapshot

| Persona | Files | Bytes |
| --- | ---: | ---: |
| p01 | 212 | 17,465,507 |
| p02 | 200 | 5,987,337 |
| p03 | 200 | 20,472,981 |
| p04 | 200 | 11,929,120 |
| p05 | 200 | 25,381,289 |
| p06 | 200 | 58,297,415 |
| p07 | 200 | 135,635,749 |
| p08 | 200 | 52,666,315 |
| p09 | 200 | 61,354,069 |
| p10 | 200 | 43,923,771 |
| **Total** | **2,012** | **433,113,553** |

The root manifest has 2,013 JSONL lines (one header plus 2,012 artifacts), is
574,841 bytes, and has SHA-256
`ba46cacce99a61e2c2cecda22c3895f2813864dea1e65242003b2d18d9da8f32`.
The largest artifact is 9,138,385 bytes. No final artifact exceeds the
104,857,600-byte GitHub-compatible guard.

Portable authority includes `canonical/persona-plan.json` (357,751 bytes,
SHA-256 `f4e84efd49a98760733d628aaa44342dc7039cc845aced936e8a158eada95236`)
and `canonical/persona-schedule.json` (14,540,322 bytes, SHA-256
`77a12bacfecc4750a0175db1d9c10c7709994a1c747289c121bb27087dd4e7a9`).
Portable progress comprises 19 newly managed persona records plus the existing
`progress/README.md`, totalling 6,955,528 bytes at migration time. Machine
absolute prefixes in the human M1 displays were normalized to repository-
relative paths; frozen filenames, families, counts, and artifact bytes were
not changed.

## Safety inspection

- No symlink or nonregular entry exists below any persona home.
- Maximum observed final-tree directory width is 32 entries and maximum final
  artifact path depth is 10 components.
- Gitleaks 8.30.1 scanned the final tree, including nested archives to depth 2,
  and returned no findings.
- Every managed path is selected by the documented `.gitignore` allowlist;
  force-add was not used.
- `./bin/corpus-manifest verify` regenerated every path, byte count, and digest
  and returned `result: ok`.

This migration does not rerun Kio indexing, history, search, chunking, or
performance evaluation and makes no claim about them.

## Intentionally local exclusions

- `.runtime/` and its macOS ARM64 pinned executable;
- `workspace/_control/`, `workspace/persona-workspace-owner.json`, and the
  redundant workspace plan copy;
- `scratch/` render, temporary, QA, and local production logs;
- `canonical/scaffold-result.json` and the machine-specific materialization
  receipt;
- redundant `canonical/materialized/` copies; and
- both 155,409,781-byte canonical render JSON copies, which exceed the ordinary
  Git blob guard. Git LFS is not installed on this machine.

These exclusions contain no accepted final artifact. A clone receives the
accepted corpus and portable verification evidence, but needs an approved
local runtime/owner/lease bootstrap before it can become a formal production
controller.

## GitHub operational envelope

GitHub's current [repository limits](https://docs.github.com/en/repositories/creating-and-managing-repositories/repository-limits)
enforce a 100 MiB single-object ceiling and a 2 GiB push ceiling, and recommend
at most 10 GiB of compressed `.git` on-disk size, 3,000 entries per directory,
and depth 50. Its [large-file guidance](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github)
recommends keeping a repository ideally below 1 GiB and strongly below 5 GiB.
There is no simple published hard total-file-count ceiling. Current artifacts
are within the blob and tree limits; Full growth must remain persona/batch
committed and measured before each push.

No push is part of this migration. Remote publication remains pending explicit
user authorization.
