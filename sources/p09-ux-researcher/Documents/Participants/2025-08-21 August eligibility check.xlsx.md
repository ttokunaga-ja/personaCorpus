---
path: Documents/Participants/2025-08-21 August eligibility check.xlsx
kind: carver-participant-eligibility.xlsx
author: Carolyn Davis
derived_from:
---

# Eligibility

B3:D3: CWR-HO-20250821-ELG-01; F3: v1.0
B4:C4: 2025-08-21; E4:F4: Carolyn Davis
Title: August eligibility check
For the weekly meeting with Jennifer and Mihoko. Decide in the order: operational handoff in the last 30 days; incoming responsibility; ordinary shared-screen use. Unassessed answers are not failures. Numbers identify candidates only in this check.

| Row | A: No. | B: Handoff in last 30 days | C: Incoming responsibility | D: Shared list use | E: Eligibility | F: Basis / follow-up |
|---:|---:|---|---|---|---|---|
| 7 | 1 | Yes | Yes | Yes | Eligible | Recent operational handoff; receives open work and checks the shared screen. |
| 8 | 2 | Yes | Yes | Yes | Eligible | Received outstanding work at a recent handoff; normally checks it on the shared screen. |
| 9 | 3 | Yes | No | Not assessed | Ineligible | Incoming responsibility: passes information on orally only; does not take on the receiving role. |
| 10 | 4 | Yes | Yes | Yes | Eligible | Recent actual handoff; takes over pending actions and uses the shared screen. |
| 11 | 5 | Yes | Yes | No | Ineligible | Shared list use: receives pending work but checks it on paper only. |
| 12 | 6 | Yes | Yes | Yes | Eligible | Recent receiving handoff included outstanding work; checks the shared screen. |
| 13 | 7 | No | Not assessed | Not assessed | Ineligible | Handoff in last 30 days: training only; no actual operational handoff in that period. |
| 14 | 8 | Yes | Yes | Yes | Eligible | Actual recent handoff; receives unresolved tasks and normally uses the shared screen. |
| 15 | 9 | Yes | Yes | Yes | Eligible | Received work still needing action in a recent shift and checks the shared screen. |
| 16 | 10 | Yes | No | Not assessed | Ineligible | Incoming responsibility: only hands work over to the next person; no receiving duty. |
| 17 | 11 | Yes | Yes | Yes | Eligible | Recent operational shift change; takes responsibility for open work and uses the shared screen. |
| 18 | 12 | Yes | Yes | Yes | Eligible | Received open items in a recent real handoff; normally checks on the shared screen. |
| 19 | 13 | Yes | Yes | No | Ineligible | Shared list use: receives outstanding work but uses oral communication only for checking. |
| 20 | 14 | Yes | Yes | Yes | Eligible | Actual recent handoff, incoming responsibility for pending work, shared-screen checking. |
| 21 | 15 | Yes | Yes | Yes | Eligible | Recently received a shift with work remaining; checks it on the shared screen. |
| 22 | 16 | Yes | Yes | Yes | Eligible | Recent operational handoff; takes on open actions and uses the shared screen. |
| 23 | 17 | Yes | Yes | Yes | Eligible | Recent receiving handoff, pending work accepted, shared screen used for checking. |
| 24 | 18 | Yes | Yes | Yes | Eligible | Received outstanding actions at a recent actual handoff and checks the shared screen. |

| Row | A: formula | B:C merged label | Expected |
|---:|---|---|---:|
| 25 | `=COUNT(A7:A24)` | Candidates | 18 |
| 26 | `=COUNTIF(E7:E24,"Eligible")` | Eligible | 13 |
| 27 | `=COUNTIF(E7:E24,"Ineligible")` | Ineligible | 5 |
| 28 | `=COUNTIF(F7:F24,"Handoff in last 30 days:*")` | No actual handoff in 30 days | 1 |
| 29 | `=COUNTIF(F7:F24,"Incoming responsibility:*")` | No incoming responsibility | 2 |
| 30 | `=COUNTIF(F7:F24,"Shared list use:*")` | No shared-screen use | 2 |

Each exclusion is counted once at the first unmet condition. Job title and frequency are not eligibility rules. This is a recruitment check, not a denominator for prototype outcomes.
