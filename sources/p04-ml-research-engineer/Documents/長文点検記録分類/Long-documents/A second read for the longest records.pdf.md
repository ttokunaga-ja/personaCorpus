---
path: Documents/長文点検記録分類/Long-documents/A second read for the longest records.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---
# A second read for the longest records

Document ID: MDR-LDI-20260804-RP-01-R01  
Issued: August 4, 2026  
Author: Vanessa Nguyen  
For: Emily Chen and Fatima Rahman

## Question and fixed comparison

Can a second pass over the ends of the longest signed inspection records reduce misses of actions required at signature? Compare the two routes on the same independent 4,000 records, using the same fixed model, tokenizer, control tokens and reference labels. Only the inspection body enters the model. Do not tune the route on these records.

The first route reads up to the first 2,048 body tokens once for every record. The proposed route makes that same first pass for every record. Measure the original body before truncation: only when it exceeds 3,200 body tokens, run a second inference on the first 1,536 and last 1,024 tokens in their original order, and replace the first prediction with the second. A body of exactly 3,200 takes one pass. Neither the first prediction nor the reference label selects the route.

## Records and measures

Joshua's series check must keep this evaluation separate from the 2025 training, development and fixed evaluation series. Preserve the existing eight signed-disposition classes. For the four action-required classes, count a miss when the prediction falls in any of the four non-action classes; report the count and rate over the action-required reference records. Separately count false alarms: non-action reference records predicted into an action-required class, with their own denominator. Pair the decisions by record as well as reporting totals, so improvements and newly introduced misses remain visible.

Emily, please check the length boundary and the two windows before scoring. Fatima, retain the reference support and both predictions for each record and report the two error directions separately. The second-read results are not yet available. The 2025 same-budget follow-up conditions are not part of this plan; this comparison does not by itself establish operational suitability.
