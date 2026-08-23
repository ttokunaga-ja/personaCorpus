# Dependency escalation notes - 2026-08-12

**Juniper Works | Product Alpha Q3 | M-27: 2026-09-18**

## Purpose

Resolve dependencies that could prevent an evidence-based M-27 guided rollout decision.

## Attendees

Product Management, Engineering, Data Analytics, Security and Privacy, Customer Support.

## Facts

- DR-081 fixes the adoption definition: eligible admins completing a guided first outcome within seven days of enablement.
- DR-082 limits the pilot to 120 synthetic workspaces; baseline reporting remains the separate 1,240-workspace cohort.
- DR-083 requires export-confidence evidence, and DR-084 sets a weekly digest as the notification default.

## Debate

Engineering reported that export-preview telemetry can show completion but not whether users understood access boundaries. Security and Privacy rejected substituting completion for confidence. Customer Support proposed adding a concise permission and export acknowledgment before preview, while Product Management emphasized that the addition must not change the frozen adoption event.

## Decisions

- **DR-089:** Export confidence will be assessed with the preview, permission acknowledgment, and synthetic usability evidence; preview completion alone does not pass DR-083.
- **DR-090:** Any change to the guided flow must retain the DR-081 event semantics and be documented in the M-27 packet.

## Actions

| Owner role | Due | Action |
| --- | --- | --- |
| Security and Privacy | 2026-08-19 | Publish acceptance criteria for the export-confidence evidence bundle. |
| Engineering | 2026-08-21 | Add non-PII telemetry for preview, acknowledgment, and exit states. |
| Product Design | 2026-08-20 | Review acknowledgment language for clarity and accessibility. |
| Data Analytics | 2026-08-25 | Confirm new signals do not alter the adoption calculation. |

## Risks

The acknowledgment could add friction to the guided path; it must be monitored separately from adoption.

## Unresolved

The material-regression threshold for access-related support contacts is still pending.
