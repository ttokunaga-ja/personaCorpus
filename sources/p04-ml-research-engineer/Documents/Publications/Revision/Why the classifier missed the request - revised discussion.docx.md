---
path: Documents/Publications/Revision/Why the classifier missed the request - revised discussion.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
created: 2026-01-08 12:00
derived_from:
---

# Why the classifier missed the request

Document ID: MDR-STI-20260108-RM-01-R02  
Issued: February 6, 2026  
Author: Vanessa Nguyen  
For: Fatima Rahman — working discussion

## Scope of this reading

Nicole and I read 160 selected misclassifications from saved predictions on the 935-document development partition. We selected across reference and prediction combinations for close reading. These are not the fixed evaluation set and their counts cannot estimate the prevalence of errors there. We read each received body before revealing its reference and prediction. The reading describes distinctions supported by the text; it does not establish why the classifier made a particular prediction.

## Negation scope in 41 readings

In 41 of the selected documents our agreed reading turned on the scope of negation: 18 rejected an action, 14 negated an attribute and 9 explicitly requested an alternative action. Each document has one category. “Don't cancel; move it to Tuesday” rejects cancellation while requesting a move. “Not Friday” may reject a date while leaving the request to move intact. In both, a reader must separate the rejected part from the requested action; the nearby action word alone does not settle the intent.

These observations justify discussing scope separately. They do not show that the model attended to the same phrase or that negation caused its prediction. Testing that explanation would require a controlled comparison of inputs and predictions, which this reading did not perform.

## Other readings

The remaining 119 do not support a single scope explanation. “Has the crew been yet?” can ask whether a visit has happened rather than when it is scheduled; there is no negation word. The surrounding body is needed before assigning the reference. “Please send the inspection report” asks the recipient for a document, even without a destination. “Please correct the amount recorded for the visit” asks for a correction, whereas a question about why a charge has a given amount calls for a different reading. These examples identify places to inspect, not exhaustive categories for the 119.

## Consequence for the paper

I withdraw the January 8 proposal that a mismatch around a salient or negated action explains all 160 misses. We should present the scope readings as observations, retain the evidence for other bodies separately, and distinguish a reader's reason from a tested model mechanism. The original reference labels and frozen scores remain unchanged; this revision is not a rescoring.
