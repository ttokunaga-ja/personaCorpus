---
path: Downloads/Fare Payments Q4 2024 availability.csv
kind: text
author: Daniel Jennings
derived_from:
---

```csv
service,period_start_utc_inclusive,period_end_utc_exclusive,eligible_seconds,unavailable_seconds,faregate_seconds,availability_percent
BRT Fare Payments,2024-10-01T00:00:00Z,2024-11-01T00:00:00Z,2678400,552,0,99.97939
BRT Fare Payments,2024-11-01T00:00:00Z,2024-12-01T00:00:00Z,2592000,2520,2520,99.90278
BRT Fare Payments,2024-12-01T00:00:00Z,2025-01-01T00:00:00Z,2678400,1728,1368,99.93548
```

Availability is calculated as 100 × (1 − unavailable_seconds / eligible_seconds), rounded to five decimal places. Faregate is a subset of unavailable seconds; the remainder is included in the total. Bounds are inclusive at the start and exclusive at the end.
