---
path: Documents/Confidence-abstention/What counts as an error caught.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# What counts as an error caught

Document ID: MDR-CA-20260605-RM-01-R01  
Issued: June 5, 2026  
Author: Vanessa Nguyen  
For: Emily Chen and the evaluation team

## Scope and routing rule

The comparison uses the same 800 reference-labelled intake documents and the saved top prediction and unrounded confidence for each. Compare each prediction with its reference label before applying either hold rule: 100 predictions are errors. The 126 bodies left without a gold label in the July 2024 annotation guide are outside these 800.

At each cutoff, hold a document only if its unrounded confidence is strictly less than the cutoff. A score equal to the cutoff is automatically routed. The 0.60 and 0.45 rules change the destination, not the saved predicted label.

## Two rates, two questions

**Hold rate** is the number of documents sent to hold divided by all 800 labelled documents. It asks how much of this set is held.

**Error capture rate** is the number of baseline errors sent to hold divided by the 100 errors in the saved predictions before either hold rule. It asks what share of those errors is intercepted. A caught error is a document whose saved prediction disagrees with its reference and whose score places it on hold; the rate does not say that a reviewer has corrected it.

| Cutoff | Held / labelled documents | Hold rate | Errors held / baseline errors | Error capture rate |
|---|---:|---:|---:|---:|
| 0.60 | 144 / 800 | 18% | 22 / 100 | 22% |
| 0.45 | 72 / 800 | 9% | 15 / 100 | 15% |

## A worked distinction

At 0.60, 22 of the 100 baseline errors are held, so error capture is 22%. The remaining 78 errors are among the 656 documents automatically routed. Thus 78/656, approximately 11.9%, answers a different question: what fraction of automatically routed documents are errors? It is not the capture rate. Neither 22% nor 11.9% is a share of all 800 documents.

Use the fixed baseline of 100 for both capture rates. A smaller held set is not by itself evidence that routing is suitable for use; the errors that move to automatic routing still need examination.
