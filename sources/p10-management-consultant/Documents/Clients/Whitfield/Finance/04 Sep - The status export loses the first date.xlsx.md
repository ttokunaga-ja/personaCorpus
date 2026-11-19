---
path: Documents/Clients/Whitfield/Finance/04 Sep - The status export loses the first date.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Status export: first circulation versus latest update

Prepared by Connor Atkinson for Nicholas Hargreaves, 04 Sep 2025. Times are JST.

## Sheet: WP-P01 check

| Row | Case | Source or field | Event | Date/time | Finding |
|---|---|---|---|---|---|
| 2 | WP-P01 | Status export, latest update first | Re-circulation | 21 Aug 2025 | Picked before the first 06 when using latest-update order. |
| 3 | WP-P01 | Initial circulation record | First eligible 06 | 18 Aug 2025 09:00 | Correct start, retained after return. |
| 4 | WP-P01 | AP correction copy | Returned to AP | 20 Aug 2025 | Cost-bearer support did not match; AP corrected it. |
| 5 | WP-P01 | AP correction copy | Re-circulated | 21 Aug 2025 | Separate event, not a replacement start. |
| 6 | WP-P01 | 09 record | Approval | 22 Aug 2025 15:00 | Correct end. |
| 7 | WP-P01 | Calculation | Elapsed days | `=(DATE(2025,8,22)+TIME(15,0,0)-DATE(2025,8,18)-TIME(9,0,0))` | 4.25 days including the return. |

## Sheet: Pairing rules

| Field | Use | If unavailable |
|---|---|---|
| First eligible 06 | Start of clock, independent of later updates | Leave blank; investigate source. |
| Return | Record reason and date separately | Do not reset start. |
| Re-circulation | Record separately from first 06 | Do not use as start. |
| 09 approval | End of clock | Leave end and elapsed days blank if time is missing; never substitute a status-update time. |

Nicholas: latest-status order helps chase a file but cannot select its first eligible circulation. Keep this case as a check when the comparison is assembled.
