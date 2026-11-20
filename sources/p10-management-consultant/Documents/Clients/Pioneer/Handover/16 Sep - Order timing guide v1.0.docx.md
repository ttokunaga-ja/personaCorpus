---
path: Documents/Clients/Pioneer/Handover/16 Sep - Order timing guide v1.0.docx
kind: holt-handoffs.docx
author: Nicholas Hargreaves
modified_by: Nicholas Hargreaves
created: 2026-09-10 10:33 BST
derived_from: Documents/Clients/Pioneer/Handover/10 Sep - Order timing guide v0.4.docx
---

# Changes from v0.4

- Replace the title with “When to count an order”. Change the document control to `HRA-PIO-BUY-EV-2026-002 v1.0`, date `16 Sep 2026`, and audience `Meera Kulkarni and Buying practitioners`. The author remains Nicholas Hargreaves.
- Replace “Before the walk-through” with “For each monthly check”. Keep the rule comparing the first supplier dispatch with the last deadline approved before that dispatch in EDT, including equality as on time. Add: “Use the approved deadline date to select the month. Do not use Created or Last changed as a substitute for the first supplier dispatch. Record Count only when the comparison can be made; use Review and leave the late flag blank when the evidence is insufficient.”
- Replace the three-row evidence table with this three-column table:

| No. | Situation | Record to follow and who to ask |
|---|---|---|
| 1 | PO-84217 — Count, late | August register A001 / EV-84217 and the order dispatch history show the first supplier send on 18 Aug at 15:08 EDT. The approval history shows the deadline of 18 Aug 15:00 EDT. Ask Buying for the dispatch or approval history if either record cannot be traced. Do not substitute the draft creation time. |
| 2 | PO-84241 — Count, on time | A002 / EV-84241 and dispatch history show 19 Aug 15:00 EDT; the approved deadline was 19 Aug 15:00 EDT. Equality is on time. Ask Buying for the send and deadline histories if needed. |
| 3 | PO-84255 — Review | A003 / EV-84255 records only 20 Aug; the approved deadline is 20 Aug 15:00 EDT. The same date does not establish whether the send was before or after the deadline. Leave the late flag blank. Ask Buying for the first supplier dispatch timestamp and match it to the PO and approval history before deciding. |
| 4 | PO-84263 — Exclude | A004 / EV-84263 records cancellation before dispatch. Ask Buying for the cancellation and dispatch history if the event is unclear. It is not a sent order in the denominator. |
| 5 | PO-84279 resend — Exclude | A006 / EV-84279-2 is a resend. Trace A005 / EV-84279-1 for the first send at 24 Aug 14:53 EDT against the 15:00 deadline: count that first send once as on time. Ask Buying for the send history if the two events cannot be linked. |

- Replace “Walk-through notes” with “Price evidence and the next month”: “Keep the separate S036 receipt group in Review until Buying supplies the approved terms version effective for the August receipts. For the separate S011 group, ask Buying for the August credit approval and Finance for its posting tied to those transactions. Leave each difference blank until the required evidence is in hand; do not use zero for an unknown amount or carry January's USD 5,500 into August. Keep both held groups outside the confirmed totals. For the next check, save the monthly checks workbook under a new name, change the period and trace that month's records. Carry unresolved evidence requests forward without treating them as confirmed.”
