---
path: Documents/Clients/Whitfield/Diagnosis/26 Jun - Time by payment step.xlsx
kind: holt-process-timings.xlsx
author: Connor Atkinson
derived_from:
---

# Sheet: Process timings

| Cell | Value |
|---|---|
| A2 | Japan supplier payments — time by payment step |
| A4 | HRA-WHI-DIAG-TIM-2024-001 v1.0 |
| D4 | 26 Jun 2024 |
| F4 | Connor Atkinson |

| Excel row | A Step | B Process step | C Work (h) | D Wait (h) | E Approval (h) | F Share of wait | G Event class | H Evidence / condition |
|---:|---:|---|---:|---:|---:|---:|---|---|
| 7 | 01 | Receive invoice | 4.0 | 20.0 | 0.0 | =E7/D7 | Intake | Waiting to pick up a logged invoice |
| 8 | 02 | Check completeness | 4.0 | 20.0 | 0.0 | =E8/D8 | Completeness | Missing intake information |
| 9 | 03 | Enter invoice | 19.0 | 20.0 | 0.0 | =E9/D9 | Entry | Keying starts and ends within 03 |
| 10 | 04 | Match PO and receipt | 8.0 | 40.0 | 0.0 | =E10/D10 | Matching | Match and renewed match logged at 04 |
| 11 | 05 | Resolve exceptions | 4.0 | 80.0 | 0.0 | =E11/D11 | Correction | Time awaiting evidence while the item is back with AP |
| 12 | 06 | Route for cost-centre approval | 4.0 | 30.0 | 0.0 | =E12/D12 | Routing | Time before the complete package is sent |
| 13 | 07 | Approve cost centre | 4.0 | 340.0 | =D13 | =E13/D13 | Approval queue | Complete 06 routing to 07 review start |
| 14 | 08 | Review for Finance approval | 4.0 | 30.0 | 0.0 | =E14/D14 | Finance readiness | Check cost-centre approval and closure of exceptions |
| 15 | 09 | Approve in Finance | 4.0 | 220.0 | =D15 | =E15/D15 | Approval queue | Complete 08 routing to 09 review start |
| 16 | 10 | Build payment proposal | 4.0 | 80.0 | 0.0 | =E16/D16 | Proposal | Approved items awaiting inclusion in a proposal |
| 17 | 11 | Check payment run | 4.0 | 30.0 | 0.0 | =E17/D17 | Treasury checks | Review package becomes ready after both checks |
| 18 | 12 | Authorise bank release | 3.0 | 50.0 | =D18 | =E18/D18 | Approval queue | Checked proposal routed to 12 review start |
| 19 | 13 | Release payment | 5.0 | 40.0 | 0.0 | =E19/D19 | Execution | Awaiting release of an authorised payment |
| 20 | 14 | Reconcile settlement | 5.0 | 0.0 | 0.0 | =IF(D20=0,"",E20/D20) | Reconciliation | No separate wait attributed to 14 in the included records |
| 21 | Total · 38 payments | | =SUM(C7:C20) | =SUM(D7:D20) | =SUM(E7:E20) | =E21/D21 | 610.0 / 1,000.0 h of waiting | |
| 22 | For Nicholas Hargreaves and Neha Patel. Complete pairs for 38 domestic supplier payments received and reconciled 08 Apr–31 May, JST. 28 with no return and 10 with a return. | | | | | | | |
| 23 | Outside the totals: WB-0439 lacked the 07 review start and WB-0440 lacked the 09 review start. Their waiting times are unmeasured. | | | | | | | |
| 24 | Count approval waiting from complete routing to review start. Count the review itself as work. Work on returned items stays with the relevant activity. Waiting for evidence while the item is back for correction is other waiting. | | | | | | | |
| 25 | Hours are elapsed case-hours, including nights and holidays. Approval waiting is part of total waiting. Blank share at 14 reflects a zero denominator. | | | | | | | |

# Sheet: Reconciliation

| Row | A Measure | B Formula or value | C Basis |
|---:|---|---|---|
| 2 | Payments selected | 40 | 12 Jun pair check · HRA-WHI-DIAG-TIM-2024-003 |
| 3 | Review start missing | 2 | WB-0439 at 07 and WB-0440 at 09 |
| 4 | Included | =B2-B3 | All required events present |
| 5 | Included without a return | 28 | WB-0401 to WB-0428 |
| 6 | Included with a return | 10 | WB-0429 to WB-0438 |
| 7 | Difference against group count | =B4-SUM(B5:B6) | Expected zero |
| 8 | Recorded work (h) | ='Process timings'!C21 | 76.0 h |
| 9 | Invoice entry work (h) | ='Process timings'!C9 | 19.0 h, included in recorded work |
| 10 | All waiting (h) | ='Process timings'!D21 | 1,000.0 h |
| 11 | Three approval queues (h) | =SUM('Process timings'!E13,'Process timings'!E15,'Process timings'!E18) | 610.0 h |
| 12 | Difference against approval column (h) | =B11-'Process timings'!E21 | Expected zero |
| 13 | Other waiting (h) | =B10-B11 | 390.0 h |
| 14 | Receipt to completed settlement reconciliation (h) | =SUM(B8,B10) | 1,076.0 h across 38 cases |
| 15 | Approval share of all waiting | =B11/B10 | 61.0% of 1,000.0 waiting hours |
