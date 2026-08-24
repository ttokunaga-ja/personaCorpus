# p01 Full structure v2 policy

## Purpose and scope

This revision makes the completed p01 Full corpus navigable as a realistic
software-engineering workspace without changing an artifact's contents,
declared family, extension, or canonical scope.  It applies only to the
11,788 non-pilot Full additions.  It does not create or remove artifacts.

## Immutable material

- The 200 accepted M1 artifacts remain at their existing paths and remain
  byte-frozen.
- The 12 accepted p01 Full pilot artifacts remain at their existing paths and
  remain byte-frozen.
- Each selected Full artifact is moved byte-for-byte within its existing
  canonical scope.  Its SHA-256 and byte count are therefore invariant.
- The frozen Full assignment ledger and its batch checkpoints remain
  historical allocation and acceptance evidence.  They are not rewritten to
  express this later filesystem organization.

## Current path authority

`path-revision.jsonl` is the append-only current-path overlay for the 11,788
selected Full additions.  Its header binds the overlay to the frozen Full
assignment and to the predecessor accepted manifest.  The live home path for
a selected addition is its `new_path` in that overlay; the old Full-ledger path
and earlier checkpoints remain historical evidence.  M1 and the 12 pilot rows
are outside the overlay and retain their original paths.

## Hierarchy rules

The revision retains the canonical p01 scope roots and adds only descendants
that reflect ordinary work organization:

- meaning-oriented areas such as architecture, API contracts, release work,
  repositories, decisions, code review, meetings, vendor material, migration,
  desktop, and reference material;
- format- or technology-oriented subareas such as specifications, tests,
  models, diagrams, prototypes, evidence, attachments, exports, captures, and
  fixtures; and
- time-oriented series beneath `2026-q3/<kind>/series-00` or `series-01`.

When a Full artifact is named by another Full artifact, both are co-located in
an `evidence/linked/cNNN` component.  The referenced target retains its
existing basename only where required for that internal reference to remain
valid.  This compatibility rule accounts for 307 preserved basenames; it does
not preserve the former flat directory layout.

## Invariants and acceptance criteria

The completed home contains 12,000 regular files, 756 leaf parents, and no
directory with more than 131 direct files.  It contains no symlink,
non-regular, or empty artifact.  The v2 audit must prove all of the following:

- 200 M1 and 12 pilot paths and bytes are unchanged;
- all 11,788 selected Full artifacts are present exactly once at their overlay
  paths with their predecessor bytes and SHA-256 values;
- no path crosses a canonical scope boundary; and
- all declared families and extensions remain at their fixed 12,000-file
  distribution.

The format distribution is unchanged: Markdown 2,640 (22%), TXT/log 960 (8%),
code 3,360 (28%), structured text 1,440 (12%), CSV/TSV 360 (3%), HTML/EML 600
(5%), notebook 120 (1%), searchable PDF 840 (7%), scan PDF 120 (1%), DOCX 360
(3%), XLSX 240 (2%), PPTX 240 (2%), PNG 360 (3%), and domain binary 360 (3%).
Consequently there are 960 PDFs in total (840 searchable and 120 scans), 360
DOCX files, 240 XLSX files, 240 PPTX files, and 360 PNG files.

## Evidence

- Current overlay: `path-revision.jsonl`
- Format and physical-artifact audit: `format-audit.json`
- Predecessor accepted manifest: `../../manifests/p01-full-wave-006.after.jsonl`
- Post-revision manifest: `p01-full-structure-v2.after.jsonl`
  (`sha256:d10f4a64c5e653d8050049467fdab19990c1daca437bc20caa6ed25190a4787b`).
