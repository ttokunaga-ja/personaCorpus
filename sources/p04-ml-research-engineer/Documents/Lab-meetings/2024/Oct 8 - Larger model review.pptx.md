---
path: Documents/Lab-meetings/2024/Oct 8 - Larger model review.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1
Layout: Cover
Title: Can the bigger model earn its place?
Subtitle: Short intake intent classification — cost review
Record: MDR-STI-20241003-LM-01-R02 | Issued October 8, 2024 | Author Vanessa Nguyen
Speaker notes: Revised after the 2:00 discussion with Michael, Brandon and Emily. The decision in this deck concerns the larger contextual model.

## Slide 2
Layout: Heading and Body
Title: One development set, two models
Body:
- Both models trained on 2,800 bodies; input is the received body only.
- Score one top label for each of 934 development documents against the July twelve-class reference.
- The 940-document evaluation partition remains closed. The development set includes the August replay's 800 bodies.
Speaker notes: No confidence routing is applied here. This is a development comparison, not an independent replication of August's queue exercise.

## Slide 3
Layout: Table
Title: Higher mean, one class moving the other way

| Measure | Linear word-frequency | Larger contextual |
|---|---:|---:|
| Macro-F1, 12 classes | 0.827 | 0.846 |
| Cancel request F1 | 0.877 | 0.832 |
| Cancel request support | 57 | 57 |

Table note: 934 development documents; macro-F1 is the equal-weight mean of unrounded class F1 values, rounded at the end.
Speaker notes: The aggregate gain does not describe every class. Cancel request false negatives rise from 7 to 10; we have not yet established why.

## Slide 4
Layout: Table
Title: Cost comparison — same full path

| Measure | Linear | Larger |
|---|---:|---:|
| Requests per timed run | 1,000 | 1,000 |
| Median full-path seconds, three runs | 100 | 210 |
| USD per 10,000 requests | 0.3333 | 0.7000 |
| Cost relative to linear | 1.00 | 2.10 |

Table note: Same resident worker at USD 1.20/hour. Full path is first parse through final destination output. Costs shown to four decimals; ratio uses unrounded costs.
Speaker notes: Brandon pointed out that the working 1.50 paired the linear full path with the larger model's forward-only median of 150 seconds. Both endpoints now cover the full path. Each run comprises the 934 development bodies plus the first 66 repeated for timing only. Startup, external waiting and human review are excluded.

## Slide 5
Layout: Heading and Body
Title: Decision on the larger model
Body:
- Do not adopt the larger contextual model for automatic intake under these conditions; keep the current intake arrangement.
- Retain its development results and aligned cost calculation as a research comparator.
- Before revisiting use, prepare a lighter alternative at comparable cost and examine the cancellation errors.
Speaker notes: Michael made the application decision at the October 8 review. Vanessa will retain the comparison and cost calculation; Emily will retain the timing record. The August routing threshold was not decided here.
