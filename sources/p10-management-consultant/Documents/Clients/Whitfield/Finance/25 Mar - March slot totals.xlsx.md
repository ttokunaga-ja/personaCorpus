---
path: Documents/Clients/Whitfield/Finance/25 Mar - March slot totals.xlsx
kind: holt-whitfield-finance.xlsx
author: Connor Atkinson
derived_from:
---

# Sheet: Daily slots

A2: Whitfield | March 07 working windows
A4: HRA-WHI-DIAG-PLN-2025-008 v1.0
D4: 25 Mar 2025
F4: Connor Atkinson

| Row | A Date | B Shared rota start (JST) | C Shared rota end (JST) | D Required (min) | E Close start (JST) | F Close end (JST) | G Both free (min) | H Valid working slot |
|---|---|---|---|---|---|---|---|---|
| 7 | 17 Mar 2025 | 15:00 | 16:30 | 90 | | | =ROUND((C7-B7-IF(COUNT(E7:F7)=2,MAX(0,MIN(C7,F7)-MAX(B7,E7)),0))*1440,0) | =IF(G7>=D7,1,0) |
| 8 | 18 Mar 2025 | 15:00 | 16:30 | 90 | | | =ROUND((C8-B8-IF(COUNT(E8:F8)=2,MAX(0,MIN(C8,F8)-MAX(B8,E8)),0))*1440,0) | =IF(G8>=D8,1,0) |
| 9 | 19 Mar 2025 | 15:00 | 16:30 | 90 | 14:30 | 16:30 | =ROUND((C9-B9-IF(COUNT(E9:F9)=2,MAX(0,MIN(C9,F9)-MAX(B9,E9)),0))*1440,0) | =IF(G9>=D9,1,0) |
| 10 | 24 Mar 2025 | 15:00 | 16:30 | 90 | | | =ROUND((C10-B10-IF(COUNT(E10:F10)=2,MAX(0,MIN(C10,F10)-MAX(B10,E10)),0))*1440,0) | =IF(G10>=D10,1,0) |
| 11 | 25 Mar 2025 | 15:00 | 16:30 | 90 | | | =ROUND((C11-B11-IF(COUNT(E11:F11)=2,MAX(0,MIN(C11,F11)-MAX(B11,E11)),0))*1440,0) | =IF(G11>=D11,1,0) |
| 12 | 26 Mar 2025 | 15:00 | 16:30 | 90 | 15:00 | 16:45 | =ROUND((C12-B12-IF(COUNT(E12:F12)=2,MAX(0,MIN(C12,F12)-MAX(B12,E12)),0))*1440,0) | =IF(G12>=D12,1,0) |

Times are local Excel time values in JST. B:C is the overlap of the primary's and deputy's regular duty. Where both are committed to close work, G deducts the part of E:F inside that overlap once. Blank E:F pairs indicate that the supplied extract shows no close commitment affecting that window. On 19 March both roles are on account reconciliation and on 26 March both are on the variance review. Each commitment covers the full 90 minutes.

# Sheet: Weekly totals

A2: Whitfield | Working cover on the first three days
A4: HRA-WHI-DIAG-PLN-2025-008 v1.0
D4: 25 Mar 2025
F4: Connor Atkinson

| Row | A Candidate week | B Dates included | C Slots required | D Slots with both roles | E Required minutes | F Minutes with both roles | G Three-day cover | H Constraint |
|---|---|---|---|---|---|---|---|---|
| 7 | Week of 17 Mar | 17–19 Mar | =COUNT('Daily slots'!D7:D9) | =SUM('Daily slots'!H7:H9) | =SUM('Daily slots'!D7:D9) | =SUM('Daily slots'!G7:G9) | =IF(D7=C7,"Complete","Short") | 19 Mar account reconciliation |
| 8 | Week of 24 Mar | 24–26 Mar | =COUNT('Daily slots'!D10:D12) | =SUM('Daily slots'!H10:H12) | =SUM('Daily slots'!D10:D12) | =SUM('Daily slots'!G10:G12) | =IF(D8=C8,"Complete","Short") | 26 Mar variance review |

For Kenneth Townsend and Siobhan Barrett. Each week shows 2 of 3 working slots and 180 of 270 required minutes. Use these rows to check the slot counts in the separate March start comparison for the 26 March meeting. The schedule shows working coverage only. Deputy authority, evidence at handover and approval to start are separate confirmations.

Basis — 10 February Japan Finance duty extract and Kenneth's 24 February confirmation of both close assignments, as reconciled in the 25 February cover model.
