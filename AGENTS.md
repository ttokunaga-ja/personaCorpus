# Persona corpus production instructions

## Purpose and authority

This repository coordinates production of a fully synthetic, twenty-persona
filesystem corpus. The accepted Rust plan at `canonical/persona-plan.json`
and the Rust workspace owner record at `workspace/persona-workspace-owner.json`
are authoritative for persona IDs, scope IDs, paths, formats, and counts.
Runbooks and prompts are operational guidance only.

## Mandatory session model

- Run every persona parent task in **Local / same-directory mode** against this
  exact repository. Do not use a separate Git worktree for production.
- One parent task owns exactly one persona (`p01` through `p20`).
- A parent may run several subagents concurrently only when every subagent owns
  a different plan-defined leaf scope. Never assign one scope twice at once.
- Each subagent writes final artifacts only below its assigned
  `workspace/people/<persona>-<role>/home/<scope-path>/` leaf folder.
- The parent alone invokes `./bin/persona lease ...`, retains release tokens,
  checks the output, and releases leases. Never put a release token in a
  prompt, artifact, progress file, Git file, or subagent message.
- A stopped folder worker should be continued through the same subagent when
  practical. Recovery commands require explicit user/coordinator confirmation.

## Artifact production

- Read `README.md`, all common files named by the selected parent prompt, and
  the matching `runbooks/personas/pXX-*.md` brief before producing anything.
- Use Documents for DOCX, PDF for PDF, Spreadsheets for XLSX, Presentations for
  PPTX, and ImageGen for images or scan source artwork whenever the brief calls
  for those formats. Read each applicable `SKILL.md` before using it.
- The user's priority is high-quality, realistic persona content. Do not design
  a separate search or Kio QA suite. Skill-required rendering and visual/file
  verification remain mandatory before an artifact is accepted.
- Text, code, CSV/TSV, JSON/YAML, HTML/EML, notebooks, and similar files may be
  created normally. Extensions must match real file contents.
- Use synthetic names and facts only. Never include real PII, PHI, credentials,
  secrets, private documents, customer data, or copyrighted source documents.
- Put temporary inputs and render/inspection output under
  `scratch/<persona-id>/<scope-id>/`, never inside the final workspace.

## Ownership and repository safety

- Persona parents own only their persona's `workspace/people/.../` subtree and
  `progress/<persona-id>/`. Folder workers own only the exact leaf folder given
  to them by that parent.
- Do not directly edit `canonical/`, the Rust plan/owner records, `_control/`,
  runbooks, prompts, scripts, or another persona's files during production.
  Only the pinned Rust lease commands may mutate `_control/`.
- Corpus bytes, runtime binaries, leases, scratch files, and local progress are
  intentionally ignored by Git. Do not force-add them.
- Do not commit from parallel persona production tasks. The coordinating task
  alone may commit tracked operational documentation.

## Initial production boundary

The first launch targets the persona brief's named seed artifacts, then the
first 200 final files for that persona in bounded folder batches. The full
profile (195,000 files total) is a long-term topology target, not a requirement
for one task turn. Do not claim Kio indexing, history, search, or performance.
