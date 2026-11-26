---
path: Documents/Clients/Whitfield/Finance/25 Feb - March cover model v0.2.xlsx
kind: holt-whitfield-finance.xlsx
author: Connor Atkinson
created: 2025-02-19 11:24
modified_by: Connor Atkinson
derived_from:
---

# Sheet: Cover model

A2: Whitfield | March 07 cover by person  
A4: HRA-WHI-DIAG-PLN-2025-003 v0.2  
D4: 25 Feb 2025  
F4: Connor Atkinson

| Row | A Date | B 07 start JST | C 07 end JST | D On regular rota | E Primary blocked | F Deputy blocked | G Free on rotas | H Trial authority |
|---|---|---|---|---|---|---|---|---|
| 7 | 17 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A7,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C7,'Close duties'!$C$7:$C$10,">"&B7)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A7,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C7,'Close duties'!$C$7:$C$10,">"&B7)>0)` | `=D7-SUM(E7:F7)` | Unconfirmed |
| 8 | 18 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A8,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C8,'Close duties'!$C$7:$C$10,">"&B8)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A8,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C8,'Close duties'!$C$7:$C$10,">"&B8)>0)` | `=D8-SUM(E8:F8)` | Unconfirmed |
| 9 | 19 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A9,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C9,'Close duties'!$C$7:$C$10,">"&B9)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A9,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C9,'Close duties'!$C$7:$C$10,">"&B9)>0)` | `=D9-SUM(E9:F9)` | Unconfirmed |
| 10 | 24 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A10,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C10,'Close duties'!$C$7:$C$10,">"&B10)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A10,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C10,'Close duties'!$C$7:$C$10,">"&B10)>0)` | `=D10-SUM(E10:F10)` | Unconfirmed |
| 11 | 25 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A11,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C11,'Close duties'!$C$7:$C$10,">"&B11)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A11,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C11,'Close duties'!$C$7:$C$10,">"&B11)>0)` | `=D11-SUM(E11:F11)` | Unconfirmed |
| 12 | 26 Mar 2025 | 15:00 | 16:30 | 2 | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A12,'Close duties'!$E$7:$E$10,'Person keys'!$B$7,'Close duties'!$B$7:$B$10,"<"&C12,'Close duties'!$C$7:$C$10,">"&B12)>0)` | `=--(COUNTIFS('Close duties'!$A$7:$A$10,A12,'Close duties'!$E$7:$E$10,'Person keys'!$B$8,'Close duties'!$B$7:$B$10,"<"&C12,'Close duties'!$C$7:$C$10,">"&B12)>0)` | `=D12-SUM(E12:F12)` | Unconfirmed |

A7:A12 contains dates. B7:C12 contains Excel time values in JST. The two people in D are JP07-P and JP07-D, both listed on each date in Regular payments, HRA-WHI-DIAG-PLN-2025-002 v0.1. Each role gets one blocked flag if any close duty for that person on the same date begins before 07 ends and ends after 07 begins. Multiple matching entries for the same person still give one flag. The working counts are 2, 2, 0 in each week. The authority column records a separate open check.

# Sheet: Person keys

A2: Whitfield | Match display names across the two rotas  
A4: HRA-WHI-DIAG-PLN-2025-003 v0.2  
D4: 25 Feb 2025  
F4: Connor Atkinson

| Row | A Display name | B Person key | C 07 role | D Evidence |
|---|---|---|---|---|
| 7 | JP07-P | JP07-P | Primary | Same display name on both rotas. Kenneth confirmed the primary's close commitments on 24 Feb. |
| 8 | JP07-D | JP07-D | Deputy | Regular payments rota, 10 Feb. |
| 9 | Close Reviewer B | JP07-D | Deputy | Kenneth's 24 Feb email, “Identity of the close reviewer”. Japan Finance confirmed the match on both dates. |

# Sheet: Close duties

A2: Whitfield | Close duties matched to 07 people  
A4: HRA-WHI-DIAG-PLN-2025-003 v0.2  
D4: 25 Feb 2025  
F4: Connor Atkinson

| Row | A Date | B Start JST | C End JST | D Close display name | E Person key | F Close assignment |
|---|---|---|---|---|---|---|
| 7 | 19 Mar 2025 | 14:30 | 16:30 | JP07-P | `=VLOOKUP(D7,'Person keys'!$A$7:$B$9,2,FALSE)` | Account reconciliation |
| 8 | 19 Mar 2025 | 14:30 | 16:30 | Close Reviewer B | `=VLOOKUP(D8,'Person keys'!$A$7:$B$9,2,FALSE)` | Account reconciliation |
| 9 | 26 Mar 2025 | 15:00 | 16:45 | JP07-P | `=VLOOKUP(D9,'Person keys'!$A$7:$B$9,2,FALSE)` | Month-end variance review |
| 10 | 26 Mar 2025 | 15:00 | 16:45 | Close Reviewer B | `=VLOOKUP(D10,'Person keys'!$A$7:$B$9,2,FALSE)` | Month-end variance review |

Column A contains dates and B:C contain Excel time values in JST. The four commitments are from the 10 Feb extract. Kenneth confirmed on 24 Feb that both people stay on the close work throughout and that no other deputy is nominated for either day.

# Sheet: Weekly comparison

A2: Whitfield | Correction to the first three 07 windows  
A4: HRA-WHI-DIAG-PLN-2025-003 v0.2  
D4: 25 Feb 2025  
F4: Connor Atkinson

| Row | A Candidate week | B Opening days | C Required slots | D v0.1 reported slots | E v0.2 both roles free | F Change in slots | G Condition for working cover |
|---|---|---|---|---|---|---|---|
| 7 | 17 Mar 2025 | 17–19 March | `=COUNT('Cover model'!A7:A9)` | 3 | `=COUNTIF('Cover model'!G7:G9,2)` | `=E7-D7` | `=IF(E7=C7,"Complete","Short")` |
| 8 | 24 Mar 2025 | 24–26 March | `=COUNT('Cover model'!A10:A12)` | 3 | `=COUNTIF('Cover model'!G10:G12,2)` | `=E8-D8` | `=IF(E8=C8,"Complete","Short")` |

For Siobhan and Nicholas. The 19 February model accepted one person as sufficient and left JP07-D on 07 while listing the same person on close work as Close Reviewer B. Following Kenneth's confirmation, both named roles are removed from the 19 and 26 March windows. Each week now has 2 of 3 working slots, down from the 3 of 3 shown at the Wednesday review. We need both roles for the full 90 minutes on each of the first three days. The four slots with two people free still need an authority check before they can be used for the trial.
