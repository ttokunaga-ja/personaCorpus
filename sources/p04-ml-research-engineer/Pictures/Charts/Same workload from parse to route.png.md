---
path: Pictures/Charts/Same workload from parse to route.png
kind: chart
author: Vanessa Nguyen
derived_from:
---

# Same workload from parse to route

Horizontal bar chart. Horizontal axis: USD per 10,000 requests, starting at 0; ticks 0, 0.2, 0.4, 0.6, 0.8. Two bars, with legend identifying Linear word-frequency and Larger contextual. Direct labels on bars:

| Model | USD per 10,000 requests |
|---|---:|
| Linear word-frequency | 0.3333 |
| Larger contextual | 0.7000 |

Annotation: Larger / linear = 2.10× baseline. Costs shown to four decimal places; ratio calculated before rounding and shown to two decimal places.

Caption: Same resident worker and full path, first parse through final destination output. Each timing covers 1,000 requests; bars use the median of three runs for each model. Worker allocation USD 1.20 per hour. Startup, external waiting and human review excluded.
