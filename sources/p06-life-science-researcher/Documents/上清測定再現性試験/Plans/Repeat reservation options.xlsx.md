---
path: Documents/上清測定再現性試験/Plans/Repeat reservation options.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---
# Sheet: Slots

A2: Repeat reservation options — proposed slots
B4: BCE-CE-2024-TBL-010-R00
E4: Melissa Watkins
B5: Aug 2, 2024
E5: CE-Reader-2 | Nominal positions per slot

For Robert Nguyen. Staff coverage awaiting Lauren Brooks's confirmation.

| Row | A: Slot key | B: Culture date | C: Service date | D: Start (EDT) | E: End (EDT) | F: Condition | G: Proposed positions |
|---:|---|---|---|---|---|---|---:|
| 8 | T1 | Sep 9, 2024 | Sep 10, 2024 | 09:00 | 09:30 | Reference | 4 |
| 9 | T2 | Sep 9, 2024 | Sep 10, 2024 | 09:30 | 10:00 | Buffered | 4 |
| 10 | T3 | Sep 9, 2024 | Sep 10, 2024 | 10:00 | 10:30 | Supplemented | 4 |
| 11 | E1 | Sep 11, 2024 | Sep 12, 2024 | 09:00 | 09:30 | Buffered | 4 |
| 12 | E2 | Sep 11, 2024 | Sep 12, 2024 | 09:30 | 10:00 | Supplemented | 4 |
| 13 | E3 | Sep 11, 2024 | Sep 12, 2024 | 10:00 | 10:30 | Reference | 4 |
| 14 | L1 | Sep 11, 2024 | Sep 12, 2024 | 13:00 | 13:30 | Reference | 4 |
| 15 | L2 | Sep 11, 2024 | Sep 12, 2024 | 13:30 | 14:00 | Buffered | 4 |
| 16 | L3 | Sep 11, 2024 | Sep 12, 2024 | 14:00 | 14:30 | Supplemented | 4 |

Reader windows from Lauren's Aug 1 diary check (BCE-CE-2024-TBL-009-R00). On Thursday morning I put Reference last so it can follow Lauren's standing handoff. The start and end of her available time still need confirmation.

# Sheet: Options

A2: Repeat reservation options — Early and Split
B4: BCE-CE-2024-TBL-010-R00
E4: Melissa Watkins
B5: Aug 2, 2024
E5: CE-Reader-2 | Nominal positions per set

For Robert Nguyen.

| Row | A: Option | B: Culture date | C: Service date | D: Reader window (EDT) | E: Slot keys | F: Nominal positions | G: Staff coverage |
|---:|---|---|---|---|---|---:|---|
| 8 | Early | Sep 9, 2024 | Sep 10, 2024 | 09:00–10:30 | T1–T3 | =SUM(Slots!G8:G10) | Awaiting Lauren |
| 9 | Early | Sep 11, 2024 | Sep 12, 2024 | 09:00–10:30 | E1–E3 | =SUM(Slots!G11:G13) | Awaiting Lauren |
| 10 | Split | Sep 9, 2024 | Sep 10, 2024 | 09:00–10:30 | T1–T3 | =SUM(Slots!G8:G10) | Awaiting Lauren |
| 11 | Split | Sep 11, 2024 | Sep 12, 2024 | 13:00–14:30 | L1–L3 | =SUM(Slots!G14:G16) | Awaiting Lauren |

Choose one Thursday alternative. Tuesday is the same visit in either proposal. These counts assume every half-hour is covered end to end. Check that assumption with Lauren before choosing a hold. Run authorization remains pending.

## 印刷時の指定（本文外）

Slots を1頁目、Options を2頁目とし、各シートを1ページに収める。両シートに型の印刷フッタを継承する。左は `LABORATORY RECORD`、右は `Page &P / &N`。
