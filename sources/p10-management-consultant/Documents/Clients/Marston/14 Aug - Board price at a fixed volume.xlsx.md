---
path: Documents/Clients/Marston/14 Aug - Board price at a fixed volume.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---

## Sheet: Purchase lines

Source: Caroline Wood’s 06 Aug 2024 email, “Marston board costs and current work standard”, attachment “Marston board costs Jun-Jul 2024.csv”. June and July purchase lines for Trafford T2 and Warrington W1 under the same price contract only. The extract covers neither all T2 contracts nor all B420 fed through the T2 feeder. Purchase kg is the weight on the purchasing records.

| Excel row | A Month | B Site | C Board grade | D Purchase kg | E Net material GBP |
|---:|---|---|---|---:|---:|
| 2 | 2024-06 | Trafford T2 | B420 | 28000 | 17360 |
| 3 | 2024-06 | Warrington W1 | B420 | 32000 | 19840 |
| 4 | 2024-07 | Trafford T2 | B420 | 30000 | 21300 |
| 5 | 2024-07 | Warrington W1 | B420 | 34000 | 24140 |
| 6 | 2024-06 | Trafford T2 | B510 | 8000 | 5600 |
| 7 | 2024-06 | Warrington W1 | B510 | 6000 | 4200 |
| 8 | 2024-07 | Trafford T2 | B510 | 9000 | 6930 |
| 9 | 2024-07 | Warrington W1 | B510 | 7000 | 5390 |

## Sheet: Monthly rates

Board price at a fixed volume — Connor Atkinson, 14 Aug 2024, for Nicholas’s Wednesday review.

Sum purchase kg and net material GBP across the two sites within each month and grade. Divide net material GBP by purchase kg for the weighted rate.

| Excel row | A Board grade | B Month | C Purchase kg | D Net material GBP | E GBP/kg |
|---:|---|---|---|---|---|
| 2 | B420 | 2024-06 | =SUM('Purchase lines'!D2:D3) | =SUM('Purchase lines'!E2:E3) | =D2/C2 |
| 3 | B420 | 2024-07 | =SUM('Purchase lines'!D4:D5) | =SUM('Purchase lines'!E4:E5) | =D3/C3 |
| 4 | B510 | 2024-06 | =SUM('Purchase lines'!D6:D7) | =SUM('Purchase lines'!E6:E7) | =D4/C4 |
| 5 | B510 | 2024-07 | =SUM('Purchase lines'!D8:D9) | =SUM('Purchase lines'!E8:E9) | =D5/C5 |

The B420 rate checks back to each site’s purchase line: £0.62/kg in June and £0.71/kg in July at both sites. B510 is shown separately for reconciliation and is excluded from the sensitivity below.

## Sheet: Fixed volume

| Excel row | A Site | B Fixed B420 kg | C June GBP/kg | D July GBP/kg | E Change GBP/kg | F At June rate GBP | G At July rate GBP | H Price effect GBP |
|---:|---|---|---|---|---|---|---|---|
| 2 | Trafford T2 | ='Purchase lines'!D4 | ='Monthly rates'!E2 | ='Monthly rates'!E3 | =D2-C2 | =B2*C2 | =B2*D2 | =G2-F2 |
| 3 | Warrington W1 | =$B$2 | ='Monthly rates'!E2 | ='Monthly rates'!E3 | =D3-C3 | =B3*C3 | =B3*D3 | =G3-F3 |

Use Trafford’s July B420 purchase weight of 30,000 kg as the common comparison weight for each site. Warrington’s July B420 purchase line shows 34,000 kg. At the fixed weight the £0.09/kg rise moves the cost from £18,600 to £21,300, a £2,700 price effect for one plant. Use this to explain a change in price at constant weight. A year-on-year invoice comparison would require the full invoices for both years. A loss saving would require production weight evidence.
