---
path: Documents/Clients/Whitfield/Finance/04 Mar - Cover stress cases.xlsx
kind: holt-whitfield-finance.xlsx
author: Connor Atkinson
derived_from:
---

# Sheet: Stress cases

A2: Whitfield | How a 07 slot loses cover
A4: HRA-WHI-DIAG-PLN-2025-005 v0.1
D4: 04 Mar 2025
F4: Connor Atkinson

| Row | A Case | B Duty assumption | C Regular candidates left | D Temporary deputy proposed | E Temporary authority | F Temporary handover | G Candidates counted | H Two-role finding |
|---|---|---|---|---|---|---|---|---|
| 7 | Ordinary day, one away | One regular role leaves during the slot | 1 | 0 | N/A | N/A | =C7+IF(AND(D7=1,E7="Confirmed",F7="Confirmed"),1,0) | One person remains. The primary and deputy cannot both be protected. |
| 8 | Close day, both away | Both regular roles stay on close work throughout 15:00–16:30 JST | 0 | 0 | N/A | N/A | =C8+IF(AND(D8=1,E8="Confirmed",F8="Confirmed"),1,0) | Both regular roles are unavailable. No deputy is nominated in their place. |
| 9 | Temporary deputy proposed | Assume the regular primary is available and someone is proposed for the deputy slot | 1 | 1 | Unconfirmed | Unconfirmed | =C9+IF(AND(D9=1,E9="Confirmed",F9="Confirmed"),1,0) | Count the primary as a working candidate only. The temporary deputy contributes no cover until both checks are evidenced. |

For Siobhan Barrett · desk comparison for a single 90-minute slot

The schedule assumptions in B–D and the evidence checks in E–F answer different questions. Even with two candidates counted, check that both are free for the full slot and that the primary and deputy have the relevant 07 authority and can exchange the complete bundle. An entry of zero in column D means no temporary deputy is assumed in that case. An unconfirmed entry in E or F means the authority or handover record is still needed.

The close case is grounded in the corrected 25 February model (HRA-WHI-DIAG-PLN-2025-003 v0.2). Both regular roles are committed on 19 and 26 March. The temporary deputy case is a separate desk assumption and does not replace either of those two zero-cover rows. Reassigning close work to another person has not been evaluated.
