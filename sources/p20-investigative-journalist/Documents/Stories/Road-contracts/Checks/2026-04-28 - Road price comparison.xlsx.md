---
path: Documents/Stories/Road-contracts/Checks/2026-04-28 - Road price comparison.xlsx
kind: acn-source-check.xlsx
author: Miguel Valenzuela
derived_from:
---

# Same priced work on both sides

## Sheet: Tax basis

A2: Same priced work on both sides — v1.0  
C3: ACN-INV-20260428-LOG-001  
F3: Miguel Valenzuela  
C4: April 28, 2026  
A5: For Leslie Crawford and Veronica Huang. PW-FY23-PRICE-EXTRACT, pp. 2–6; amounts in USD, excluding tax.

| Row | A: Contract | B: Source | C: Work and line type | D: Basis | E: Original USD | F: Revised USD | G: Difference USD | H: Treatment |
|---:|---|---|---|---|---:|---:|---:|---|
| 7 | PW-23-01 | p.2 | Alder surface; work item | Tax excluded | 120000 | 120000 | =F7-E7 | Include |
| 8 | PW-23-02 | p.3 | Mill approach; work item | Tax excluded | 64000 | 82000 | =F8-E8 | Include |
| 9 | PW-23-02 | p.4 | Mill work around outlet; work item | Tax excluded | 98000 | 148000 | =F9-E9 | Include; contains row 10 |
| 10 | PW-23-02 | p.4 | Outlet protection; included detail | Tax excluded | | 28000 | | Exclude; part of F9 |
| 11 | PW-23-02 | p.5 | Mill return; work item | Tax excluded | 38000 | 52000 | =F11-E11 | Include |
| 12 | PW-23-03 | p.6 | Quarry repair transfer; work item | Tax excluded | 180000 | 180000 | =F12-E12 | Include |
| 13 | | | Comparable five items | Tax excluded | =SUM(E7:E9,E11:E12) | =SUM(F7:F9,F11:F12) | =F13-E13 | Five independent work items |
| 14 | | | Increase | | | | =G13/E13 | Percentage: 16.4% |
| 16 | | | Earlier calculation, revised | | | =SUM(F7:F12) | =F16-E13 | Includes detail twice |
| 17 | | | Earlier increase | | | | =G16/E13 | Percentage: 22% |

The independent-item totals are $500,000 original and $582,000 revised, a difference of $82,000. The earlier addition yields $610,000 and $110,000. The empty original field on row 10 is not a zero-priced independent item: it is a detail of the parent above. No estimated tax is included. These are comparable priced estimates, not expenditures, completed quantities or all contract totals. The 2024 tax-basis reconciliation is a separate comparison.
