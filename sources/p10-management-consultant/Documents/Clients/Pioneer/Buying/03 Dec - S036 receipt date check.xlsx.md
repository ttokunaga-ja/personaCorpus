---
path: Documents/Clients/Pioneer/Buying/03 Dec - S036 receipt date check.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Keep the receipt date on the line
For Deepa • 03 Dec 2025 • S036 / T3, protective sleeves

## Sheet: Boundary

| Row | Document | Receipt | Posted | Cases | Receipt-date expected USD/case | Invoice-date expected USD/case | Difference invoice basis less receipt basis | Expected amount USD | Invoice-date amount USD |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| 2 | AP-62418 | 2025-11-28 | 2025-12-02 | 1250 | 20 | 18 | =G2-F2 | =E2*F2 | =E2*G2 |
| 3 | AP-62431 | 2025-12-01 | 2025-12-02 | 1250 | 18 | 18 | =G3-F3 | =E3*F3 | =E3*G3 |
| 4 | Total | | | =SUM(E2:E3) | | | | =SUM(I2:I3) | =SUM(J2:J3) |

The term changes for receipts from 01 Dec, not invoices posted from that date. AP-62418 would be assigned a price USD 2/case too low by invoice date: USD 2,500 on this line. Keep both dates on the extract; the later posting does not move the receipt across the boundary. Source: PTD-BUY-TRM-2025-001 Rev. 1.
