---
path: Documents/Accounts/Hollis/2025-03-28_Allocation sequence.xlsx
kind: halstead-adoption.xlsx
author: Divya Rao
derived_from:
---

# Allocation Sequence

Sheet: Allocation Sequence  
Document: HBC-HOL-20250328-IMP-01 v1.0  
As of: March 28, 2025  
Prepared by: Divya Rao  
For: Michelle Whitaker and Lawrence Sutton

| Row | A: Order | B: Group | C: Seats | D: Planned date | E: Checker | F: Condition before notice |
|---:|---:|---|---:|---|---|---|
| 8 | Order | Group | Seats | Planned date | Checker | Condition before notice |
| 9 | 1 | Day shift | 32 | April 1, 2025 | Lawrence Sutton | Lawrence confirms the roster; Divya manually imports and reconciles the result with Lawrence before notice. |
| 10 | 2 | Night shift | 14 | April 3, 2025 | Lawrence Sutton | Complete April 2 night-shift practice; Lawrence confirms the roster and Divya reconciles the manual import with him before notice. |
| 11 | | Scheduled confirmed seats | =SUM(C9:C10) | | | 46 planned; not an allocation result. |
| 12 | | Returned candidate rows | 4 | | Lawrence Sutton | Two pairs held from this import pending source-roster response. |

The returned four are candidate rows, not four scheduled seats. Contract start does not mean that the full 180-seat limit is allocated on April 1.
