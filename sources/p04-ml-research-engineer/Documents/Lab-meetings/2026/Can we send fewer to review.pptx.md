---
path: Documents/Lab-meetings/2026/Can we send fewer to review.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1 — Cover

**Can we send fewer to review?**

June 11, 2026 · 2:00 p.m. PDT  
Vanessa Nguyen · Michael Serrano · Emily Chen · Brandon Romero

Document ID: MDR-CA-20260611-LM-01-R01 · Issued: June 11, 2026 · Author: Vanessa Nguyen

Speaker notes: We have a smaller review queue in the comparison, but we need to decide what the extra wrong routes mean before considering a change in use.

## Slide 2 — Heading and Body

**The comparison holds the predictions fixed**

- 800 labelled intake documents: 360 Web form, 264 Shared inbox, 176 Service desk relay.
- The same saved top prediction, unrounded confidence and reference label are used at both cutoffs. There are 100 prediction errors before either hold rule.
- Hold when confidence is strictly below the cutoff; a score equal to the cutoff goes to automatic routing. Removing hold releases the saved prediction, not a new label.
- At 0.450, four documents move from hold to automatic; at 0.600, four are automatic under both rules.

Speaker notes: Emily and I checked the boundary and the common error denominator on June 8. The 126 July 2024 holds without reference labels are not in these 800.

## Slide 3 — Figure

**Fewer holds, fewer errors caught**

Figure: use the complete image `Pictures/Charts/Same cutoffs two denominators.png`.

Caption: At 0.60, 144/800 documents are held and 22/100 baseline errors are caught. At 0.45, the corresponding counts are 72/800 and 15/100. The two panels use different denominators; baseline errors are counted before either hold rule.

Speaker notes: The 22% is a share of the 100 errors, not a share of all 800 documents. Lowering the cutoff releases seven errors that the higher cutoff would have held.

## Slide 4 — Table

**Where the decisions moved**

| At 0.60 | At 0.45 | Documents | Errors | Correct predictions |
|---|---|---:|---:|---:|
| Hold | Hold | 72 | 15 | 57 |
| Hold | Automatic | 72 | 7 | 65 |
| Automatic | Automatic | 656 | 78 | 578 |
| Automatic | Hold | 0 | 0 | 0 |

Table note: Errors compare the same saved prediction with the reference label before holding. Automatic errors rise from 78 to 85.

Speaker notes: The 72 released documents account for the whole reduction in holds. Sixty-five are correct automatic decisions; seven are additional wrong routes. Neither count measures work time.

## Slide 5 — Two Columns

**Two different wrong routes**

**CA-281 · Shared inbox**

“Thursday instead” requests a change; the later Monday confirmation is background. Reference: Reschedule. Saved prediction: Schedule check. A change could wait in a confirmation queue while the old date is confirmed.

**CA-614 · Service desk relay**

After an earlier cancellation, “Has the pump repair been signed off?” asks for current status. Reference: Job status. Saved prediction: Cancel request. A status inquiry could reach the cancellation queue and need rerouting.

Speaker notes: These are plausible operational consequences, not observed service incidents. Both cases separate the action requested now from other words in the body, but the work needed to recover each route can differ.

## Slide 6 — Heading and Body

**What the reading can and cannot tell us**

- On June 8, Emily and I read all seven released errors and three correct controls, selecting the requested action before opening predictions and references.
- The seven comprise three changes routed as checks, two status inquiries routed as cancellations, and two price requests routed as new work. The three controls are contrasts, not a representative sample.
- The reading explains these cases; it does not establish a general cause or a field error rate. The research records do not measure detection, handoff or recovery time.
- We still need an independent evaluation and an agreed way to measure recovery and acceptable operational burden before choosing a use.

Speaker notes: In CA-390, “what a replacement seal would cost” asks for a quote, not work intake. We should not tune the cutoff to these seven examples and then treat the same reading as an independent evaluation.

## Slide 7 — Heading and Body

**Proposal for today**

- Keep the current research conditions and both cutoffs available for comparison.
- Hold the adoption decision; do not change the current routing setting on the strength of this comparison.
- Prepare a separate evaluation on independent labelled documents, with the cutoff and measures fixed in advance.
- Define the recovery handoffs and measurement points with Brandon before estimating the operational trade-off.

Speaker notes: Brandon, who would detect a wrong route, confirm it and return it to hold? Who receives it at the correct destination, contacts the requester if needed, and confirms recovery? Can we measure from recorded detection to receipt by the correct owner, and which records provide those timestamps? Which later correction and completion points should be recorded separately? Michael, does holding adoption while we establish those conditions make sense?
