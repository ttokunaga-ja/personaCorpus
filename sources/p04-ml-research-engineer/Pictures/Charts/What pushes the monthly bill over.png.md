---
path: Pictures/Charts/What pushes the monthly bill over.png
kind: chart
author: Brandon Romero
derived_from:
---
# What pushes the monthly bill over

Waterfall chart: monthly processing cost for 100,000 original inspection records. Vertical axis: USD per month, starting at zero. Horizontal axis, left to right: Opening only; Accelerator increase; Host increase; Second-read proposal.

| Bar | Start, USD/month | End, USD/month | Label |
|---|---:|---:|---|
| Opening only | 0 | 3,600 | 3,600 |
| Accelerator increase | 3,600 | 4,600 | +1,000 |
| Host increase | 4,600 | 4,800 | +200 |
| Second-read proposal | 0 | 4,800 | 4,800 |

Connect the steps at 3,600 and 4,600. Label the intermediate level “4,600 after accelerator increase”. Draw a horizontal line at USD 4,500 labelled “Monthly ceiling: 4,500”. Label the final total “300 over ceiling”.

Caption: Both estimates include USD 400 input preparation and USD 600 standby monitoring per month. Each uses 100,000 original records. Opening only has no second reads; the second-read proposal assumes a 25% second-read share. Estimates scale mean resource use from three timed runs on 10,000 originals per condition; both passes are included in the proposal's resource use.
