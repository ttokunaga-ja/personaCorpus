---
path: Documents/Incidents/2026/2026-07-13/What came out of the first count.md
kind: text
author: Matthew Pickering
derived_from:
---
# What came out of the first count

For Daniel and Claire — INC-260713, 13 July 2026

At 08:18 UTC we sent an initial estimate of about 380 failure indications from the overview. That was 384 rounded, before checking what the indications represented. It was not a count of failed customers.

The disposition reconciliation we signed off at 10:20 UTC accounts for all 384 indications:

| Disposition | Indications | Treatment |
|---|---:|---|
| Unique eligible failed Faregate attempts | 312 | Counted against the service measure |
| Additional retry indications for those same attempts | 48 | Excluded to avoid counting an attempt twice |
| Diagnostic indications outside the eligible population | 24 | Excluded from the service measure |
| Total | 384 | |

The confirmed result is 312 failed eligible attempts out of 4,800 eligible Faregate checkout attempts from 08:07 inclusive to 08:41 exclusive UTC. The denominator is the incident window, not the whole month or the whole 08:00 hour.

An attempt is not a person: one person may make more than one attempt, and the aggregate does not establish how many people were affected. The figure covers the monitored Faregate path across the service; it cannot all be assigned to Westbury. Claire should use that scope if citing the number.

Matthew
