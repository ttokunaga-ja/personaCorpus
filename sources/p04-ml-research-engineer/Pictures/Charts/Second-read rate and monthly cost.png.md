---
path: Pictures/Charts/Second-read rate and monthly cost.png
kind: chart
author: Emily Chen
derived_from:
---
# How much room for a second read?

Audience: Vanessa Nguyen and Brandon Romero.

Horizontal axis: Second-inference share of original records (%), 0 to 25. Vertical axis: Estimated cost (USD/month), 0 to 5,000. Fixed volume: 100,000 original records/month.

| Second-inference share (%) | Cost (USD/month) | Basis |
|---:|---:|---|
| 0 | 3,600 | Proportional increment assumption |
| 12.5 | 4,200 | Proportional increment assumption |
| 18.75 | 4,500 | Proportional increment assumption |
| 25 | 4,800 | Extrapolated from three measured runs of 10,000 original records, each with 2,500 reread |

Draw a line through the points, with the 25% point distinguished from the assumed points. A horizontal line marks “Monthly ceiling: USD 4,500”; annotate its intersection “18.75% under proportional increment assumption”. The assumed points use 3,600 + 1,200 × (share / 25%). Monthly standby monitoring of USD 600 is included once. The 18.75% cost is unmeasured; the intersection is a sensitivity estimate, not an operating threshold.
