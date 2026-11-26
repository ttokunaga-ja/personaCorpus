---
path: Documents/Confidence-abstention/Dec 5 - Queue reconciliation.pdf
kind: tex:mercer-research-brief.cls
author: Emily Chen
derived_from:
---

# December 5 queue reconciliation

Document ID: MDR-CA-20241213-RM-01-R01  
Issued: December 13, 2024  
Author: Emily Chen  
To: Brandon Romero and Vanessa Nguyen

## What counts at restart

I checked the December 5 panel in *December review queue by time* against the keyed comparison and the cumulative counts in *Queue balance checks.tsv*. Use the lunch-pause run and count only arrivals marked for review. The clock is PST.

Seven reviews arrived and finished before noon. Intake admitted another 18 review items during the pause. The only reviewer takes them in arrival order starting at 13:00, at three minutes to completion per item. The two ends of that block give a direct check on the plotted plateau and the last drop.

| Intake key | Lunch-block position | Arrival | Start | Completion |
|---|---|---|---|---|
| QD-287 | First | 12:05 | 13:00 | 13:03 |
| QD-327 | Last | 12:58 | 13:51 | 13:54 |

## Check the drops against completions

At 12:58 the last lunch arrival brings the balance to 18. At 13:00, 25 cumulative arrivals less seven cumulative completions still leaves 18. Subtracting the one new start would give 17, which counts only the items waiting to start. QD-287 is in review until 13:03. That's when the outstanding line first drops to 17.

| Time | Cumulative review arrivals | Cumulative completions | Outstanding |
|---|---:|---:|---:|
| 13:00 | 25 | 7 | 18 |
| 13:03 | 25 | 8 | 17 |
| 13:15 | 25 | 12 | 13 |
| 13:30 | 25 | 17 | 8 |
| 13:54 | 25 | 25 | 0 |

The first 15 minutes after restart fit five full reviews, with completions at 13:03, 13:06, 13:09, 13:12 and 13:15. This leaves 13 of the lunch arrivals open at 13:15. The eighteenth completion is 54 minutes after restart, at 13:54. The next review arrival is at 14:15, so there is no arrival to offset any of the steps between those endpoints.
