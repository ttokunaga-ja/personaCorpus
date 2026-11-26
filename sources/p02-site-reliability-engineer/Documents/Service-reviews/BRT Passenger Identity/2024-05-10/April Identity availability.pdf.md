---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-10/April Identity availability.pdf
kind: tex:brt-monthly-availability.cls
author: Daniel Jennings
derived_from:
---

# April Passenger Identity availability

Reference: BRT-AV-20240510-001  
Issued: 10 May 2024  
Prepared by: Daniel Jennings, Reliability Engineering  
To: Anjali Shah and Emma Wilkinson

## Scope and closed result

The Passenger Identity monthly sign-in measure counts one scheduled end-to-end outcome per UTC minute. The April window begins at 1 April 00:00 UTC and ends at 1 May 00:00 UTC, inclusive of the start and exclusive of the end. All 43,200 minutes remain in the denominator. The monthly target is 99.9%.

Availability is **99.89583%–99.90509%**. Four minutes are unobserved. The exact target lies between the unrounded bounds, so April's target status is **undetermined**.

| Measure | Value | Basis / treatment |
|---|---:|---|
| Eligible minutes | 43,200 | Full April UTC window |
| Confirmed successful minutes | 43,155 | 43,141 primary successes plus 14 recovered successes |
| Known failed minutes | 41 | Observed failed scheduled sign-ins |
| Unobserved minutes | 4 | No matched outcome at 23:51–23:55 UTC on 30 April |
| Lower availability bound | 99.89583% | 1 − (41 + 4) / 43,200; all four assumed failed |
| Upper availability bound | 99.90509% | 1 − 41 / 43,200; all four assumed successful |
| Monthly target | 99.9% | Target status undetermined |

The bounds describe the possible treatment of the four unknown outcomes, not a measurement of their actual results. They use the same complete April denominator. At 99.9%, the exact allowance is 43.2 minutes; 41 known failures are below that allowance, while 45 including all four unknowns are above it.

## Revision of the flash

This report supersedes the provisional treatment in BRT-AV-20240501-001, issued 1 May. That flash charged 59 minutes: 41 confirmed failures plus all 18 minutes absent from the primary collector. It reported 99.86343% on that basis. The 18 blanks were not observed failures; the subsequent verifier reconciliation establishes 14 successes and leaves four outcomes unknown. The flash remains the record of the initial estimate.

## Failure windows and collector gap

The 41 known failed minutes are the scheduled sign-in checks observed to fail on 11 April 10:12–10:24 UTC (12 minutes) and 24 April 17:21–17:50 UTC (29 minutes). Interval starts are inclusive and ends exclusive.

The primary collector has no outcomes for 30 April 23:42 to 1 May 00:00 UTC. Emma's separately retained verifier aggregate records completed sign-in outcomes against the same scheduled UTC minutes. It deduplicates retries within each scheduled minute, leaving at most one matched outcome per slot. It confirms nine distinct successful minutes from 23:42 to 23:51 and five from 23:55 to 00:00, including the slot starting at 23:55. The 00:00 slot belongs to May.

The four slots starting at 23:51, 23:52, 23:53 and 23:54 have no matched outcome in the verifier. The successes on either side do not establish what happened in those slots; no outcome has been carried forward or interpolated.

## Decision and next review

At the 9 May 10:00 UTC readout, Anjali accepted the interval and an undetermined target status (BRT-MT-20240509-001). She directed that unobserved minutes be reported alongside availability from the May report. Emma Wilkinson and Daniel Jennings will check the May report's missing-minute field on 3 June 2024 at 10:00 UTC.
