---
path: Documents/Service-reviews/BRT Passenger Identity/2024-05-10/April Identity availability bounds.xlsx
kind: brt-availability-bounds.xlsx
author: Daniel Jennings
derived_from:
---

# April Passenger Identity availability bounds

## Bounds

B3: BRT-AB-20240507-001  
E3: Daniel Jennings  
B4: 07 May 2024  
E4: Passenger Identity · 01 Apr 2024 00:00 to 01 May 2024 00:00 UTC

Each interval includes its start and excludes its end. The denominator in each calculation remains the full eligible window.

| Row | A: UTC window | B: Eligible min | C: Confirmed failed min | D: Unobserved min | E: Lower bound | F: Upper bound |
|---:|---|---:|---:|---:|---|---|
| 7 | 01 Apr 00:00–11 Apr 00:00 | 14400 | 0 | 0 | =IF(B7=0,"",1-(C7+D7)/B7) | =IF(B7=0,"",1-C7/B7) |
| 8 | 11 Apr 00:00–24 Apr 00:00 | 18720 | 12 | 0 | =IF(B8=0,"",1-(C8+D8)/B8) | =IF(B8=0,"",1-C8/B8) |
| 9 | 24 Apr 00:00–30 Apr 00:00 | 8640 | 29 | 0 | =IF(B9=0,"",1-(C9+D9)/B9) | =IF(B9=0,"",1-C9/B9) |
| 10 | 30 Apr 00:00–01 May 00:00 | 1440 | 0 | 4 | =IF(B10=0,"",1-(C10+D10)/B10) | =IF(B10=0,"",1-C10/B10) |
| 11 | April total | =SUM(B7:B10) | =SUM(C7:C10) | =SUM(D7:D10) | =IF(B11=0,"",1-(C11+D11)/B11) | =IF(B11=0,"",1-C11/B11) |

A12:F12 note: 43,200 eligible; 41 confirmed failed; four unobserved. The bounds are 99.89583%–99.90509%. The exact 99.9% target lies within the interval, so target status is undetermined. Compare unrounded rates. The lower bound charges all four unobserved minutes as failed; the upper treats them as successful.
