---
path: Documents/Confidence-abstention/June cutoff comparison - revised.docx
kind: mercer-research-note.docx
author: Vanessa Nguyen
derived_from:
---

# June cutoff comparison — conditions checked

Document ID: MDR-CA-20260603-RM-01-R02  
Issued: June 9, 2026  
Author: Vanessa Nguyen  
For: Michael Serrano and Emily Chen

## Purpose and conditions

Michael and Emily, this revision resolves Emily's two questions on the June 3 working copy: the boundary rule and the base for error capture. On June 8 Emily and I checked the same 800 reference-labelled intake documents under both conditions. For each document we held the reference label, saved top prediction and unrounded confidence fixed before applying either rule. A change in routing does not change the saved prediction. The 126 documents held without a reference label in July 2024 are outside these 800.

A document goes to hold when its confidence is strictly below the cutoff; a score equal to the cutoff goes to automatic routing. At 0.450 exactly, four documents are held at 0.60 and automatically routed at 0.45. At 0.600 exactly, four documents are automatically routed under both rules. These boundary counts use the unrounded scores.

## Comparable results

The hold rate divides held documents by all 800 documents. The error capture rate divides errors sent to hold by the same 100 saved predictions that disagreed with their reference labels before either hold rule was applied. Errors among automatically routed documents are a different group and are not the denominator for capture.

| Cutoff | Hold rate | Error capture rate | Automatic routing | Errors remaining in automatic routing |
|---|---:|---:|---:|---:|
| 0.60 | 144/800 (18%) | 22/100 (22%) | 656 | 78 |
| 0.45 | 72/800 (9%) | 15/100 (15%) | 728 | 85 |

Lowering the cutoff releases 72 documents from hold: 65 had correct saved predictions and seven had errors. Another 72 remain on hold, including 15 errors; the 656 already automatic remain automatic, including 78 errors. Thus the seven released errors account for the fall from 22 to 15 captured errors and the rise from 78 to 85 errors in automatic routing. The 22% is a share of the 100 baseline errors, not of the 800 documents.

## What the released errors ask for

At our June 8 reading Emily and I selected the requested action from the body before opening the saved prediction and reference. We read all seven released errors and one correct control from each intake route. Three errors confused a request to reschedule with a schedule check; two confused a current job-status inquiry with a past cancellation; two treated a request for a price as a new work request.

In CA-281, “Thursday instead” requests a change; the Monday confirmation is background, while the saved label would send it as a schedule check. In CA-614, “Has the pump repair been signed off?” asks for current status after a description of an earlier cancellation. In CA-390, “what a replacement seal would cost” asks for a quote, not an order for work. In these bodies the requested action and words describing background point to different actions. This reading explains the seven observed mismatches; it does not establish that the same pattern causes errors generally.

## Implication for use

The smaller hold queue would leave 72 fewer documents for review in this comparison, but seven more incorrect saved labels would be routed automatically. A change request could go to a confirmation queue, a status inquiry to a cancellation queue, or a price inquiry to work intake. Those misroutes may need discovery and correction before the request reaches the right person. We have not measured the time or burden of that recovery in use, and these ten readings do not establish a field error rate.

I recommend holding the decision on adoption for use. The comparison is useful for research, but the smaller queue alone does not show that the additional misrouting burden is acceptable. We should discuss recovery and check the behavior on independent documents before deciding whether to use the lower cutoff.
