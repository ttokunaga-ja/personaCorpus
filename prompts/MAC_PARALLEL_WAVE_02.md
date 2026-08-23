# Mac parallel wave 02

This is the coordinator handoff after the accepted p01 Full pilot and p08--p10
M1 wave were migrated to Git. It does not itself own a persona.

## Coordinator preflight

From the shared Local root, confirm the accepted baseline before opening any
production task:

```bash
git status --short
./bin/check-ready
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```

The status must be clean and global `active_leases=0` at wave start. The corpus
manifest must report `result: ok`. Do not start from a worktree.

## Open three M1 Local tasks

Open three separate Codex Local tasks against the same `personaCorpus`
repository root currently used by the coordinator. Confirm each task's `pwd`
is that root; do not embed or reuse another machine's absolute path.

Paste exactly one prompt into each task:

1. `prompts/p11-account-executive.md`
2. `prompts/p12-support-success-lead.md`
3. `prompts/p13-corporate-privacy-counsel.md`

Each parent owns only its persona. Its Subagents own different plan-defined
leaf scopes, with one writer per scope. Other personas' leases and unstaged
files are expected parallel state. No production task stages, commits, pushes,
updates the root corpus manifest, or requires global Git cleanliness.

## Parallel p01 allocation task

The separate prompt `prompts/p01-full-master-allocation.md` may run concurrently
as a fourth Local task. It claims only the p01 parent coordination lease and
creates no final artifact. Its sole purpose is to freeze and verify the complete
11,800-row p01 Full master assignment while retaining all 12 accepted pilot
rows. Do not launch a p01 mass artifact worker until that complete ledger passes
`full-ledger verify` and the coordinator accepts it.

## Coordinator wave end

Wait for every intended task to stop and normally release its own leases. Then:

1. require global `active_leases=0` twice across the idle interval;
2. verify p11--p13 ledgers and exactly 200 final files each;
3. verify p01 remained exactly 212 files if the allocation-only task ran;
4. reject unexpected paths, secrets, symlinks, nonregular entries, or blobs
   above 100 MiB;
5. regenerate and verify `manifests/corpus-sha256.jsonl`;
6. commit each accepted persona separately, followed by the updated manifest
   and portable evidence; and
7. do not push without explicit user authorization.

M1 and Full filesystem production do not establish Kio indexing, history,
search, actual chunk totals, or performance.
