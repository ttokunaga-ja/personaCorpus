---
path: Documents/Capacity/April peak review decisions.docx
kind: brt-reliability-meeting.docx
author: Daniel Jennings
derived_from:
---
# Fare Payments peak capacity review

Reference  BRT-MT-20250428-001     |     Issued  28 Apr 2025  
Prepared by  Daniel Jennings

Meeting: 28 April 2025, 11:00 UTC (12:00 BST)  
Present: Anjali Shah, Operations Director; Matthew Pickering, Fare Payments lead; Daniel Jennings, SRE

## Agenda

Agree a planning ceiling for the predicted peak and the checks needed before relying on the additional capacity. Daniel presented BRT-RB-20250428-001 and the comparison BRT-CP-20250428-001, using Louise Turner's forecast PCS-DF-20250425-001.

## Evidence considered

Palmer's hourly p95 one-minute forecast uses a common 1,000 eligible attempts/min reference, without duplicate retry indications or diagnostics. Demand is 1.70×, 2.05×, 2.30× and 1.90× in the four successive UTC bands from 15:00 to 19:00. The 2.00× option falls short by 0.05× in 16:00–17:00 and 0.30× in 17:00–18:00. Waiting until the busiest hour to provide extra capacity would leave the forecast shortfall beginning at 16:00 UTC uncovered.

The proposed 2.80× ceiling adds 0.50×, or 500 eligible attempts/min on the reference basis, to the 2.30× busiest-band forecast. Of that, 0.25× (250/min) is protected for short bursts and 0.25× (250/min) for retaining capacity during worker replacement. These are distinct sizing allowances; they are not observed spare capacity, and the forecast itself contains no BRT reserve.

## Decision and actions

Anjali approved 2.80× as the Fare Payments planning ceiling. The approval is a sizing decision; it does not establish that the proposed capacity has been verified.

| Owner | Action | When |
|---|---|---|
| Matthew Pickering | Verify a short demand burst within the planning envelope and separately verify worker replacement at steady load. Record both outcomes; a burst pass does not establish replacement capacity. | Before relying on the extra capacity, ahead of peak |
| Daniel Jennings | Retain the superseded 2.00× comparison with the proposal. | With the review record |
| Daniel Jennings | Identify the reserve separately in the cost readback to Richard Fenner. | August cost readback |

If either verification is incomplete or fails, Matthew will bring the result back before the team relies on the extra capacity.
