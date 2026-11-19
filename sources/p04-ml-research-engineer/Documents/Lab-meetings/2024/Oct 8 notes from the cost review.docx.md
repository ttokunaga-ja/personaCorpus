---
path: Documents/Lab-meetings/2024/Oct 8 notes from the cost review.docx
kind: mercer-lab-record.docx
author: Vanessa Nguyen
derived_from:
---
# October 8 cost review

Document ID: MDR-STI-20241008-LR-01-R01  
Issued: October 8, 2024  
Author: Vanessa Nguyen  
To: Michael Serrano, Brandon Romero, Emily Chen

## Meeting

October 8, 2024, 2:00 p.m. PDT. Present: Michael Serrano, Brandon Romero, Emily Chen and Vanessa Nguyen.

## Agenda

1. Compare the larger contextual model with the word-frequency linear model on the short-intake development set.
2. Agree on a comparable cost basis and decide whether the larger model belongs in automatic intake.
3. Record what to retain for research. The August confidence-abstention threshold was not a decision item for this meeting.

## Comparison and cost discussion

Both models used the same 2,800 training documents and were scored on all 934 development bodies, without an abstention rule. Macro-F1 was 0.827 for the linear model and 0.846 for the larger model. Cancel request moved in the other direction, from 0.877 to 0.832. The development result is not a score on the reserved 940-document evaluation partition.

This morning Brandon and I reconciled the timing endpoints. Emily's three runs used the same resident worker and 1,000 requests per run: the 934 development bodies followed by a repeat of the first 66. The repeat is for timing only. We use the whole path from first parse to final route output for both models. Median times are 100 seconds for linear and 210 seconds for the larger model; startup, external waiting and human review are outside the measurement.

The October 3 working slide compared 100 seconds for the linear whole path with 150 seconds for the larger model's forward pass, yielding a provisional 1.50 ratio. Brandon pointed out that those endpoints were unlike. At the 2:00 meeting we replaced that cost comparison in the issued slide with the common whole-path figures. At USD 1.20 per worker hour, the estimates per 10,000 requests are USD 0.3333 and USD 0.7000 respectively; the ratio before display rounding is 2.10. This is a resident-worker calculation, not a complete operating cost.

## Decisions and follow-up

| Owner | Decision or action |
|---|---|
| Michael | Do not adopt the larger model for automatic intake under these conditions. The development improvement does not settle the use decision given the cost and the Cancel request decline. |
| Vanessa | Keep the development comparison and the aligned cost estimate as research comparators. Issue the corrected meeting slides and record the decision for Michael and Brandon. |
| Emily | Retain the timing sheet, including the separate whole-path and forward-only measurements, so the cost basis can be checked. |

The current intake arrangement remains in place. No new August review threshold was selected here.
