# p02 Full structure v2 migration report

## Result

p02 Full structure v2 reorganizes the completed persona home from a mostly
scope-flat Full layout into a realistic SRE workspace hierarchy. The operation
is a byte-preserving path migration, not content production or regeneration.

| Item | Result |
| --- | ---: |
| Total regular artifacts | 15,000 |
| Fixed M1 artifacts | 200 |
| Byte-preserving Full moves | 14,600 |
| Reference-bound Full paths retained | 200 |
| File-bearing directories before / after | 160 / 999 |
| Largest direct-file count before / after | 405 / 82 |
| Allocation-suffix filenames before / after | 14,800 / 200 |
| Symlinks / non-regular artifacts / empty artifacts | 0 / 0 / 0 |
| Empty directories / duplicate in-home inodes | 0 / 0 |
| Final artifacts with link count greater than one | 0 |

The current paths are supplied by `path-revision.jsonl`. The frozen Full
assignment ledger remains portable allocation authority, while the six
immutable wave before/after manifests and the post-revision manifest provide
portable acceptance evidence. The 85 raw batch checkpoints remain local
historical receipts because some contain machine-specific render paths; none
of these records was rewritten by this organization-only revision.

## Layout outcome

Within each unchanged canonical scope root, moved artifacts now group under
their existing work area, then an approved date or quarter bucket, and finally
a content-kind directory. Examples include:

- `documents/operations/runbooks/evidence/2026-07-13/logs/`
- `documents/operations/runbooks/automation/2026-07-13/source/`
- `infrastructure/kubernetes/clusters/evidence/2026-07-15/logs/`
- `services/.../incident-reviews/2026-07-13/documents/`

Naturalized filenames no longer expose the internal `-sNNNNNN` allocation ID,
and redundant tokens such as an extension name immediately before that
extension were removed. Colliding natural names receive a deterministic
`-2`, `-3`, and later suffix. The 200 retained allocation-style names are
restricted to immutable embedded-reference targets or referrers; preserving
them prevents broken links without altering artifact bytes.

The longest home-relative path is 158 characters and the longest basename is
87 characters. There are no cross-scope moves.

## Format result

The structure revision leaves the fixed 15,000-file format distribution
unchanged.

| Format / classification | Count | Corpus share |
| --- | ---: | ---: |
| Searchable PDF | 600 | 4% |
| Scan PDF | 0 | 0% |
| DOCX | 300 | 2% |
| XLSX | 150 | 1% |
| PPTX | 150 | 1% |
| PNG image | 300 | 2% |

Legacy `.doc` and `.xls` files are not allocated (0 each); document and
spreadsheet artifacts use the portable OOXML `.docx` and `.xlsx` forms.

The remaining family ratios also match the frozen plan exactly: Markdown 20%,
TXT/log 22%, code 15%, structured text 20%, CSV/TSV 5%, HTML/EML 3%, and
domain binary 5%. Notebook and media artifacts are not allocated for p02.

## Verification record

`format-audit.json` records `result: ok` for all 15,000 artifacts. It verifies
the exact family and extension histograms, SHA-256 uniqueness, searchable PDF
classification, OOXML container/part structure, fully decoded PNG structure,
and PCAP packet records. Its SHA-256 is
`80f9f275df31a3177ccaa12d06f6507609ec97fa0a4cf1ed184dbfef4e04637e`.

The overlay header binds the revision to:

- Full assignment `deb618bdd9d7452ea93936494a5b0caf2c1adb39e454aeb12cf1eb79ab806e91`;
- M1 baseline `6030fddb66249a6f480ea34ccd08ded56402f2cfa6d9e365700b237e22968fc2`;
  and
- accepted predecessor manifest
  `f3ec038ca49534d2c2cec07e8145ec40ecfd3cdc3283880c043e7695210e41d0`.

The post-revision manifest is `p02-full-structure-v2.after.jsonl`, SHA-256
`aef2c95e46e6b3a53048ec8b840c9fc968efe0dc28cd021edade83466e4de7ca`.
Its 15,000 `(SHA-256, byte-count)` members exactly equal the predecessor
manifest, and all 200 M1 rows remain at the same paths with the same bytes.
Because every PDF, DOCX, XLSX, PPTX, and PNG hash is unchanged, the prior
full-production render/visual-QA evidence remains bound to the same physical
artifacts; the v2 audit adds fresh path, container, and classification checks.

Before Git staging, 12,932 paths that shared pre-existing inodes with ignored
local QA/staging material were atomically materialized as independent files.
The operation copied 663,816,113 bytes, left the staging entries untouched,
and rechecked every one of the 15,000 final files against the post-revision
manifest. The final link-count histogram is therefore `{1: 15000}` with no
content-hash change.

## Scope and exclusions

This report covers only p02's `home` tree and token-free p02 revision evidence.
The structure operation itself did not update the repository root corpus
manifest, stage or commit Git changes, touch another persona, or claim Kio
indexing, history, search, chunking, or performance results. Git migration and
root-manifest refresh are separate coordinator actions.
