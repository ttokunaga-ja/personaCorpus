---
path: OneDrive - Holt/Team-room/28 Aug - Pioneer dispatch date check.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

# Sheet: Date trial
For Nicholas — 28 Aug 2026. This is a field-selection check, not a rate calculation. Times shown in EDT.

| Row | PO | Created | Last changed | Approved deadline | What the history must establish | Consequence of using the export field |
|---|---|---|---|---|---|---|
| 2 | PO-84217 | 18 Aug 2026 14:42 | 18 Aug 2026 15:08 | 18 Aug 2026 15:00 | Whether the 15:08 change is the first supplier dispatch | Created would appear on time even if the first dispatch is later than the deadline. |
| 3 | PO-84279 | 24 Aug 2026 14:48 | 24 Aug 2026 16:10 | 24 Aug 2026 15:00 | Find the first dispatch and distinguish it from a resend | Last changed may be a later resend and make an on-time first dispatch look late. |

The export does not identify the event behind either timestamp. Request the supplier dispatch history and deadline approval history from Buying. Until the event is verified, neither field is a defensible proxy for first dispatch; do not calculate a rate from this trial.
