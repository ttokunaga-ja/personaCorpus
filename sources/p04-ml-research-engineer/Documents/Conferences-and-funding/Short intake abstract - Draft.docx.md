---
path: Documents/Conferences-and-funding/Short intake abstract - Draft.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# Reference Labels for Short Intake Text

Document ID: MDR-STI-20250210-RM-01-R01  
Issued: February 10, 2025  
Author: Vanessa Nguyen  
Affiliation: Mercer Document Research Inc

## Abstract

Short intake messages often identify a visit, document, or problem without making the requested action explicit. A classifier can nevertheless return one of its labels. Our question is whether a single intent can be supported by the received body itself, before considering a model prediction or the destination of the message. We describe a reference labeling procedure for twelve intake intents and a planned comparison that keeps related documents together during allocation.

The collection contains 4,800 received bodies from three intake channels. Each body is one document. Readers see the body and a matching identifier, but not the subject, channel, routing tag, outcome, or later forwarding comments. They read the whole body rather than assuming the final sentence contains the request. The restriction makes the evidence for a label inspectable and prevents information added during handling from deciding what the sender asked.

Two readers independently record a candidate and the words supporting it. Their candidates and marked phrases are then compared with the body. A disagreement goes to adjudication, but agreement is not sufficient on its own: the words must establish the same single intent. If the requested action or object remains unclear, or two distinct requests have no stated priority, the disposition is Hold. The gold-label field remains empty rather than recording a guessed category.[^hold] Candidate fields remain as a history of the reading. The paired schedule examples below distinguish a stated change from an unresolved question.

Of the 4,800 documents, 126 were held after the rereading and adjudication; the remaining 4,674 have text-supported reference labels. These counts describe the labeling work, not a classifier's performance. Held documents are retained in the accounting but excluded from the population to be allocated and scored. The labeled documents are candidates for a later evaluation partition, not an evaluation set in themselves.

Our planned comparison addresses a separate dependency between documents. An individual case can generate more than one received body, including messages arriving through different channels. Assigning bodies independently can place related text in more than one partition. We will use case links supported by receipt records or confirmed alias mappings to assign each linked group to one partition. Similar wording alone is not evidence that two messages belong to one case. Before scoring, we will check for shared cases across partition pairs and for support for all twelve intents in the evaluation partition.

We will compare a majority-class baseline determined from the training partition with a word-frequency linear classifier. Both will be scored on the same labeled evaluation bodies, with each received body counted separately. Classifier input is the received body, not the case key or channel; choices of features and settings belong to development data. We plan to report macro-F1 across the twelve intents. The comparison under the case-based allocation has not yet been scored, so the procedure here is a plan rather than a claim about relative accuracy.

The reference process may also help teams decide where human attention is most useful. A future workflow could place documents with unclear actions ahead of routine requests for confirmation, while preserving the message and the evidence used to make each decision. Across intake channels, a common language for uncertainty could make review practices easier to discuss and reduce the temptation to infer an answer from routing information. These possibilities would require study with the people responsible for intake, including measurement of review time, the consequences of delay, and differences among channels. We do not propose replacing existing routing decisions on the basis of the present labeling work. A practical deployment would need ownership of unresolved requests, an audit trail, and a way to return a request to its originator when the intended action cannot be established from the text alone.

## Schedule examples

| Labelable | Hold |
|---|---|
| “Move the visit to Thursday.” — Reschedule | “Thursday visit?” — action unspecified |

[^hold]: Hold is a disposition used when the received body does not establish one intent. It is not a thirteenth intent category or a model confidence threshold.
