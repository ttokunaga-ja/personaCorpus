---
path: Documents/Capacity/Peak options with the old cap.xlsx
kind: brt-capacity-comparison.xlsx
author: Daniel Jennings
derived_from:
---
# Sheet: Capacity

B3: BRT-CP-20250428-001  
E3: Daniel Jennings  
B4: 28 Apr 2025  
E4: BRT Fare Payments; reference 1,000 eligible attempts/min

| Row | A: No. | B: Option / UTC band | C: Forecast × | D: Limit × | E: Limit − demand × | F: Basis / decision |
|---:|---:|---|---:|---:|---|---|
| 7 | 1 | Old cap 15:00–16:00 | 1.70 | 2.00 | =D7-C7 | PCS-DF-20250425-001 |
| 8 | 2 | Old cap 16:00–17:00 | 2.05 | 2.00 | =D8-C8 | Shortfall begins |
| 9 | 3 | Old cap 17:00–18:00 | 2.30 | 2.00 | =D9-C9 | Peak shortfall |
| 10 | 4 | Old cap 18:00–19:00 | 1.90 | 2.00 | =D10-C10 | Superseded option retained |
| 11 | 5 | Proposal 15:00–16:00 | 1.70 | 2.80 | =D11-C11 | Proposed planning ceiling |
| 12 | 6 | Proposal 16:00–17:00 | 2.05 | 2.80 | =D12-C12 | Same forecast basis |
| 13 | 7 | Proposal 17:00–18:00 | 2.30 | 2.80 | =D13-C13 | 0.50× above forecast peak |
| 14 | 8 | Proposal 18:00–19:00 | 1.90 | 2.80 | =D14-C14 | Same forecast basis |
| 15 | Old cap minimum gap (A:B merged) | | | | =MIN(E7:E10) | Worst old-cap gap: −0.30× |
| 16 | Proposal minimum gap (A:B merged) | | | | =MIN(E11:E14) | Worst proposed gap: +0.50× |

Row 17, A:F merged: Difference is limit less forecast, in multiples of 1,000 eligible attempts/min. The forecast is the hourly p95 of one-minute rates, excluding duplicate retry indications and diagnostics. A negative gap is a shortfall. Multipliers are not additive across time bands. Proposal margin is a sizing allowance, not observed headroom.
