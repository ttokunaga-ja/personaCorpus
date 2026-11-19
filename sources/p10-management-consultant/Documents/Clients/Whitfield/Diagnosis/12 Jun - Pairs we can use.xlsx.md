---
path: Documents/Clients/Whitfield/Diagnosis/12 Jun - Pairs we can use.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---
# Sheet: Pair check

HRA-WHI-DIAG-TIM-2024-003 v0.1 | 12 Jun 2024 | Connor Atkinson

Forty standard payments received and settled between 08 Apr and 31 May, JST. Presence denotes a usable event or pair, not a duration. A return requires a fresh complete hand-off and subsequent review start. No missing start is replaced by zero.

| Excel row | A Case | B Receipt | C Settlement | D 07 pair | E 09 pair | F 12 pair | G Returned | H Decision | I Reason |
|---|---|---|---|---|---|---|---|---|---|
| 2–29 | WB-0401 through WB-0428, one case per row in ascending order | Present | Present | Present | Present | Present | No | Include | All pairs present |
| 30–39 | WB-0429 through WB-0438, one case per row in ascending order | Present | Present | Present | Present | Present | Yes | Include | Fresh routing and review starts present |
| 40 | WB-0439 | Present | Present | Missing start | Present | Present | No | Exclude | 07 review start unavailable |
| 41 | WB-0440 | Present | Present | Present | Missing start | Present | No | Exclude | Approval review start unavailable |

# Sheet: Checks

| Cell | Measure | Formula | Expected |
|---|---|---|---|
| B2 | Included | =COUNTIF('Pair check'!H2:H41,"Include") | 38 |
| B3 | Included, no return | =COUNTIFS('Pair check'!H2:H41,"Include",'Pair check'!G2:G41,"No") | 28 |
| B4 | Included, returned | =COUNTIFS('Pair check'!H2:H41,"Include",'Pair check'!G2:G41,"Yes") | 10 |
| B5 | Included groups reconcile | =B2-B3-B4 | 0 |
| B6 | Excluded | =COUNTIF('Pair check'!H2:H41,"Exclude") | 2 |
| B7 | Total decisions | =B2+B6 | 40 |
