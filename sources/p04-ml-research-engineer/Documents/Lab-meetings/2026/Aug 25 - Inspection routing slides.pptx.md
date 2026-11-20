---
path: Documents/Lab-meetings/2026/Aug 25 - Inspection routing slides.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---
# Slide 1
Layout: Cover
Title: What is still open for inspection routing
Subtitle: MDR-LDI-20260825-LM-01-R01 | Issued August 25, 2026 | Author Vanessa Nguyen
Speaker notes: For Michael, Brandon, Emily and Fatima at the month-end inspection check. Today is about what we still need to measure and who will read it; the August 11 deck was for the cost discussion.

# Slide 2
Layout: Table
Title: Start with the six that moved the wrong way
Table:
| Same 1,000 action-required records | Records |
|---|---:|
| Missed only by first-only | 14 |
| Missed only by second-read rule | 6 |
| Missed by both | 18 |
| Missed by neither | 962 |
Caption: First-only 32/1,000; second-read rule 24/1,000. Four of the six new misses lost middle evidence from the window; two retained evidence but favored an old disposition.
Speaker notes: The net reduction of eight conceals movement in both directions. Fatima and I read the six with the signed originals. A smaller reread share must be assessed on the same records, including new misses and false alarms, before discussing use.

# Slide 3
Layout: Table
Title: Cost room is a sensitivity calculation
Table:
| Reread share at 100,000 original records/month | Estimated USD/month | Basis |
|---|---:|---|
| 0% | 3,600 | Proportional increment assumption |
| 12.5% | 4,200 | Proportional increment assumption |
| 18.75% | 4,500 | Proportional increment assumption |
| 25% | 4,800 | Three measured 10,000-record runs, converted to month |
Caption: Ceiling USD 4,500/month. At 18.75%, cost and accuracy are unmeasured; the point is not an operating threshold.
Speaker notes: The first-only estimate leaves USD 900. Scaling the USD 1,200 increment from 25% gives 18.75% at the ceiling. Ask Brandon what must be remeasured with the new selection and same boundary; the August 11 comparison used 25%, not this sensitivity point.

# Slide 4
Layout: Two Columns
Title: Three conditions remain unfilled
Left: Cost remeasurement — selected share and actual host and accelerator use at the same original-record volume. Response — arrival pattern by time of day and elapsed times through the second result.
Right: Recovery — who receives a delayed or incorrect route, who takes over if absent, and the work needed to correct it. No response-time or workload tolerance has been agreed.
Speaker notes: Brandon's comments remove automatic return of the first prediction as a settled path. A handoff to a reviewer is a measurement proposal, not a delivered feature. Keep the current confirmation route while these are open.

# Slide 5
Layout: Table
Title: Capture the path and assign readers
Table:
| Event or record | What to capture next | Who reads it |
|---|---|---|
| Original record received | Receipt time and arrival time of day | Brandon, with Vanessa |
| First inference complete | Completion time linked to receipt | Brandon |
| Second inference complete | Completion time or pending status, linked to the same record | Brandon and Emily |
| Human handoff and review | Receipt by reviewer, review time, absence and correction work | Vanessa with Brandon |
| New comparison | Selection rule fixed first; paired misses and false alarms | Emily and Fatima |
Caption: These are proposed measurements, not evidence that the events are already captured.
Speaker notes: Agree on a common clock and record identity. Ask Brandon to establish the arrival and inference measurements, Emily and Fatima to inspect the new series, and discuss with Vanessa who can accept and read a handoff. Reconsideration date and acceptable burden remain unset.
