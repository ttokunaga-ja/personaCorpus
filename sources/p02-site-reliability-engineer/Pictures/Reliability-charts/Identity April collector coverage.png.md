---
path: Pictures/Reliability-charts/Identity April collector coverage.png
kind: chart
author: Daniel Jennings
derived_from:
---

# Passenger Identity — primary collector coverage, April 2024

For Emma · UTC days · Daniel Jennings

A daily bar chart of observed scheduled sign-in outcomes. The horizontal axis is **April 2024 (UTC day)**, labelled 1 through 30. The vertical axis is **Observed minutes**, starting at 0 and extending above 1,440. A labelled reference line at 1,440 reads **Expected per UTC day: 1,440 minutes**. The legend identifies the bars as **Primary collector outcomes (success or failure)** and the line as **Expected minutes**.

| UTC day | Observed minutes |
|---:|---:|
| 1 | 1,440 |
| 2 | 1,440 |
| 3 | 1,440 |
| 4 | 1,440 |
| 5 | 1,440 |
| 6 | 1,440 |
| 7 | 1,440 |
| 8 | 1,440 |
| 9 | 1,440 |
| 10 | 1,440 |
| 11 | 1,440 |
| 12 | 1,440 |
| 13 | 1,440 |
| 14 | 1,440 |
| 15 | 1,440 |
| 16 | 1,440 |
| 17 | 1,440 |
| 18 | 1,440 |
| 19 | 1,440 |
| 20 | 1,440 |
| 21 | 1,440 |
| 22 | 1,440 |
| 23 | 1,440 |
| 24 | 1,440 |
| 25 | 1,440 |
| 26 | 1,440 |
| 27 | 1,440 |
| 28 | 1,440 |
| 29 | 1,440 |
| 30 | 1,422 |

Callout on 30 April: **18 minutes missing: 23:42–00:00 UTC**.

Source note on chart: **BRT-AV-20240501-001; passenger_identity_hourly_apr30.csv. The flash records 18 collector blanks across the 43,200-minute month, all in the final 18 minutes. The 30 April hourly export has 1,422 outcomes and 18 blanks. With one scheduled outcome per minute, the other 29 UTC days therefore each have 1,440 observed outcomes. Observed failures are included; coverage is not availability.**
