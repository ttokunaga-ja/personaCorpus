---
path: Documents/Recovery-evidence/Archive interval measurements.md
kind: text
author: Sanjay Mehta
derived_from:
---

# Ticket archive interval measurements

4 September 2024 · Sanjay Mehta  
For Patrick Sutcliffe and Daniel Jennings

## Observation windows

| Date | Schedule | UTC window |
|---|---:|---|
| 9 September 2024 | 15 minutes | 09:00:00 to 15:00:00 |
| 11 September 2024 | 10 minutes | 09:00:00 to 15:00:00 |

Start both runs from the same conditions in the isolated ticket archive. Before 09:00 UTC, confirm the queue is empty and no write is active, and record that check. Run the same synthetic request profile on both days, with the busier part from 12:00 to 13:00 UTC.

## Counting starts

Record the actual starts from 09:00:00 UTC inclusive to 15:00:00 UTC exclusive. Use the actual start time to assign each write to its UTC hour. Include the first start in the count and record that it had no predecessor.

At each subsequent start, check the immediately preceding write. Count an overlap when that write is still open at the new start. If the predecessor completed at or before the start second, mark the start clear.

## Matching completion marks

Keep full UTC seconds on both marks. Use the earlier write's reference to identify its completion. If it's still running at 15:00 UTC, keep following that reference until it completes. Keep that finish with the original observation and leave starts at or after 15:00 out of the denominator. If a completion cannot be located, leave the check unresolved.

Keep the paired marks alongside the hourly tally so we can read back each flag. The comparison measures whether the prior write was open. Restorable coverage needs a separate readback, and sustained load testing is a separate decision.
