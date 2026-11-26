---
path: Documents/Clients/Marston/23 Oct - T2 return trace.xlsx
kind: Working sheet
author: Oliver Hewitt
derived_from:
---

# Trafford T2 · HT-07 return trace
Oliver Hewitt · 23 Oct 2024 · For Connor Atkinson

## Sheet: Ticket and tag

| Local time (BST) | Record to pull | What the record shows | Link to prior entry |
|---|---|---|---|
| 22 Oct evening | Changeover trial ticket T2-241022-B | Julie Naylor held three S3 first-off sheets at changeover under tag HT-07. | Keep the original ticket against the evening shift. |
| 22 Oct handover | HT-07 hold record and T2 evening good record | The box accounts for the 1.8 kg held aside. Good stands at 488.2 kg for the whole shift frame. | Match the tag to the held entry before looking for the next morning's return. |
| 23 Oct 08:12 | HT-07 and trial ticket | Gareth Pritchard approved the sheets after the specification check. | Stock position still at handover until the box moves. |
| 23 Oct 08:16 | Signed HT-07 and T2 good record / HT-07 | Gareth signed at the actual return. Good stock includes the box at this point. | Apply this movement to the originating evening frame once. |

## Sheet: Evening frame

The 22 Oct frame includes the earlier runs on the evening shift. Both positions concern the same 510.0 kg of input. Keep the 23 Oct morning's new output in its own frame.

| Position | Input kg | Good kg | Changeover kg | Restart kg | Other kg | Hold kg | Residual kg |
|---|---:|---:|---:|---:|---:|---:|---:|
| 22 Oct handover | 510.0 | 488.2 | 14.0 | 0.0 | 6.0 | 1.8 | =ROUND(B2-SUM(C2:G2),1) |
| As posted after 23 Oct 08:16 | 510.0 | 490.0 | 14.0 | 0.0 | 6.0 | 0.0 | =ROUND(B3-SUM(C3:G3),1) |

## Sheet: Movement checks

| Check | kg | Read against the record |
|---|---:|---|
| Increase in good | =ROUND('Evening frame'!C3-'Evening frame'!C2,1) | T2 good record / HT-07 |
| Reduction in hold | =ROUND('Evening frame'!G2-'Evening frame'!G3,1) | HT-07 hold and return records |
| Difference | =ROUND(B2-B3,1) | Should be zero for a transfer with unchanged losses. |

Connor, take 490.0 kg as the posted good total after the return. The 1.8 kg transfer is already in that figure.
