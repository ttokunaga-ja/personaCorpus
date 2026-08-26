# Full production protocol

## Purpose and authority

This protocol governs the long-running **Full physical-artifact** phase after
a persona's M1 milestone.  It is an operational protocol, not a second plan.
`canonical/persona-plan.json` and `canonical/persona-render.json`, produced by
the accepted Rust implementation, remain the authority for persona IDs, role,
`raw_files`, Rust scope IDs/paths, source IDs, source family/variant, and
quotas.  The Rust plan's `raw_files` is the final physical-file target for a
persona; the Full addition is exactly:

```text
full_addition_count = persona.raw_files - 200
```

M1's existing 200 final files are reserved.  Their paths and bytes are
grandfathered and must remain byte-for-byte unchanged.  Never regenerate,
rename, move, delete, or “improve” M1 while producing Full additions.

This phase creates synthetic filesystem artifacts only.  It does **not**
establish Kio prepare/index/replay, actual chunk counts, search quality,
history/W0--W5 materialization, performance, or any readiness claim.  The
canonical schedule continues to be planning evidence only.

## Required preparation

Before a Full worker writes a batch, its parent freezes all of the following
under the persona's token-free operational area. Portable ledgers, accepted
manifests, and checkpoints are committed later by the coordinator; active
production tasks do not stage them:

```text
progress/<persona>/
  m1-assignment-ledger.jsonl
  full/
    content-spine.md
    canonical-source-inventory.jsonl
    m1-reservations.jsonl
    m1-reservation-reconciliation.jsonl  # accepted legacy exception only
    full-assignment-ledger.jsonl
    manifests/
      m1-baseline.jsonl
      <batch-id>.before.jsonl
      <batch-id>.after.jsonl
    checkpoints/
scratch/<persona>/full/
  qa/
```

The parent first verifies the accepted plan/render digests, creates an M1
baseline manifest, and confirms exactly 200 M1 files.  Use the create-only
`./bin/full-ledger` commands described in `FULL_LEDGER_SPEC.md`; do not replace
their output in place.  The canonical-source inventory records all accepted
Rust sources. The M1 assignment ledger is the sole machine-readable authority
for each frozen M1 artifact's ID, exact path, scope, declared family, physical
extension and state; never reconstruct family from extension alone (especially
not `.sql`). `m1-assignments.md` is derived from and subordinate to that ledger.
The separate M1 reservation ledger pairs those existing 200 files to source
IDs and SHA-256 values.  The content spine is a small,
token-free synthetic facts contract: organizations, aliases, projects, dates,
IDs, measures, terminology, and cross-format relationships.  It must not
contain credentials, private facts, or release tokens.

The tracked M1 baseline is created once, while the persona contains exactly M1's 200
files, and is never replaced by a later cumulative manifest. The
full-assignment ledger is immutable once production starts.  Before mass
expansion, it fixes every additional source ID, scope, final relative path,
family/variant, dependencies, and batch ID.  It is the sole worker assignment
input; a prompt may quote its batch rows but must not allocate ad hoc files.  A
correction requires a new, append-only revision record naming the predecessor
and the reason; it may never alter an accepted row or M1 reservation.  The
tracked p01 pilot assignment is an explicitly bounded 12-row exception used to
validate this machinery before the remaining 11,788 p01 additions are
allocated. The eventual complete p01 ledger has all 11,800 additions and
retains those 12 accepted pilot rows unchanged.

Strict M1 reservation pairing is by exact Rust scope and family. A legacy M1
file whose physical family differs from its canonical source must not be
silently coerced or regenerated. It requires a separate frozen reconciliation
record bound to the exact plan, render, inventory, M1 assignment and baseline
digests. Each exception preserves both physical and canonical family facts,
maps only within the same Rust scope, and is accepted only when exact scope
reservation counts and global reservation-family totals still reconcile with
the frozen M1 ledger. The
default has no reconciliation record and remains strict. When one exists,
pass it to `full-ledger reserve-m1` and every `full-ledger verify`; workers may
not edit it or add exceptions. The CLI accepts only a reviewed persona-specific
canonical path, reconciliation ID, and SHA-256 pinned in its trust anchor; a
different path, symlink, or otherwise valid balanced mapping is not an approved
substitute.

## Production order and artifact quality

For each scope, produce coherent batches in this dependency order:

1. Freeze the content spine and create the small base data needed by the batch
   (IDs, measures, dates, CSV/TSV or structured facts).
2. Create dependent skill artifacts: XLSX/DOCX/searchable PDF/PPTX consume the
   shared factual data; ImageGen creates original image assets; scan PDFs are
   derived from those image assets.
3. Create the batch's ordinary text, code, mail, and other non-skill artifacts
   using the same facts.
4. Record the actual manifest and the batch checkpoint only after structural
   and visual QA passes.

Use Documents for DOCX, PDF for PDF, Spreadsheets for XLSX, Presentations for
PPTX, and ImageGen for images and scan-source artwork.  Read each applicable
skill instruction before use.  Skill artifacts are real files, never a text
substitute with a rich extension.  Required QA is recorded per artifact family:
PDF page rendering/text or OCR suitability; DOCX rendering; XLSX sheet,
formula/error and type checks; PPTX rendering and overflow checks; image
dimensions, pixels and visual inspection.  Store temporary sources and rendered
inspection output only in `scratch/<persona>/<scope>/` or `scratch/<persona>/full/`.

The final Full profile includes 89,360 skill-routed files: 27,340 searchable
PDFs, 11,160 scan PDFs, 16,470 DOCX, 13,630 XLSX, 9,620 PPTX, and 11,140
images. M1 accounts for 2,004 of them, so Full additions must physically create
the remaining 87,356. These are authoritative-plan quotas to reconcile across
M1 reservations plus the Full assignment, not a claim that they have already
been generated or quality-approved.

## Batches, leases, and parallel ownership

Use bounded batches of a few hundred files (normally 100--500; lower only when
visual QA time or artifact complexity requires it). A parent may run workers
for different scopes concurrently as one acceptance group. The parent captures
one cumulative before manifest before any group member writes, then verifies
the union by passing one `--batch-id` argument per member after every member has
finished. Exactly one worker may own a given Rust scope ID at any instant, and
that worker writes only that scope's ledger rows under its exact
`home/<scope-path>/` leaf. Never accept one member against a manifest that also
contains an unselected concurrent member.

For **every batch**, the parent claims the scope lease immediately before
dispatch and retains the returned release token privately. For a sequential
batch, it validates, checkpoints, and normally releases that scope before the
next batch. For a concurrent acceptance group, it holds every member lease,
performs parent QA for every member, creates one cumulative after manifest,
verifies the selected union, writes each member's token-free checkpoint with
the shared group ID/manifests, then releases each scope normally. If any member
fails, no member is accepted. The parent lease is released only after its child
scope leases are absent. Tokens must never appear in prompts, JSONL, manifests,
logs, source files, Git, or subagent messages.

Leases belonging to other personas are expected during parallel production.
They are informational and must not be inspected, recovered, released, or used
as a reason to alter another persona's artifacts.  A parent checks its own
persona and scope lease state only.  The coordinator checks global
`active_leases=0` only after a whole parallel wave has intentionally ended.

An interrupted batch remains unaccepted: retain its before-manifest and report
the exact owner/session to the coordinator.  `lease recover` or `lease scope
recover` requires explicit user/coordinator approval for that named stopped
lease, one recovery only, followed by a fresh claim.  Do not edit `_control/`
or attempt manual recovery.

## Batch acceptance and checkpointing

Before work, create a cumulative SHA-256 JSONL manifest of the persona final
root and retain it as
`progress/<persona>/full/manifests/<batch-id>.before.jsonl`. After work, create
`<batch-id>.after.jsonl` and compare it against the complete master assignment.
Both manifests are create-only outputs from `./bin/full-ledger manifest`. For a
mass batch, run `./bin/full-ledger verify` with the immutable 200-file
`--baseline`, cumulative `--before`, complete `--assignment`, current
`--batch-id`, and `--after`. Repeat `--batch-id` for every member of a concurrent
acceptance group. The verifier requires every non-reserved canonical source
exactly once in a mass ledger; only the fixed 12-row p01 pilot is an
incomplete-ledger exception. It also regenerates the current persona-home
manifest and requires exact equality with the supplied after manifest.
Acceptance requires all of the following:

- only this batch's planned addition paths changed or appeared;
- all 200 M1 paths and SHA-256 values equal the M1 baseline;
- before contains the unchanged M1 baseline plus only previously allocated
  Full paths, and after equals before plus exactly the selected batch/group
  rows;
- no exact duplicate actual SHA-256 values;
- family/variant, scope, syntax/structure and applicable skill QA pass; and
- no placeholders, secrets, credentials, real PII/PHI, or unplanned files.

The checkpoint is an operational acceptance record, distinct from the
immutable source inventory and the actual byte manifest.  A worker can report
only token-free counts, failed paths, QA evidence locations, and manifest
digests.  Parents must not release a failed or incomplete batch as accepted.

Prepared p04--p20 packages additionally run the pinned, read-only
`bin/full-resume-gate` before any initial write and before every resume. It
requires one unambiguous append-only v2 acceptance chain and exact equality
between the live home and its latest trusted manifest. The v2 checkpoint and
final-image scan provenance contract is specified in `FULL_LEDGER_SPEC.md`.
This is a production safety gate, not a change to persona ownership, helper
trust anchors, ordinary scope lease claims, release-token handling, or recovery
approval requirements. p01--p03 have completed accepted v1/persona-isolated
chains; keep those closed histories unchanged and do not convert them to v2.

## Pilot and expansion gate

The p01 pilot is historical and persona-specific. Its first batch was the fixed 12-row assignment
in `prompts/p01-full-pilot-assignment.jsonl`; it contains base data plus every
required skill family and is intentionally smaller than an ordinary Full
batch.  It must preserve the 200-file M1 baseline and demonstrate normal
claim/QA/checkpoint/release handling.  Do **not** manufacture an interruption
to test recovery.  If a real interruption occurs, stop and apply the explicit
approval rule above. Passing this pilot authorizes construction of the complete
p01 assignment ledger, retaining the 12 pilot rows and allocating the remaining
11,788, and wider physical production only. It does not make
Kio/history/chunk assertions true.
