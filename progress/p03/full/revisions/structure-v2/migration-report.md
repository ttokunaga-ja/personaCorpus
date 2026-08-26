# p03 Full structure-v2 migration report

Status: `structure_v2_complete_format_remediation_pending`.

The parent reconciled **9,753** exclusive same-filesystem `rename(2)` operations; **247** paths remained fixed. All 10,000 final paths, bytes, and SHA-256 values equal the approved map, and the predecessor/final `(SHA-256, bytes)` multisets are identical.

- Before manifest SHA-256: `9ff9969f794fecd9e5946be19ea7084654d91391790799d4d9e70ddd7be816e9`
- After manifest SHA-256: `0fa9bc821a483bdf6449b17479ca5a6fc27264c8f5b1736817c6bfb68a818c56`
- Path-revision overlay SHA-256: `a5bde184f8faa4f305415bb91468ffb2fde7a05871fcab17dfea553c00023ef0`
- M1: 200 paths/bytes/SHA-256 unchanged
- Internal references: 530 non-self pairs, 107 targets, 163 exact-path pairs, 94 self hits, 0 broken
- Maximum files directly in one directory: 66
- Casefold, Unicode, Windows-component, trailing-dot/space, and path-length portability: `pass`
- Git/Kio/root-manifest operations: none

The **19** frozen Full JSONL payloads remain intentionally invalid and are listed with artifact ID, scope, predecessor/current path, bytes, and SHA-256 in `format-audit.json`. Format QA is therefore not complete and p03 is not Git-migration ready.

## Next session instruction

```text
Perform only p03 Full JSONL format remediation in the same canonical personaCorpus checkout. This is a byte-changing format repair after structure-v2, not another structure revision and not a Git migration.

Before any claim or write, read AGENTS.md, the Full production/ledger/Git runbooks, progress/p03/full/revisions/structure-v2/{structure-policy.md,path-revision.jsonl,p03-full-structure-v2.before.jsonl,p03-full-structure-v2.after.jsonl,portable-summary.json,reference-audit.json,format-audit.json,migration-report.md}, the immutable M1 baseline, and the QA-v2/structure-v2 lineage. Verify the canonical root identities; 10,000 regular files; M1 200 paths/bytes/SHA-256 unchanged; Full overlay coverage 9,800 exactly once; the structure-v2 after-manifest SHA shown above; current paths equal the overlay; p03 parent and scope leases absent via formal CLI and p03-only filesystem paths; and exactly the 19 Full blockers listed in format-audit.json remain invalid JSONL. Do not inspect or operate another persona's leases and do not require global active_leases=0.

Save the pre-work Git status without staging or reverting anything. Parent only: claim a fresh p03 parent lease and only the affected p03 scope leases through the formal CLI, retaining release tokens in the parent process. If any claim fails, normally release those already claimed and stop. Never recover or edit _control manually.

Create append-only authority under progress/p03/full/revisions/jsonl-format-v1/ containing a structure-v2 predecessor manifest, a create-only repair ledger, a final manifest, a portable QA summary, and a JSONL format report. Bind its header to the frozen Full assignment, M1 baseline, QA-v2 remediation-after, structure-v2 path overlay, and structure-v2 after manifest. Do not record absolute paths, tokens, locks, temporary paths, or machine-specific receipts.

Repair exactly the 19 listed current paths in place. Preserve each path, artifact identity, scope, intended Security GRC facts, record order, and semantic content; encode every nonblank UTF-8 line as one valid JSON object or array. Do not create token-padding, placeholders, secrets, credentials, real PII, dangerous external references, or network behavior. Do not alter M1 or any of the other 9,981 files. Record artifact ID, current path, scope, old SHA/bytes, reason, new SHA/bytes, and parser result for each repair.

After all repairs, independently re-read all 10,000 files. Require regular=10,000; symlink/nonregular/empty/hardlink/>100 MiB=0; path set unchanged from structure-v2; M1 unchanged; Full overlay coverage exactly once; changed-path set exactly the declared 19 Full JSONL files; all 19 parse as nonempty JSON Lines with every nonblank line an object or array; other format issues=0 apart from the immutable M1 EML newline exception already classified; family/extension distribution unchanged; portable paths pass; recorded internal-reference target compatibility remains 530 pairs/107 targets/163 exact-path pairs/94 self hits with broken=0; and no exact SHA-256 duplicate is introduced. Use overlay-aware lineage rather than treating old-path-only verifiers as a success condition.

Normally release all claimed scope leases, then the parent lease, and prove p03-only absence by CLI and filesystem. If a token is lost or normal release fails, do not recover; report the exact session/scope and request explicit approval. Do not run Git add/commit/push, update the root corpus manifest, operate Kio, write another persona, rename any path, or claim Git-migration readiness. Report format remediation separately and request a fresh final migration audit.
```
