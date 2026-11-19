---
path: Documents/Clients/Whitfield/Diagnosis/19 Jun - Same invoice on two clocks.pptx
kind: holt-diagnosis.pptx
author: Connor Atkinson
derived_from:
---

## Slide 1 — Cover

**The same invoice on two clocks**

Analysis review for Nicholas Hargreaves and Neha Patel | 19 Jun 2024 | Connor Atkinson  
HRA-WHI-DIAG-BRF-2024-003 | v0.1

Speaker notes: Use this case to check the boundaries before aggregating; it is not a representative average.

## Slide 2 — Process Figure

**WB-0417: entry and approval wait coexist**

Horizontal actual-time sequence, JST (+09:00): 14 May 10:10 entry starts → 10:28 entry ends → 10:35 complete item routed to cost-centre approver → 15 May 11:05 review starts. Mark entry 18 minutes, entry end to routing 7 minutes, and routing to review 24 hours 30 minutes. The long overnight interval is interrupted by an axis break, labelled as such.

Caption: The seven minutes before routing are not part of the 07 approval wait.

Speaker notes: 10:35 to midnight is 13 hours 25 minutes; midnight to 11:05 is 11 hours 5 minutes. Together they are 24 hours 30 minutes.

## Slide 3 — Two Columns

**Two different intervals, one case**

Left — Entry work: 10:10–10:28, 18 minutes. This is an activity interval at 03.

Right — Approval wait: 10:35–11:05 next day, 24 hours 30 minutes. This is the interval before review at 07 after the complete item is routed.

Speaker notes: The two intervals answer different questions. Seven minutes separates them; do not assign it to the queue simply because entry has finished.

## Slide 4 — Headline and Body

**Missing start means the interval cannot be determined**

If a review-start event is absent, leave the approval wait undetermined; neither zero nor an approval-completion timestamp is a substitute. WB-0439 illustrates a missing start. Retain the case for checking, but do not include its unknown interval in a complete-pair total.

One case shows the definitions working. Its ratio of waiting to entry cannot be extrapolated to the selected payments as a whole.

Speaker notes: The cleared extract contains three cases for definition checks; the wider record set requires its own pair assessment.
