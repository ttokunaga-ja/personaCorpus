# Git management for accepted corpus artifacts

## Tracked acceptance boundary

The repository shares accepted physical artifacts and their portable evidence
with ordinary Git:

- `workspace/people/**/home/**` final artifacts;
- `canonical/persona-plan.json` and `canonical/persona-schedule.json`;
- allowlisted, token-free assignment ledgers, Full inventories/reservations,
  accepted manifests and checkpoints under `progress/`;
- `manifests/corpus-sha256.jsonl`, which covers every managed final artifact.

Line-ending conversion is disabled for identity-sensitive corpus, authority,
ledger, and manifest bytes. Corpus artifacts are also excluded from Git's
textual diff driver; byte review uses the deterministic manifest rather than
binary/text heuristics. A clone verifies the accepted final tree with:

```bash
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```

When another persona is still producing untracked files in the same checkout,
the coordinator must bind manifest generation and verification to the Git
index instead of the live tree:

```bash
./bin/corpus-manifest generate --tracked-only --replace \
  --manifest manifests/corpus-sha256.jsonl
git add manifests/corpus-sha256.jsonl
./bin/corpus-manifest verify --tracked-only \
  --manifest manifests/corpus-sha256.jsonl
```

`--tracked-only` rejects index conflicts, unsafe entries, and any unstaged
drift in a selected artifact or the canonical persona plan. It therefore
describes exactly the artifact snapshot that the next commit will contain;
untracked output from an active, excluded persona is not silently accepted.

## Intentionally local state

Never force-add `.runtime/`, `workspace/_control/`, the workspace owner record,
scratch renders/temp output, release or lock state, machine-specific scaffold
and materialization receipts, or canonical render JSON. The two render copies
currently exceed 100 MiB and Git LFS is not installed; the other exclusions
also encode local machine state even when small.

A clone is therefore an accepted corpus baseline, not a writable production
controller. Formal production still requires a coordinator-approved local
bootstrap that restores the pinned runtime, canonical render/materialization,
workspace owner, and fresh lease topology without rewriting tracked artifacts.

## Wave acceptance

Persona production tasks work in the shared Local root and never run `git add`,
`git commit`, or `git push`. Other personas' unstaged files are expected while
a parallel wave is active. After the wave stops, the coordinator:

1. confirms target and global leases are absent and repeats the idle check;
2. validates each persona against its frozen ledger and previous manifest;
3. rejects symlinks, nonregular entries, secrets, unexpected paths, and any
   artifact larger than 100 MiB;
4. stages only the accepted persona artifacts, then regenerates and verifies
   the corpus manifest with `--tracked-only` when other production is active;
5. stages only the accepted personas and portable evidence;
6. commits one persona per reviewable artifact commit, then commits the updated
   root manifest/report; and
7. pushes only when the user explicitly authorizes it.

Do not use force-add to defeat an ignore rule. If a desired portable record is
ignored, update the documented allowlist and review that policy change first.

## Repository sizing

There is no simple published hard limit on total repository file count. GitHub
blocks regular Git blobs above 100 MiB and pushes above 2 GiB, and recommends
keeping repositories small (ideally below 1 GiB and strongly below 5 GiB).
Consequently Full expansion is committed persona-by-persona and batch-by-batch;
the coordinator checks the largest blob, aggregate staged bytes, and repository
growth before every push. Git LFS or release/object storage can be introduced
later if ordinary Git growth becomes impractical, but it is not required for
the current artifacts.
