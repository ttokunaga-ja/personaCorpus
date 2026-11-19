---
path: Documents/長文点検記録分類/Long-documents/Paired scoring for the October read.pdf
kind: tex:mercer-research-brief.cls
author: Fatima Rahman
derived_from:
---
# Paired scoring for the October read

Document ID: MDR-LDI-20250924-RM-01-R01  
Issued: September 24, 2025  
Author: Fatima Rahman  
To: Emily Chen and Vanessa Nguyen

## Population and reference

Score both inputs against the same August fixed evaluation partition: 480 complete inspection records, with one reference label per record. The reference is the disposition effective at signature, established from the full body, not the label suggested by either truncated input. The series-based partition is fixed; do not add development readings to these counts. Of the 480 references, 180 are in the four action-required categories.

| Reference category | Evaluation records | Action required? |
|---|---:|---|
| Repair required | 64 | Yes |
| Repeat inspection | 48 | Yes |
| Remove from service | 38 | Yes |
| Engineering review | 30 | Yes |
| Routine closeout | 112 | No |
| Monitor at next service | 78 | No |
| Record correction | 48 | No |
| No equipment finding | 62 | No |
| Total | 480 | 180 yes |

## Scoring and pairing

For each record, retain its reference and the two predictions: first 2,048 body tokens, and first 1,536 plus last 1,024 under Emily's September 18 input rule. A miss is an action-required reference predicted as one of the four non-action categories. An action-required reference predicted as a *different* action-required category is an eight-class error, but not a miss under this definition. Score all eight labels as well; the miss count does not replace that assessment. A non-action reference is outside the 180-record miss denominator.

Pair results by record. Among the 180 action-required references, count first-only misses (recovered by the two-end condition), two-end-only misses (new misses), misses under both, and misses under neither. The first condition's total is first-only plus both; the second's is two-end-only plus both. Keep the two directions separate rather than reporting only their net difference.

For records missed in either condition, record independently the effective disposition evidence in the complete body and the position of the end of that phrase, using the full body as denominator. Then inspect what each input retained. A visible phrase is not itself a correct classifier prediction. Store the position and prediction fields separately, and retain errors between action labels in the eight-class results even though they do not enter the miss table.
