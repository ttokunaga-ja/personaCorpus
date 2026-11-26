---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-01/April Identity availability - Flash.pdf
kind: tex:brt-monthly-availability.cls
author: Daniel Jennings
derived_from:
---

# April Passenger Identity availability — Flash

Reference: BRT-AV-20240501-001  
Issued: 1 May 2024  
Prepared by: Daniel Jennings, Reliability Engineering  
To: Emma Wilkinson and Anjali Shah

## Scope and provisional result

The monthly sign-in measure covers one scheduled end-to-end outcome per minute from 1 April 00:00 UTC up to, but excluding, 1 May 00:00 UTC. The full window contains 43,200 eligible minutes. The target is 99.9%.

| Measure | Value | Basis / treatment |
|---|---:|---|
| Eligible minutes | 43,200 | Full April UTC window |
| Confirmed failed minutes | 41 | Observed failed sign-in outcomes |
| Collector blanks | 18 | Provisionally charged as downtime |
| Total charged minutes | 59 | 41 confirmed failures plus 18 blanks |
| Flash availability | 99.86343% | (43,200 − 59) / 43,200 |
| Target | 99.9% | Monthly Passenger Identity target |

The provisional figure is below the target. The primary collector has no outcomes for 30 April 23:42 UTC to 1 May 00:00 UTC. Those 18 minutes are included in the charge above, not established as failed sign-ins. An independent minute-by-minute check is pending; the figure should be revisited before the April report is closed.
