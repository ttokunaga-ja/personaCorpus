# p02 Full structure v2 policy

## Purpose and scope

This revision makes the completed p02 Full corpus navigable as a realistic
site-reliability-engineering workspace without changing an artifact's contents,
declared family, extension, or canonical scope. It applies to the 14,800 Full
additions and does not create or remove an artifact.

## Immutable material

- The 200 accepted M1 artifacts remain at their existing paths and remain
  byte-frozen.
- Every selected Full artifact retains its predecessor SHA-256 and byte count.
  A selected artifact is either moved byte-for-byte or retains its predecessor
  path solely to preserve an embedded path or basename reference.
- The frozen Full assignment ledger remains portable allocation authority. The
  six immutable wave before/after manifests and this revision's post manifest
  provide portable acceptance evidence. The 85 raw batch checkpoints remain
  local historical evidence because some contain machine-specific render paths;
  they are neither tracked nor rewritten by this revision.
- No content regeneration, Office/PDF resave, metadata rewrite, or extension
  conversion is part of this revision.

## Current path authority

`path-revision.jsonl` is the append-only current-path overlay for all 14,800
Full additions. Its header binds the overlay to the frozen Full assignment,
the 200-file M1 baseline, and the predecessor accepted manifest. The live path
for a Full addition is its `new_path` in the overlay; the Full-ledger path and
wave manifests remain historical evidence. Raw checkpoints are local-only
receipts. M1 rows are outside the overlay and retain their original paths.

The overlay contains 14,600 byte-preserving moves and 200 compatibility rows
whose `old_path` and `new_path` are identical. Those 200 paths are retained
because another accepted artifact contains the old home-relative path or
basename. Artifact bytes are immutable, so changing such a target would leave
an invalid embedded reference.

## Hierarchy rules

The revision retains every canonical p02 scope root and adds only descendants
that reflect normal SRE work organization:

- the existing meaning-oriented parent, such as runbooks, postmortems,
  infrastructure, services, capacity, observability, or archived operations;
- a fixed time bucket: `2026-07-13`, `2026-07-14`, `2026-07-15`,
  `2026-07-20`, or `2026-q3`; and
- a physical-work kind: `source`, `notes`, `records`, `logs`, `data`,
  `messages`, `documents`, `workbooks`, `presentations`, `images`,
  `captures`, `notebooks`, `scans`, or `media`.

Generated filename collisions are resolved deterministically with a compact
ordinal suffix. The former terminal allocation suffix (`-sNNNNNN`) and a
redundant extension label are removed from naturalized names. Only the 200
reference-bound compatibility paths retain the allocation suffix.

## Invariants and acceptance criteria

The completed home contains 15,000 regular, non-empty files in 999 file-bearing
directories. No directory contains more than 82 direct files, compared with
405 before the revision. It contains no symlink, non-regular artifact, empty
artifact, empty directory, or duplicate inode under another path inside the
p02 home.

Before Git acceptance, 12,932 final paths still shared an inode with ignored
local QA/staging material. They were copied byte-for-byte onto independent
inodes after their sizes and SHA-256 values were checked against the signed
post-revision manifest. The local staging entries were not modified or
deleted. All 15,000 current p02 artifacts now have link count one, so writes
through a local staging path cannot mutate a Git-managed final artifact.

The v2 audit must prove all of the following:

- all 200 M1 paths, byte counts, and SHA-256 values are unchanged;
- all 14,800 selected Full artifacts are present exactly once at their overlay
  paths with predecessor bytes and SHA-256 values;
- the predecessor and post-revision manifests have the same 15,000-member
  `(SHA-256, byte-count)` multiset;
- no path crosses a canonical scope boundary; and
- every final artifact has an independent inode with link count one; and
- all declared families and physical extensions retain their fixed
  15,000-file distribution.

The fixed family distribution is Markdown 3,000 (20%), TXT/log 3,300 (22%),
code 2,250 (15%), structured text 3,000 (20%), CSV/TSV 750 (5%), HTML/EML
450 (3%), searchable PDF 600 (4%), DOCX 300 (2%), XLSX 150 (1%), PPTX 150
(1%), PNG 300 (2%), and domain binary 750 (5%). Scan PDF, notebook, and media
are not allocated for p02. Consequently, the real-file totals are 600 PDFs,
300 DOCX files, 150 XLSX files, 150 PPTX files, and 300 PNG files.

## Evidence

- Current overlay: `path-revision.jsonl`
  (`sha256:db64a48837ce6978880f66c09250e16d9c86302fb73d08f242997bc653880c2e`)
- Format and physical-artifact audit: `format-audit.json`
  (`sha256:80f9f275df31a3177ccaa12d06f6507609ec97fa0a4cf1ed184dbfef4e04637e`)
- Frozen Full assignment: `../../full-assignment-ledger.jsonl`
  (`sha256:deb618bdd9d7452ea93936494a5b0caf2c1adb39e454aeb12cf1eb79ab806e91`)
- M1 baseline: `../../manifests/m1-baseline.jsonl`
  (`sha256:6030fddb66249a6f480ea34ccd08ded56402f2cfa6d9e365700b237e22968fc2`)
- Predecessor accepted manifest: `../../manifests/p02-full-wave-006.after.jsonl`
  (`sha256:f3ec038ca49534d2c2cec07e8145ec40ecfd3cdc3283880c043e7695210e41d0`)
- Post-revision manifest: `p02-full-structure-v2.after.jsonl`
  (`sha256:aef2c95e46e6b3a53048ec8b840c9fc968efe0dc28cd021edade83466e4de7ca`)
