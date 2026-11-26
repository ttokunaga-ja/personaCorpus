---
path: Documents/Confidence-abstention/Three-day replay design.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Three-day replay design

Document ID: MDR-CA-20241203-RM-02-R01  
Issued: December 3, 2024  
Author: Vanessa Nguyen  
To: Brandon Romero

## Question and common input

Can days with similar review totals leave different amounts of unfinished work at the same point in the shift? Brandon and I will each replay the lab intake list for Wednesday, December 4 through Friday, December 6, 2024. The simulated clock runs 09:00–17:00 PST on each day. There are 200 arrivals a day and 28, 32 and 30 review-required items, respectively.

Build the same four fields from the appendix before doing any review calculations: `arrival_seq`, `intake_id`, `arrived_at` (date and time in PST), and `review_required`. Keep all 600 positions, including items that pass through without review. The review flag is given. The list contains no message bodies, labels or predictions.

## Independent replay and reconciliation

Begin each day with an empty queue. Move through the fixed sequence and take review-required items in arrival order. Keep the review schedule and the assumed handling time with each run so we can compare results under the same conditions. Use *Review timing definitions* (MDR-CA-20241203-RM-01-R01) for arrival, start, completion and the outstanding count. Items outside review have an arrival only.

Before comparing any timings, Brandon and I will match all 600 positions by sequence, intake key, dated arrival and review flag. For matching inputs and review conditions, compare the start and completion on each of the 90 review keys, then compare the outstanding count at 17:00 for each day. If the timings differ, go back to the first key with a mismatch and check whether we applied the same order and clock rules there.

## Appendix — Reconstructing the arrival list

| Simulated date | Morning review items | Midday review items | Afternoon review items | Review total | Outside review |
|---|---:|---:|---:|---:|---:|
| December 4 | 8 | 10 | 10 | 28 | 172 |
| December 5 | 7 | 18 | 7 | 32 | 168 |
| December 6 | 8 | 14 | 8 | 30 | 170 |

For each day, generate the three review series separately. In each series, begin with j = 0 and stop at j = count − 1, using the corresponding window count above. Add the calculated offset in minutes to the base clock time on that date.

| Series | Base time (PST) | Offset in minutes |
|---|---|---|
| Morning review | 09:15 | 18j |
| Midday review | 12:05 | 3j + remainder of j divided by 3 |
| Afternoon review | 14:15 | 12j |
| Outside review | 09:02 | 2j + floor(j divided by 3) |

For the outside-review series, use the same index convention with the daily count of 200 minus the review total. Mark the three review series true and the outside-review series false. Combine the four series for all three dates, sort by date and then ascending clock minute, and put the true row first if a true and a false row share a minute.

Assign `arrival_seq` 1 through 600 to this combined order. Then assign `intake_id` as QD- followed by that sequence padded to three digits (QD-001 through QD-600). The date blocks occupy positions 1–200, 201–400 and 401–600. The keys belong to the merged order and stay attached to their rows when looking at any subset.
