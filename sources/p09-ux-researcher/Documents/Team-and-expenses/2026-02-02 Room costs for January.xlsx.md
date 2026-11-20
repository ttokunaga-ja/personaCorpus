---
path: Documents/Team-and-expenses/2026-02-02 Room costs for January.xlsx
kind: carver-venue-expenses.xlsx
author: Jennifer Salazar
derived_from:
---

# Venue expenses

B3:D3: CWR-OPS-20260202-EXP-01; F3: v1.0

B4:C4: 2026-02-02; E4:F4: Jennifer Salazar

Title: Room costs for January

A5:F5: For Carolyn Davis · January 2026 research venue costs for month-end close

| Row | A: Session date | B: Room / slot | C: Charge type | D: Statement line | E: Amount (USD) | F: Reason / matching note |
|---:|---|---|---|---:|---:|---|
| 7 | 2026-01-05 | Lake Room | Completed session | 1 | 200.00 | January 5 room booking matched by date and room to the completed session on statement line 1. |
| 8 | 2026-01-12 | Lake Room | Completed session | 2 | 200.00 | January 12 room booking matched by date and room to the completed session on statement line 2. |
| 9 | 2026-01-20 | Lake Room | Completed session | 3 | 200.00 | January 20 room booking matched by date and room to the completed session on statement line 3. |
| 10 | 2026-01-22 | Lake Room / 13:30 CST | No-show cancellation | 4 | 80.00 | Line 4 bills USD 80 for the reserved 13:30 slot. See Ayesha Siddiqui’s `2026-01-22 Lake Room no-show log.txt` in this folder. She recorded no contact and no arrival by 13:45 CST, when she released the room. Operations checked the reservation record and found no cancellation by 17:00 CST on January 21, the previous business day. Charge the reserved slot to research venue costs. |

| Row | A:D label | E: Amount (USD) | F: Note |
|---:|---|---|---|
| 11 | Completed sessions | `=SUMIF(C7:C10,"Completed session",E7:E10)` | Three completed bookings. |
| 12 | No-show cancellation | `=SUMIF(C7:C10,"No-show cancellation",E7:E10)` | One reserved slot. No interview took place. |
| 13 | Total requested | `=SUM(E11:E12)` | Submit together as research venue costs, with the cancellation shown separately in the breakdown. |
| 14 | Paper statement total | 680.00 | Printed total on CWR-OPS-20260130-VEN-01 v1.0, issued January 30 by Carver Research Operations. Scan: `2026-01-30 January room charges.pdf`. |
| 15 | Difference vs. statement | `=E13-E14` | Requested amount less the printed statement total. |
