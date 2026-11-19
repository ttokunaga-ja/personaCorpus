---
path: Documents/Lab-meetings/2025/May 23 can we compare yet.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
modified_by: Vanessa Nguyen
created: 2025-05-23 14:00:00 -0700
derived_from:
---

## Slide 1

Layout: Cover

Title: Can we compare yet?

Document ID: MDR-STI-20250523-LM-01-R01 | Issued: May 23, 2025 | Author: Vanessa Nguyen

Speaker notes: Weekly research meeting, 2:00 p.m. Joshua Ramirez and I are reviewing origin evidence for the next comparison's additional candidates.

## Slide 2

Layout: Heading and Body

Title: What this check can tell us

- 96 additional documents were checked against receipt records and confirmed aliases.
- These are separate from the 935 documents in the April 3 fixed evaluation.
- An origin check establishes evidence for a candidate; it does not establish partition independence or class support.
- April's 0.832 is not recalculated by this review.

Speaker notes: Keep the April result and today's candidate queue separate. We are not measuring model errors or F1 in the ledger check.

## Slide 3

Layout: Figure

Title: Origin evidence by channel

Image: Pictures/Charts/May ledger evidence by channel.png

Image content: Stacked counts of direct confirmation / confirmed alias / origin unknown and excluded: Web form 33 / 6 / 1 (40); Shared inbox 19 / 11 / 2 (32); Service desk relay 16 / 7 / 1 (24). Vertical axis is documents checked; legend identifies the three evidence dispositions. Total 96.

Caption: 68 direct + 24 confirmed alias = 92 candidates; 4 unresolved origins excluded.

Speaker notes: The small excluded segments are labeled 1, 2 and 1. The figure summarizes evidence disposition, not model performance.

## Slide 4

Layout: Two Columns

Title: Two reasons to exclude

Left: Receipt reference missing

WF-040 and SI-031 have no receipt reference in the ledger. The channel alone does not establish where either document came from. Joshua will trace each original receipt.

Right: Alias correspondence unsupported

SI-032 and SD-024 have an alias whose correspondence cannot be verified. Joshua will check the alias table and original record; a similar label is not sufficient evidence of a link.

Speaker notes: Both categories remain excluded. A receipt reference and a verified alias-to-receipt link are different kinds of evidence; neither should be inferred from a similar subject.

## Slide 5

Layout: Table

Title: Disposition and next step

| Evidence disposition | Documents | Next step |
|---|---:|---|
| Direct receipt confirmed | 68 | Retain as candidates |
| Confirmed alias linked to receipt | 24 | Retain as candidates |
| Origin unresolved | 4 | Exclude from next comparison; Joshua checks records |
| Total checked | 96 | 92 candidates, 4 excluded |

Table note: The 92 are not yet a fixed evaluation set.

Speaker notes: Joshua's source check is MDR-STI-20250523-AW-01-R01. Evidence is sufficient to keep 92 in the candidate queue, not to claim the new comparison is ready.

## Slide 6

Layout: Heading and Body

Title: Decision: hold the model ranking

- Exclude the four unresolved documents from the next comparison while Joshua follows up on the missing receipt references and alias evidence.
- Keep the 92 supported documents as candidates. Check case independence across partitions and support for all twelve classes before fixing evaluation conditions.
- Fatima will wait for those conditions to be fixed before scoring. Score both models on the same fixed evaluation documents and reference labels before judging their relative performance.
- Keep the April fixed evaluation and this additional-candidate review as separate records.

Speaker notes: Decision added after the meeting. The 92 have passed an origin check only; neither their allocation nor the outcome of a model comparison has been established.
