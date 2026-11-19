---
path: Documents/短文受付意図分類/Research-questions/Short intake intent research plan.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---
# Short intake intent research plan

Document ID: MDR-STI-20240416-RP-01-R01  
Issued: April 16, 2024  
Author: Vanessa Nguyen  
For: Michael Serrano and the research team

## Question and scope

Can we identify the sender's intended action from a short received intake message body? On April 15, Michael and I agreed to study the following twelve intents as one classification problem, with a first-year target of 0.82 macro-F1. This is a target for the research, not a measured result or an approval to use a classifier.

| Intent | Distinction to preserve |
|---|---|
| New work request | Ask for work to be done |
| Quote request | Ask for a price or estimate |
| Schedule check | Ask about an appointment's timing |
| Reschedule | Ask to move an appointment |
| Cancel request | Ask to cancel an arrangement |
| Job status | Ask about progress on work |
| Send document | Provide a document |
| Request document | Ask for a document |
| Correct record | Ask for a recorded detail to be corrected |
| Access issue | Report difficulty gaining access |
| Report defect | Report a defect |
| Invoice question | Ask about an invoice |

A question about the time of a visit and a request to move it may share the same nouns but call for different actions. We should assess the twelve classes together and inspect where the distinctions fail, rather than report twelve separate tasks.

## Reference labels

The unit is one received message body. The study asks what can be supported by the sender's words, not what a routing destination suggests. Readers should work independently, record a proposed intent and the words supporting it, then compare their readings. Differences go to adjudication against the body. A reference label enters a scored comparison only when its meaning and supporting evidence have been confirmed through reading and adjudication; agreement in a label alone is not a substitute for checking the evidence. If the text does not support a single intent, retain the case for examination without inventing a label for scoring.

Fatima will develop the reading and adjudication procedure. We should document what is included in the received body and what context is withheld before readers begin. The eventual score will describe the labeled population we can establish, not necessarily every message received.

## Partitions and comparison

We plan separate training, development and evaluation partitions at the document level. The training partition will fit the classifiers; development will be used for choices such as feature and model settings; evaluation will be reserved for the final comparison. The actual allocation and counts are not yet fixed. Before freezing the evaluation partition, check that every one of the twelve reference classes has at least one supported document and that labels have evidence. If any class has no reference support, revisit the proposed partition before fixing it rather than supplying an arbitrary class score.

Once fixed, record the evaluation document identifiers, reference labels and applicable reading rules in the partition record. Keep that same set and the same labels for both comparisons. Do not use evaluation outcomes to choose features or settings; if the evaluation set or labels must change, record the reason and treat the resulting comparison as a new evaluation condition.

The first comparator predicts the most frequent class in the training partition for every evaluation document. The second is a linear classifier using word frequencies from the message body, fitted on the training partition with choices made on development data. Both receive the same evaluation documents and are scored against the same reference labels. Report each class's reference support and F1 as well as the twelve-class macro-F1, so the majority-class result cannot conceal performance on the other intents.

## Comparison and measure

For each intent, count true positives (TP), false positives (FP) and false negatives (FN) by treating that intent as positive and the other eleven as negative. Precision is TP divided by TP + FP; recall is TP divided by TP + FN. When there are no predictions for an intent, TP + FP is zero: set precision to zero for this report. For an intent with positive reference support, calculate F1 directly as 2TP / (2TP + FP + FN); equivalently, it is the harmonic mean of precision and recall when their sum is positive, and zero when both are zero. Positive reference support ensures this F1 denominator is nonzero, including for the eleven intents never predicted by the majority-class comparator.

Macro-F1 is the sum of the twelve per-intent F1 values divided by twelve, with equal weight per intent. Reference support means TP + FN, not the number of predictions. A class with zero reference support in a proposed evaluation partition triggers the partition review above; the zero-prediction convention does not replace that requirement. Small supports can make individual scores sensitive to a few documents, so include the counts when interpreting the average.

## Use is a separate question

The 0.82 target tells us what to aim for in the first-year comparison; reaching it would not by itself establish suitability for an operational workflow. Brandon Romero will separately examine conditions for use, including costs, constraints and how cases can return to human handling. The research report should give him the population, per-intent results and observed errors needed for that assessment, without treating the research target as a use decision.
