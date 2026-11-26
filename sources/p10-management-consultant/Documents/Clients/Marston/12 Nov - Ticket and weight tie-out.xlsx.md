---
path: Documents/Clients/Marston/12 Nov - Ticket and weight tie-out.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Marston trial tickets and weight tie-out

12 Nov 2024 · Connor Atkinson · For Caroline Wood and the Trafford T2 and Warrington W1 shift leads

## Sheet: Ticket check

Headers at row 1, three cases at rows 2–4. The trial ran from 14 October to 10 November for B420 / S3. These are the three evening changeover cases supplied for the cross-shift check. Times retain their source offsets. Empty tag, approval or return entries indicate missing evidence.

| Run | Event | Sending lead | Sheets | Tag | Approval and lead | Actual return and signature | Handover evidence | Finding |
|---|---|---|---:|---|---|---|---|---|
| T2-241017-B | Changeover first-off | Julie Naylor | 3 | | | | Julie's 17 Oct comments place the changeover at 16:42 BST and all three sheets on hold at 17:00 BST. The ticket supports the count and the weight record shows 1.8 kg held. | Return unproved. The 18 Oct query found neither a return signature nor an actual return time linked to the three sheets. Leave later weights open. |
| T2-241022-B | Changeover first-off | Julie Naylor | 3 | HT-07 | 23 Oct 08:12 BST · Gareth Pritchard | 23 Oct 08:16 BST · Gareth Pritchard | Same tag on the ticket, the hold entry and the signed return. The good-stock posting for HT-07 is at 08:16 BST. | One return of 1.8 kg to good stock, posted to the 22 Oct evening frame. |
| W1-241029-B | Changeover first-off | Helen Rigby | 3 | HW-04 | 30 Oct 08:18 GMT · Martin Devlin | 30 Oct 08:24 GMT · Martin Devlin | HW-04 links the ticket to the hold record. The shelf check agrees at 1.5 kg. The return signature and good-stock posting are both at 08:24 GMT. | One return of 1.5 kg to good stock, posted to the 29 Oct evening frame. |

## Sheet: Weight balance

Headers at row 1, rows 2–4 in ticket order. Each handover frame covers all in-scope runs on the originating evening shift, including earlier runs. The run reference identifies the event being followed into that frame. Weight columns are in kg.

| Run | Input kg | Good kg | Changeover loss kg | Restart loss kg | Other kg | Hold kg | Unaccounted kg |
|---|---:|---:|---:|---:|---:|---:|---:|
| T2-241017-B | `='Weight extract'!E2` | `='Weight extract'!E3` | `='Weight extract'!E4` | `='Weight extract'!E5` | `='Weight extract'!E6` | `='Weight extract'!E7` | `=IF(COUNT(B2:G2)=6,ROUND(B2-SUM(C2:G2),1),"")` |
| T2-241022-B | `='Weight extract'!E8` | `='Weight extract'!E9` | `='Weight extract'!E10` | `='Weight extract'!E11` | `='Weight extract'!E12` | `='Weight extract'!E13` | `=IF(COUNT(B3:G3)=6,ROUND(B3-SUM(C3:G3),1),"")` |
| W1-241029-B | `='Weight extract'!E16` | `='Weight extract'!E17` | `='Weight extract'!E18` | `='Weight extract'!E19` | `='Weight extract'!E20` | `='Weight extract'!E21` | `=IF(COUNT(B4:G4)=6,ROUND(B4-SUM(C4:G4),1),"")` |

The handover residual is 0.0 kg in all three frames. T2's 17 October frame already balances with 1.8 kg in hold. This establishes the position at 17:00 only. W1's 5.0 kg of restart loss belongs to the earlier run and stays in the restart column.

## Sheet: Return movement

Headers at row 1. Rows 2–4 follow the same order as Weight balance. Good and hold after return are the posted totals for the same evening frame. Keep input and all three loss categories at their handover values. Any new production from the receiving shift belongs to its own frame.

| Run | Tag | Good at handover kg | Hold at handover kg | Good after return kg | Hold after return kg | Good increase kg | Hold decrease kg | Movement difference kg | Residual after return kg |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| T2-241017-B | | `='Weight balance'!C2` | `='Weight balance'!G2` | | | `=IF(COUNT(C2,E2)=2,ROUND(E2-C2,1),"")` | `=IF(COUNT(D2,F2)=2,ROUND(D2-F2,1),"")` | `=IF(COUNT(G2:H2)=2,ROUND(G2-H2,1),"")` | `=IF(COUNT(E2:F2)=2,ROUND('Weight balance'!B2-'Weight balance'!D2-'Weight balance'!E2-'Weight balance'!F2-SUM(E2:F2),1),"")` |
| T2-241022-B | HT-07 | `='Weight balance'!C3` | `='Weight balance'!G3` | `='Weight extract'!E14` | `='Weight extract'!E15` | `=IF(COUNT(C3,E3)=2,ROUND(E3-C3,1),"")` | `=IF(COUNT(D3,F3)=2,ROUND(D3-F3,1),"")` | `=IF(COUNT(G3:H3)=2,ROUND(G3-H3,1),"")` | `=IF(COUNT(E3:F3)=2,ROUND('Weight balance'!B3-'Weight balance'!D3-'Weight balance'!E3-'Weight balance'!F3-SUM(E3:F3),1),"")` |
| W1-241029-B | HW-04 | `='Weight balance'!C4` | `='Weight balance'!G4` | `='Weight extract'!E22` | `='Weight extract'!E23` | `=IF(COUNT(C4,E4)=2,ROUND(E4-C4,1),"")` | `=IF(COUNT(D4,F4)=2,ROUND(D4-F4,1),"")` | `=IF(COUNT(G4:H4)=2,ROUND(G4-H4,1),"")` | `=IF(COUNT(E4:F4)=2,ROUND('Weight balance'!B4-'Weight balance'!D4-'Weight balance'!E4-'Weight balance'!F4-SUM(E4:F4),1),"")` |

HT-07 moves 1.8 kg and HW-04 moves 1.5 kg. In each case the increase in good matches the decrease in hold and the evening frame still balances. The updated good weights of 490.0 kg and 478.0 kg already include their respective returns. If an original signed and timed return for T2-241017-B is located, match it to the held sheets and verify the resulting good and hold entries before closing the query.

## Sheet: Trial tickets

Source: Marston trial tickets Oct-Nov.csv. Source headers at row 1. Preserve the original row order, empty cells and timestamp offsets.

| Run | Shift | Event | Changeover date | Checked sheets | Sending lead | Tag | Approval date and time | Approving lead | Return date and time | Return signature |
|---|---|---|---|---:|---|---|---|---|---|---|
| T2-241017-B | Evening | Changeover first-off | 2024-10-17 | 3 | Julie Naylor | | | | | |
| T2-241022-B | Evening | Changeover first-off | 2024-10-22 | 3 | Julie Naylor | HT-07 | 2024-10-23 08:12 +0100 | Gareth Pritchard | 2024-10-23 08:16 +0100 | Gareth Pritchard |
| W1-241029-B | Evening | Changeover first-off | 2024-10-29 | 3 | Helen Rigby | HW-04 | 2024-10-30 08:18 +0000 | Martin Devlin | 2024-10-30 08:24 +0000 | Martin Devlin |

## Sheet: Weight extract

Source: Marston weight extracts Oct-Nov.csv. Source headers at row 1, data at rows 2–23. Preserve the original row order for trace-back from the two calculation sheets. Blank times are handover positions where the extract has no clock time.

| Run | Weighing / record time | Frame position | Weight type | kg | Source reference |
|---|---|---|---|---:|---|
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Input | 480.0 | T2 17 Oct evening input ticket |
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Good | 457.2 | T2 17 Oct evening good record |
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Changeover | 15.0 | T2 17 Oct evening weight record |
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Restart | 0.0 | T2 17 Oct evening weight record |
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Other | 6.0 | T2 17 Oct evening weight record |
| T2-241017-B | 2024-10-17 17:00 +0100 | Handover | Hold | 1.8 | T2 17 Oct evening hold record |
| T2-241022-B | | Handover | Input | 510.0 | T2 22 Oct evening input ticket |
| T2-241022-B | | Handover | Good | 488.2 | T2 22 Oct evening good record |
| T2-241022-B | | Handover | Changeover | 14.0 | T2 22 Oct evening weight record |
| T2-241022-B | | Handover | Restart | 0.0 | T2 22 Oct evening weight record |
| T2-241022-B | | Handover | Other | 6.0 | T2 22 Oct evening weight record |
| T2-241022-B | | Handover | Hold | 1.8 | HT-07 hold record |
| T2-241022-B | 2024-10-23 08:16 +0100 | After return | Good | 490.0 | T2 good record / HT-07 |
| T2-241022-B | 2024-10-23 08:16 +0100 | After return | Hold | 0.0 | HT-07 return record |
| W1-241029-B | | Handover | Input | 500.0 | W1 29 Oct evening input ticket |
| W1-241029-B | | Handover | Good | 476.5 | W1 29 Oct evening good record |
| W1-241029-B | | Handover | Changeover | 12.0 | W1 29 Oct evening weight record |
| W1-241029-B | | Handover | Restart | 5.0 | W1 29 Oct earlier-run restart record |
| W1-241029-B | | Handover | Other | 5.0 | W1 29 Oct evening weight record |
| W1-241029-B | | Handover | Hold | 1.5 | HW-04 hold record |
| W1-241029-B | 2024-10-30 08:24 +0000 | After return | Good | 478.0 | W1 good record / HW-04 |
| W1-241029-B | 2024-10-30 08:24 +0000 | After return | Hold | 0.0 | HW-04 return record |
