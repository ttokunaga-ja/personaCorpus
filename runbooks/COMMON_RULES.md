# Common production rules

1. Treat `canonical/persona-plan.json`, created by the accepted Rust
   `kio-eval persona plan` implementation, as authoritative.
   Do not rename, omit, add, reweight, or derive personas, Rust scope IDs,
   home paths, source IDs, families, variants, or quotas. Rust is the only
   plan parser, renderer, materializer, and scaffold.
2. The coordinator has created `canonical/materialized/` and `workspace/`
   through the create-only Rust commands. Production tasks must not recreate,
   adopt, replace, or edit either root.
3. All content is synthetic: never use real PII, PHI, credentials, secrets,
   customer data, or copied private documents. Keep invented people,
   organizations, identifiers, dates, amounts, terminology, and conclusions
   internally consistent across the plan-authorized files.
4. Write final corpus files only below the plan row's exact
   `people/<persona-id>-<role>/home/<scope-path>/` directory. `scope_path` is
   not the Rust `scope_id`; never derive either value from the other.
5. Use ordinary generation for text, code, and data. When quality benefits,
   use the named **Documents**, **PDF**, **Spreadsheets**, **Presentations**, or
   **ImageGen** skill for DOCX, XLSX, PPTX, `pdf_text`, `pdf_scan`, or image
   artifacts. Read the applicable `SKILL.md` before use and inspect every final
   artifact as that skill requires.
6. Temporary sources and render outputs stay under
   `scratch/<persona-id>/<scope-id>/`; operational notes stay under
   `progress/<persona-id>/`. Both are outside the Rust-published workspace.
7. Do not design product-search QA or assert searchability from raw artifacts.
   This workflow produces files and visual/structural inspection only. It does
   not establish Kio prepare/index/replay, chunks, search correctness, history
   readiness, or latency.
8. One parent chat coordinates one persona at a time. Before assigning a
   worker, it obtains the Rust parent lease through `./bin/persona lease`.
   A worker owns one Rust scope ID at
   a time; the parent alone retains release tokens. Scope lease commands use
   `--scope-id <Rust scope id>`, never a home path. The CLI derives owner
   bindings itself; callers do not supply a record digest.
9. Workers may create only the planned final files in their assigned home path.
   They must not write `_control/`, replace the Rust plan/owner/materialization
   records, or alter lease, lock, or recovery files. The parent verifies planned
   placement and artifact quality before releasing the scope lease.
10. Persona and scope leases prevent accidental duplicate assignment among
    cooperating sessions. They are not security boundaries against processes
    with direct write access as the same OS user. Forced recovery is a trusted
    parent/user action and must never be delegated to an artifact-producing
   worker.
11. After all leases are released, use Rust `kio-eval persona attest` for a
    create-only bounded observation of the materialized root. Its Kio evidence
    and history readiness claims remain false.
12. All persona chats use this exact repository in Local/same-directory mode.
    Do not use Git worktrees for production because `canonical/`,
    `workspace/`, `scratch/`, and local progress are intentionally untracked
    and must remain shared across the twenty parent chats.
