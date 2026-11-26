---
path: Documents/Clients/Marston/18 Oct - T2 ticket query.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# T2 ticket query
Connor Atkinson · 18 Oct 2024, 09:24 BST · For Julie Naylor and Gareth Pritchard

## Sheet: Record checks

One open event, T2-241017-B, changeover on the Trafford T2 evening shift, 17 October. Times are local (BST). A blank in the value column means the entry could not be verified at the morning check on 18 October.

| Source checked | Field | Date and time of event or snapshot | Recorded value | Unit | What the source supports |
|---|---|---|---|---|---|
| Changeover ticket / Julie's 17 Oct comments | Changeover | 17 Oct 2024 16:42 | T2-241017-B | Run | Julie identifies the time in her comments on First-off instructions v0.2. Use the run on the changeover ticket to follow the event into the weight and good-output records. |
| Changeover ticket | First-off sheets held at change of shift | 17 Oct 2024 17:00 | 3 | Sheets | The ticket identifies the three sheets. Julie's note places them on hold at the change of shift. |
| Weight slip | Weight held at change of shift | 17 Oct 2024 17:00 | 1.8 | kg | Weight of the three held sheets at the boundary. This is the held weight to trace. |
| Good-output record | Return signature | | | | No signed return could be linked to these three sheets by the morning check. |
| Good-output record | Actual time of return | | | | No return time could be linked to the changeover. |
| Good-output record | Good weight after return | | | kg | A post-return weight for this box cannot be verified from the records available. |

## Sheet: Open query

Header row at row 1, query at row 2.

| Run | Event | Held sheets | Held at 17:00 kg | Verified return kg | Outstanding tie-out kg | Status | Reply requested |
|---|---|---:|---:|---:|---:|---|---|
| T2-241017-B | Changeover | 3 | 1.8 | | =IF(ISNUMBER(E2),ROUND(D2-E2,1),D2) | Return unverified | Julie: confirm the outgoing ticket and slip describe the same three sheets. Gareth: identify any original entry that links the box to a signed, timed return to good stock. |

The 1.8 kg outstanding is the weight awaiting evidence. It does not establish where the sheets were on Friday morning. Leave the verified-return field blank until a source identifies the box and records the return. Keep the original 17:00 hold and the good-output record as recorded. Please bring any source reference you find to the 11:30 review.

## Sheet: Weight position

The 17:00 position covers the evening shift's target runs, including runs before T2-241017-B. The good-output value is the shift's running total at handover. Headers are in row 1.

| Position | Input kg | Good kg | Changeover loss kg | Restart loss kg | Other kg | Hold kg | Residual kg |
|---|---:|---:|---:|---:|---:|---:|---:|
| 17 Oct evening at 17:00 | 480.0 | 457.2 | 15.0 | 0.0 | 6.0 | 1.8 | =ROUND(B2-SUM(C2:G2),1) |

The handover weights balance with the 1.8 kg in hold. The balance only accounts for the weight at 17:00. There is no verified later position for this box to add to the sheet.
