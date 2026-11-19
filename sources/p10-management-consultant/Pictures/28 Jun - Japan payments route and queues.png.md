---
path: Pictures/28 Jun - Japan payments route and queues.png
kind: diagram
author: Neha Patel
derived_from:
---

# Japan supplier payments — route and approval queues

A wide swimlane diagram. The horizontal direction shows process order. The four lanes, from top to bottom, are labelled “Accounts Payable”, “Cost-centre owner”, “Japan Finance” and “Treasury”. Solid arrows carry the invoice or payment file from one activity to the next. The step number and activity below are the words shown in each work box.

| Lane | Numbered work boxes, in flow order within the lane |
|---|---|
| Accounts Payable | 01 Receive invoice → 02 Check completeness → 03 Enter invoice → 04 Match PO and receipt → 06 Route for cost-centre approval. Below the main AP path sits 05 Resolve exceptions. |
| Cost-centre owner | 07 Approve cost centre |
| Japan Finance | 08 Review for Finance approval → 09 Approve in Finance. At the right end of the diagram sits 14 Reconcile settlement. |
| Treasury | 10 Build payment proposal → 11 Check payment run → 12 Authorise bank release → 13 Release payment |

At 04, the branch labelled “Unresolved difference” goes down to 05. An arrow labelled “Re-match” returns from 05 to 04. The forward line from 04 to 06 reads “Match complete”. Connect 07 to 08, 09 to 10 and 13 to 14 across the lane boundaries. Step 13 has its own box after 12. Step 14 ends the route in Japan Finance.

Interrupt each of these three arrows with a dashed band in the receiving lane, directly before the approval work box.

| Arrow | Words inside dashed band |
|---|---|
| 06 to 07 | Complete package routed → review starts · 340 h |
| 08 to 09 | Finance file routed → review starts · 220 h |
| 11 to 12 | Both checks complete, proposal routed → review starts · 50 h |

Legend · “Numbered box: recorded work” and “Dashed band: approval waiting from complete routing to review start”. A note below the return loop reads “Correction and re-match come before fresh routing.” Box and band widths convey sequence.

Footer · “38 payments · 76 h of work · 1,000 h of waiting, including 610 h before approval reviews (61%). Hours are summed across cases.”  
Credit · “Neha Patel · 28 Jun 2024 · Timing: Connor Atkinson, 26 Jun.”
