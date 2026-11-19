---
path: Documents/短文受付意図分類/Measurements/Intake cost at equal volume.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Intake cost at equal volume

## Sheet: Adjudication

- A2: Intake cost at equal volume
- B3: MDR-STI-20241008-AW-01-R01
- E3: October 8, 2024
- B4: Vanessa Nguyen
- E4: Resident worker; full path; 1,000 requests per run

| Row | A: Input / model | B: Run 1 seconds | C: Run 2 seconds | D: Run 3 seconds | E: Median seconds | F: USD per 10,000 requests |
|---:|---|---:|---:|---:|---|---|
| 6 | Linear word-frequency | 98 | 100 | 103 | =MEDIAN(B6:D6) | =E6/3600*$B$9*$B$10/$B$11 |
| 7 | Larger contextual | 206 | 210 | 214 | =MEDIAN(B7:D7) | =E7/3600*$B$9*$B$10/$B$11 |
| 8 | Larger / linear |  |  |  |  | =F7/F6 |
| 9 | Worker rate, USD/hour | 1.20 |  |  |  | Input; same worker for both |
| 10 | Comparison volume, requests | 10000 |  |  |  | Input |
| 11 | Requests per timed run | 1000 |  |  |  | Input |

Display F6 and F7 to four decimal places: USD 0.3333 and USD 0.7000 per 10,000 requests. Display F8 to two decimal places: 2.10; the ratio uses the unrounded costs.

Brandon and I aligned both measures this morning to the first parse through final destination output. The larger model's 150-second forward-only median is inside that path; comparing it with the linear full path produces 1.50 but omits work on one side. Each timed run contains all 934 development bodies and a repeat of the first 66, for volume only. Startup, external waiting and human review are excluded; this is a resident-worker allocation, not a full operating-cost estimate.
