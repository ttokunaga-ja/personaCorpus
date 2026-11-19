---
path: Documents/長文点検記録分類/Long-documents/What each input window actually sees.pdf
kind: tex:mercer-research-brief.cls
author: Emily Chen
derived_from:
---
# What each input window actually sees

Document ID: MDR-LDI-20250918-RM-01-R01  
Issued: September 18, 2025  
Author: Emily Chen  
To: Vanessa Nguyen and Fatima Rahman

## Body selection for the fixed comparison

For a body of N tokens, numbered from 1 in original order, the opening condition passes tokens 1 through min(N, 2,048). The two-end condition passes the entire body once if N is at most 2,560. For a longer body it passes tokens 1–1,536 followed by tokens N−1,023–N. Preserve original order and do not duplicate overlapping text. The omitted middle is not replaced by the reference label or any ledger field.

| Body length | First 2,048 | First 1,536 + last 1,024 |
|---|---|---|
| 2,048 | 1–2,048, all once | 1–2,048, all once |
| 2,049 | 1–2,048; omit 2,049 | 1–2,049, all once |
| 2,560 | 1–2,048; omit 2,049–2,560 | 1–2,560, all once |
| 2,561 | 1–2,048; omit 2,049–2,561 | 1–1,536 then 1,538–2,561; omit 1,537 |

At lengths within 2,560, nominal first and last segments can overlap. Passing the body once resolves that overlap rather than repeating it. The opening condition has a smaller body budget; these two conditions change both the budget and the retained positions.

## Held fixed

Use the August fixed evaluation partition of 480 records and the same reference label per record for both conditions. Use the same classifier and tokenization. Apply the selection to the inspection body, excluding the series key and structured reference label. Common control tokens are outside the stated body budgets and must be identical in both inputs; the numbers 2,048 and 2,560 count body tokens only.

Before scoring, inspect boundary-length inputs against this table and confirm the concatenation order. Record the selected condition with every prediction so a result cannot be detached from its input definition.
