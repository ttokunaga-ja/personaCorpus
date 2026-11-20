---
path: Documents/Clients/Marston/23 Sep - Changeover and restart losses.xlsx
kind: Working sheet
author: Deepa Desai
derived_from:
---

# Loss buckets after the walk-through
Deepa Desai · For Connor Atkinson and Nicholas Hargreaves · 23 Sep 2024

## Sheet: Reconciliation

July B420 / S3 only. Input and good weights are from Oliver's cleared run pairs, 12 at each site. For a combined setup entry, match the timestamped July weight entries to the changeover and restart event times, then allocate each weight once to its event window. The September walkthrough guides where to look in the records. The two September restart observations are outside this table.

| Row | A Measure | B Trafford T2 kg | C Warrington W1 kg | D Trafford less Warrington kg | E Source or check |
|---|---|---:|---:|---:|---|
| 2 | Changeover to first-off approval | 1020 | 600 | =B2-C2 | July setup and approval entries matched to the weight log. End at first-off approval and remove weight assigned to a jam restart. |
| 3 | Jam restart to approval | 780 | 480 | =B3-C3 | July restart times and approval entries matched to the weight log. Include restart weight traced out of a combined setup entry only here. |
| 4 | Other loss | 1800 | 1320 | =B4-C4 | July losses outside the two event windows. Keep outside the standardisation case. |
| 5 | Classified loss | =SUM(B2:B4) | =SUM(C2:C4) | =B5-C5 | All three buckets. |
| 6 | Input | 30000 | 30000 | =B6-C6 | Cleared July input tickets. |
| 7 | Good output | 26400 | 27600 | =B7-C7 | Good weight on the matched July output records. |
| 8 | Input less good | =B6-B7 | =C6-C7 | =B8-C8 | Independent weight balance from the run pairs. |
| 9 | Unreconciled difference | =B8-B5 | =C8-C5 | =D8-D5 | Expect zero in each plant column before using the difference. |
| 10 | Gap in the two event windows | | | =SUM(D2:D3) | 720 kg to carry to Connor's comparison as the amount to test. |
| 11 | Gap outside the two windows | | | =D8-D10 | 480 kg of the 1,200 kg difference remains outside the hypothesis. |

If a combined entry cannot be traced to the underlying weights, flag the allocation for the plant to resolve. Moving weight from setup to restart changes the two subtotals and leaves the run's total loss unchanged.
