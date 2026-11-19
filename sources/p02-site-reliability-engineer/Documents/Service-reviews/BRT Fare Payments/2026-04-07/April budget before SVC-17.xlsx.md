---
path: Documents/Service-reviews/BRT Fare Payments/2026-04-07/April budget before SVC-17.xlsx
kind: brt-error-budget.xlsx
author: Daniel Jennings
derived_from:
---
# Monthly error budget

## Sheet: Budget

- B3: BRT-EB-20260407-001
- E3: Daniel Jennings
- B4: 7 Apr 2026
- E4: BRT Fare Payments; 1 Apr 00:00–7 Apr 08:00 UTC, end exclusive
- For: Anjali Shah

| Row | A: Line | B: Window (UTC) | C: Eligible attempts | D: Failures used | E: Budget left to date | F: Basis / reconciliation |
|---:|---:|---|---:|---:|---|---|
| 7 | 1 | 1 Apr 00:00–3 Apr 00:00 | 200000 | 176 | `=IF(SUM(C$7:C7)=0,"",1-SUM(D$7:D7)/(SUM(C$7:C7)*0.001))` | First window; 200 allowed cumulatively, 24 left. |
| 8 | 2 | 3 Apr 00:00–6 Apr 00:00 | 300000 | 294 | `=IF(SUM(C$7:C8)=0,"",1-SUM(D$7:D8)/(SUM(C$7:C8)*0.001))` | 500 allowed cumulatively, 470 used, 30 left. |
| 9 | 3 | 6 Apr 00:00–7 Apr 08:00 | 100000 | 100 | `=IF(SUM(C$7:C9)=0,"",1-SUM(D$7:D9)/(SUM(C$7:C9)*0.001))` | 600 allowed cumulatively, 570 used, 30 left. |
| 10 | Month to cutoff (A10:B10) | | `=SUM(C7:C9)` | `=SUM(D7:D9)` | `=IF(C10=0,"",E9)` | 99.9% target; balance for SVC-17. |
| 11 | Note (A11:F11) | | | | | |

A11: Windows include their start and exclude their end. At 08:00 UTC on 7 April, 600,000 eligible attempts allow 600 failures at the 99.9% target. Of these, 570 have been used and 30 remain: 5.0% of the allowance. This is the cumulative balance to the cutoff, not a forecast for the rest of April or May.
