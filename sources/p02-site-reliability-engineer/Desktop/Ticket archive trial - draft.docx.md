---
path: Desktop/Ticket archive trial - draft.docx
kind: brt-change-decision.docx
author: Anjali Shah
derived_from:
---

# Ticket archive trial scope

Reference  BRT-CD-20240919-001  |  Planned issue  19 Sep 2024  
Prepared by  Anjali Shah

v0.1 · Working draft 17 Sep 2024 · Pending approval  
For Daniel Jennings, Reliability Engineering

## Decision

Proposed for 19 September review. Keep the 10-minute schedule in scope for an isolated trial under sustained foreground load. Run the 15-minute schedule as the comparator using the same load profile. Leave the live schedule and current operator recovery terms in force while this work is reviewed.

## Basis and restrictions

The shorter interval improves our recovery point. The 10-minute schedule did have two starts during an open write in the busier hour, so I want both schedules observed under comparable sustained load before we consider a change.

| Item | September evidence | Proposed action |
|---|---|---|
| 15-minute comparator | On 9 September, 0 of 24 starts had an open predecessor. | Repeat as the comparator under the agreed foreground load. |
| 10-minute candidate | On 11 September, 2 of 36 starts had an open predecessor. Daniel and Patrick located both finishes. | Retain for the isolated load trial and record starts and matching predecessor finishes. |
| Evidence needed next | BRT-IC-20240916-001 gives the overlap comparison. The sustained-load run and eligible-set readback are outstanding. | Agree matched windows and obtain a fixed-reference readback of restorable coverage against the existing 15-minute target. |

## Review conditions

Daniel, please check the rationale against the overlap evidence and flag any wording that goes beyond what we measured. Sanjay's proposal will need Patrick's agreement on the load profile and observation windows before the trial is booked. Bring the scope and evidence conditions to the 19 September review.
