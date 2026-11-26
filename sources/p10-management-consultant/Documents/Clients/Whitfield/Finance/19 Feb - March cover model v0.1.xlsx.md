---
path: Documents/Clients/Whitfield/Finance/19 Feb - March cover model v0.1.xlsx
kind: holt-whitfield-finance.xlsx
author: Connor Atkinson
derived_from:
---

# Sheet: Cover model

A2: March 07 cover — first overlay for Wednesday review  
A4: HRA-WHI-DIAG-PLN-2025-003 v0.1  
D4: 19 Feb 2025  
F4: Connor Atkinson

| Row | A Date | B 07 start JST | C 07 end JST | D Regular on rota | E Overlap by matching label | F People remaining | G Slot with at least one | H Close rows in extract |
|---|---|---|---|---|---|---|---|---|
| 7 | 17 Mar 2025 | 15:00 | 16:30 | 2 | 0 | `=D7-E7` | `=IF(F7>=1,1,0)` | None shown |
| 8 | 18 Mar 2025 | 15:00 | 16:30 | 2 | 0 | `=D8-E8` | `=IF(F8>=1,1,0)` | None shown |
| 9 | 19 Mar 2025 | 15:00 | 16:30 | 2 | 1 | `=D9-E9` | `=IF(F9>=1,1,0)` | Account reconciliation, 14:30–16:30. JP07-P and Close Reviewer B |
| 10 | 24 Mar 2025 | 15:00 | 16:30 | 2 | 0 | `=D10-E10` | `=IF(F10>=1,1,0)` | None shown |
| 11 | 25 Mar 2025 | 15:00 | 16:30 | 2 | 0 | `=D11-E11` | `=IF(F11>=1,1,0)` | None shown |
| 12 | 26 Mar 2025 | 15:00 | 16:30 | 2 | 1 | `=D12-E12` | `=IF(F12>=1,1,0)` | Variance review, 15:00–16:45. JP07-P and Close Reviewer B |
| 13 | Week of 17 Mar | | | | | | `=SUM(G7:G9)` | of 3 slots |
| 14 | Week of 24 Mar | | | | | | `=SUM(G10:G12)` | of 3 slots |

Dates and times in rows 7–12 are cell values. Source: 12 February duty windows, HRA-WHI-DIAG-PLN-2025-002 v0.1. The regular table lists JP07-P and JP07-D on each date. Column E counts the overlapping close rows with a matching regular-duty label. The only such label in the first pass is JP07-P, so JP07-D remains against 07 on 19 and 26 March. The model treats Close Reviewer B as a separate person on each of those close blocks.

Note for Siobhan and Nicholas: both weekly counts are 3 of 3 under the current one-person rule. The match by person is still outstanding with Kenneth. This pass leaves the deputy in both weekly calculations without verifying the close label. The counts do not establish that both 07 roles are free or that the deputy has authority.
