---
path: Downloads/RJS archive interval findings.pdf
kind: tex:rjs-storage-witness.cls
author: Patrick Sutcliffe
derived_from:
---

# Ticket archive interval findings

Witness record: RJS-WS-20240912-001  
Issued: 12 September 2024  
Witness: Patrick Sutcliffe, RJS Data Storage Ltd  
Prepared for: Daniel Jennings and Sanjay Mehta, BRT Booking Systems Ltd

## Observation conditions

Sanjay and I watched the isolated ticket archive on 9 September with a 15-minute schedule and on 11 September with a 10-minute schedule. Both runs started with an empty queue and no write in progress. The request profile was the same on both days, with the busier hour from 12:00 to 13:00 UTC.

We counted actual starts in each 09:00–15:00 UTC window, including the opening start and excluding 15:00. At each start we checked whether the immediately preceding write was still open, matching the finish to that write's reference and comparing full UTC seconds. A finish at the same second as the new start was clear.

## Observed overlaps

At 15 minutes we saw 0 overlaps in 24 starts (0/24). At 10 minutes we saw 2 overlaps in 36 starts (2/36). Both fell among the six starts in the 12:00–13:00 UTC block on 11 September. The separate RJS archive hourly observations.csv contains the six hourly counts per run.

Daniel followed both flagged predecessors to completion with Sanjay and me. We checked each finish against the earlier write's reference.

| No. | Evidence read with BRT | Witness observation |
|---:|---|---|
| 1 | 11 Sep, W-3184 started 12:10:00 UTC | Still open at the 12:20:00 UTC start. W-3184 completed at 12:22:10 UTC, 2 min 10 s after that start. |
| 2 | 11 Sep, W-3186 started 12:30:00 UTC | Still open at the 12:40:00 UTC start. W-3186 completed at 12:41:35 UTC, 1 min 35 s after that start. |

## Follow-up

Keep the 10-minute interval as a candidate for a controlled load trial and run the 15-minute comparator under the same sustained load. Carry the write references through to the finish so each overlap can be checked again. Assess restorable coverage separately with a fixed-reference readback. These windows provide start and finish evidence only. Keep the operator recovery statement on its existing terms.
