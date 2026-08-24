# p01 Full structure v2 migration report

## Result

p01 Full structure v2 reorganizes the completed persona home from a
scope-flat Full layout into a realistic workspace hierarchy.  The operation is
a byte-preserving path migration, not content production or regeneration.

| Item | Result |
| --- | ---: |
| Total regular artifacts | 12,000 |
| Fixed M1 artifacts | 200 |
| Fixed accepted Full pilot artifacts | 12 |
| Byte-preserving Full moves | 11,788 |
| Compatibility basenames retained for internal Full references | 307 |
| Leaf parents after organization | 756 |
| Largest direct-file count in one directory | 131 |
| Symlinks / non-regular artifacts / empty artifacts | 0 / 0 / 0 |

The current paths are supplied by `path-revision.jsonl`.  The frozen Full
assignment ledger and accepted batch checkpoints remain historical authority
for the original allocation and acceptance chain; they are not altered by this
organization-only revision.

## Layout outcome

Within each unchanged canonical scope root, artifacts now group by work
meaning, then by format or technology, and where applicable by
`2026-q3/<kind>/series-00` or `series-01`.  Examples of the grouping classes
are architecture, API specifications and tests, repository documentation and
fixtures, decision records, code-review exports, meeting records, vendor
material, migration material, and desktop/reference material.

The 307 retained compatibility basenames are restricted to Full-to-Full
reference targets.  Their linked artifacts are co-located under
`evidence/linked/cNNN`, so references remain valid without editing artifact
bytes.  No M1 or accepted pilot artifact is moved or renamed.

## Format result

The structure revision leaves the 12,000-file format distribution unchanged.
The validated real-file extension counts include:

| Extension / classification | Count |
| --- | ---: |
| PDF | 960 |
| Searchable PDF | 840 |
| Scan PDF | 120 |
| DOCX | 360 |
| XLSX | 240 |
| PPTX | 240 |
| PNG | 360 |

Legacy `.doc` and `.xls` files are not allocated (0 each); document and
spreadsheet artifacts use the portable OOXML `.docx` and `.xlsx` forms.

The audit also confirms the fixed family ratios: Markdown 22%, TXT/log 8%,
code 28%, structured text 12%, CSV/TSV 3%, HTML/EML 5%, notebooks 1%,
searchable PDF 7%, scan PDF 1%, DOCX 3%, XLSX 2%, PPTX 2%, PNG 3%, and domain
binary 3%.  No media artifacts are allocated for p01.

## Verification record

`format-audit.json` records an `ok` result for the 12,000 artifacts and checks
the physical extension histogram, PDF classifications, real OOXML structure,
PNG, notebook, PCAP, SQLite, and ZIP formats.  The predecessor-manifest binding
and the overlay header bind the move set to the frozen Full allocation.  The
post-revision manifest is `p01-full-structure-v2.after.jsonl`, with SHA-256
`d10f4a64c5e653d8050049467fdab19990c1daca437bc20caa6ed25190a4787b`.

## Scope and exclusions

This report covers only p01's `home` tree and token-free p01 revision evidence.
It does not claim Kio indexing, history, search, chunking, or performance
results, and it does not change any other persona.
