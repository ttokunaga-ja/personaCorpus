---
path: Documents/Clients/Whitfield/Finance/Japan pilot approval events.csv
kind: text
author: Whitfield Japan Finance
derived_from:
---

# Cleared approval event extract

Six CSV columns and all rows follow. An empty Event time means the event is known but its time is unavailable. No 09 event is included for a case still awaiting approval. These eleven cases are a permitted subset of 67 candidates; supplier names, accounts and amounts are excluded.

| Case ID | Cohort | Department | Event | Event time | Hold reason |
|---|---|---|---|---|---|
| WP-B01 | Baseline | Components | First 06 | 2025-07-23T09:00:00+09:00 | |
| WP-B01 | Baseline | Components | Returned to AP | 2025-07-24T10:00:00+09:00 | Cost bearer correction |
| WP-B01 | Baseline | Components | Re-circulated | 2025-07-25T10:00:00+09:00 | Cost bearer correction |
| WP-B01 | Baseline | Components | 09 approval | 2025-07-29T15:00:00+09:00 | |
| WP-B16 | Baseline | Packaging | First 06 | 2025-07-29T09:00:00+09:00 | |
| WP-B16 | Baseline | Packaging | 09 approval | 2025-08-04T15:00:00+09:00 | |
| WP-B31 | Baseline | Components | First 06 | 2025-08-14T09:00:00+09:00 | |
| WP-B31 | Baseline | Components | Returned to AP | 2025-08-15T10:00:00+09:00 | Cost bearer reconfirmation |
| WP-B32 | Baseline | Packaging | First 06 | 2025-08-15T09:00:00+09:00 | |
| WP-B32 | Baseline | Packaging | Hold at 08 | 2025-08-18T10:00:00+09:00 | Exception evidence awaited |
| WP-B33 | Baseline | Components | First 06 | | Time missing; date 12 Aug confirmed in AP daily report |
| WP-P01 | Pilot | Components | First 06 | 2025-08-18T09:00:00+09:00 | |
| WP-P01 | Pilot | Components | Returned to AP | 2025-08-20T10:00:00+09:00 | Coding query |
| WP-P01 | Pilot | Components | Re-circulated | 2025-08-21T10:00:00+09:00 | Coding query |
| WP-P01 | Pilot | Components | 09 approval | 2025-08-22T15:00:00+09:00 | |
| WP-P16 | Pilot | Packaging | First 06 | 2025-08-27T09:00:00+09:00 | |
| WP-P16 | Pilot | Packaging | 09 approval | 2025-09-01T09:00:00+09:00 | |
| WP-P31 | Pilot | Components | First 06 | 2025-09-10T09:00:00+09:00 | |
| WP-P31 | Pilot | Components | Returned to AP | 2025-09-11T10:00:00+09:00 | Cost bearer confirmation |
| WP-P32 | Pilot | Packaging | First 06 | 2025-09-10T10:00:00+09:00 | |
| WP-P32 | Pilot | Packaging | Hold at 08 | 2025-09-11T11:00:00+09:00 | Exception-resolution evidence |
| WP-P33 | Pilot | Packaging | First 06 | 2025-09-11T09:00:00+09:00 | |
| WP-P33 | Pilot | Packaging | Hold at 08 | 2025-09-12T10:00:00+09:00 | Confirm 07 deputy for case |
| WP-P34 | Pilot | Components | First 06 | 2025-09-12T09:00:00+09:00 | |
| WP-P34 | Pilot | Components | 09 approval | | Approval confirmed; time missing |

The Hold reason field on WP-B33 and WP-P34 carries the missing-time explanation for the recipient; it does not assert a hold.
