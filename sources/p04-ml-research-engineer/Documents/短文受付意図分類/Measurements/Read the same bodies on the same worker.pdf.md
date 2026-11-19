---
path: Documents/短文受付意図分類/Measurements/Read the same bodies on the same worker.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Read the same bodies on the same worker

Document ID: MDR-STI-20240925-RM-01-R01  
Issued: September 25, 2024  
Author: Vanessa Nguyen  
To: Brandon Romero and Emily Chen

## Timing question

For a comparable resident-worker cost, measure the complete processing path for each short-intake model at equal volume. This is a timing workload, not a new scoring set. The linear word-frequency model and the larger contextual model should run on the same resident worker, with no concurrent processing in a run.

## Run sequence and endpoints

For each run, send the 934 development bodies once in the prescribed order, then resend the first 66 in that same order. This gives 1,000 requests per run. Perform three serial runs for each model. Record the elapsed seconds from the first parse of the run through the final destination output. Record the model and run order alongside each result.

The extra 66 requests exist only to make the timing workload equal and convenient for costing. Do not add them to the 934 unique development documents used for F1. If the larger model's forward pass is observed separately, retain it as a separate measurement; it is inside the complete path and cannot stand in for the other model's full-path time.

## Outside this measure

Worker startup, external waiting and human review are excluded. These are resident-processing timings, not end-to-end customer latency or total operating cost. Brandon and I can apply a common worker rate once the full-path observations are available; Emily, please keep all three individual runs rather than just a summary.
