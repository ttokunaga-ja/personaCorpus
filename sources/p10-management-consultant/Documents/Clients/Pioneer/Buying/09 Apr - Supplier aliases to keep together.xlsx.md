---
path: Documents/Clients/Pioneer/Buying/09 Apr - Supplier aliases to keep together.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Supplier aliases to keep together

Prepared by Connor Atkinson for Nicholas Hargreaves | 09 Apr 2025

## Sheet: Alias check

| Line key | Original display name | Supplier ID | Document | Signed net USD | Treatment |
|---|---|---|---|---:|---|
| AP-48106 | Pioneer Supplier 042 | S042 | Invoice | 121000 | Combine under S042 |
| AP-49271 | Supplier 042 Trading | S042 | Invoice | 78000 | Combine under S042 |
| AP-49308 | Supplier 042 Trading | S042 | Credit | -9000 | Deduct under S042 |
| AP-48962 | Supplier 018 | S018 | Invoice | 186400 | Boundary check, rank 12 |
| AP-49115 | Supplier 021 | S021 | Invoice | 182900 | Boundary check, rank 13 |
| S042 total | Both display names | S042 | Net | =SUM(E2:E4) | USD 190,000; rank 11 |

Grouping by the displayed name splits S042 into USD 121,000 and USD 69,000, with the credit only against the second name. Grouping by supplier ID gives USD 190,000. Against the full Q1 ranking, S042 is 11th, S018 12th and S021 13th, consistent with the ID-based provisional order shown at the start meeting. The extract is not the 42-supplier population.
