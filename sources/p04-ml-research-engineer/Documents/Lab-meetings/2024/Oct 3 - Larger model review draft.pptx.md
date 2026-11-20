---
path: Documents/Lab-meetings/2024/Oct 3 - Larger model review draft.pptx
kind: mercer-lab-meeting.pptx
author: Vanessa Nguyen
derived_from:
---

## Slide 1
Layout: Cover
Title: Can the bigger model earn its place?
Subtitle: Working discussion with Brandon Romero and Michael Serrano
Record: MDR-STI-20241003-LM-01-R01 | Issued October 3, 2024 | Author Vanessa Nguyen
Speaker notes: This is a working comparison. The cost endpoint still needs checking.

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
Speaker notes: The aggregate gain does not describe every class. The Cancel request result merits a closer reading.

## Slide 4
Layout: Table
Title: Cost comparison — provisional

| Measure | Linear | Larger |
|---|---:|---:|
| Requests per run | 1,000 | 1,000 |
| Median seconds used | 100 full path | 150 forward only |
| Working ratio | 1.00 | 1.50 |

Table note: Timing scope awaiting confirmation. These endpoints are not yet matched.
Speaker notes: The larger model also has a full-path timing. Ask Brandon which endpoint is appropriate before carrying this ratio into a decision.

## Slide 5
Layout: Heading and Body
Title: Can we decide use from this number?
Body:
- Does the cost comparison cover the same work for both models?
- How should the Cancel request loss be weighed against the aggregate improvement?
- What evidence would be needed before changing intake use?
Speaker notes: Leave the application decision open until the timing scope is agreed.
