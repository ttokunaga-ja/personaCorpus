---
path: Downloads/April Identity hourly figures.csv
kind: text
author: Daniel Jennings
derived_from:
---

# Reconciled hourly figures — CSV specification

One header and 720 data rows, ordered by `hour_start_utc`. Start at 2024-04-01T00:00:00Z and advance exactly one hour per row through 2024-04-30T23:00:00Z. `hour_end_utc` is one hour after start, exclusive; the last end is 2024-05-01T00:00:00Z. Each row has 60 eligible minutes. All counts are integer minutes. For every ordinary hour, primary successes = 60 and each other outcome count = 0. Use the following exceptions only. Each row's four outcome columns sum to eligible minutes.

```csv
hour_start_utc,hour_end_utc,eligible_minutes,primary_success_minutes,confirmed_failure_minutes,verifier_recovered_success_minutes,unobserved_minutes
2024-04-01T00:00:00Z,2024-04-01T01:00:00Z,60,60,0,0,0
2024-04-01T01:00:00Z,2024-04-01T02:00:00Z,60,60,0,0,0
2024-04-02T12:00:00Z,2024-04-02T13:00:00Z,60,60,0,0,0
2024-04-10T23:00:00Z,2024-04-11T00:00:00Z,60,60,0,0,0
2024-04-11T09:00:00Z,2024-04-11T10:00:00Z,60,60,0,0,0
2024-04-11T10:00:00Z,2024-04-11T11:00:00Z,60,48,12,0,0
2024-04-11T11:00:00Z,2024-04-11T12:00:00Z,60,60,0,0,0
2024-04-24T16:00:00Z,2024-04-24T17:00:00Z,60,60,0,0,0
2024-04-24T17:00:00Z,2024-04-24T18:00:00Z,60,31,29,0,0
2024-04-24T18:00:00Z,2024-04-24T19:00:00Z,60,60,0,0,0
2024-04-30T22:00:00Z,2024-04-30T23:00:00Z,60,60,0,0,0
2024-04-30T23:00:00Z,2024-05-01T00:00:00Z,60,42,0,14,4
```

Column totals for reconciliation, outside the CSV: eligible 43,200; primary successes 43,141; confirmed failures 41; verifier-recovered successes 14; unobserved 4. The two success columns total 43,155. Do not add a totals row to the CSV.
