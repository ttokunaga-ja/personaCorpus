---
path: Pictures/Reliability-charts/Ticket archive starts and finishes.png
kind: chart
author: Daniel Jennings
derived_from:
---

# Ticket archive starts and finishes · 11 September 2024

Two horizontal tiers share one linear axis, “Time (UTC)”, running from 12:05 to 12:45. Tick labels: 12:05, 12:10, 12:15, 12:20, 12:25, 12:30, 12:35, 12:40, 12:45. Plot the marks at their full UTC seconds.

| Tier label | UTC mark | Text at mark |
|---|---|---|
| Scheduled starts · 10-minute interval | 12:10:00 | 12:10 · W-3184 starts |
| Scheduled starts · 10-minute interval | 12:20:00 | 12:20 · Next start |
| Scheduled starts · 10-minute interval | 12:30:00 | 12:30 · W-3186 starts |
| Scheduled starts · 10-minute interval | 12:40:00 | 12:40 · Next start |
| Confirmed finishes · Exception extract | 12:22:10 | 12:22:10 · W-3184 finishes |
| Confirmed finishes · Exception extract | 12:41:35 | 12:41:35 · W-3186 finishes |

Use a distinct connector for each labelled write between its start and finish. The key identifies “Scheduled start”, “Confirmed predecessor finish” and “Overlap after next start”. Show the overlap spans separately from the write connectors:

| Write | Overlap from (UTC) | Overlap to (UTC) | Span label |
|---|---|---|---|
| W-3184 | 12:20:00 | 12:22:10 | 2m 10s |
| W-3186 | 12:40:00 | 12:41:35 | 1m 35s |

Caption: The finish tier shows only the two predecessors followed through to completion. Source: Ticket archive overlap pairs.csv.
