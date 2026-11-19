---
path: Documents/Clients/Whitfield/Diagnosis/12 Jul - Which queue to take first.xlsx
kind: holt-process-timings.xlsx
author: Connor Atkinson
derived_from:
---

# Sheet: Process timings

A2: Japan supplier payments — which queue to take first  
A4: HRA-WHI-DIAG-TIM-2024-005 v1.0 | D4: 12 Jul 2024 | F4: Connor Atkinson

| Row | A Step | B Activity | C Work (h) | D Waiting (h) | E Approval wait (h) | F Share of waiting | G Event class | H Basis |
|---:|---:|---|---:|---:|---:|---:|---|---|
| 7 | 07 | Approve cost centre | 4.0 | 340.0 | =D7 | =E7/D7 | First | Confirm invoice, match evidence and allocation at hand-off |
| 8 | 09 | Approve in Finance | 4.0 | 220.0 | =D8 | =E8/D8 | Second | Confirm cost-centre approval and closure of exceptions |
| 9 | 12 | Authorise bank release | 3.0 | 50.0 | =D9 | =E9/D9 | Retain control | Keep both Treasury checks before routing to authoriser |
| 10 | Total, three approval steps | | =SUM(C7:C9) | =SUM(D7:D9) | =SUM(E7:E9) | =E10/D10 | Three step rows only | |

These three rows contain the approval work and the ready-to-review waiting attributed to each approval step. The F column is the approval share of waiting on that row. It is distinct from both the share of the three queues below and the share of all waiting in the full route.

# Sheet: Priority

| Row | A Measure | B Queue hours | C Denominator (h) | D Share | E Reading |
|---:|---|---:|---:|---:|---|
| 2 | 07 and 09 combined | =SUM('Process timings'!E7:E8) | ='Process timings'!E10 | =B2/C2 | 560.0 of 610.0 approval-wait hours (91.8%) |
| 3 | 12 bank release authorisation | ='Process timings'!E9 | =C2 | =B3/C3 | 50.0 of 610.0 approval-wait hours |
| 4 | Combined queue coverage | =SUM(B2:B3) | =C2 | =B4/C4 | All three measured queues |
| 5 | Approval waiting / all waiting | =B4 | 1000.0 | =B5/C5 | 610.0 of 1,000.0 hours of waiting (61.0%) |

For Nicholas. Basis: HRA-WHI-DIAG-TIM-2024-001 v1.0, 26 Jun, 38 complete cases. Work and waiting are both expressed in hours. Start with the two hand-offs in rows 7–8 because they account for most of the observed approval wait. At 12, both checks at 11 and subsequent authorisation remain in force. The hours measure elapsed time across those cases. They do not estimate hours recoverable from any change.
