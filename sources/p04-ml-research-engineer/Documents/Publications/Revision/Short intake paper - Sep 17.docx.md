---
path: Documents/Publications/Revision/Short intake paper - Sep 17.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
modified_by: Vanessa Nguyen
created: 2026-04-20 16:44
derived_from:
---

# Short intake intent classification: comparison and error readings

Document ID: MDR-STI-20260420-RM-01-R03  
Issued: September 17, 2026  
Author: Vanessa Nguyen  
For: Eric Sandoval

## Abstract

Short intake messages can express a request in very few words, making distinctions between a request, a question and a rejected action consequential. We compared a word-frequency linear classifier with a majority-class baseline on a fixed evaluation set. Macro-F1 was 0.832 for the classifier and 0.021 for the baseline on the same 935 evaluation documents. Separately, we read 160 misclassified documents selected from saved predictions for the development partition to examine how a reader identifies the requested action. Forty-one of these selected development errors involved the scope of negation. This reading is a diagnostic, not a second evaluation or an estimate of the prevalence of negation errors.

## Introduction

An intake classifier must distinguish, for example, a request to move a visit from a question about its progress. Similar words can appear in bodies that ask for different actions. We compare a linear classifier with a majority-class baseline and then examine selected mistakes to describe distinctions that a performance summary cannot show.

## Methods

### Case Allocation

Cases were the allocation unit. The fixed training, development and evaluation partitions contain 2,804, 935 and 935 documents, respectively. The training–development, training–evaluation and development–evaluation pairs each shared zero cases. The pairwise checks use the cross-channel case correspondence maintained for allocation; document counts and shared case counts are different units.

### Scoring Set

One received intake body is the scoring unit. The 126 documents held as ambiguous were excluded from allocation and scoring. Both methods were scored on the same fixed 935 evaluation documents against the same twelve intent reference labels determined under the July-revised annotation guide.

### Input and Selection

The linear classifier used word frequencies from the received body only. Case keys, channel information and other administrative fields were not model features. Feature and model-setting choices were completed using the development partition before evaluation; the evaluation results were not used to change those choices.

### Comparator

The majority-class baseline predicts New work request for every evaluation document. This is the most frequent class in the training partition; the evaluation labels were not used to select it. The comparator and the linear classifier use the same evaluation documents and reference labels.

### Macro-F1

For each of the twelve intents, F1 is calculated as 2TP/(2TP+FP+FN). Macro-F1 is the equally weighted mean of the twelve unrounded class F1 values. The mean is rounded only after averaging, for both methods.

### Diagnostic Reading

For a separate diagnostic reading, we selected 160 misclassified documents from saved predictions on the development partition, so that different reference and prediction combinations could be examined. Nicole Foster and I read the received bodies and compared the phrases on which we relied with the reference and predicted labels. We marked whether negation scope mattered and discussed disagreements. A document was assigned to one scope category when applicable. These selected development errors are not the evaluation scoring set; the selection was intended for close reading, not for estimating an error rate in the fixed evaluation.

## Results

On the same 935 evaluation documents, the linear classifier obtained macro-F1 of 0.832 and the majority-class baseline 0.021. These are the results of the fixed comparison; the 160 selected development errors are not its denominator.

### Figure 1. Fixed comparison on the evaluation set

Bar chart: vertical axis “12-class macro-F1”, from 0 to 1. Horizontal axis: “Training-majority baseline” and “Word-frequency linear classifier”. Bars labelled 0.021 and 0.832 respectively. The figure states “Same 935 evaluation documents”.

Figure 1. Twelve-class macro-F1 for the training-majority baseline and the word-frequency linear classifier on the same 935 evaluation documents. Values are rounded after averaging class F1 scores.

### Selected Errors

After discussion, 41 of the 160 selected errors involved the scope of negation: 18 rejected an action, 14 negated an attribute, and 9 explicitly requested an alternative action. The remaining 119 were not assigned to that scope group. Among the 41, our initial flags agreed on 29 and differed on 12; among the 119 they agreed on 102 and differed on 17. These are agreements between readers about a scope flag, not classifier accuracy.

## Discussion

### Scope Examples

The scope examples call for separating what is rejected from what is requested. “Don't cancel; move it to Tuesday” rejects cancellation while asking for a change of date. “Not Friday” can reject a date without rejecting a move. In the 41 selected cases, the categories describe what the readers observed in the wording: rejecting an action, negating an attribute, or requesting an alternative action. The reading alone does not establish which input feature caused a model prediction.

The other 119 cannot be explained by the same scope account. A question about whether a crew has already visited can require a progress reading rather than a schedule reading without a negation. A request for a document needs the direction of the requested action; a request to correct a recorded amount needs to be distinguished from a question about a charge. These examples motivate closer examination of the relevant phrases, not a claim that the three examples exhaust the remaining errors.

The earlier document-split score of 0.851 was withdrawn after related cases were found across partitions. It is part of the allocation history, not a paired estimate of the difference between methods on the present evaluation set. The current comparison does not establish suitability for deployment; a separate use assessment is needed.

## Records Consulted

Case split reevaluation, April 17, 2025; January 12 reading decisions; revised discussion, February 6, 2026; comparison conditions for the editor copy, April 27, 2026.
