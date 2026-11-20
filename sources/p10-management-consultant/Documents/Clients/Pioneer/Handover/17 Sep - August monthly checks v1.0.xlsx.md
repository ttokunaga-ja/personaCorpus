---
path: Documents/Clients/Pioneer/Handover/17 Sep - August monthly checks v1.0.xlsx
kind: holt-pioneer-monthly-checks.xlsx
author: Nicholas Hargreaves
derived_from:
---

# Sheet: Order timing
A2: August order timing — 14-row extract; A4: HRA-PIO-BUY-MCK-2026-001 v1.0; D4: 17 Sep 2026; F4: Nicholas Hargreaves.

| Row | A Index | B Register / PO | C First dispatch EDT | D Approved deadline EDT | E Decision | F Late | G Reason / next enquiry | H Evidence / zone |
|---|---:|---|---|---|---|---:|---|---|
| 7 | 1 | A001 / PO-84217 | 18 Aug 2026 15:08 | 18 Aug 2026 15:00 | Count | 1 | First send after deadline | EV-84217; EDT −04:00 |
| 8 | 2 | A002 / PO-84241 | 19 Aug 2026 15:00 | 19 Aug 2026 15:00 | Count | 0 | At deadline | EV-84241; EDT −04:00 |
| 9 | 3 | A003 / PO-84255 | 20 Aug 2026 (date only) | 20 Aug 2026 15:00 | Review |  | Same-day date only; ask Buying for first supplier dispatch history | EV-84255; EDT −04:00 |
| 10 | 4 | A004 / PO-84263 |  | 21 Aug 2026 15:00 | Exclude |  | Cancelled before dispatch | EV-84263; EDT −04:00 |
| 11 | 5 | A005 / PO-84279 | 24 Aug 2026 14:53 | 24 Aug 2026 15:00 | Count | 0 | First send | EV-84279-1; EDT −04:00 |
| 12 | 6 | A006 / PO-84279 | 24 Aug 2026 16:10 | 24 Aug 2026 15:00 | Exclude |  | Resend of A005 | EV-84279-2; EDT −04:00 |
| 13 | 7 | A007 / PO-84283 | 24 Aug 2026 13:20 | 24 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84283; EDT −04:00 |
| 14 | 8 | A008 / PO-84288 | 25 Aug 2026 15:12 | 25 Aug 2026 15:00 | Count | 1 | First send after deadline | EV-84288; EDT −04:00 |
| 15 | 9 | A009 / PO-84291 | 25 Aug 2026 14:11 | 25 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84291; EDT −04:00 |
| 16 | 10 | A010 / PO-84295 | 26 Aug 2026 10:30 | 26 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84295; EDT −04:00 |
| 17 | 11 | A011 / PO-84302 | 27 Aug 2026 14:05 | 27 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84302; EDT −04:00 |
| 18 | 12 | A012 / PO-84308 | 28 Aug 2026 12:40 | 28 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84308; EDT −04:00 |
| 19 | 13 | A013 / PO-84311 |  | 28 Aug 2026 15:00 | Review |  | Send time missing; ask Buying | EV-84311; EDT −04:00 |
| 20 | 14 | A014 / PO-84316 | 31 Aug 2026 14:50 | 31 Aug 2026 15:00 | Count | 0 | First send before deadline | EV-84316; EDT −04:00 |
| 21 | August extract |  |  |  | =COUNTIFS(E7:E20,"Count",F7:F20,0,F7:F20,"<>")+COUNTIFS(E7:E20,"Count",F7:F20,1) | =COUNTIFS(E7:E20,"Count",F7:F20,1) | =IF(E21=0,"",F21/E21) | Late / counted orders |

A22:H22: `="Excluded: "&COUNTIF(E7:E20,"Exclude")&"; Review: "&COUNTIF(E7:E20,"Review")&". Selected extract only; June 36/400 and August 21/420 are separate full-register aggregates. Compare Count entries to the judged total."` (evaluates to Excluded: 2; Review: 2). E21, F21 and G21 evaluate to 10, 2 and 20%. Compare `=COUNTIF(E7:E20,"Count")` with E21 before using the rate; a mismatch means a Count row lacks a confirmed late flag.

# Sheet: Savings checks
A2: August confirmed purchases — receipts and linked returns; A4: HRA-PIO-BUY-MCK-2026-001 v1.0; D4: 17 Sep 2026; F4: Nicholas Hargreaves.

| Row | A Index | B Group | C Baseline USD | D Net USD | E Saving USD | F Status | G Terms and evidence | H Follow-up |
|---|---:|---|---:|---:|---|---|---|---|
| 7 | 1 | S018 R-AUG-018-01–08 | 180000 | 168000 | =IF(AND(F7="Confirmed",ISNUMBER(C7),ISNUMBER(D7)),C7-D7,"") | Confirmed | T-AUG-018 rev 1; PC-AUG-018 | Linked returns; posted through 04 Sep |
| 8 | 2 | S042 R-AUG-042-01–06 | 160000 | 152000 | =IF(AND(F8="Confirmed",ISNUMBER(C8),ISNUMBER(D8)),C8-D8,"") | Confirmed | T-AUG-042 rev 1; PC-AUG-042 | Linked returns; posted through 04 Sep |
| 9 | 3 | S008 R-AUG-008-01–05 | 140000 | 133000 | =IF(AND(F9="Confirmed",ISNUMBER(C9),ISNUMBER(D9)),C9-D9,"") | Confirmed | T-AUG-008 rev 1; PC-AUG-008 | Linked returns; posted through 04 Sep |
| 10 | 4 | S036 R-AUG-036-01–04 | 200000 | 187000 | =IF(AND(F10="Confirmed",ISNUMBER(C10),ISNUMBER(D10)),C10-D10,"") | Confirmed | T-AUG-036 rev 1; PC-AUG-036 | Separate from held receipts 05–06 |
| 11 | 5 | S011 R-AUG-011-01–03 | 120000 | 112000 | =IF(AND(F11="Confirmed",ISNUMBER(C11),ISNUMBER(D11)),C11-D11,"") | Confirmed | T-AUG-011 rev 1; PC-AUG-011 | Separate from held receipts 04–05 |
| 12 | 6 | S036 R-AUG-036-05–06 |  |  |  | Review | INV-AUG-036-05–06; applicable approved version unconfirmed | Ask Meera / Buying for effective terms |
| 13 | 7 | S011 R-AUG-011-04–05 |  |  |  | Review | INV-AUG-011-04–05; August credit not established | Ask Buying for approval and Finance for posting |
| 14 | Confirmed total |  | =SUMIFS(C7:C13,F7:F13,"Confirmed",E7:E13,"<>") | =SUMIFS(D7:D13,F7:F13,"Confirmed",E7:E13,"<>") | =C14-D14 | =IF(C14=0,"",E14/C14) | Realised / verified basis |  |

A15:H15: USD 48,000 / USD 800,000 = 6.0% for five confirmed groups, August receipts and linked returns posted through 04 Sep; tax and freight excluded. Held groups are outside both numerator and basis.
