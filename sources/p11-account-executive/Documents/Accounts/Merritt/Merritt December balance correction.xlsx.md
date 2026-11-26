---
path: Documents/Accounts/Merritt/Merritt December balance correction.xlsx
kind: halstead-adoption.xlsx
author: Darren Vasquez
derived_from:
---
# Merritt December balance correction

Sheet: December correction  
Document: HBC-MER-20260106-ADP-01 v1.0  |  As of: January 7, 2026  |  Prepared by: Darren Vasquez  
For: Michelle Whitaker  |  Account: Merritt Equipment Repair Inc  
Allowance period: December 1, 2025–November 30, 2026  |  All hours in decimal hours.

| Row | A: Booking | B: Purpose / date | C: Previous deduction (h) | D: Corrected deduction (h) | E: Change (h) | F: Basis |
|---|---|---|---:|---:|---:|---|
| 8 | Booking | Purpose / date | Previous deduction (h) | Corrected deduction (h) | Change (h) | Basis |
| 9 | MER-2417 | Workflow organization / Dec 16 | 1.25 | 1.25 | =D9-C9 | Jan 6 December usage export row MER-2417 records deduction 1.25; Dec 16 workflow review notes confirm 09:00–10:15 worked. |
| 10 | MER-2418 | Administrator consultation / Dec 17 | 2.00 | 0.00 | =D10-C10 | Jan 6 December usage export row MER-2418 records deduction 2.00 and actual 0.00; Dec 12 change HBC-MER-20251212-WRK-01 cancelled the slot and excludes it from deduction. |
| 11 | MER-2419 | Staff transition handoff / Dec 18 | 1.75 | 1.75 | =D11-C11 | Jan 6 December usage export row MER-2419 records deduction 1.75; attendance HBC-MER-20251218-ADP-01 confirms 13:00–14:45 worked. |
| 12 | December total | | =SUM(C9:C11) | =SUM(D9:D11) | =SUM(E9:E11) | Previous 5.00; corrected 3.00; change −2.00. |
| 13 | Annual allowance | | | 40.00 | | New allowance beginning December 1, 2025. |
| 14 | December closing balance | | | =D13-D12 | | 37.00 hours after correction. |

The previous deductions in C9:C11 come from the January 6 export `merritt_usage_2025-12.csv`, before amendment, and match the v0.1 preliminary balance. Only MER-2418 changes: its booked 2.00 hours remained in the deduction field although actual time was 0.00. Keep the 1.25 and 1.75 hours worked. Michelle and Darren reviewed the mismatch on January 6; Darren is providing this reconciliation for Michelle's review before the customer statement.

MER-2441 is a separate January 8 reservation for 1.25 hours. It is excluded from December and will be recorded in January after the consultation takes place.
