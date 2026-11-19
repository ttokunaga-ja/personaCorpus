---
path: Documents/Publications/Revision/Short intake paper - first draft.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# Short intake intent classification: comparison and error readings

Document ID: MDR-STI-20260420-RM-01-R01  
Issued: April 20, 2026  
Author: Vanessa Nguyen  
For: Research team review before submission to Eric Sandoval

## Abstract

Short intake messages can express a request in very few words, making distinctions between a request, a question and a rejected action consequential. We compared a word-frequency linear classifier with a majority-class baseline on a fixed evaluation set. Macro-F1 was 0.832 for the classifier and 0.021 for the baseline on the same 935 documents. We also read a selected set of development errors to examine how a reader identifies the requested action. Forty-one of 160 selected errors involved the scope of negation. This reading is a diagnostic, not a second evaluation or an estimate of the prevalence of negation errors.

## Introduction

An intake classifier must distinguish, for example, a request to move a visit from a question about its progress. Similar words can appear in bodies that ask for different actions. We compare a linear classifier with a majority-class baseline and then examine selected mistakes to describe distinctions that a performance summary cannot show.

## Methods

We assigned intake documents by case and classified the received text into twelve intents. The fixed comparison used 935 evaluation documents. We report macro-F1 for the linear classifier and a majority-class baseline.

For the separate reading, we selected 160 misclassified documents from saved development predictions so that different reference and prediction combinations could be examined. Nicole Foster and I read the received bodies and compared the phrases on which we relied with the reference and predicted labels. We marked whether negation scope mattered and discussed disagreements. A document was assigned to one scope category when applicable. This selection was intended for close reading, not for estimating an error rate in the fixed evaluation.

## Results

On the same 935 evaluation documents, the linear classifier obtained macro-F1 of 0.832 and the majority-class baseline 0.021. These are the results of the fixed comparison; the 160 selected development errors are not its denominator.

### Selected errors

After discussion, 41 of the 160 selected errors involved the scope of negation: 18 rejected an action, 14 negated an attribute, and 9 explicitly requested an alternative action. The remaining 119 were not assigned to that scope group. Among the 41, our initial flags agreed on 29 and differed on 12; among the 119 they agreed on 102 and differed on 17. These are agreements between readers about a scope flag, not classifier accuracy.

## Discussion

The scope examples call for separating what is rejected from what is requested. “Don't cancel; move it to Tuesday” rejects cancellation while asking for a change of date. “Not Friday” can reject a date without rejecting a move. The words around a negation matter to the reading, but the reading alone does not establish which input feature caused a model prediction.

The other 119 cannot be explained by the same scope account. A question about whether a crew has already visited can require a progress reading rather than a schedule reading without a negation. A request for a document needs the direction of the requested action; a request to correct a recorded amount needs to be distinguished from a question about a charge. These examples motivate closer examination of the relevant phrases, not a claim that the three examples exhaust the remaining errors.

The earlier document-split score of 0.851 was withdrawn after related cases were found across partitions. It is part of the allocation history, not a paired estimate of the difference between methods on the present evaluation set. The current comparison does not establish suitability for deployment; a separate use assessment is needed.

## Records consulted

Case split reevaluation, April 17, 2025; January 12 reading decisions; revised discussion, February 6, 2026.
