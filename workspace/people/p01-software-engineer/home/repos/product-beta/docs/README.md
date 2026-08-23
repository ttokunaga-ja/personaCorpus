# Product Beta documentation
Aster Loop 株式会社 | Product Alpha R7 | release 2026-07-13 | p95 184 ms | ADR-042

> Preview maturity notice: Beta is an internal, reversible projection for synthetic local development, not a production migration instruction.

## Local synthetic setup
Build the adapter, run the dry run, then verify state before enabling any cohort. No credentials, customer data, or external endpoints are required.

## Document map
- beta-migration-guide.md: phases and operating gates
- compatibility-notes.md: interface and data decisions
- feature-flag-rollout.md: cohort criteria
- troubleshooting-beta.md: safe diagnosis and recovery
- beta-migration-field-guide.docx: operator handoff

## Suggested verification flow
Read the guide; run migrate_config dry-run; inspect migration-dry-run.log; run verify_beta_state; then review rollback procedure.
