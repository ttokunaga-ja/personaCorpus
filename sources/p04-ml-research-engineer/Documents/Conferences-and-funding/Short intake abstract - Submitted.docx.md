---
path: Documents/Conferences-and-funding/Short intake abstract - Submitted.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
created: 2025-02-10 15:46
modified_by: Vanessa Nguyen
derived_from:
---

# Reference Labels for Short Intake Text

Document ID: MDR-STI-20250210-RM-01-R02  
Issued: February 14, 2025  
Author: Vanessa Nguyen  
Affiliation: Mercer Document Research Inc

## Abstract

Short intake messages identify a visit, document, or problem without making the requested action explicit. A classifier can return one of its labels. Our question is whether a single intent can be supported by the received body, before considering a model prediction or the destination of the message. We describe a reference labeling procedure for twelve intake intents and a planned comparison that keeps related documents together during allocation.

The collection contains 4,800 received bodies from three intake channels. Each body is one document. Readers see the body and a matching identifier, but not the subject, channel, routing tag, outcome, or later forwarding comments. They read the whole body rather than assuming the final sentence contains the request. This restriction makes the evidence for a label inspectable and prevents information added during handling from deciding what the sender asked.

Two readers independently record a candidate and the words supporting it. Their candidates and marked phrases are then compared with the body. A disagreement goes to adjudication, but agreement alone is not sufficient: the words must establish the same single intent. If the requested action or object remains unclear, or two distinct requests have no stated priority, the disposition is Hold and the gold-label field remains empty. Candidate fields remain as a history of the reading. Hold is a reference-label disposition, not a thirteenth intent or a model confidence threshold. “Move the visit to Thursday.” states a rescheduling action, whereas “Thursday visit?” identifies a visit and day without establishing whether the sender wants to check or change it.

Of the 4,800 documents, 126 were held after rereading and adjudication; the remaining 4,674 have text-supported reference labels. These counts describe the labeling work, not a classifier's performance. Held documents remain in the accounting but are excluded from both allocation and scoring. The labeled documents are candidates for allocation, not an evaluation set in themselves.

Our planned comparison addresses a dependency between documents. A case can generate more than one received body, including messages arriving through different channels. Assigning bodies independently can place related text in more than one partition. We will use case links supported by receipt records or confirmed alias mappings to assign each linked group to one partition. Similar wording alone is not evidence of a shared case. Before scoring, we will check for shared cases across partition pairs and for support for all twelve intents in the evaluation partition.

We will compare a majority-class baseline determined from the training partition with a word-frequency linear classifier. Both will be scored against the same reference labels on the same evaluation documents, with each received body counted separately even when several belong to one case. Classifier input is the received body, not the case key or channel; feature and setting choices belong to development data. We plan to report macro-F1 across the twelve intents. The case-based comparison has not yet been scored, so this is a plan rather than a claim about relative accuracy.
