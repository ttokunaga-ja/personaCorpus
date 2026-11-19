---
path: Documents/Clients/Whitfield/Diagnosis/10 May - Mailbox dates first pass.xlsx
kind: holt-process-timings.xlsx
author: Connor Atkinson
derived_from:
---
# Sheet: Process timings

A2: Mailbox dates — first pass, four cases
A4: HRA-WHI-DIAG-TIM-2024-002 v0.1 | D4: 10 May 2024 | F4: Connor Atkinson

JST throughout. The available points are initial receipt and approval completions, not review starts. The interval from receipt to each completion is an upper-bound candidate, not a measured approval queue. It can include other process steps, correction and the approval work itself.

| Row | A | B Case | C Receipt JST | D 07 complete JST | E 09 complete JST | F 12 complete JST | G Settlement JST | H Note |
|---:|---:|---|---|---|---|---|---|---|
| 7 | 1 | WB-0401 | 08 Apr 2024 08:00 | 08 Apr 2024 19:00 | 09 Apr 2024 14:00 | 10 Apr 2024 09:00 | 10 Apr 2024 15:00 | Receipt to completion only |
| 8 | 2 | WB-0402 | 09 Apr 2024 09:00 | 10 Apr 2024 10:00 | 11 Apr 2024 11:00 | 12 Apr 2024 09:00 | 12 Apr 2024 11:00 | Receipt to completion only |
| 9 | 3 | WB-0403 | 10 Apr 2024 08:30 | 12 Apr 2024 13:00 | 15 Apr 2024 10:00 | 16 Apr 2024 09:00 | 16 Apr 2024 12:00 | Receipt to completion only |
| 10 | 4 | WB-0404 | 11 Apr 2024 10:00 | 12 Apr 2024 11:00 | 15 Apr 2024 12:00 | 16 Apr 2024 10:00 | 16 Apr 2024 15:00 | Receipt to completion only |

## Candidate intervals, hours

| Row | A Case | B Receipt to 07 complete | C Receipt to 09 complete | D Receipt to 12 complete | E 07 approval wait | F 09 approval wait | G 12 approval wait | H Interpretation |
|---:|---|---|---|---|---|---|---|---|
| 14 | WB-0401 | =(D7-C7)*24 | =(E7-C7)*24 | =(F7-C7)*24 | | | | Review starts unavailable |
| 15 | WB-0402 | =(D8-C8)*24 | =(E8-C8)*24 | =(F8-C8)*24 | | | | Review starts unavailable |
| 16 | WB-0403 | =(D9-C9)*24 | =(E9-C9)*24 | =(F9-C9)*24 | | | | Review starts unavailable |
| 17 | WB-0404 | =(D10-C10)*24 | =(E10-C10)*24 | =(F10-C10)*24 | | | | Review starts unavailable |

I previously labelled these receipt-to-completion values “Approval wait”. That heading was wrong: each interval includes approval work and may contain time before the item was ready. The three approval-wait columns remain blank until complete hand-off and review-start pairs can be identified. None of these candidates is suitable for an approval-wait total.
