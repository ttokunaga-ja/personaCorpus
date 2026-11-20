---
path: OneDrive - Holt/Team-room/05 Nov - Why these pack prices do not join.xlsx
kind: Working sheet
author: Deepa Desai
derived_from:
---
## Sheet: Pack conversion

Deepa to Connor — S042 threshold comparison, provisional illustration using AP-61972. Keep the receipt unit for the contract test and the invoice columns for the later reconciliation separate.

Row 1 is the header. Row 2 contains the record.

| A Supplier | B Document | C Receipt quantity | D Receipt unit | E Cases per pack | F Receipt cases for threshold | G Invoice quantity | H Invoice unit | I USD per invoice unit | J Original USD | K USD per case | L Converted USD | M Difference USD |
|---|---|---:|---|---:|---:|---:|---|---:|---:|---:|---:|---:|
| S042 | AP-61972 | 150 | pack | 10 | =C2*E2 | 150 | pack | 180 | =G2*I2 | =I2/E2 | =F2*K2 | =L2-J2 |

F2 is 1,500 cases; J2 and L2 are both USD 27,000; M2 is zero. Receipt cases are the numerator for the eventual net-quarter threshold test after linked returns. The invoice fields are retained for matching the posting and any credit; they do not replace the receipt quantity.

## Sheet: Join check

| A Check | B Value | C Meaning |
|---|---:|---|
| Incorrect raw-number comparison: packs | 150 | Joining this number directly to a case threshold confuses units. |
| Cases per pack | 10 | Buying terms T2. |
| Comparable receipt cases | ='Pack conversion'!F2 | Use cases, not the pack count. |
| Threshold, net cases | 7000 | Apply only after the full quarter and linked returns are available. |
| Value preservation, USD | ='Pack conversion'!M2 | Zero means this conversion preserves the amount; it does not establish the final quarter price. |

Reference: PTD-BUY-TRM-2025-001 Rev. 1. The example is not a quarter outcome.
