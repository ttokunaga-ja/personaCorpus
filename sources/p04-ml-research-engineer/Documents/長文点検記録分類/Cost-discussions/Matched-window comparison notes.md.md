---
path: Documents/長文点検記録分類/Cost-discussions/Matched-window comparison notes.md
kind: text
author: Vanessa Nguyen
derived_from:
---

# What to match before the next run

October 15, 2025  
To: Emily Chen, Fatima Rahman, Brandon Romero  
From: Vanessa Nguyen

The October comparison changed both the window and its budget. Please keep these two pairs together when we prepare the next run:

| Body-token budget | First only | First and last |
|---|---|---|
| 2,048 | first 2,048 | first 1,536 + last 512 |
| 2,560 | first 2,560 | first 1,536 + last 1,024 |

Use the same fixed 480 records, reference labels, model, tokenizer and control tokens. Emily, please check the actual input spans and ensure a body within budget is passed once, and longer bodies are in original order without overlap. Record the realized lengths as well as the ceilings.

Fatima and I will pair predictions by record. Count misses in each condition and both directions of change, then read them by the effective evidence position in the full original body. In particular, return separately to the two ends-only misses: the middle repair instruction and the middle condition governing a reinspection deadline. Evidence present and correct prediction are separate observations.

Brandon, please use the same measurement boundary, equipment, parallelism and unit prices for each pass, including preparation, host processing and accelerator inference. Keep the measured 100 versus 118 with its original unequal-budget conditions; neither new condition has been measured yet.

We should decide whether an ends window is preferable only after seeing the matched-budget tradeoff, including new misses and cost. This is a comparison request, not an adoption decision.
