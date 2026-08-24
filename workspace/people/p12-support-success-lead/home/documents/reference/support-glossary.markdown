# HelioDesk Support Glossary

**Synthetic internal reference — 2026-08-04 example snapshot.**

| Term | Meaning |
|---|---|
| Acknowledgement | A useful first response that confirms receipt, ownership, current severity, and next update. Due within four elapsed hours of reporting. |
| Case | A single support record identified by `case_id`; it may contain redacted evidence and links to approved secure evidence locations. |
| Coverage window | A rostered interval with a named Primary and Backup role responsible for intake and handoffs. |
| Customer-impacting | A condition that prevents or materially impairs a requester’s normal use of the supported service. |
| Escalation | Transfer of technical or incident-management attention through the defined role path while the original owner remains accountable until acceptance. |
| Handoff | A documented ownership transition with receiving-role acceptance, current state, evidence summary, and next update time. |
| Incident Commander | The role coordinating Sev 1 response, cross-team decisions, and stakeholder communication. |
| Next update | The timestamp committed to the requester or internal responders for the next status communication; stored as `next_update_at_utc`. |
| Owner role | A functional role, not a personal identity, currently accountable for the next case action; stored as `owner_role`. |
| PII | Personally identifiable information. Do not include it in routine support evidence; redact before attachment or case entry. |
| Redaction | Removing or replacing sensitive values while preserving enough context for diagnosis. Redaction is not encryption. |
| Severity | Impact-and-urgency classification (`Sev 1` through `Sev 4`); it does not indicate requester importance or case age. |
| SLA acknowledgement due | `reported_at_utc` plus four elapsed hours; stored as `sla_ack_due_at_utc`. |
| Status | Workflow condition (`New`, `Acknowledged`, `Investigating`, `Waiting on Customer`, `Escalated`, `Resolved`, or `Closed`); stored as `status`. |
| Unresolved | A case whose `status` is not `Resolved` or `Closed`. The 17-case count in this pack is a worked example only. |
| UTC | Coordinated Universal Time. All timestamps in reference fields use ISO 8601 UTC form, for example `2026-08-04T14:00:00Z`. |
