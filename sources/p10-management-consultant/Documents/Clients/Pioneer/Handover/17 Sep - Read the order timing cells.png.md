---
path: Documents/Clients/Pioneer/Handover/17 Sep - Read the order timing cells.png
kind: screenshot
author: Nicholas Hargreaves
derived_from: Documents/Clients/Pioneer/Handover/17 Sep - August monthly checks v1.0.xlsx
---

# Differences from the workbook

A screen capture of the `Order timing` sheet in Excel, cropped to the heading, control information, column headings and the first three data rows. The workbook is [17 Sep - August monthly checks v1.0.xlsx](17%20Sep%20-%20August%20monthly%20checks%20v1.0.xlsx). All sheet values remain as in that workbook; the capture adds the annotations below.

The visible sheet title is “August order timing — 14-row extract”; the control line shows `HRA-PIO-BUY-MCK-2026-001 v1.0`, `17 Sep 2026` and `Nicholas Hargreaves`. The visible column headings A–H are `No.`, `PO record`, `Order issued`, `Order due`, `Denominator`, `Late flag`, `Decision / follow-up`, and `Record / source time`.

The three visible rows, in column order A–H, read:

| No. | PO record | Order issued | Order due | Denominator | Late flag | Decision / follow-up | Record / source time |
|---|---|---|---|---|---|---|---|
| 1 | A001 / PO-84217 | 18 Aug 2026 15:08 | 18 Aug 2026 15:00 | Count | 1 | First send after deadline | EV-84217; EDT −04:00 |
| 2 | A002 / PO-84241 | 19 Aug 2026 15:00 | 19 Aug 2026 15:00 | Count | 0 | At deadline | EV-84241; EDT −04:00 |
| 3 | A003 / PO-84255 | 20 Aug 2026 | 20 Aug 2026 15:00 | Review |  | Same-day date only; ask Buying for first supplier dispatch history | EV-84255; EDT −04:00 |

Add arrows and these readable callouts around the captured cells:

- To C7:D8: “Compare first send and approved deadline in EDT. Equal to the deadline is on time.”
- To C9:D9: “A date alone cannot establish whether the send was before or after 15:00.”
- To E9:F9: “Review; leave the late flag blank until the time is established.”
- To G9:H9: “Follow EV-84255 and ask Buying for the first supplier dispatch history.”

Add the note: “First three rows of the 14-row August extract.”
