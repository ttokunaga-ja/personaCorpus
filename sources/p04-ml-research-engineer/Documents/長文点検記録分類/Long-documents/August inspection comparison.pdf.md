---
path: Documents/長文点検記録分類/Long-documents/August inspection comparison.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# August inspection comparison

Document ID: MDR-LDI-20260810-RM-02-R01  
Issued: August 10, 2026  
Author: Vanessa Nguyen  
To: Michael Serrano and the research team

## Comparison and input boundary

We fixed the comparison on August 4. The 4,000 original inspection records are from series independent of the 2025 training, development and evaluation series. Both conditions use the same reference labels, model, tokenizer and control tokens. The input is the record text, including the inspection narrative and final disposition; the series key and structured reference label are not model inputs.

The first-only condition takes up to the first 2,048 text tokens once for every record. The second-read condition starts identically, then reruns only records whose untruncated text exceeds 3,200 tokens using the first 1,536 and last 1,024 tokens. Its second prediction replaces the first. A record of exactly 3,200 tokens is not rerun. In this evaluation 1,000 of 4,000 records were rerun; this count is not an additional set of original records.

## Misses among records requiring action

A miss is a non-action prediction for a reference in one of the four action categories. The denominator is the same 1,000 action-required records in both columns.

| Reference disposition | Support | First-only misses | Second-read misses |
|---|---:|---:|---:|
| Repair required | 350 | 11 | 7 |
| Repeat inspection | 280 | 9 | 6 |
| Remove from service | 210 | 7 | 6 |
| Engineering review | 160 | 5 | 5 |
| Total | 1,000 | 32 (3.2%) | 24 (2.4%) |

The observed difference is 0.8 percentage points on this evaluation. On the paired records, 14 misses under first-only were rescued, six became new misses under the second-read condition, 18 were missed by both and 962 by neither. The net change of eight does not reveal the six moving in the unfavorable direction.

## False alarms among records not requiring action

An action prediction for a non-action reference is a false alarm. Among the separate 3,000 non-action records, first-only produced 90 and the second-read condition 103. Four were false alarms only under first-only, 17 only under second-read, 86 under both and 2,893 under neither.

Fatima and I reread the six new misses: four lost a middle condition or active instruction from the rerun window; in two, the relevant evidence remained but the prediction favored an older disposition. Length alone does not guarantee an improvement for every record.

This comparison does not establish peak-time behavior, improvement across all eight categories, or operational acceptability. Those require separate evidence and a use decision.
