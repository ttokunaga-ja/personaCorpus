---
path: Documents/Confidence-abstention/Reviewer shift assumptions.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

## Sheet: Adjudication

A2: Reviewer shift assumptions  
B3 (Document ID): MDR-CA-20241211-AW-01-R01  
E3 (Issued): December 11, 2024  
B4 (Author): Vanessa Nguyen  
E4 (Scope): December 4–6 intake replay · For Brandon Romero

Headings at row 5, conditions at rows 6–7. B6:C7 are replay clock times in PST.

| Condition | Shift opens | Shift closes | Pause (min) | Available (min) | Theoretical slots (items) |
|---|---|---|---:|---:|---:|
| Continuous review | 09:00 | 17:00 | 0 | `=ROUND((C6-B6)*1440,0)-D6` | `=INT(E6*$D$9/$B$9)` |
| Review paused 12:00–13:00 | 09:00 | 17:00 | `=ROUND((TIME(13,0,0)-TIME(12,0,0))*1440,0)` | `=ROUND((C7-B7)*1440,0)-D7` | `=INT(E7*$D$9/$B$9)` |

A9:F9: Minutes per item | 3 | Reviewers | 1 | Service order | Arrival order (FIFO)

A10:F10 merged note: Open with an empty queue. Use the same arrivals in both runs, including during lunch. No review spans noon in this sequence. Overtime and repeat reviews are outside this calculation. Clock and event definitions: MDR-CA-20241203-RM-01-R01.

Headings at row 12, replay dates at rows 13–15, total at row 16. A13:A15 are date values.

| Replay date | All arrivals | Review required | Review work (min) | Unused slots, continuous | Unused slots, with pause |
|---|---:|---:|---:|---:|---:|
| December 4, 2024 | `=COUNT('Replay order'!A6:A205)` | `=COUNTIF('Replay order'!D6:D205,TRUE)` | `=C13*$B$9` | `=$F$6-C13` | `=$F$7-C13` |
| December 5, 2024 | `=COUNT('Replay order'!A206:A405)` | `=COUNTIF('Replay order'!D206:D405,TRUE)` | `=C14*$B$9` | `=$F$6-C14` | `=$F$7-C14` |
| December 6, 2024 | `=COUNT('Replay order'!A406:A605)` | `=COUNTIF('Replay order'!D406:D605,TRUE)` | `=C15*$B$9` | `=$F$6-C15` | `=$F$7-C15` |
| Three-day total | `=SUM(B13:B15)` | `=SUM(C13:C15)` | `=SUM(D13:D15)` | `=SUM(E13:E15)` | `=SUM(F13:F15)` |

A17:F17 merged note: Continuous review allows 480 minutes or 160 theoretical slots daily. The lunch pause leaves 420 minutes or 140 slots. The fixed 28, 32 and 30 reviews require 84, 96 and 90 minutes. Unused slots are a whole-shift measure. For the queue at 13:00, count review arrivals less completions through that minute, including any item in review in the balance.

## Sheet: Replay order

A2: Replay order  
B3 (Document ID): MDR-CA-20241211-AW-01-R01  
E3 (Issued): December 11, 2024  
B4 (Author): Vanessa Nguyen  
E4 (Scope): 600 positions reconciled with Brandon Romero on December 9

A5:D5: Arrival sequence | Intake key | Arrived at (PST) | Review required

A6:D605 hold the complete fixed order. Column A is an integer, B is the text intake key, C is a date and time with minute precision and D is a logical TRUE or FALSE. One row per arrival, including the 510 arrivals outside review.

The source is the fixed arrival appendix of MDR-CA-20241203-RM-02-R01. On each date use the counts below with four separate lists. In each list start the integer index j at zero and stop at count minus one. All offsets are in minutes.

| Replay date | Review morning | Review midday | Review afternoon | Outside review |
|---|---:|---:|---:|---:|
| December 4, 2024 | 8 | 10 | 10 | 172 |
| December 5, 2024 | 7 | 18 | 7 | 168 |
| December 6, 2024 | 8 | 14 | 8 | 170 |

- Morning review: 09:15 plus 18 × j.
- Midday review: 12:05 plus 3 × j plus the remainder when j is divided by 3.
- Afternoon review: 14:15 plus 12 × j.
- Outside review: 09:02 plus 2 × j plus the whole-number quotient when j is divided by 3.

Merge by date and ascending minute. Within a tied minute put review required first and keep both rows. Assign positions 1–600 and keys QD-001–QD-600 after merging. Use TRUE for the three review lists and FALSE for the outside-review list. Each of the three consecutive 200-row blocks must tally to its day's review count. Vanessa and Brandon matched the key, timestamp and flag at every position on December 9.

Selected rows in fixed sequence:

| Arrival sequence | Intake key | Arrived at (PST) | Review required |
|---:|---|---|---|
| 1 | QD-001 | 2024-12-04 09:02 | FALSE |
| 7 | QD-007 | 2024-12-04 09:15 | TRUE |
| 24 | QD-024 | 2024-12-04 09:51 | TRUE |
| 25 | QD-025 | 2024-12-04 09:51 | FALSE |
| 200 | QD-200 | 2024-12-04 16:03 | TRUE |
| 201 | QD-201 | 2024-12-05 09:02 | FALSE |
| 287 | QD-287 | 2024-12-05 12:05 | TRUE |
| 327 | QD-327 | 2024-12-05 12:58 | TRUE |
| 401 | QD-401 | 2024-12-06 09:02 | FALSE |
| 600 | QD-600 | 2024-12-06 15:39 | TRUE |

Print the complete sequence, keeping the three replay dates in separate page groups. December 4 covers sheet rows 6–205, December 5 rows 206–405 and December 6 rows 406–605. Repeat the sheet title, record fields and column labels (rows 1–5) on every page. Keep all four populated columns together across the width and continue each date over as many pages as needed. Include page numbers. No filter is applied to the printout.
