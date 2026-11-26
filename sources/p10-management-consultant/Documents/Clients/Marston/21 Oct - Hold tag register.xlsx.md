---
path: Documents/Clients/Marston/21 Oct - Hold tag register.xlsx
kind: Working sheet
author: Siobhan Barrett
derived_from:
---

# Hold tag register
21 Oct 2024 · Siobhan Barrett · For Gareth Pritchard and Martin Devlin

## Sheet: Register

| Tag | Run | Event | Sending shift | Receiving lead | Approval date/time | Return signature | Actual return date/time | Status |
|---|---|---|---|---|---|---|---|---|
| | | | | | | | | =IF(A2="","",IF(AND(LEN(TRIM(G2))>0,ISNUMBER(H2)),"Closed","Open")) |

The header is on row 1. Leave A2:H51 empty for entries. Fill I2 down to I51 with relative references. Enter approval and actual return as local Excel date/time values including the date.

Keep one row per box. At handover, leave the tag attached and tell the incoming lead which box is still on the shelf. Record the incoming lead in column E when they take custody. Use the original run and event when you update the same row on the next shift.

Match the tag and all three sheets to the job instruction before approving. Keep the box on the hold shelf and raise it with the line coordinator if the tag is absent, the specification differs or approval is outstanding. At the actual return, the receiving lead signs and records the time in columns G and H. Filter Status to Open at handover. Count only Closed rows as recorded returns. The status records completion of the return fields. The lead makes the release decision under MCP-OPS-STD-2024-014 Rev 3.

## Sheet: Example

| Tag | Run | Event | Sending shift | Receiving lead | Approval date/time | Return signature | Actual return date/time | Status |
|---|---|---|---|---|---|---|---|---|
| EX-01 | Example run | Changeover first-off, 3 sheets | 20 Oct evening, Julie Naylor | Gareth Pritchard | 21 Oct 2024 08:10 | Gareth Pritchard | 21 Oct 2024 08:15 | Closed |

At 08:10 the box is still open. Gareth completes the two return fields when he moves the sheets at 08:15. Keep this practice entry on the Example sheet.
