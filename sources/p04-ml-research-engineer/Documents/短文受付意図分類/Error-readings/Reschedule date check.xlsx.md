---
path: Documents/短文受付意図分類/Error-readings/Reschedule date check.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Reschedule date check

## Sheet — Adjudication

A2: Reschedule date check  
B3: MDR-STI-20241114-AW-01-R01  
E3: November 14, 2024  
B4: Vanessa Nguyen  
E4: For Fatima Rahman — 12 development replies with source date lost

| Row | A: Reading ID | B: Boundary | C: Source date in original | D: Date requested in reply | E: Date readable in excerpt | F: Referring expression |
|---|---|---|---|---|---|---|
| 6 | Q27 | Clear | Apr 23 | Apr 26 | Apr 26 | that one |
| 7 | Q03 | Clear | Apr 18 | Apr 22 | Apr 22 | that appointment |
| 8 | Q11 | Clear | Apr 19 | Apr 24 | Apr 24 | that slot |
| 9 | Q19 | Clear | Apr 22 | Apr 25 | Apr 25 | that one |
| 10 | Q32 | Clear | Apr 24 | Apr 30 | Apr 30 | the appointment below |
| 11 | Q38 | Clear | Apr 25 | Apr 29 | Apr 29 | that time |
| 12 | Q45 | Clear | Apr 26 | Apr 30 | Apr 30 | that booking |
| 13 | Q63 | Clear | Apr 17 | Apr 23 | Apr 23 | that one |
| 14 | Q76 | Clear | Apr 22 | Apr 29 | Apr 29 | the slot below |
| 15 | Q84 | Unclear | Apr 18 | Apr 24 | Apr 24 | that appointment |
| 16 | Q102 | Unclear | Apr 23 | Apr 29 | Apr 29 | that time |
| 17 | Q116 | Unclear | Apr 25 | Apr 30 | Apr 30 | that one |

| Row | A:C merged summary label | D: Replies | Expected result |
|---|---|---|---:|
| 18 | Clear boundary | `=COUNTIF(B6:B17,"Clear")` | 9 |
| 19 | Unclear boundary | `=COUNTIF(B6:B17,"Unclear")` | 3 |
| 20 | Total source-date losses | `=SUM(D18:D19)` | 12 |

Expected results are for checking the formulas. The sheet displays the formula results in D.

A21:F21: Read the referring expression against the original before checking the excerpt. E records the date the excerpt itself gives. In all 12 it is the requested date. Leave the source date in C.
