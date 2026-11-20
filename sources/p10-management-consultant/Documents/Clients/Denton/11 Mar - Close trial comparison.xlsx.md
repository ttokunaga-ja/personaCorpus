---
path: Documents/Clients/Denton/11 Mar - Close trial comparison.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# December and February on the same clock
For Nicholas Hargreaves and Angela Wilkins | 11 Mar 2026 | All times GMT

D1 is the first working day after month end; include Angela's approval day. Saturdays, Sundays and England and Wales bank holidays are excluded. There are no such bank holidays within either displayed interval. Receipt, reconciliation, circulation and approval are distinct events.

## Sheet: Periods
Dates in B and C are Excel dates; D is calculated from them using NETWORKDAYS.

| Period | Start | Approval / target date | Working days | Basis |
|---|---|---|---|---|
| December actual | 2026-01-02 | 2026-01-14 | =NETWORKDAYS(B2,C2) | December close - Finance event extract.csv, DEC-PACK-APPROVAL at 14:36; calendar DPF-FIN-CAL-2026-001 v1.0 recorded dates |
| February trial actual | 2026-03-02 | 2026-03-10 | =NETWORKDAYS(B3,C3) | February trial - Close events.csv, close pack approval at 14:48 |
| February target | 2026-03-02 | 2026-03-09 | =NETWORKDAYS(B4,C4) | DPF-FIN-CAL-2026-001 v1.0, planned D6 approval |
| Days shorter |  |  | =D2-D3 | Previous actual less trial actual: 2 |
| Days above target |  |  | =D3-D4 | Trial actual less target: 1 |

## Sheet: Daily progress
Row 1 is the header. Dates in A and E are Excel dates; B and F calculate inclusive working-day numbers with fixed period start cells. Empty February cells on the last two rows are empty cells, not zeroes. Event references name rows in the respective source CSVs.

| December date | Day | Events, GMT | December evidence | February date | Day | Events, GMT | February evidence |
|---|---|---|---|---|---|---|---|
| 2026-01-02 | =NETWORKDAYS(Periods!$B$2,A2) | First working day | Calendar recorded start | 2026-03-02 | =NETWORKDAYS(Periods!$B$3,E2) | First working day | Calendar D1 |
| 2026-01-05 | =NETWORKDAYS(Periods!$B$2,A3) |  |  | 2026-03-03 | =NETWORKDAYS(Periods!$B$3,E3) |  |  |
| 2026-01-06 | =NETWORKDAYS(Periods!$B$2,A4) |  |  | 2026-03-04 | =NETWORKDAYS(Periods!$B$3,E4) |  |  |
| 2026-01-07 | =NETWORKDAYS(Periods!$B$2,A5) |  |  | 2026-03-05 | =NETWORKDAYS(Periods!$B$3,E5) |  |  |
| 2026-01-08 | =NETWORKDAYS(Periods!$B$2,A6) |  |  | 2026-03-06 | =NETWORKDAYS(Periods!$B$3,E6) | 15:42 supplier statement received, after 15:00 window | Close events: supplier statement received; calendar D5 |
| 2026-01-09 | =NETWORKDAYS(Periods!$B$2,A7) |  |  | 2026-03-09 | =NETWORKDAYS(Periods!$B$3,E7) | 09:18 unsigned GRNI held; 10:25 supplier reconciliation complete; 11:26 signed replacement usable; 13:10 stock reconciliation complete | Close events: supplier completion and all three stock cutoff rows; calendar D6 11:00 circulation target |
| 2026-01-12 | =NETWORKDAYS(Periods!$B$2,A8) | 15:49 supplier statement received | DEC-AP-STATEMENT | 2026-03-10 | =NETWORKDAYS(Periods!$B$3,E8) | 10:37 pack circulated; 14:48 Angela approved | Close events: pack circulation and approval rows |
| 2026-01-13 | =NETWORKDAYS(Periods!$B$2,A9) | 10:19 supplier reconciliation; 11:28 stock evidence received; 13:08 stock reconciliation | DEC-AP-RECON; DEC-STOCK-GRNI; DEC-STOCK-RECON |  |  |  |  |
| 2026-01-14 | =NETWORKDAYS(Periods!$B$2,A10) | 10:42 pack circulated; 14:36 Angela approved | DEC-PACK-CIRCULATION; DEC-PACK-APPROVAL |  |  |  |  |

The December event extract does not record pack assembly; 13 January is not the approval endpoint. The result is nine working days against seven for the trial, with six as the management target.
