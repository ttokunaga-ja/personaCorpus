# M-27 steering preparation - 2026-09-11

**Juniper Works | Product Alpha Q3 | decision gate: 2026-09-18**

## Purpose

Prepare the decision packet for M-27 and define the choices available to the steering group.

## Attendees

Product Management, Engineering, Product Design, Data Analytics, Security and Privacy, Customer Support, Go-to-Market Operations.

## Facts

- The decision compares an 18.4% baseline against a 26.0% target; the 1,240-workspace baseline denominator remains frozen under DR-081.
- The controlled cohort is 120 synthetic workspaces under DR-082, with a weekly digest default under DR-084.
- DR-089 clarifies that export confidence needs preview, permission acknowledgment, and synthetic usability evidence.

## Debate

The group considered an automatic expansion if the pilot moved upward. Data Analytics opposed a single-metric rule because early movement does not establish confidence or regression safety. Security and Privacy requested an explicit hold option if the evidence bundle is incomplete. Go-to-Market Operations asked for a clear decision time to preserve downstream planning.

## Decisions

- **DR-091:** M-27 will choose among expand, hold, or rollback; no automatic expansion is authorized.
- **DR-092:** The packet will present adoption movement, export-confidence disposition, notification and access signals, and rollback readiness as separate evidence lines.

## Actions

| Owner role | Due | Action |
| --- | --- | --- |
| Product Management | 2026-09-16 | Circulate the decision packet and decision-log extract. |
| Data Analytics | 2026-09-16 | Lock the seven-day cohort read and comparison notes. |
| Security and Privacy | 2026-09-16 | Record pass, hold, or fail disposition for DR-083 and DR-089. |
| Engineering | 2026-09-17 | Rehearse the rollback path and provide evidence. |

## Risks

Evidence arriving after 2026-09-16 may force a hold rather than a time-boxed expansion decision.

## Unresolved

Final pilot measurements and the export-confidence disposition are pending the M-27 packet.
