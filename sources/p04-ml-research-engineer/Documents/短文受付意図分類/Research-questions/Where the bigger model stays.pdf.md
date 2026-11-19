---
path: Documents/短文受付意図分類/Research-questions/Where the bigger model stays.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---
# Where the bigger model stays

Document ID: MDR-STI-20241009-RM-01-R01  
Issued: October 9, 2024  
Author: Vanessa Nguyen  
To: Michael Serrano and Brandon Romero

## Decision

At the October 8 review Michael decided not to put the larger contextual model into automatic intake under the measured conditions. I will retain its run as a research comparator, together with the development results and the aligned timing calculation. This leaves the current intake arrangement unchanged.

## Target and development evidence

Our first-year research target is 0.82 macro-F1 across the 12 intents. It is a research target, not an authorization to route requests automatically. On the same 934 development documents, the word-frequency linear model reached 0.827 and the larger model 0.846. Each number is the equally weighted mean of the 12 unrounded class F1 values, rounded to three decimals. Both models used the same 2,800 training bodies; all development bodies were scored without a review threshold. The reserved 940-document evaluation partition was not used for this comparison.

The overall gain is accompanied by a loss on Cancel request: F1 falls from 0.877 to 0.832, with false negatives increasing from 7 to 10 among 57 reference cancellations. We should read the changed cases before claiming a reason for that decline. The 934 include the 800 bodies used in the August routing replay, so the two exercises are not independent replications.

## Cost on a common basis

Brandon and I aligned the timing scope on October 8. Each model processed 1,000 requests per run on the same resident worker, three times, from first parse to final route output. The 1,000 comprise the 934 development bodies and a repeat of the first 66 for timing only. Median whole-path time was 100 seconds for linear and 210 for the larger model. At USD 1.20 per worker hour, the corresponding costs per 10,000 requests are USD 0.3333 and USD 0.7000, a 2.10 ratio before rounding. Startup, external waiting and human review are excluded.

The earlier 1.50 working ratio paired the linear whole path with the larger model's forward-only time. It is not the like-for-like cost comparison. The corrected ratio and the decline on cancellations matter to the use decision even though the larger model's development macro-F1 is higher.

## Next comparison

Keep the run and its conditions available for research; do not treat it as an automatic-intake candidate on the present evidence. Before revisiting the use question, prepare a lighter alternative at comparable cost, then compare it under the same documented conditions and examine the cancellation errors. Preserve the unused evaluation partition for its intended role.

References: October 8 cost review notes; September intake model comparison; Intake cost at equal volume, October 8, 2024.
