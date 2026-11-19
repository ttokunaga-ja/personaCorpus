---
path: Documents/Confidence-abstention/Before using a smaller review queue.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Before using a smaller review queue

Document ID: MDR-CA-20260619-RM-01-R01  
Issued: June 19, 2026  
Author: Vanessa Nguyen  
For: Michael Serrano

## Recommendation

Keep adoption for use on hold. I propose a research comparison on independent labelled documents and a separate measurement of recovery from a wrong route. Neither has been carried out. The 0.45 condition reduced holds by 72 of the June 800 but released seven additional incorrect predictions as well as 65 correct ones; it is not yet evidence that the operational trade is acceptable.

## Fix the comparison before scoring

With Emily's reconciliation conditions, specify the existing cutoff and comparison cutoff before examining the next set's outcomes. Preserve one document ID, reference label, saved top prediction and unrounded confidence per record. Hold only scores strictly below the fixed cutoff; equality goes to automatic assignment. Use the same predictions and labels under both rules.

Report hold rate as held documents divided by all scored, labelled documents. Report error capture as held errors divided by all prediction errors counted before either hold rule. Also cross-tabulate decisions and prediction correctness so that newly released errors are visible. These denominators answer different questions; do not use the number of held items as the error-capture denominator.

Select a separate labelled set and use evidence-backed case keys to prevent documents from the same case appearing in both the material used to choose conditions and the new evaluation. Resolve uncertain case links before treating the set as independent. Read newly released errors against their request evidence; the June examples suggest useful questions but should not be used to tune a cutoff on the same 800 and then claim an independent result.

## Recovery measurement

Brandon's proposed sequence distinguishes detection, confirmation of the original route, return to hold, receipt by the correct owner, requester correction and completion. Measure detection to receipt as its own interval, using recorded timestamps linked to the same request; retain the later communication and completion points separately. The method for obtaining the correct owner's receipt timestamp remains unresolved. Agree the responsible roles, evidence and the amount of extra work that would be acceptable before using the interval for a decision.

## Decision requested

Please consider this as the next research comparison only. We still need an agreed way to obtain receipt times and an agreed acceptable workload. Until both the independent comparison and the recovery evidence are available for review, leave the routing configuration and adoption decision unchanged.
