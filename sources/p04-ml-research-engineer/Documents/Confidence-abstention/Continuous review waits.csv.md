---
path: Documents/Confidence-abstention/Continuous review waits.csv
kind: text
author: Brandon Romero
derived_from:
---

```csv
simulated_date,wait_minutes,review_count
2024-12-04,0,23
2024-12-04,1,2
2024-12-04,2,3
2024-12-05,0,22
2024-12-05,1,5
2024-12-05,2,5
2024-12-06,0,22
2024-12-06,1,4
2024-12-06,2,4
```

### Check on the continuous-review comparison

The table is the arrival-to-start wait distribution for the 90 items requiring review out of 600 arrivals. I used the December 6 working-copy assumption of one reviewer available 09:00–17:00 PST, first come, first served, three minutes per item, and an empty queue each morning. I counted each review once, on its simulated arrival date.

The three daily counts sum to 28, 32 and 30. Across days there are 67 immediate starts, eleven 1-minute waits and twelve 2-minute waits. Even in the short succession of midday arrivals, the longest wait is 2 minutes and the maximum outstanding count is 2 items on each day, including any item being checked. Each day ends with zero outstanding.
