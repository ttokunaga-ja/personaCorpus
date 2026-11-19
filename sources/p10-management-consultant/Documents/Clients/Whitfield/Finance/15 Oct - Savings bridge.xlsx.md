---
path: Documents/Clients/Whitfield/Finance/15 Oct - Savings bridge.xlsx
kind: holt-whitfield-finance.xlsx
author: Connor Atkinson
derived_from:
---

# Savings bridge

A2: Japan savings — annualised correction v1.0  
A4: HRA-WHI-DIAG-REC-2024-001  
D4: 15 Oct 2024  
F4: Connor Atkinson

| Row | A: No. | B: Item | C: Original reference and carried row | D: Original annual JPY | E: Duplicate deducted JPY | F: Corrected annual JPY | G: Work key | H: Basis and decision |
|---|---:|---|---|---:|---:|---:|---|---|
| 7 | 1 | Routine checks | AP model!D12; Original estimate row 7 | 8000000 | 0 | =D7-E7 | AP-CHECK | 16,000 checks × 7.5 minutes × JPY 4,000/hour; retain work capacity value |
| 8 | 2 | Independent follow-up contacts | AP model!D13; Original estimate row 8 | 6000000 | 0 | =D8-E8 | AP-CALL | 3,000 contacts × 30 minutes × JPY 4,000/hour; retain work capacity value |
| 9 | 3 | Exception document packs | AP model!D14; Original estimate row 9 | 4000000 | 0 | =D9-E9 | EXC-PACK | 2,000 packs × 30 minutes × JPY 4,000/hour; retain AP preparation |
| 10 | 4 | Avoidable overtime premium | Overtime!D8; Original estimate row 10 | 4000000 | 0 | =D10-E10 | OT-PREM | 1,000 approved premium hours × JPY 4,000/hour premium, excluding base pay; retain subject to actual reduction |
| 11 | 5 | Handover preparation | Finance model!D11; Original estimate row 11 | 4000000 | 4000000 | =D11-E11 | EXC-PACK | Same 2,000 packs and 30 minutes as AP preparation; Finance receives the packs rather than preparing a second set |
| 12 | Total | | | =SUM(D7:D11) | =SUM(E7:E11) | =SUM(F7:F11) | | |

Row 13: The corrected JPY 22m per year comprises JPY 18m of potentially redeployable work capacity and JPY 4m of overtime premium avoided only if approved premium expenditure actually falls. It is an annualised steady-operation estimate, not first-year realised savings or a headcount reduction. AP preparation and Finance receipt were traced for EP-184, EP-219 and EP-267; the annual volumes are from Japan Finance's closed quarterly totals for July 2023 to June 2024.
