# Known-issue triage — 2026-08-04

HelioDesk has 17 unresolved support tickets associated with four published known issues. The support response SLA is four hours. This triage record distinguishes a visible symptom from the customer action that is safe to take now.

| Issue | Symptom and scope | Status / owner role | Safe next step |
| --- | --- | --- | --- |
| KI-1042 — Queue counter delay | Queue totals can lag ticket activity by up to 12 minutes. | Monitoring / Support Operations Lead | Refresh or use filtered search before reassigning work. |
| KI-1047 — CSV import retry | A CSV import can pause after temporary validation timeout. | Workaround ready / Data Import Specialist | Correct quoted fields, wait 10 minutes, retry once. |
| KI-1051 — Session timeout | An inactive tab can request sign-in sooner than expected. | Investigating / Identity Platform Liaison | Save drafts, refresh, and sign in again. |
| KI-1054 — Notification lag | A notice can arrive after its ticket event. | Validating / Messaging Reliability Coordinator | Check ticket activity; do not repeat the action. |

Publication decision: all four items are appropriate for customer-facing use because they contain observable scope, a reversible workaround, owner role, and a verification step. No customer should be promised a resolution date.
