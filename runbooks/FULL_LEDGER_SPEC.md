# Full ledger specification

## Record separation

Full production uses five records. Their purposes are deliberately separate;
never merge them or overwrite an earlier record.

| Record | Path | Meaning |
| --- | --- | --- |
| Canonical source inventory | `progress/<persona>/full/canonical-source-inventory.jsonl` | Read-only projection of all plan/render source IDs, scopes, families and variants. |
| M1 assignment ledger | `progress/<persona>/m1-assignment-ledger.jsonl` | Sole machine-readable M1 allocation authority: frozen artifact ID, path, scope, declared family, physical extension and state; no source reservation yet. |
| M1 reservations | `progress/<persona>/full/m1-reservations.jsonl` | Deterministic pairing of the existing 200 M1 paths and bytes to canonical source IDs. |
| M1 reservation reconciliation | `progress/<persona>/full/m1-reservation-reconciliation.jsonl` | Rare, frozen, digest-bound exception mapping for accepted legacy M1 files whose physical family differs from the canonical family, while preserving the same Rust scope and all aggregate totals. |
| Fixed Full assignment | `progress/<persona>/full/full-assignment-ledger.jsonl` | Frozen destination path, batch and dependency allocation for each Full addition. |
| Actual manifests/checkpoints | `progress/<persona>/full/manifests/m1-baseline.jsonl`, `<batch-id>.{before,after}.jsonl`, and `progress/<persona>/full/checkpoints/<batch-id>.json` | Immutable M1 bytes, cumulative observed bytes, and token-free batch acceptance evidence. |

All JSONL is UTF-8 with exactly one JSON object per line. Paths are POSIX paths
relative to the selected persona's `home/`. Records must contain no release
token, credentials, real PII/PHI, or secret. The accepted Rust plan/render
remain authoritative; these records make their facts actionable but do not
claim Kio indexing, actual chunks, history, or renderer-byte equality for
skill-generated files.

## Create-only helper

`./bin/full-ledger` has no lease or recovery API. It verifies the accepted
plan/render/owner digests and publishes new records with create-only semantics:
`reserve-m1` and `verify` independently regenerate the supplied inventory from
the current canonical plan/render and require byte-for-byte equality, so a
same-count or same-header edited inventory cannot be used.
It also requires `canonical/materialized/persona-materialization.json`: its
plan SHA-256 must pin the current canonical plan and its render SHA-256 must
pin the current canonical render bytes.

Historical p03 production used the mechanically isolated
`./bin/p03-full-ledger` for every ledger operation while the p01 and p02 waves
were active. It retained the same validation and create-only behavior, kept
p02's anchor byte-compatible, and added only the closed p03 reconciliation
anchor. That accepted p03 chain is closed: preserve the isolated helper as
provenance, but do not rerun or convert the completed chain without an
explicitly approved revision.

```bash
./bin/full-ledger inventory --persona p01 \
  --out /absolute/progress/p01/full/canonical-source-inventory.jsonl

./bin/full-ledger manifest --persona p01 \
  --out /absolute/progress/p01/full/manifests/m1-baseline.jsonl

# Legacy backfills only; new M1 tasks already publish this frozen ledger.
./bin/p01-m1-assignment \
  --out /absolute/progress/p01/m1-assignment-ledger-v2.jsonl

./bin/p02-m1-assignment \
  --out /absolute/progress/p02/m1-assignment-ledger.jsonl

./bin/p03-m1-assignment \
  --out /absolute/progress/p03/m1-assignment-ledger.jsonl

./bin/full-ledger reserve-m1 --persona p01 \
  --inventory /absolute/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /absolute/progress/p01/m1-assignment-ledger-v2.jsonl \
  --baseline /absolute/progress/p01/full/manifests/m1-baseline.jsonl \
  --out /absolute/progress/p01/full/m1-reservations-v3.jsonl

# Strict scope/family pairing is the default. Only a frozen, accepted legacy
# exception may pass the optional reconciliation record. p02 uses the shared
# helper below; p03 uses the isolated helper in the following example.
./bin/full-ledger reserve-m1 --persona p02 \
  --inventory /absolute/progress/p02/full/canonical-source-inventory.jsonl \
  --m1-assignment /absolute/progress/p02/m1-assignment-ledger.jsonl \
  --baseline /absolute/progress/p02/full/manifests/m1-baseline.jsonl \
  --reconciliation /absolute/progress/p02/full/m1-reservation-reconciliation.jsonl \
  --out /absolute/progress/p02/full/m1-reservations.jsonl

# p03 uses its isolated helper and reviewed two-row reconciliation.
./bin/p03-full-ledger reserve-m1 --persona p03 \
  --inventory /absolute/progress/p03/full/canonical-source-inventory.jsonl \
  --m1-assignment /absolute/progress/p03/m1-assignment-ledger.jsonl \
  --baseline /absolute/progress/p03/full/manifests/m1-baseline.jsonl \
  --reconciliation /absolute/progress/p03/full/m1-reservation-reconciliation.jsonl \
  --out /absolute/progress/p03/full/m1-reservations.jsonl

# Validate the complete mass ledger before any mass worker starts. This command
# accepts an incomplete ledger only for the exact tracked p01 12-row pilot.
./bin/full-ledger verify --persona p01 \
  --inventory /absolute/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /absolute/progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations /absolute/progress/p01/full/m1-reservations-v3.jsonl \
  --assignment /absolute/progress/p01/full/full-assignment-ledger.jsonl \
  --baseline /absolute/progress/p01/full/manifests/m1-baseline.jsonl

# At each mass batch boundary, first create <batch-id>.before.jsonl, then select
# that batch from the same complete master ledger. After production, create the
# cumulative after manifest and repeat with --after.
./bin/full-ledger verify --persona p01 \
  --inventory /absolute/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /absolute/progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations /absolute/progress/p01/full/m1-reservations-v3.jsonl \
  --assignment /absolute/progress/p01/full/full-assignment-ledger.jsonl \
  --baseline /absolute/progress/p01/full/manifests/m1-baseline.jsonl \
  --batch-id <batch-id> \
  --before /absolute/progress/p01/full/manifests/<batch-id>.before.jsonl \
  --after /absolute/progress/p01/full/manifests/<batch-id>.after.jsonl
```

For concurrently produced batches, capture one shared before manifest and
repeat the option, for example `--batch-id p01-primary-01-b003 --batch-id
p01-primary-02-b004`. The selected union is accepted atomically against one
shared after manifest; never verify only one member after another member has
already written. `verify --after` independently regenerates the current
persona-home manifest, so a hand-written or stale after JSONL cannot pass.

The output directory must already exist. If an output path exists, verify and
reuse it or choose a revisioned path; never delete it merely to rerun a command.

## Canonical source inventory

`full-ledger inventory` reads the selected persona from
`canonical/persona-render.json`, sorts by `source_id`, and records every source.
Its source-row schema is `persona-corpus.full-inventory/v1` and includes:

```json
{"schema":"persona-corpus.full-inventory/v1","kind":"source","persona":"p01","source_id":"p01-src-000001","scope_id":"p01-primary-01","scope_path":"documents/work/product-alpha/architecture","family":"md","canonical_variant":"md","canonical_extension":"md","media_type":"text/markdown","planned_chunks":1,"planned_identity":"sha256:<digest>","renderer_byte_digest":"sha256:<digest>"}
```

This inventory is not an assignment and does not itself say whether a source
belongs to M1 or to a Full addition. It is an immutable canonical projection:
pass only the create-only output of `full-ledger inventory` to `reserve-m1` or
`verify`; both commands reject any byte-level divergence from a freshly
regenerated projection.

## M1 reservation construction

`full-ledger reserve-m1` requires a 200-file baseline and a frozen
`persona-corpus.m1-assignment/v1` ledger. The ledger comes from the frozen
pre-production M1 assignment and gives every stable artifact ID, home-relative
path, exact Rust scope ID/path, declared M1 family, physical extension, and
`state:"frozen"`. Extension inference is not authoritative: for example, an
accepted `.sql` may have been assigned as `code` in one M1 scope and as
`structured_text` in another. For PDFs, the helper additionally verifies that
the bytes match the declared searchable or image-only family. It then pairs
records within
each `(scope_id, family)` group:

1. sort M1 physical paths lexically;
2. sort canonical sources by `source_id`;
3. pair the two sequences positionally; and
4. fail if a scope/family is ambiguous, unknown, or has too few canonical
   sources, or if the final reservation count is not exactly 200.

New M1 parents must publish this frozen ledger before launching workers, then
derive `m1-assignments.md` from it for human display. Existing M1 personas need
a checked backfill from their frozen assignment; p01, p02 and p03 use the
create-only persona-specific helpers `./bin/p01-m1-assignment`,
`./bin/p02-m1-assignment`, and `./bin/p03-m1-assignment`. Because p02 and p03
are already Full-complete, their helpers first require the entire physical home
path set to match that persona's accepted root corpus-manifest subset, then
require all 200 frozen M1 paths within it. The reservation header binds
`m1_assignment_sha256`; an M1 ledger and a reservation cannot be mixed across
revisions.

The reservation schema is `persona-corpus.full-m1-reservations/v1`. Each row preserves the
grandfathered M1 path, byte count, and SHA-256 alongside the paired source ID.
The original extension and bytes remain authoritative for that accepted M1
file even if they differ from canonical renderer bytes. Never rename or
regenerate M1 to force parity.

### Exceptional M1 reconciliation

The normal rule is strict: every M1 file pairs positionally inside its exact
`(scope_id, family)` group. A mismatch must fail; an extension, display name,
or a same global total is never an implicit exception.

An accepted legacy mismatch may be reconciled only with the optional,
create-only `persona-corpus.full-m1-reservation-reconciliation/v1` record. Its
header is bound to the exact plan, render, inventory, M1-assignment, and
baseline SHA-256 values and declares one named policy. Each frozen exception
row records both the physical M1 scope/path/family/extension and the canonical
source scope/family/variant/extension, plus a reason. The exception must stay
inside the same Rust scope; the record must be one-to-one; all non-exception
rows remain strict positional pairings; and the M1 ledger, reconciliation, and
reservation records together preserve both the accepted physical M1 facts and
the canonical source facts.

`same-scope-global-family-balanced-v1` is the currently accepted policy. It
permits only the explicit rows in the digest-bound record after checking exact
per-scope reservation counts and global reservation-family totals against the
frozen M1 ledger. It does not permit an arbitrary family swap, an out-of-scope source,
or a new exception after production begins. `reserve-m1` and every `verify`
invocation must receive the same reconciliation record; its digest is bound in
the reservation header. Omitting `--reconciliation` retains the strict default.

Self-consistency is not sufficient approval. `full-ledger` also pins every
accepted reconciliation by persona, canonical repository path,
`reconciliation_id`, and exact SHA-256. It rejects alternate paths, symlinks,
unapproved-but-balanced mappings, and other personas. The approved file is
copied once to a private snapshot, then the snapshot alone is hashed and
parsed, so a concurrent pathname replacement cannot change the mapping being
validated. The shared helper's current allowlist contains only p02's reviewed
four-row record. The isolated p03 helper contains that identical p02 anchor plus
p03's reviewed two-row record; all p03 inventory, manifest, reservation, and
verify operations must use the isolated helper until a coordinated idle migration.

## Fixed Full assignment

For mass expansion, allocate each non-reserved source exactly once. A mass
preflight succeeds only when the assignment source-ID set equals the canonical
inventory minus the 200 reservations; a valid one-row subset is not a complete
ledger. Every path
must be unique, lie below the canonical `scope_path`, use the canonical physical
extension, avoid every M1 path, and have a stable batch ID. Ordinary Full
batches normally contain 100--500 rows from one scope; smaller batches are
allowed for expensive visual artifacts and the first p01 pilot.

Use `dependencies` for cross-format production order. Dependencies must resolve
to an `artifact_id` or `source_id` inside the same complete assignment input.
For a selected batch, an external dependency's allocated path must already be
present in the cumulative before manifest. Base
structured/CSV facts precede XLSX, DOCX, searchable PDF and PPTX; an ImageGen
source image precedes a scan PDF derived from it. These edges are operational,
not canonical W0--W5 history edges.

Assignment rows use `persona-corpus.full-addition-assignment/v1`:

```json
{"schema":"persona-corpus.full-addition-assignment/v1","kind":"artifact","persona":"p01","artifact_id":"p01-full-000201","source_id":"p01-src-000201","scope_id":"p01-primary-01","scope_path":"documents/work/product-alpha/architecture","family":"pptx","canonical_variant":"pptx","canonical_extension":"pptx","physical_extension":"pptx","path":"documents/work/product-alpha/architecture/2026-08-architecture-review-0201.pptx","batch_id":"p01-primary-01-b003","dependencies":["p01-full-000198"],"skill_required":"presentations","state":"planned"}
```

Once a row has entered production, do not edit it. A pre-write allocation
correction is a new append-only revision naming its predecessor and reason. If
the old path was already written, stop for coordinator direction.

The tracked p01 pilot assignment is a bounded historical exception: it allocates
12 verified tail source IDs, all outside the deterministic M1 reservations. The
completed 11,800-row p01 master retains those 12 rows exactly and adds the other
11,788 rows. p01 now has 12,000 accepted files; neither the pilot nor master is
a start instruction for another production run.

## Actual manifests and checkpoints

`full-ledger manifest` rejects symlinks and every non-directory/non-regular
entry below persona `home/`, then emits `persona-corpus.full-manifest/v1`
JSONL: one header and one file row containing the home-relative path, byte
count, and SHA-256 for every observed final file. The immutable M1 baseline has
exactly 200 rows. Every batch before/after manifest is cumulative and therefore
also protects accepted Full additions from later modification. An accepted
batch then receives one token-free
checkpoint, for example:

```json
{"schema":"persona-corpus.full-batch-checkpoint/v1","kind":"checkpoint","persona":"p01","scope_id":"p01-primary-01","batch_id":"p01-primary-01-b003","assignment_count":240,"accepted_additions":240,"m1_file_count":200,"m1_sha256_unchanged":true,"before_manifest":"progress/p01/full/manifests/p01-primary-01-b003.before.jsonl","after_manifest":"progress/p01/full/manifests/p01-primary-01-b003.after.jsonl","qa":{"pdf":true,"docx":true,"xlsx":true,"pptx":true,"image":true,"structure":true},"accepted_at":"2026-08-23T12:00:00Z","notes":"token-free; physical-artifact evidence only"}
```

The v1 example above is retained only for accepted p01--p03 v1 evidence. Do
not rewrite, convert, or mix those completed persona-isolated chains.

For prepared p04--p20 Full packages, `bin/full-resume-gate` is the required
read-only, fail-closed start/resume gate. Its pinned SHA-256 is
`3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069`.
It accepts only one append-only checkpoint chain rooted at the immutable M1
baseline and requires the live persona home to be byte-identical to the unique
latest trusted manifest. A fork, gap, duplicate checkpoint, uncheckpointed
delta, stale after manifest, symlink, nonregular file, cache, or modified prior
artifact is a stop condition, not a recovery invitation.

Each p04--p20 accepted batch uses exactly the following checkpoint v2 fields:

```json
{"schema":"persona-corpus.full-batch-checkpoint/v2","kind":"checkpoint","persona":"pNN","plan_digest":"sha256:<64-lowercase-hex>","scope_id":"pNN-primary-01","batch_id":"pNN-primary-01-full-b001","assignment_count":120,"accepted_additions":120,"m1_file_count":200,"m1_sha256_unchanged":true,"acceptance_group":"pNN-group-001","before_manifest":"progress/pNN/full/manifests/pNN-group-001.before.jsonl","after_manifest":"progress/pNN/full/manifests/pNN-group-001.after.jsonl","before_manifest_sha256":"<64-lowercase-hex>","after_manifest_sha256":"<64-lowercase-hex>","family_counts":{"md":120},"qa":{"structure":true,"family":true,"content_spine":true,"dependencies":true,"skill_required":true,"secrets_pii":true},"accepted_at":"2026-08-24T00:00:00Z","scan_provenance":[]}
```

The object has no optional or extra fields. `family_counts` must equal the
master rows for that batch. Every `qa` value is required and true. The
manifest references must be either their basename or the shown logical
repository-relative path; their bytes are pinned by the two manifest SHA-256
fields. All members of one `acceptance_group` share identical before/after
manifest paths and byte digests.

When `--require-scan-provenance` is used, every accepted `pdf_scan` row has one
v2 `scan_provenance` object with exactly these fields:
`scan_artifact_id`, `scan_path`, `scan_sha256`, `source_kind`,
`dependency_artifact_id`, `image_path`, `image_sha256`, `pixel_qa`,
`source_pixel_sha256`, and `rendered_pixel_sha256`. It proves that the scan
extracts exactly one image whose normalized RGB pixels equal its one final,
same-scope image dependency; scratch-only scan sources are not accepted.
`source_kind` is exactly `dependency`; `pixel_qa` is exactly
`{"passed":true,"width":<positive-int>,"height":<positive-int>,"method":"normalized-rgb-sha256/v1"}`.
The dependency must be a declared final `image` artifact accepted in an earlier
acceptance group, and the PDF must extract exactly one raster image with the
same normalized RGB digest and dimensions.

Write a checkpoint only after parent QA. Release tokens stay solely in the
parent process and never appear in any record. A failed or interrupted batch
gets a token-free failure note and no accepted checkpoint; recovery remains an
explicit user/coordinator decision. Concurrent members each keep their own
checkpoint but record the same `acceptance_group`, before-manifest and
after-manifest; none is written until the selected union passes. The p04--p20
package work changes neither helper trust anchors nor lease/recovery rules.

## Reconciliation gates

Before the p01 pilot and after every accepted batch, verify:

- canonical inventory source rows equal `persona.raw_files`;
- the M1 assignment ledger, baseline and M1 reservation ledger each contain exactly
  200 matching unique paths and M1 SHA-256 values remain unchanged;
- when a reconciliation record is supplied, its header digest bindings, frozen
  exception set, same-scope constraint, exact scope reservation totals and
  global reservation-family totals all match the frozen M1 ledger; otherwise
  strict `(scope_id, family)` pairing is required;
- a mass assignment contains every canonical non-reserved source ID exactly
  once; only the exact tracked 12-row p01 pilot may be incomplete;
- assignment paths are unique, canonical-scope-local, extension-compatible,
  and absent from the baseline;
- every dependency resolves within the complete assignment input, and an
  out-of-batch dependency is already present in the cumulative before manifest;
- the immutable 200-file M1 manifest is an unchanged subset of every batch
  before/after manifest;
- only the union of the selected `--batch-id` paths appeared between cumulative
  manifests;
- family/scope totals reconcile to canonical sources before mass expansion.

This proves operational allocation and observed local files only. It is not
evidence for actual Kio chunk totals, search quality, history, or performance.

## Regression validation safety

`./tests/full-ledger.sh` exercises manifest rejection and cumulative batch
acceptance by creating uniquely named, short-lived entries in the live p01
home. Run it only while p01 production is stopped. The script refuses to start
if any p01 parent/scope lease is already active, atomically claims the p01
parent lease for its full mutation window, and retains the release token only
in its shell process until normal release. It also holds
`scratch/p01/.full-ledger-test.lock`, so two validator processes cannot race to
claim. If the lock exists with no validator process, or the test reports that
normal lease release failed, stop and inspect it as interrupted-test state; do
not bypass the guard or force recovery without explicit coordinator approval.
