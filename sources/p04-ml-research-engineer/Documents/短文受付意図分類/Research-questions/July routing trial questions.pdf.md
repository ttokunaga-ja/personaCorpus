---
path: Documents/短文受付意図分類/Research-questions/July routing trial questions.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# What July still leaves for a routing trial

Document ID: MDR-STI-20260723-RP-01-R01  
Issued: July 23, 2026  
Author: Vanessa Nguyen  
For: Brandon Romero

## What we would need to observe

Brandon, I'd like to pick up the receipt-time question we left open on June 19. Before we could discuss a routing trial, we would need to know where a low-confidence item would wait, who would take it up and what records would let us time a correction after a wrong assignment.

Both July runs gave one top intent to each of the same 1,200 reference-labelled bodies. R173 scored 0.852 and R184 scored 0.873 in macro-F1, with equal weight across 12 intents. The comparison records agreement with the reference labels. It gives us no measurement of a low-confidence holding destination or of the time it takes to recover from a misroute. In June the lower cutoff freed 72 of the 800 documents from hold and seven of those 72 had wrong predictions. We left the use decision open because we still needed to understand the work after such an error.

## Three proposed measurement points

| Event | Timestamp to look for | Evidence to discuss |
|---|---|---|
| Wrong route detected | When the detecting reviewer records that the assignment is wrong | Wrong-route report linked to the request. Keep the original routing record available to confirm the destination. |
| Correction routed | When the corrected assignment is sent to the intended owner | Reassignment record with the same request identifier. If the request goes back to hold first, retain that status change as a separate event. |
| Correct owner receives it | When the correct owner receives the request | A receipt or handoff record from the receiving side. We still need to establish whether that time can be obtained and who can supply it. |

Use detection to correct receipt as the proposed recovery interval. The time of reassignment lets us distinguish the wait to send the correction from the wait for the handoff. If the receipt time is missing, leave the full interval unknown. Keep any later requester notification or closure time separate from the handoff.

## What I need from you

Could we walk through which role would record each event, where the shared request identifier is available and whether obtaining a receipt would add work for the receiver? I'd also like to pin down who would own a held item and what workload we could reasonably ask the review team to record. For now I am asking for a discussion of measurement methods. Michael's July 16 permission covers the oral presentation and the specified deck and figure. The June use decision remains deferred.
