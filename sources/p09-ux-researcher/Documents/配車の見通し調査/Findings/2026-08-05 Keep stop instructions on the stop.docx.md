---
path: Documents/配車の見通し調査/Findings/2026-08-05 Keep stop instructions on the stop.docx
kind: carver-research-decisions.docx
author: Jeremy Sutton
derived_from:
---

# Keep stop instructions on the stop

CWR-DR-20260805-DEC-01 · v1.0  
August 5, 2026 · Jeremy Sutton  
To: Jennifer Salazar

## Placement decision

Jennifer and I reviewed the Stops placement today, 1:00–2:00 p.m. The next design pass should put a route-level Hold reason beside Held in the route list. A person who only reads the list would otherwise have to open Stops to reach the proposed reason, and a reason in stop 02 could appear to qualify that stop rather than the whole route.

| Location | Proposed information | Purpose and boundary |
|---|---|---|
| Route list | Hold reason adjacent to Held for the selected route; Unknown when the reason for that route and time has not been confirmed | Carry the known reason or its absence with the status. Keep Ready and Held as the existing status values. |
| Selected route detail | Source and Checked at for the reason; where a contact or source to check is known, show that next check | Help the receiver judge where the statement came from and when it was checked. A known contact is not proof of release authority. Do not invent a contact when none is known. |
| Stop row | Retain the stop's Note on its own row | A door or handling instruction belongs to that stop; it is not a substitute for the route's hold reason. |

## Why this arrangement

In the R-476 observation the west loading door instruction was readable at stop 02 but did not explain Held. The earlier reading-order decision likewise kept the stop Note attached to its row. Moving it to a route-level reason or placing the reason among stop instructions would blur their scope.

## Open work

A reason needs an identifiable source and a relevant check in time before being presented as confirmed. We still need to work out how stale information is recognized and how a receiver reads Unknown alongside a possible next contact. This decision does not add a release button or change release authority. It is a direction for a prototype, not a claim that the current screen has these fields or that the arrangement has been validated.
