---
path: Documents/Clients/Marston/30 Oct - W1 return trace.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# W1 · HW-04 shelf check and return
30 Oct 2024 · Connor Atkinson · For Martin Devlin and Helen Rigby

## Sheet: Hold evidence

| Point in handover (GMT) | Evidence | Entries to match | Hold kg |
|---|---|---|---:|
| 29 Oct evening, outgoing | W1-241029-B trial ticket and HW-04 hold record | Helen Rigby left three first-off sheets from the changeover in the tagged box. The evening weight record carries the held weight under HW-04. | 1.5 |
| 30 Oct morning, before approval | Shelf check with Martin Devlin | We weighed the contents of the box marked HW-04. Same box as on Helen's ticket. | 1.5 |

The morning shelf weight agrees with the evening hold entry. The two readings refer to the same box and are recorded at different points in the handover.

## Sheet: Posting check

| Time on 30 Oct (GMT) | Record | Martin's entry | Effect on evening frame |
|---|---|---|---|
| 08:18 | HW-04 / W1-241029-B trial ticket | Approval after checking the sheets against the job instruction | Keep the box in held weight while it remains on the shelf. |
| 08:24 | Signed HW-04 and W1 good record / HW-04 | Signature and actual return time recorded when the box went into good stock | Move the box weight from hold to good within the same evening frame. |

## Sheet: Evening weight

| Weight in the 29 Oct evening frame | At handover kg | After 30 Oct 08:24 kg | Source / treatment |
|---|---:|---:|---|
| Input | 500.0 | 500.0 | W1 29 Oct evening input ticket. |
| Good | 476.5 | 478.0 | W1 evening good record and W1 good record / HW-04. |
| Changeover loss | 12.0 | 12.0 | W1 evening weight record. |
| Restart loss | 5.0 | 5.0 | Earlier-run restart record. It belongs to a separate event. |
| Other loss | 5.0 | 5.0 | W1 evening weight record. |
| Hold | 1.5 | 0.0 | HW-04 hold record and HW-04 return record. |
| Residual | =ROUND(B2-SUM(B3:B7),1) | =ROUND(C2-SUM(C3:C7),1) | Input less all accounted weight. |
| Shift in good matched to reduction in hold | | =ROUND((C3-B3)-(B7-C7),1) | Zero confirms that the whole good movement is accounted for by this box. |

Martin and Helen, the 478.0 kg is the updated total for the 29 Oct evening runs. It includes the 1.5 kg from HW-04 once. Keep the earlier restart loss at 5.0 kg and record new morning production in its own shift.
