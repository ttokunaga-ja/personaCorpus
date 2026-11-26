---
path: Desktop/14 Oct - Marston shift tally.xlsx
kind: Working sheet
author: Siobhan Barrett
derived_from:
---

# Marston shift tally
Siobhan Barrett · 14 Oct 2024 · For Connor Atkinson

## Sheet: Shift tally

The two shift columns in this scratch count use a worked input to check what happens when one box appears twice. No production tickets are included. Put headers in row 1 and the rows shown in rows 2–5.

| Measure | Outgoing evening | Following morning | Across entries | Counting basis |
|---|---:|---:|---:|---|
| Entries for the held box | 1 | 1 | =SUM(B2:C2) | Count of appearances of the same box, entered in both shift columns. |
| Entries showing a three-sheet check | 1 | 1 | =SUM(B3:C3) | Count of entries carrying the check, each for three sheets. The morning column carries forward the same box. |
| Entries with a blank approval time | 1 | 0 | =SUM(B4:C4) | Evening approval time is missing in this input. The morning entry has an approval time. |
| Entries with a blank checked weight | 0 | 0 | =SUM(B5:C5) | Both entries show a weight in this input. |

The total for the three-sheet check reads 2, although the two entries describe one changeover and one box. The blank approval time count tells us where to look in the shift columns. It cannot tell us whether the event still lacks an approval record after the two entries are connected. Keep this sheet out of the weekly return.

## Sheet: Event detail layout

| Field group | Keep in the detail | Connor's weekly check |
|---|---|---|
| Event key | Run reference and event reference. Preserve the event type and separate references for repeated events on the same run. | Check that one changeover contributes once. Treat a jam restart as a separate event on the same run. |
| Shift trail | Originating shift and source ticket; follow-up shift and source record if the box carries over. | Link the second shift's entry to the original key. Query entries whose event reference cannot be verified. |
| Three-sheet check | Number of sheets and where the check is recorded. | Count the event once once the evidence is linked. Keep the count open if the sheets checked cannot be confirmed. |
| Approval and weight | Approving lead and time, recorded weight in kg, and source for each. | Keep missing entries visible at their source. Carry forward verified follow-up under the same key and leave unresolved fields blank. |

Connor — I'd move the weekly return to the detail layout before adding the first week's tickets. The shift columns are handy for finding which shift wrote something down, but the subtotal will grow every time an open box is copied across.
