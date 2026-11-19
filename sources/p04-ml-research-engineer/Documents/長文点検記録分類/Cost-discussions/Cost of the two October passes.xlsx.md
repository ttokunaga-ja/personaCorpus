---
path: Documents/長文点検記録分類/Cost-discussions/Cost of the two October passes.xlsx
kind: mercer-adjudication.xlsx
author: Brandon Romero
derived_from:
---

# Cost of the two October passes

## Sheet: Adjudication

- B3: MDR-LDI-20251013-AW-01-R01
- E3: October 13, 2025
- B4: Brandon Romero
- E4: Fixed inspection holdout; 480 records per pass
- A2: Relative cost per pass

| Cell | A | B | C | D | E | F |
|---|---|---|---|---|---|---|
| 5 | Ref. | Component | First 2,048 | First 1,536 + last 1,024 | Difference | Basis |
| 6 | 1 | Accelerator | 80 | 98 | =D6-C6 | Measured inference |
| 7 | 2 | Host processing | 12 | 12 | =D7-C7 | Same boundary |
| 8 | 3 | Input preparation | 8 | 8 | =D8-C8 | Same boundary |
| 9 | | Total | =SUM(C6:C8) | =SUM(D6:D8) | =D9-C9 | First-only total = 100 |
| 10 | | Relative increase | | =D9/C9-1 | | 18% |
| 11 | | Misses, evaluation result | 37 | 11 | =D11-C11 | Counts, not cost components |

Same 480 records, hardware, parallelism and unit prices for each pass. Indices cover input preparation, host processing and accelerator inference. The result is for one pass over the set, not a monthly estimate. Window location and token budget both differ; the increment is specific to these two measured conditions.
