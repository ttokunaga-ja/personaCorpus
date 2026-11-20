---
path: Documents/長文点検記録分類/Cost-discussions/Matched-window run checklist.pdf
kind: tex:mercer-research-brief.cls
author: Emily Chen
derived_from:
---

# Before we book another run

Document ID: MDR-LDI-20251029-RM-01-R01  
Issued: October 29, 2025  
Author: Emily Chen  
To: Vanessa Nguyen and Brandon Romero

## Input checks before execution

Keep the fixed 480-record holdout, reference labels, model, tokenizer and common control tokens unchanged. The budgets below refer to body tokens. For each pair, check the actual selected token indices and original ordering; a body within the budget must appear once, while a longer body must contain no overlap.

| Full body length | 2,048 budget | 2,560 budget | Check before running |
|---|---|---|---|
| ≤2,048 | Full body once for both windows | Full body once for both windows | Same model and one copy of every body token |
| 2,049–2,560 | First 2,048 or first 1,536 + last 512 | Full body once for both windows | 2,048 selected without overlap; no truncation at 2,560 |
| ≥2,561 | First 2,048 or first 1,536 + last 512 | First 2,560 or first 1,536 + last 1,024 | Each ceiling respected, ends ordered and non-overlapping |

These are checks to perform, not completed results. Resolve any failure before booking the pass.

## Collection and reconciliation

For each of the four conditions retain the same record identifiers, reference and prediction; report misses and pairwise movements in both directions within each budget. Fatima and Vanessa should inspect original evidence positions and the two previously new middle-evidence misses. Brandon should collect preparation, host and accelerator costs for each pass using the same equipment, parallelism, boundary and unit prices. Keep costs alongside the corresponding predictions and input definition.

The existing first 2,048 and first 1,536 + last 1,024 passes yielded 37 and 11 misses; use those counts to reconcile the original conditions, not as results for the additional first 1,536 + last 512 or first 2,560 conditions. Those two conditions remain unmeasured. Record inputs and outcomes for all four before returning the comparison for an adoption decision.
