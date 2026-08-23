# Feature flag rollout
Aster Loop 株式会社 | Product Alpha R7 | release 2026-07-13 | p95 184 ms | ADR-042

## 5% synthetic cohort
Entry: dry run, checkpoint parity, and lag below five minutes. Observe 30 minutes; Software Engineering owns adapter errors, p95, lag, and counts. Abort on mismatch or p95 above 184 ms.
## 25% synthetic cohort
Entry: reviewed 5% evidence. Observe 60 minutes; Platform owns duplicate events, freshness, and dashboard completeness. Abort on rising lag or flag drift.
## 100% synthetic cohort
Entry: clean 25% window and Release Engineering approval. Observe one release window. Abort on SLO breach, schema incompatibility, or verifier finding.
## Retirement
Retire only after two clean windows and a rehearsed Alpha fallback.
