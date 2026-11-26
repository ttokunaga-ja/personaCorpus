---
path: Documents/Confidence-abstention/Review timing definitions.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Review timing definitions

Document ID: MDR-CA-20241203-RM-01-R01  
Issued: December 3, 2024  
Author: Vanessa Nguyen  
To: Brandon Romero and Emily Chen

## What to record

Use the simulated local date and 24-hour time in PST for each observation. The December replay contains 600 intake items, of which 90 are marked for review. Keep the intake key with each observation so that we can match times for the same item across runs.

| Observation | Record it for | Clock event |
|---|---|---|
| Arrival | All 600 items | The item enters intake at the minute on the fixed arrival list. |
| Review start | Review-required items | The reviewer begins checking the item. |
| Review completion | Review-required items | The reviewer finishes checking the item. |

For an item outside review, leave both review timestamps and the two review durations blank. For an item sent to review, waiting time is start minus arrival, in minutes. Time to completion is completion minus arrival, also in minutes. An immediate start gives a wait of zero.

## Outstanding review

At a clock reading, count review-required items whose arrival is at or before that time and whose completion is still in the future. An item being checked remains outstanding until it is complete. For a minute containing both a completion and an arrival, remove the completed item first, add the newly arrived item, and report the resulting count. Starting an item does not reduce that count.

## Clock example

| Arrival (PST) | Start (PST) | Completion (PST) | Wait (min) | Time to completion (min) |
|---|---|---|---:|---:|
| 12:05 | 13:00 | 13:03 | 55 | 58 |

The item counts as outstanding from 12:05 up to, but excluding, 13:03. At 13:00 it moves from waiting to in review. The outstanding total changes for this item only upon arrival and completion.
