---
path: Documents/Clients/Denton/04 Feb - December on the agreed clock.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# December on the agreed clock

For Nicholas · Wednesday review, 04 Feb 2026. D1 is the first working day after 31 December; the final day is Angela's approval, included. Weekends and England and Wales public holidays are excluded. There are no public holidays between 2 and 14 January inclusive.

## Sheet: Working days

| Date | Day | Working day | Evidence or observation |
|---|---|---|---|
| 02 Jan 2026 | Fri | =NETWORKDAYS(DATE(2026,1,2),A2) | D1: first working day after month end |
| 05 Jan 2026 | Mon | =NETWORKDAYS(DATE(2026,1,2),A3) | |
| 06 Jan 2026 | Tue | =NETWORKDAYS(DATE(2026,1,2),A4) | |
| 07 Jan 2026 | Wed | =NETWORKDAYS(DATE(2026,1,2),A5) | |
| 08 Jan 2026 | Thu | =NETWORKDAYS(DATE(2026,1,2),A6) | |
| 09 Jan 2026 | Fri | =NETWORKDAYS(DATE(2026,1,2),A7) | |
| 12 Jan 2026 | Mon | =NETWORKDAYS(DATE(2026,1,2),A8) | Supplier statement 15:49; DEC-AP-STATEMENT |
| 13 Jan 2026 | Tue | =NETWORKDAYS(DATE(2026,1,2),A9) | Supplier reconciliation 10:19, DEC-AP-RECON; stock evidence 11:28, DEC-STOCK-GRNI; stock reconciliation 13:08, DEC-STOCK-RECON. Assembly is identified separately in the task list and Angela's email. |
| 14 Jan 2026 | Wed | =NETWORKDAYS(DATE(2026,1,2),A10) | Pack circulated 10:42, DEC-PACK-CIRCULATION; Angela approved 14:36, DEC-PACK-APPROVAL. |

## Sheet: Endpoint check

| Candidate | Date | Time GMT | Evidence | Inclusive days | Decision |
|---|---|---|---|---|---|
| Pack assembly | 13 Jan 2026 | Not recorded in task list | 28 Jan workbook, Task list excerpt: Close pack / Assemble pack for export; Angela's 02 Feb email identifies 13 Jan as assembly | =NETWORKDAYS(DATE(2026,1,2),B2) | 8; exclude, not approval |
| Pack circulation | 14 Jan 2026 | 10:42 | Finance event extract, DEC-PACK-CIRCULATION | =NETWORKDAYS(DATE(2026,1,2),B3) | 9; hand-off, not approval |
| Angela's approval | 14 Jan 2026 | 14:36 | Finance event extract, DEC-PACK-APPROVAL; Angela's 02 Feb email cites Finance approval history | =NETWORKDAYS(DATE(2026,1,2),B4) | 9; agreed endpoint |

The extract does not itself contain assembly. The 28 Jan task-list excerpt supplies the candidate date, and Angela's email identifies the event; neither makes it the approval. Keep D9 as the December baseline. Taking assembly instead gives D8, one working day earlier.
