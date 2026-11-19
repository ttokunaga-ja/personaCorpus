---
path: Documents/Service-reviews/BRT Fare Payments/July error budget after Faregate.xlsx
kind: brt-error-budget.xlsx
author: Daniel Jennings
derived_from:
---
# Monthly error budget

## Sheet: Budget

- B3: BRT-EB-20260713-001
- E3: Daniel Jennings
- B4: 13 Jul 2026
- E4: BRT Fare Payments; 1 Jul 00:00–13 Jul 11:00 UTC, end exclusive

| Row | A: Line | B: Window (UTC) | C: Eligible events | D: Budget used | E: Budget left to date | F: Basis / reconciliation |
|---:|---:|---|---:|---:|---|---|
| 7 | 1 | 1 Jul 00:00–13 Jul 00:00 | 1200000 | 908 | `=IF(SUM(C$7:C7)=0,"",1-SUM(D$7:D7)/(SUM(C$7:C7)*0.001))` | July 1–12; eligible Fare Payments attempts and failures. |
| 8 | 2 | 13 Jul 00:00–13 Jul 11:00 | 50000 | 312 | `=IF(SUM(C$7:C8)=0,"",1-SUM(D$7:D8)/(SUM(C$7:C8)*0.001))` | Includes 312 confirmed eligible Faregate failures in the incident window. |
| 9 | Month to cutoff (A9:B9) | | `=SUM(C7:C8)` | `=SUM(D7:D8)` | `=IF(C9=0,"",E8)` | 99.9% target; refer balance and queue to Anjali. |
| 10 | Note (A10:F10) | | | | | |

A10: All windows include the start and exclude the end. At 11:00 UTC, 1,250,000 eligible attempts at the 99.9% target allow 1,250 failures. 1,220 have been used; 30 remain, or 2.4% of the allowance. This is a running July balance, not a full-month forecast.
