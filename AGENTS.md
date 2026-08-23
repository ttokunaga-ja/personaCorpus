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
- Parent tasks for different personas may run concurrently. Another persona's
  lease is expected shared-workspace state: do not inspect, recover, release,
  or use it as a reason to stop. Global `active_leases=0` is a coordinator-only
  batch-end check, not an individual persona completion condition.
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
- Accepted final artifacts below `workspace/people/**/home/**`, portable
  canonical plan/schedule records, and allowlisted token-free progress records
  are Git-managed. Production tasks leave their own accepted additions as
  ordinary working-tree changes; they never stage or commit them.
- Runtime binaries, workspace owner/lease topology, canonical render and
  materialization receipts, scratch renders, local logs, and release tokens
  remain ignored. Never force-add an ignored path.
- Do not require a globally clean Git status while parallel persona tasks are
  active: another persona's uncommitted artifacts are expected shared state.
  The coordinating task alone verifies the corpus manifest, stages only an
  accepted wave, and commits directly to `main`. Production tasks never push.

## Production milestones

The first launch targets the persona brief's named seed artifacts, then the
first 200 final files for that persona in bounded folder batches. Freeze the
machine-readable M1 assignment ledger before the first worker starts, then
derive its human assignment display; Full production must never infer an
accepted M1 family from extension alone. The full
profile (195,000 files total) proceeds only through
`runbooks/FULL_PRODUCTION_PROTOCOL.md` and a frozen, token-free Full addition
assignment. Full work must preserve the first 200 files byte-for-byte, use
bounded resumable batches, and never reinterpret the Rust plan. Do not claim
Kio indexing, history, search, measured chunks, or performance from either
milestone.

For M1, the sole machine-readable allocation authority is
`progress/<persona>/m1-assignment-ledger.jsonl` with schema
`persona-corpus.m1-assignment/v1`. It is frozen before the first worker starts;
the human `m1-assignments.md` is its derived display/narrative, never an
independent allocation. Full reservations consume the frozen ledger directly;
do not infer a family (especially `.sql`) from an extension.
