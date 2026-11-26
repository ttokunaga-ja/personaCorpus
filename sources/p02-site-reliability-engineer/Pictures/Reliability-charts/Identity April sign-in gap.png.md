---
path: Pictures/Reliability-charts/Identity April sign-in gap.png
kind: chart
author: Daniel Jennings
derived_from:
---

# Passenger Identity — 30 April collector gap

Two aligned horizontal lanes share a UTC time axis from 30 April 23:42 to 1 May 00:00, with ticks at 23:42, 23:51, 23:55 and the end tick 00:00. The intervals include the starting minute and exclude the ending minute.

| Lane | Start UTC | End UTC | Duration | Display and label |
|---|---|---|---:|---|
| Primary collector | 23:42 | 00:00 | 18 min | Blank: no outcome collected |
| Reconciled outcomes | 23:42 | 23:51 | 9 min | Blue: confirmed successful |
| Reconciled outcomes | 23:51 | 23:55 | 4 min | Amber hatch: unobserved |
| Reconciled outcomes | 23:55 | 00:00 | 5 min | Blue: confirmed successful |

Legend: blue = verifier-confirmed success; amber hatch = unobserved; blank = no primary outcome. The verifier recovered 14 successes; the four interior minutes remain unknown.
