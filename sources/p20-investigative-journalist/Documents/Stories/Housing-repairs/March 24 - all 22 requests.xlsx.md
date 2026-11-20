---
path: Documents/Stories/Housing-repairs/March 24 - all 22 requests.xlsx
kind: acn-source-check.xlsx
author: Veronica Huang
derived_from:
---

# Housing repairs | Completion-date source check

For Leslie Crawford and Richard Espinoza. Review with Theresa Coleman and Leslie, March 24, 2026, 14:00–14:45. The reporting codes refer to requests, not residences. A confirmed date is a date the records identify as completion; a blank does not mean the work was unfinished.

## Sheet: Sources

C3: ACN-INV-20260324-LOG-001 v1.0  
F3: Veronica Huang  
C4: 2026-03-24 (date)

Row 6 headers: A `Source row`, B `Reference / request`, C `Page`, D `Completion date`, E `Status`, F `Evidence and limit`. Rows 7–28 are the complete detail. A contains the numeric worksheet row number; dates in D are dates, not text. Blank D cells remain empty.

| Row / A | B | C | D | E | F |
|---:|---|---|---|---|---|
| 7 | PCTA-RPR-20260323-001 / R-01 | 1 | 2024-04-16 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 8 | PCTA-RPR-20260323-001 / R-02 | 1 | 2024-04-30 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 9 | PCTA-RPR-20260323-002 / R-03 | 1 | 2024-08-19 | Newly confirmed | Signed work entry transcribed by association; bracket secured. Signature itself is not on the copy. |
| 10 | PCTA-RPR-20260323-001 / R-04 | 1 | 2024-05-09 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 11 | PCTA-RPR-20260323-001 / R-05 | 1 | 2024-05-20 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 12 | PCTA-RPR-20260323-002 / R-06 | 1 | 2024-09-04 | Newly confirmed | Signed work entry transcribed by association; exhaust switch replaced. Signature itself is not on the copy. |
| 13 | PCTA-RPR-20260323-001 / R-07 | 1 | 2024-06-04 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 14 | PCTA-RPR-20260323-001 / R-08 | 1 | 2024-06-13 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 15 | PCTA-RPR-20260323-002 / R-09 | 1 | 2024-10-08 | Newly confirmed | Signed work entry transcribed by association; sink shutoff valve replaced. Signature itself is not on the copy. |
| 16 | PCTA-RPR-20260323-001 / R-10 | 1 | 2024-06-21 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 17 | PCTA-RPR-20260323-001 / R-11 | 2 | 2024-06-27 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 18 | PCTA-RPR-20260323-003 / R-12 | 1 | 2024-11-12 | Newly confirmed | Resident's written completion of window-gap filling, transcribed by association; registration on 2024-11-14 is a separate date. |
| 19 | PCTA-RPR-20260323-001 / R-13 | 2 | 2024-07-08 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 20 | PCTA-RPR-20260323-001 / R-14 | 2 | 2024-07-11 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 21 | PCTA-RPR-20260323-003 / R-15 | 1 | 2025-01-16 | Newly confirmed | Resident's written completion of sink drain gasket replacement, transcribed by association; registration on 2025-01-21 is a separate date. |
| 22 | PCTA-RPR-20260323-001 / R-16 | 2 | 2024-07-16 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 23 | PCTA-RPR-20260323-004 / R-17 | 1 | | Unconfirmed | 2024-07-29 is a visit for door-brush measurement; no replacement completion is recorded. |
| 24 | PCTA-RPR-20260323-001 / R-18 | 2 | 2024-07-22 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 25 | PCTA-RPR-20260323-001 / R-19 | 2 | 2024-07-25 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 26 | PCTA-RPR-20260323-004 / R-20 | 1 | | Unconfirmed | 2024-08-02 is the date a return visit was booked, not a work completion date. |
| 27 | PCTA-RPR-20260323-001 / R-21 | 2 | 2024-08-01 | Previously confirmed | Association copy of signed work entry; recorded complete. |
| 28 | PCTA-RPR-20260323-004 / R-22 | 1 | | Unconfirmed | 2024-08-05 is transcription of a lighting ticket to the register; the original ticket's completion field has not been established. |

Summary below the detail, with labels and formulas in separate cells:

| Row | A (formula) | B:F (merged label) |
|---:|---|---|
| 29 | `=COUNTIF(E7:E28,"Previously confirmed")` | Previously confirmed completion dates: 14 |
| 30 | `=COUNTIF(E7:E28,"Newly confirmed")` | Newly confirmed completion dates: 5 |
| 31 | `=SUM(A29:A30)` | Completion dates confirmed: 19 |
| 32 | `=COUNTIF(E7:E28,"Unconfirmed")` | Completion dates unconfirmed: 3 |
| 33 | `=SUM(A31:A32)` | Confirmed plus unconfirmed: 22 |
| 34 | `=COUNT(A7:A28)` | Requests reviewed: 22 |

## Sheet: Holds

C3: ACN-INV-20260324-LOG-001 v1.0  
F3: Veronica Huang  
C4: 2026-03-24 (date)

Row 6 headers: A `Hold`, B `Request / claim`, C `Sources row`, D `Record needed`, E `Owner`, F `Status`. A and C are numeric; display A as H1, H2, H3.

| Row / A | B | C | D | E | F |
|---:|---|---:|---|---|---|
| 7 / 1 | R-17: door brush replacement completion date | 23 | Replacement completion ticket; measurement alone does not establish completion. | Leslie: request; Veronica: recheck | Hold — completion date unconfirmed |
| 8 / 2 | R-20: sink drain work completion date | 26 | Report of work performed after the return visit was booked. | Leslie: request; Veronica: recheck | Hold — completion date unconfirmed |
| 9 / 3 | R-22: communal lighting work completion date | 28 | Original ticket and its completion field. | Leslie: request; Veronica: recheck | Hold — completion date unconfirmed |

| Row | A (formula) | B:F (merged label) |
|---:|---|---|
| 10 | `=COUNT(A7:A9)` | Three requests need further evidence; Richard will reconsider publication after rechecking. |
