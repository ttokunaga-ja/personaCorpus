# Mac parallel wave 02

> **Historical completed-wave handoff. Do not execute this launch plan again.**
> p01--p03 Full and p11--p13 M1 have since completed and been accepted.

This file preserves the coordinator handoff that was used after the accepted
p01 Full pilot and p08--p10 M1 wave were migrated to Git. The sections below
describe the historical launch and acceptance procedure; they are not current
operator instructions.

## Historical coordinator preflight

The coordinator first confirmed the accepted baseline from the shared Local
root:

```bash
git status --short
./bin/check-ready
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```

The required state was a clean status, global `active_leases=0`, and a corpus
manifest result of `ok`; the wave was not started from a worktree.

## Historical M1 task launch

Three separate Codex Local tasks were opened against the same `personaCorpus`
repository root used by the coordinator. Each task's `pwd` was confirmed as
that root.

The task-to-prompt mapping was:

1. `prompts/p11-account-executive.md`
2. `prompts/p12-support-success-lead.md`
3. `prompts/p13-corporate-privacy-counsel.md`

Each parent owns only its persona. Its Subagents own different plan-defined
leaf scopes, with one writer per scope. Other personas' leases and unstaged
files are expected parallel state. No production task stages, commits, pushes,
updates the root corpus manifest, or requires global Git cleanliness.

## Historical parallel Full tasks

At that point, the p01 resume-capable production prompt ran as another Local
task. It retained the accepted 12-file pilot and produced the remaining 90
batches from the frozen 11,800-row master. That production is now complete;
neither its allocation, pilot, nor production prompt may be rerun against the
accepted p01 home.

The p02 allocation task also ran independently, verified its frozen M1 inputs,
and produced the complete 14,800-row Full master. Its later physical production
is now complete as well. These p02 allocation and production prompts are
historical records and may not be rerun against the accepted p02 home.

## Historical coordinator wave end

The coordinator waited for every intended task to stop and normally release its
own leases, then applied these acceptance checks:

1. require global `active_leases=0` twice across the idle interval;
2. verify p11--p13 ledgers and exactly 200 final files each;
3. if p01 production did not start, verify it remained exactly 212 files; if it
   ran, require the production task to have stopped with its own leases normally
   released, then reconcile its current files exactly to the accepted batch
   checkpoints (12,000 files when fully complete);
4. verify p02 remained exactly 200 files if its allocation-only task ran;
5. reject unexpected paths, secrets, symlinks, nonregular entries, or blobs
   above 100 MiB;
6. regenerate and verify `manifests/corpus-sha256.jsonl`;
7. commit each accepted persona separately, followed by the updated manifest
   and portable evidence; and
8. do not push without explicit user authorization.

M1 and Full filesystem production do not establish Kio indexing, history,
search, actual chunk totals, or performance.
