# p11 Full physical-artifact production

Paste this complete file into **one new Codex Local task** opened at
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`. Do not use a
worktree. You own only `p11` and must continue autonomously through safe,
accepted batches until either the 10,000-file completion gate is met or a
documented stop condition requires coordinator action.

## Fixed authority

Read `AGENTS.md`, `README.md`, `runbooks/COMMON_RULES.md`,
`runbooks/BATCH_PROTOCOL.md`, `runbooks/FULL_PRODUCTION_PROTOCOL.md`,
`runbooks/FULL_LEDGER_SPEC.md`, and `runbooks/personas/p11-account-executive.md`.
The canonical plan/render and these frozen p11 records are authoritative:

| Record | SHA-256 / invariant |
| --- | --- |
| M1 assignment ledger | `cc60af8c9044aab4ea9896c17594936c32c44d90a4631092d5f83ee241abc63a` |
| Canonical source inventory | `3e6b360840c5afef3528f477619de481b247f7ba34c6b8303fe18880584164fc` (10,000 sources) |
| Immutable M1 baseline | `1283e3214117c2fc6573bf78aaa3c6e828fda51b1339abf371f95c416ff9ca58` (200 files) |
| Strict M1 reservations | `dff69675648d9b6f9790d394929736579e7f2bfc39834e32f6519e773adedbf9` (200 rows) |
| Content spine | `00a44fd62c4a508801a4e9c5fc678aa79e77be3c234daf315b260bec4261d5ef` |
| Full master assignment | `7079db0873159ffea259b7ef983f3d901afc99799a8546ff846ee6beb9963381` (9,800 rows, 120 batches) |
| Shared `bin/full-ledger` | `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` |

Use the unchanged shared `./bin/full-ledger` for every p11 inventory, manifest,
reservation, and verification operation. There is **no** p11 reconciliation
file: strict same-scope/family reservations are the accepted state. Do not alter
any ledger, spine, canonical record, script, prompt, or M1 artifact.

Before claiming a lease, fail closed if the shared helper or any fixed record
does not match the table. Then run the following read-only verification. Do not
replace, bypass, or locally edit the shared helper to make this pass.

```sh
expected_p11_full_ledger_sha=f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2
actual_p11_full_ledger_sha=$(shasum -a 256 ./bin/full-ledger | awk '{print $1}')
if [ "$actual_p11_full_ledger_sha" != "$expected_p11_full_ledger_sha" ]; then
  printf '%s\n' "p11 preflight: bin/full-ledger SHA-256 mismatch" >&2
  exit 1
fi
./bin/full-ledger verify --persona p11 \
  --inventory progress/p11/full/canonical-source-inventory.jsonl \
  --m1-assignment progress/p11/m1-assignment-ledger.jsonl \
  --reservations progress/p11/full/m1-reservations.jsonl \
  --assignment progress/p11/full/full-assignment-ledger.jsonl \
  --baseline progress/p11/full/manifests/m1-baseline.jsonl
```

## Start and ownership

1. Run `./bin/check-ready`. Other personas may have active leases; do not
   inspect, recover, release, or wait on them.
2. Confirm p11 `home/` contains exactly 200 regular files, no symlinks or
   nonregular entries, and regenerate a temporary manifest to prove it equals
   `m1-baseline.jsonl` byte-for-byte.
   On resume, first reconstruct accepted batches solely from token-free,
   create-only checkpoints and their before/after manifests. If a checkpoint,
   manifest pair, selected-batch verification, or current cumulative manifest
   is missing or inconsistent, stop before claiming additional scope work.
3. Claim only a p11 parent lease with a unique session ID. Retain every release
   token only in the live parent process. Never print or record a token.
4. Claim a p11 scope lease immediately before assigning that exact scope to one
   worker. Workers write only their assigned `home/<scope-path>/` leaf.

Never use `lease recover` or edit `_control`. If a parent or scope token is lost,
or an external-path/misplacement/unplanned-file condition occurs, stop and report
the exact lease/session and evidence for coordinator approval.

## Production scheduler

The master ledger is the sole assignment input. It contains 9,800 additions; M1
is immutable, so final target is exactly 10,000 regular artifacts. Work only
ready batches: every dependency must already be accepted, be same-scope, and be
in a strictly earlier batch. Run independent ready scopes concurrently when
useful, but one worker owns one scope at a time.

For each sequential batch or concurrent acceptance group:

1. Create one cumulative `before` manifest under
   `progress/p11/full/manifests/` using create-only `full-ledger manifest`.
2. Create only selected master rows below the designated p11 final scope leaf.
   Never write to Kio, another persona, a different workspace, or an external
   path. Temporary source images and renders live only under `scratch/p11/`.
3. Perform family QA, then create an `after` manifest and run `full-ledger verify`
   with baseline, reservations, master, before/after, and every selected batch ID.
4. Accept only when the selected union is exact, M1 remains byte-identical, no
   duplicate SHA-256 values exist, and a token-free checkpoint records manifest
   digests, selected batch IDs, counts, and QA evidence paths.
5. Only after acceptance, normally release the relevant scope lease(s). Do not
   release or call an incomplete/failed batch accepted.

Do not start an arbitrary later batch to keep workers busy. Resume from existing
accepted checkpoints; never overwrite a create-only manifest/checkpoint.

## Skill and quality rules

Use Documents for DOCX, PDF for searchable and scan PDFs, Spreadsheets for XLSX,
Presentations for PPTX, and ImageGen for every image and any scan-source artwork.
Read each required SKILL.md before first use. All are real native artifacts:

- Render and inspect all PDF pages; scan PDFs have no accidental text layer and
  match their ImageGen source pixels.
- Render DOCX pages; open XLSX, inspect sheets/formulas/errors/cache and sheet
  names; render PPTX and verify no overflow; inspect images at original size.
- Validate all code, structured text, CSV/TSV, mail, media, and domain binaries
  by their native parser or safe structural validator.
- Never use `py_compile`, imports, or any action that writes `__pycache__`/`.pyc`
  into the final tree. Set `PYTHONDONTWRITEBYTECODE=1` for Python QA. Remove no
  final artifact as routine cleanup.

Use only synthetic defensive commercial content from the p11 content spine:
`.invalid` mail aliases, no real PII, secrets, credentials, private contracts,
or routable addresses. Do not claim Kio index/history/search/chunk/performance.
Respect its scope mapping: Account Alpha renewal scopes use the 92% planning
measure, Account Beta/pipeline scopes use 3.1x coverage, and pricing/contract
scopes use the 8.0% approved discount. Do not add unrelated commercial facts
merely to make every file look alike.

## Boundaries and completion

Do not stage, commit, push, create a worktree, modify Git configuration, update
root corpus manifests, or run Kio evaluation. Do not modify any M1 path.

Completion requires all of the following: exactly 10,000 p11 regular final files;
200 immutable M1 baseline files; all 9,800 master rows accepted through
checkpoints; exact manifest verification; unique SHA-256 values; all family and
skill QA passed; no placeholders, secrets, PII, symlinks, nonregular entries,
unplanned files, or bytecode caches; and p11 parent/scope leases absent after
normal release. Report only token-free evidence and leave all changes unstaged.

## Mandatory v2 acceptance and resume gate

This executable gate supersedes earlier initial/resume prose. Before any lease claim or final write, before
every group, and after writing checkpoints before release, require the pinned gate (an absent checkpoint
directory is valid initial state):

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p11 \
 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p11-account-executive/home \
 --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p11/full/manifests/m1-baseline.jsonl \
 --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p11/full/full-assignment-ledger.jsonl \
 --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p11/full/checkpoints \
 --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p11/full/manifests \
 --require-scan-provenance
```

Before every initial/candidate/official verify, re-pin shared `bin/full-ledger` to
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` and run its exact absolute-path verifier
command with the applicable before/every group batch-id/after. Each master batch gets exactly one create-only
`persona-corpus.full-batch-checkpoint/v2` file named `<batch-id>.json`: exact batch/scope, common group
before/after, plan_digest, exact counts/family_counts, six all-true QA keys
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after byte
SHA fields, and scan_provenance. Every pdf_scan requires exactly one master-declared final image dependency
accepted in an earlier group (not scratch_input), and a record binding artifact/path/file SHA and
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` with source/rendered normalized-RGB digests.
Stop on any mismatch. Release only after the post-checkpoint gate passes; completion requires the checkpoint
batch-ID set to equal the master batch-ID set exactly.
