---
path: Documents/Clients/Denton/28 Jan - Dec close pack timestamp check.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# December close: which date did the pack export give us?

For Nicholas · 28 Jan 2026 · Times GMT. The 26 Jan agreement counts from the first working day after month end through Angela's approval, inclusive. The task list records a pack assembly, not an approval. The calendar gives 14 Jan as the approval date; the approval history and time are still to be checked.

## Sheet: Task list excerpt

December close task list, “Close pack” section, entry “Assemble pack for export”. Connor's transcription of the relevant entry from the task list; the list's assembly date has no time in the entry.

| List section | Task entry | Date shown | Time shown | Status in list |
|---|---|---|---|---|
| Close pack | Assemble pack for export | 13 Jan 2026 | Not recorded | Complete |

## Sheet: Endpoint candidates

| Candidate | Date | Time GMT | Source and specific entry | Status for agreed comparison | Reason / next check |
|---|---|---|---|---|---|
| Pack assembly | 13 Jan 2026 | Not recorded | Task list excerpt, Close pack / Assemble pack for export | Exclude | Assembly is not Angela's approval. Find the Finance approval history. |
| Approval date | 14 Jan 2026 | Not yet checked | DPF-FIN-CAL-2026-001 v1.0, December actual approval | Provisional | Calendar gives date; obtain approval history and time before confirming. |

## Sheet: Count check

| Candidate end date | Start date | Inclusive working days | Use |
|---|---|---|---|
| 13 Jan 2026 | 02 Jan 2026 | =NETWORKDAYS(B2,A2) | 8; assembly only, excluded |
| 14 Jan 2026 | 02 Jan 2026 | =NETWORKDAYS(B3,A3) | 9; provisional calendar date |

There are no England and Wales public holidays in the counted interval. Nicholas: the eight-day figure came from taking the task-list assembly date as the finish. It is not the agreed endpoint. Please keep the approval date provisional until Angela's underlying record is available.
