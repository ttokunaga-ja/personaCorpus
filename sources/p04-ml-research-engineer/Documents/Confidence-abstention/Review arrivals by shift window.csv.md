---
path: Documents/Confidence-abstention/Review arrivals by shift window.csv
kind: text
author: Emily Chen
derived_from:
---

```csv
simulated_date,window_start_pst,window_end_pst,all_arrivals,review_required_arrivals
2024-12-04,09:00,12:00,85,8
2024-12-04,12:00,13:00,35,10
2024-12-04,13:00,17:00,80,10
2024-12-05,09:00,12:00,84,7
2024-12-05,12:00,13:00,43,18
2024-12-05,13:00,17:00,73,7
2024-12-06,09:00,12:00,85,8
2024-12-06,12:00,13:00,39,14
2024-12-06,13:00,17:00,76,8
```

### Reconciliation for Vanessa

Counts are by arrival minute from the fixed order in the December 3 replay design. Each window includes its start and excludes its end. A 12:00 arrival belongs to the midday window and a 13:00 arrival belongs to the afternoon window. Review-required arrivals are a subset of all arrivals.

The windows add back to 200 arrivals on each date, with 28, 32 and 30 requiring review. The midday rows account for 117 arrivals, of which 42 require review. Counting only the hours in which review can take place would leave 48 of the 90 review items in the table. The 42 still arrive and have to be accounted for when review resumes.
