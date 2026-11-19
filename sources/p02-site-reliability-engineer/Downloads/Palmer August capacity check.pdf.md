---
path: Downloads/Palmer August capacity check.pdf
kind: tex:palmer-capacity-forecast.cls
author: Louise Turner
derived_from:
---
# Fare Payments August capacity check

Reference: PCS-DF-20260818-001  
Issued: 18 Aug 2026  
Prepared by: Louise Turner, Palmer Capacity Services  
Submitted to: Daniel Jennings and Matthew Pickering, BRT Booking Systems Ltd

## Finding

The lowest remaining capacity in the four observed bands on 17 August was 28%, at 17:00–18:00 UTC. The hourly p95 of one-minute eligible attempt rates was 2,016/min, leaving 784/min against the comparable verified ceiling of 2,800/min. Headroom is (2,800 − 2,016) / 2,800, not a percentage of the normal reference rate used for the earlier planning multiplier.

## Comparison

| 17 August 2026 band, UTC | Valid minutes | p95 eligible attempts/min | Remaining attempts/min | Headroom against 2,800/min |
|---|---:|---:|---:|---:|
| 15:00–16:00 | 60 | 1,680 | 1,120 | 40% |
| 16:00–17:00 | 60 | 1,848 | 952 | 34% |
| 17:00–18:00 | 60 | 2,016 | 784 | 28% |
| 18:00–19:00 | 60 | 1,792 | 1,008 | 36% |

The bands are half-open: an observation at the end belongs to the next band. These are hourly p95s of one-minute rates, not hourly totals or a daily average.

## Basis and limits

| No. | Check | Treatment |
|---:|---|---|
| 1 | Rehearsal ceiling | The separate 17 August 13:00–13:30 UTC hold sustained 2,800 eligible attempts/min with a representative Fare Payments mix and the normal worker pool. It was the highest sustained rate verified in that rehearsal. |
| 2 | Observed workload | The four bands use the eligible Fare Payments path and 60 valid one-minute observations each. Compare with the ceiling only while pool and workload mix remain comparable. |
| 3 | Scope | Failover capacity and production change safety were outside this capacity check. The finding is not an approval to change production. |

Louise Turner  
Palmer Capacity Services
