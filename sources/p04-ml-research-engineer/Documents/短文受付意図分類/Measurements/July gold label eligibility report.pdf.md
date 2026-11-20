---
path: Documents/短文受付意図分類/Measurements/July gold label eligibility report.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# Gold labels before the first score

Document ID: MDR-STI-20240722-RM-01-R01  
Issued: July 22, 2024  
Author: Vanessa Nguyen  
To: Michael Serrano

## What the July reconciliation establishes

Fatima and I have confirmed the dispositions of the 4,800 short intake bodies in the collection. For 4,674, the body supports a single gold-label candidate under the revised guide. The remaining 126 do not identify one intent from the text and are held. These two groups account for the collection, not an evaluation score or a completed evaluation partition.

| First independent candidates | Supported candidate | Hold | Documents |
|---|---:|---:|---:|
| Agree | 4,456 | 32 | 4,488 |
| Differ | 218 | 94 | 312 |
| Total | 4,674 | 126 | 4,800 |

The hold rate is 32 of 4,488 among initial agreements and 94 of 312 among disagreements. A matching first choice can still lack a shared textual basis. Hold is a disposition; the gold-label field remains empty. The 126 are excluded from scoring and their extent must be reported alongside any later result.

## Before comparing models

Our April plan, MDR-STI-20240416-RP-01-R01, sets 0.82 as the first-year macro-F1 target for the single 12-class task. It is not a measured result. Macro-F1 is the unweighted mean of the per-class F1 values, so each class needs support in the evaluation partition.

From the 4,674 supported candidates, we still need to define document-level training, development and evaluation partitions and fix an evaluation partition with support for all 12 classes before scoring. The majority-class baseline and word-frequency linear classifier must be scored on that same fixed partition under the same conditions. Until then there is no observed macro-F1 to compare with the target.

## Scope and use

The 126 held bodies are outside the proposed scoring population, so a score on eligible cases will not describe performance on those texts. Keep that exclusion and the collection denominator visible in the report. Approval for use remains a separate question with Brandon Romero; this reconciliation does not grant it.

References: July label eligibility reconciliation, July 19, 2024; Short intake intent research plan, April 16, 2024; July holds by first-read agreement, July 19, 2024.
