---
path: Documents/Accounts/Waverley/Waverley rollout load - first pass.xlsx
kind: halstead-adoption.xlsx
author: Divya Rao
derived_from:
---
## Sheet: Implementation Load
A3: Waverley rollout — first capacity pass
C4: HBC-WAV-20250502-IMP-01 v0.1
C5: May 2, 2025
C6: Divya Rao

| Row | A: Week | B: Workstream | C: Demand (h) | D: Available (h) | E: Headroom (h) | F: Dependency or handoff |
|---|---:|---|---:|---:|---|---|
| 8 | Week | Workstream | Demand (h) | Available (h) | Headroom (h) | Dependency or handoff |
| 9 | 1 | Roster preparation | 8 | 10 | =D9-C9 | Week of May 5; Tracy's roster due May 9 |
| 10 | 2 | Assignment checks | 12 | 14 | =D10-C10 | Check the three group rosters |
| 11 | 3 | Opening and administrator practice | 18 | 14 | =D11-C11 | Opening and common practice overlap; attendance unconfirmed |
| 12 | 4 | Continued opening | 10 | 16 | =D12-C12 | Six hours of available headroom |
| 13 | 5 | Remaining opening | 12 | 14 | =D13-C13 | Complete staged assignments |
| 14 | 6 | Support handoff | 6 | 8 | =D14-C14 | Confirm support contacts |
| 15 | | Total | =SUM(C9:C14) | =SUM(D9:D14) | =D15-C15 | |

Row 16 note: The week-three shortfall is 4 hours even though the six-week total is 66 demand hours against 76 available. Week four has 6 hours of headroom. Check whether practice can move before committing to the schedule.
