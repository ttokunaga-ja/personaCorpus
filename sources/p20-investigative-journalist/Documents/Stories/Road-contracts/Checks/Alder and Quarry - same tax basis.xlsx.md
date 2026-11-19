---
path: Documents/Stories/Road-contracts/Checks/Alder and Quarry - same tax basis.xlsx
kind: acn-tax-reconciliation.xlsx
author: Leslie Crawford
derived_from:
---

# Alder and Quarry — same tax basis

## Sheet: Tax basis

A2: Alder and Quarry — same tax basis (v1.0)  
C3: ACN-INV-20241016-LOG-001  
F3: Leslie Crawford  
C4: 2024-10-16

| Row | A: No. | B: Contract | C: Source / page / field | D: Tax basis | E: As printed (USD) | F: Included tax (USD) | G: Ex tax (USD) | H: Check |
|---:|---:|---|---|---|---:|---:|---|---|
| 7 | 1 | PW-23-01 | Executed agreement, release p. 31, Original Work Subtotal | Excluded | 2400000 | | `=IF(AND(ISNUMBER(E7),D7="Excluded"),E7,IF(AND(ISNUMBER(E7),D7="Included",ISNUMBER(F7)),E7-F7,""))` | The price basis says the work subtotal excludes materials tax. |
| 8 | 2 | PW-23-01 | CO-02, release p. 48, Revised Total | Included | 2542000 | 142000 | `=IF(AND(ISNUMBER(E8),D8="Excluded"),E8,IF(AND(ISNUMBER(E8),D8="Included",ISNUMBER(F8)),E8-F8,""))` | Checked the scan by eye. The total includes the separate Estimated Materials Tax line. The result matches the printed Revised Work Subtotal of USD 2,400,000. |
| 9 | 3 | PW-23-03 | Executed agreement, release p. 76, Original Work Subtotal | Excluded | 2800000 | | `=IF(AND(ISNUMBER(E9),D9="Excluded"),E9,IF(AND(ISNUMBER(E9),D9="Included",ISNUMBER(F9)),E9-F9,""))` | The price basis says the work subtotal excludes materials tax. |
| 10 | 4 | PW-23-03 | CO-01, release p. 98, Revised Total | Included | 2968000 | 168000 | `=IF(AND(ISNUMBER(E10),D10="Excluded"),E10,IF(AND(ISNUMBER(E10),D10="Included",ISNUMBER(F10)),E10-F10,""))` | Estimated Materials Tax is between the work subtotal and the total. The result matches the printed Revised Work Subtotal of USD 2,800,000. |

A11: For Richard Espinoza and Veronica Huang. I used Amounts copied from the release.csv to locate the four fields. Miguel Valenzuela checked the tax basis on the four source pages on October 16.

A12: F is blank on the two original rows because the amount printed in E already excludes tax. For each change order, F contains the estimate printed on the same page as the revised total. Payment records would be needed to verify tax paid.

## Sheet: Comparison

| Row | A: Contract | B: Original ex tax (USD) | C: Revised ex tax (USD) | D: Revised less original (USD) |
|---:|---|---|---|---|
| 2 | Alder FY2023 (PW-23-01) | `=IF(ISNUMBER('Tax basis'!G7),'Tax basis'!G7,"")` | `=IF(ISNUMBER('Tax basis'!G8),'Tax basis'!G8,"")` | `=IF(COUNT(B2:C2)=2,C2-B2,"")` |
| 3 | Quarry FY2023 (PW-23-03) | `=IF(ISNUMBER('Tax basis'!G9),'Tax basis'!G9,"")` | `=IF(ISNUMBER('Tax basis'!G10),'Tax basis'!G10,"")` | `=IF(COUNT(B3:C3)=2,C3-B3,"")` |
| 4 | Both contracts | `=IF(COUNT(D2:D3)=2,SUM(B2:B3),"")` | `=IF(COUNT(D2:D3)=2,SUM(C2:C3),"")` | `=IF(COUNT(B4:C4)=2,C4-B4,"")` |

A6: Check against the printed work subtotals. Original and revised are both USD 2,400,000 for Alder and both USD 2,800,000 for Quarry. Each difference is USD 0. Both columns add to USD 5,200,000, with a combined difference of USD 0.

A7: The USD 310,000 in First pass - which totals move.xlsx came from comparing original work subtotals with revised totals that included USD 142,000 and USD 168,000 in estimated materials tax. Keep the September calculation for the reporting trail. Use this comparison for the two FY2023 contracts on the four pages checked. Verify the printed basis separately for any other contract.
