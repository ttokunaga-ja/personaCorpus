---
path: Documents/Clients/Pioneer/Buying/22 Oct - What goes against the price.docx
kind: holt-handoffs.docx
author: Connor Atkinson
derived_from:
---
# What goes against the price

Document: HRA-PIO-BUY-EV-2025-004  
Date: 22 Oct 2025  
Prepared by: Connor Atkinson  
For: Deepa Desai

## Q4 extract specification

Use the five selected condition lines and their identified items. Include receipts dated 01 Oct–31 Dec 2025 and the associated invoice and credit postings available through 12 Jan 2026. Retain supplier ID, condition line, item, document and original-document references, receipt and posting dates, original unit, signed quantity, unit price and tax-exclusive line amount. Preserve the source unit; conversions belong in separate fields.

| Entry | Treatment | Check |
|---|---|---|
| Purchase invoice | Include the item amount and receipt quantity for the selected condition. | Match the receipt date and unit to the relevant term. |
| Return credit with quantity | Carry a negative quantity and amount, linked to the original receipt/document. | Deduct once from that same population; do not net an unmatched credit against a convenient item. |
| Commercial credit without quantity | Reconcile separately from item price reductions and net units. | Retain approval and posting evidence; do not invent a quantity or allocate it across items. |
| Freight and tax | Exclude from the price comparison. | Keep identifiable on extraction so they are not silently included. |

## Evidence arriving after quarter end

Please ask Buying to check evidence received after 31 December by 12 January. A receipt in the quarter and a later posting are different dates; the posting cutoff determines what is recognised in this checkpoint. Flag missing original references, receipt dates, units and unapproved credits for follow-up. Do not fill missing amounts with zero or treat an unposted allowance as realised.

Reference: HRA-PIO-BUY-MIN-2025-004, 13 Oct 2025.
