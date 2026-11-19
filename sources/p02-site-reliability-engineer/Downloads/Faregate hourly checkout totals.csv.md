---
path: Downloads/Faregate hourly checkout totals.csv
kind: text
author: Daniel Jennings
derived_from:
---
```csv
window_start_utc,window_end_utc,eligible_faregate_attempts,confirmed_incident_failures
2026-07-13T07:00:00Z,2026-07-13T08:00:00Z,7960,0
2026-07-13T08:00:00Z,2026-07-13T09:00:00Z,8640,312
2026-07-13T09:00:00Z,2026-07-13T10:00:00Z,9100,0
2026-07-13T10:00:00Z,2026-07-13T11:00:00Z,8800,0
```

Each window includes its start and excludes its end. The 08:00–09:00 UTC bucket includes time on either side of the narrower 08:07–08:41 UTC incident window, which contained 4,800 eligible attempts.
