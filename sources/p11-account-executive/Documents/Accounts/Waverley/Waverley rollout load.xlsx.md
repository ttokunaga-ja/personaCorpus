---
path: Documents/Accounts/Waverley/Waverley rollout load.xlsx
kind: halstead-adoption.xlsx
author: Divya Rao
derived_from:
---
## Sheet: Implementation Load
A3: Waverley rollout — revised capacity
C4: HBC-WAV-20250502-IMP-01 v1.0
C5: May 2, 2025
C6: Divya Rao

| Row | A: Week | B: Workstream | C: Demand (h) | D: Available (h) | E: Headroom (h) | F: Dependency or handoff |
|---|---:|---|---:|---:|---|---|
| 8 | Week | Workstream | Demand (h) | Available (h) | Headroom (h) | Dependency or handoff |
| 9 | 1 | Roster preparation | 8 | 10 | =D9-C9 | Week of May 5; Tracy finalizes rosters by May 9 |
| 10 | 2 | Assignment checks | 12 | 14 | =D10-C10 | Check gaps and duplicate assignments |
| 11 | 3 | Dispatch opening | 14 | 14 | =D11-C11 | Dispatch 36 seats; zero headroom |
| 12 | 4 | Site Operations opening and administrator practice | 14 | 16 | =D12-C12 | Site Operations 48 seats; four practice hours moved from week 3 |
| 13 | 5 | Office opening | 12 | 14 | =D13-C13 | Office 36 seats; reconcile assignments |
| 14 | 6 | Support handoff | 6 | 8 | =D14-C14 | Confirm contacts and outstanding questions |
| 15 | | Total | =SUM(C9:C14) | =SUM(D9:D14) | =D15-C15 | |

Row 16 note: Six-week demand is 66 hours against 76 available. Moving only the four administrator-practice hours from week 3 to week 4 resolves the four-hour shortfall; week 3 has zero headroom and week 4 has two hours. The schedule depends on Tracy finalizing the three rosters by May 9. Opening order remains Dispatch, Site Operations, Office.
