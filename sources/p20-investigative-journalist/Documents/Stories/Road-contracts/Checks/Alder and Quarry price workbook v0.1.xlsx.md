---
path: Documents/Stories/Road-contracts/Checks/Alder and Quarry price workbook v0.1.xlsx
kind: acn-tax-reconciliation.xlsx
author: Leslie Crawford
derived_from:
---

# First pass — which totals move

## Sheet: Tax basis

A2: First pass — which totals move (v0.1)  
C3: ACN-INV-20240910-LOG-001  
F3: Leslie Crawford  
C4: 2024-09-10

| Row | A: No. | B: Contract | C: Source / page / field | D: Tax basis | E: As printed (USD) | F: Included tax (USD) | G: Ex tax (USD) | H: Check |
|---|---:|---|---|---|---:|---:|---:|---|
| 7 | 1 | PW-23-01 | Alder FY23 agreement, release p. 31, Original Work Subtotal | Unconfirmed | 2400000 | | | Read the tax terms against the subtotal. |
| 8 | 2 | PW-23-01 | Alder CO-02, release p. 48, Revised Total | Unconfirmed | 2542000 | | | Read the entries directly above the total on the scan. |
| 9 | 3 | PW-23-03 | Quarry FY23 agreement, release p. 76, Original Work Subtotal | Unconfirmed | 2800000 | | | Check what the original price schedule includes. |
| 10 | 4 | PW-23-03 | Quarry CO-01, release p. 98, Revised Total | Unconfirmed | 2968000 | | | Trace the total back to its component lines. |

A11: Figures from “Alder and Quarry price extract.csv.” Included tax and ex-tax amounts are blank pending a return to all four source pages. See “First pass calculation” for the question I want to take to Richard.

## Sheet: First pass calculation

A1: Subtotals against totals — working calculation for Richard  
A2: ACN-INV-20240910-LOG-001 / v0.1 / September 10, 2024 / Leslie Crawford

| Row | A: Contract | B: Original Work Subtotal (USD) | C: Revised Total (USD) | D: Candidate difference (USD) |
|---|---|---:|---:|---:|
| 3 | Alder FY2023, PW-23-01 | ='Tax basis'!E7 | ='Tax basis'!E8 | =C3-B3 |
| 4 | Quarry FY2023, PW-23-03 | ='Tax basis'!E9 | ='Tax basis'!E10 | =C4-B4 |
| 5 | Combined first pass | =SUM(B3:B4) | =SUM(C3:C4) | =C5-B5 |

A6: The subtraction gives $310,000 across the two pairs ($5,510,000 in the revised total column less $5,200,000 in the original work subtotal column). The headings differ and we have yet to check the tax basis. Hold this number out of copy until we can line up fields on the same basis.
