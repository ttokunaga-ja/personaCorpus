---
path: Downloads/Palmer Fare Payments peak forecast.pdf
kind: tex:palmer-capacity-forecast.cls
author: Louise Turner
derived_from:
---
# Fare Payments peak demand forecast

Palmer Capacity Services  
Reference: PCS-DF-20250425-001  
Issued: 25 April 2025  
Prepared by: Louise Turner  
Prepared for: Daniel Jennings and Matthew Pickering, BRT Booking Systems Ltd

## Forecast for the 28 April review

The forecast is expressed against a reference rate of 1,000 eligible Fare Payments attempts per minute. Each hourly value is the 95th percentile of the one-minute rates in that hour, not an hourly average. Bands are UTC, inclusive of the start and exclusive of the end.

| UTC band | Forecast / reference | Eligible attempts/min at forecast |
|---|---:|---:|
| 15:00–16:00 | 1.70× | 1,700 |
| 16:00–17:00 | 2.05× | 2,050 |
| 17:00–18:00 | 2.30× | 2,300 |
| 18:00–19:00 | 1.90× | 1,900 |

The busiest forecast band is 17:00–18:00 UTC. Its 2.30× figure represents the hourly p95 one-minute rate; it should not be read as a constant rate throughout the hour or as a maximum instantaneous rate.

## Counting and scope

| No. | Assumption or check | Treatment or constraint |
|---:|---|---|
| 1 | Reference | 1,000 eligible Fare Payments attempts/min; compare both forecast and BRT capacity against that same rate. |
| 2 | Duplicate retry indications | Excluded so a repeated indication of the same attempt is not counted as another eligible attempt. |
| 3 | Diagnostic traffic | Excluded from the forecast and reference. |
| 4 | Hour boundaries | Each band includes its start and excludes its end; 18:00 UTC belongs to the following band. |
| 5 | BRT reserve | Forecast demand does not include any reserve BRT may choose to provide. Sizing and verifying that reserve remain BRT's decision. |

Please raise any difference in the counting basis before the review; adding duplicate retry indications to one side would make the comparison misleading.
