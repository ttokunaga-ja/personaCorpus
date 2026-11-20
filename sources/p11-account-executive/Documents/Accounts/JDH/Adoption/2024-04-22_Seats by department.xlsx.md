---
path: Documents/Accounts/JDH/Adoption/2024-04-22_Seats by department.xlsx
kind: halstead-adoption.xlsx
author: Michelle Whitaker
derived_from:
---

# Department Adoption

A3: JDH seat activity through April 21
C4: HBC-JDH-20240422-ADP-01
C5: April 22, 2024
C6: Michelle Whitaker

| Row | A: No. | B: Department | C: Licensed seats | D: Active seats | E: Utilization | F: Review note |
|---|---:|---|---:|---:|---|---|
| 9 | 1 | Dispatch | 150 | 126 | =IF(C9>0,D9/C9,"") | 10 unassigned; 14 assigned without use in the window. These are different questions. Janet: which handoffs need a named user? |
| 10 | 2 | Field Operations | 140 | 106 | =IF(C10>0,D10/C10,"") | 14 unassigned; 20 assigned without use in the window. Do not assume the same cause. Janet: who can speak for the night shift? |
| 11 | 3 | Customer Care | 80 | 64 | =IF(C11>0,D11/C11,"") | 6 unassigned; 10 assigned without use in the window. Assignment and activity need separate review. Janet: are the right recipients assigned? |
| 12 | 4 | Billing | 50 | 22 | =IF(C12>0,D12/C12,"") | 10 unassigned; 18 assigned without use in the window. The reasons may differ. Janet: where do exception messages stop? |
| 13 | | Total | =SUM(C9:C12) | =SUM(D9:D12) | =IF(C13>0,D13/C13,"") | Weighted across licensed seats |

A14:F14: Active means at least one business use from March 23 through April 21, inclusive, Denver local dates. Count each contract seat once, including reassigned seats. Total is 318/420 (75.7%); 102 had no use in the window. Source: JDH seat activity through Apr 21.csv.
