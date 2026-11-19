---
path: Documents/Clients/Marston/21 Aug - Scrap spend as a yield proxy v0.1.xlsx
kind: Working sheet
author: Connor Atkinson
modified_by: Connor Atkinson
created: 2024-08-21 14:20
derived_from:
---

## Sheet: Scrap proxy

Scrap spend as a yield proxy, v0.1 — Connor Atkinson, for Nicholas Hargreaves, 21 Aug 2024.

Question for review: If both months had exactly 3,600 kg of B420 scrap, what quantity would we infer from the charge if the planned rate stayed at June’s £0.62/kg? Value each month’s assumed weight at its purchase rate from “14 Aug - Board price at a fixed volume”.

| Excel row | A Period | B Assumed scrap kg | C Purchase rate GBP/kg | D Scrap charge GBP | E Planned rate GBP/kg | F Implied scrap kg at planned rate |
|---:|---|---|---|---|---|---|
| 2 | June | 3600 | 0.62 | =B2*C2 | 0.62 | =D2/E2 |
| 3 | July | 3600 | 0.71 | =B3*C3 | =E2 | =D3/E3 |
| 4 | July less June | =B3-B2 | =C3-C2 | =D3-D2 | | =F3-F2 |

Same assumed 3,600 kg in each month, valued at £2,232 in June and £2,556 in July. The £324 increase is entirely from price in this illustration. At the unchanged planned rate, the July charge would read as about 4,123 kg, an apparent rise of about 523 kg.

## Working note added 21 Aug

Drop the spend proxy for estimating quantity. The exercise shows that a price change alone can produce an apparent deterioration. Get input and good-output weights for matched runs and calculate yield directly. Keep this worked example for explaining why we did not use the scrap charge to estimate tonnes.
