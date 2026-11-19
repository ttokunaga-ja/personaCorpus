---
path: Documents/長文点検記録分類/Long-documents/October findings for Michael.pdf
kind: tex:mercer-research-brief.cls
author: Vanessa Nguyen
derived_from:
---

# October findings: inspection records

Document ID: MDR-LDI-20251023-RM-01-R01  
Issued: October 23, 2025  
Author: Vanessa Nguyen  
To: Michael Serrano

## Scope and finding

On July 28 you approved research on 2,400 inspection records and eight classes. Joshua and Fatima kept original records, reinspections and corrections in their respective series when fixing the partitions. The fixed evaluation contains 480 records; it is separate from the short-text task. The four action-required classes have 180 reference records. A miss is an action-required reference predicted as one of the other four classes; a wrong action-required class is an eight-class error but not a miss by this definition.

We compared first 2,048 body tokens with first 1,536 plus last 1,024 on those same records, references and model. Bodies within a window's budget were passed once; longer bodies were assembled in order without overlap.

| Paired outcome among 180 action-required records | Records |
|---|---:|
| Missed by first only | 28 |
| Missed by ends only | 2 |
| Missed by both | 9 |
| Missed by neither | 141 |
| Total | 180 |

Thus misses were 37 and 11 respectively. The 28 recovered and two newly missed should both remain visible. Among the 39 missed by at least one window, 23 have the endpoint of their effective evidence in the final quarter of the full body. The two newly missed examples have middle evidence: one is a repair instruction, the other a clause governing a reinspection deadline.

## Cost and interpretation

Brandon compared one pass over the same 480 records with the same equipment, parallelism and unit prices, including input preparation and host work. First-only is indexed at 100 and first-and-last at 118, an 18% increase for these conditions. The input location and budget changed together, from 2,048 to 2,560 body tokens. We cannot attribute the difference to ends placement alone, or transfer the cost increment to a different comparison.

## Next comparison and decision

At a 2,048-token budget compare first 2,048 with first 1,536 plus last 512. At 2,560 compare first 2,560 with first 1,536 plus last 1,024. Hold the evaluation, references and model fixed; collect paired misses in both directions, evidence positions and costs under the same boundary. Read the two middle-evidence records individually. The additional conditions remain unmeasured. Application adoption remains on hold pending those results.
