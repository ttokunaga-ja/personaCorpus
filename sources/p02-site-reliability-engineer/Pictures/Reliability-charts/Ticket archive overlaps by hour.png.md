---
path: Pictures/Reliability-charts/Ticket archive overlaps by hour.png
kind: chart
author: Daniel Jennings
derived_from:
---

# Ticket archive overlaps by hour

Use paired bars at each of six shared UTC hour bins. The horizontal axis is “Hour of current start (UTC)” and the vertical axis is “Starts with an open predecessor” in counts, with integer ticks from 0 to 2. Legend entries are “9 Sep 2024 · 15-minute schedule” and “11 Sep 2024 · 10-minute schedule”. Print the fractions below as “n of d” at the corresponding bars, including baseline labels for zero counts.

| UTC hour | 9 Sep 2024 · 15-minute schedule | 11 Sep 2024 · 10-minute schedule |
|---|---:|---:|
| 09:00–10:00 | 0 of 4 | 0 of 6 |
| 10:00–11:00 | 0 of 4 | 0 of 6 |
| 11:00–12:00 | 0 of 4 | 0 of 6 |
| 12:00–13:00 | 0 of 4 | 2 of 6 |
| 13:00–14:00 | 0 of 4 | 0 of 6 |
| 14:00–15:00 | 0 of 4 | 0 of 6 |

Caption below the chart: Each hour groups the current starts, even when a preceding write finishes in a different hour. Counts are out of the starts observed in that hour.

Source line: RJS archive hourly observations.csv
