---
path: OneDrive - Holt/Team-room/10 Feb - Denton travel expenses.xlsx
kind: Working sheet
author: Teresa Green
derived_from:
---

# Travel receipts: Denton allocation

10 Feb 2026 · Checked with Nicholas · Internal billing only · GBP. The provisional Denton allocation included a hotel night belonging to Meadowbank. Move that receipt to the Meadowbank internal billing account; retain the original receipt total.

## Sheet: Receipt allocation

| Receipt reference | Category | Original account | Correct account | Original Denton GBP | Correct Denton GBP | Correct Meadowbank GBP | Reason |
|---|---|---|---|---|---|---|---|
| TR-1002-R1 | Rail | DEN-CLOSE | DEN-CLOSE | 314 | 314 | 0 | Denton travel |
| TR-1002-H1 | Hotel | DEN-CLOSE | DEN-CLOSE | 576 | 576 | 0 | Denton stay |
| TR-1002-M1 | Meals | DEN-CLOSE | DEN-CLOSE | 130 | 130 | 0 | Denton travel |
| TR-1002-L1 | Local transport | DEN-CLOSE | DEN-CLOSE | 100 | 100 | 0 | Denton travel |
| TR-1002-H2 | Hotel | DEN-CLOSE | MEADOWBANK | 220 | 0 | 220 | Separate customer stay; transfer 10 Feb |
| Total | | | | =SUM(E2:E6) | =SUM(F2:F6) | =SUM(G2:G6) | Original and corrected allocations total GBP 1,340. |

## Sheet: Transfer check

| Posting reference | Date | Receipt | From billing account | To billing account | From GBP | To GBP | Net GBP |
|---|---|---|---|---|---|---|---|
| TG-100226-01 | 10 Feb 2026 | TR-1002-H2 | DEN-CLOSE | MEADOWBANK | -220 | 220 | =SUM(F2:G2) |

| Account | Before GBP | Movement GBP | After GBP |
|---|---|---|---|
| DEN-CLOSE | 1340 | ='Transfer check'!F2 | =B5+C5 |
| MEADOWBANK, this receipt | 0 | ='Transfer check'!G2 | =B6+C6 |
| Combined | =SUM(B5:B6) | =SUM(C5:C6) | =SUM(D5:D6) |

The transfer reference and receipt reference identify both sides of the reallocation. Denton billing basis after correction is GBP 1,120; no expense has been added or removed.
