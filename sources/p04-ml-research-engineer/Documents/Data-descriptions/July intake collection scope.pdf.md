---
path: Documents/Data-descriptions/July intake collection scope.pdf
kind: tex:mercer-research-brief.cls
author: Joshua Ramirez
derived_from:
---

# Which intake bodies are in the July set

Document ID: MDR-STI-20260706-DD-01-R01  
Issued: July 6, 2026  
Author: Joshua Ramirez  
For: Vanessa Nguyen and Fatima Rahman

## Intake boundary

I matched the 6,000 reference-labelled bodies in the July M-14 collection to their receipt records. Each document is one received body from Web form, Shared inbox or Service desk relay. Keep the sender’s wording intact, even when it is only one line. The July inventory is new. It carries forward the July 2024 revised annotation guide (MDR-STI-20240513-GD-01-R02) and uses fresh membership throughout. Bodies from the earlier partitions, the May 2025 candidates and the June confidence-hold comparison stay outside this collection.

The counts by partition are 3,600 training, 1,200 development and 1,200 evaluation documents. These are counts of bodies. A cross-channel case key groups related receipts for allocation, supported by the receipt records and confirmed aliases. Keep the case together in one partition even if the channel changes. The document ID is the handle for matching a prediction back to its reference after scoring.

## Where the received text ends

Here is how to handle a relay receipt with a short staff addendum. The receipt history keeps the addendum for reconciliation. The scoring input ends at the last word the sender supplied.

| View | Entry | What to carry forward |
|---|---|---|
| Receipt reference | Subject “Thursday visit”; received body “Can you move the visit to Monday?”; staff addendum “Check the time with dispatch first.” The receipt also carries the channel and case key. | Keep the full record in the intake inventory so the body boundary and case assignment can be traced. |
| Classifier input | “Can you move the visit to Monday?” | Pass the received body only. The subject, addendum, channel and case key stay with the receipt record. |

The reference label for the example follows “move the visit to Monday” (Reschedule). The staff instruction about checking the time would point a reader toward a different action if it were appended to the body. Keep the document ID for the scoring join and the reference label on Fatima’s side of that join.

## Handoff

Use the channel coverage table to reconcile the received-document counts against the three partition totals. A route count establishes how many bodies entered each partition through that route. Checking whether related bodies remained together requires the case keys from the intake records.
