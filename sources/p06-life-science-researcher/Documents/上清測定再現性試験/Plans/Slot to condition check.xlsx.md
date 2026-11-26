---
path: Documents/上清測定再現性試験/Plans/Slot to condition check.xlsx
kind: bennett-culture-day.xlsx
author: Anthony Mendoza
derived_from:
---

# Slot to condition check

For Robert Nguyen. I started with the six fully staffed Split slots in BCE-CE-2024-TBL-010-R01 and checked them in both directions against the Aug 6 Repeat set position map. Instrument: CE-Reader-2. Service times are Philadelphia EDT.

## Sheet: Schedule backcheck

A2: Slot to condition check  
B4: BCE-CE-2024-TBL-011-R00 · E4: Anthony Mendoza  
B5: Aug 9, 2024 · E5: Planned positions (count)

Columns A:I are listed in order below. Row 7 contains the headings. B and C hold date values. D and E hold integers displayed as two-digit positions.

| Row | A Slot | B Culture date | C Service date | D First position | E Last position | F Condition | G Expected positions | H Planned map rows | I Count check |
|---:|---|---|---|---:|---:|---|---:|---|---|
| 8 | T1 | 2024-09-09 | 2024-09-10 | 01 | 04 | Reference | 4 | Fill H8 formula below | Fill I8 formula below |
| 9 | T2 | 2024-09-09 | 2024-09-10 | 05 | 08 | Buffered | 4 | ↓ | ↓ |
| 10 | T3 | 2024-09-09 | 2024-09-10 | 09 | 12 | Supplemented | 4 | ↓ | ↓ |
| 11 | L1 | 2024-09-11 | 2024-09-12 | 01 | 04 | Reference | 4 | ↓ | ↓ |
| 12 | L2 | 2024-09-11 | 2024-09-12 | 05 | 08 | Buffered | 4 | ↓ | ↓ |
| 13 | L3 | 2024-09-11 | 2024-09-12 | 09 | 12 | Supplemented | 4 | ↓ | ↓ |

H8, filled through H13: `=COUNTIFS('Position map'!$A$8:$A$31,B8,'Position map'!$B$8:$B$31,C8,'Position map'!$C$8:$C$31,"CE-Reader-2",'Position map'!$E$8:$E$31,A8,'Position map'!$F$8:$F$31,F8,'Position map'!$G$8:$G$31,"Planned",'Position map'!$H$8:$H$31,">="&D8,'Position map'!$H$8:$H$31,"<="&E8)`

I8, filled through I13: `=IF(H8=G8,"Agrees","Check count")`

A15:D15 headings: Culture date | Planned positions matched | Required conditions with full count | Expected positions

| Row | A | B | C | D |
|---:|---|---|---|---|
| 16 | 2024-09-09 | `=SUMIF($B$8:$B$13,A16,$H$8:$H$13)` | Fill C16 formula below | `=SUMIF($B$8:$B$13,A16,$G$8:$G$13)` |
| 17 | 2024-09-11 | `=SUMIF($B$8:$B$13,A17,$H$8:$H$13)` | ↓ | `=SUMIF($B$8:$B$13,A17,$G$8:$G$13)` |

C16, filled through C17: `=--(COUNTIFS($B$8:$B$13,A16,$F$8:$F$13,"Reference",$H$8:$H$13,4)>0)+--(COUNTIFS($B$8:$B$13,A16,$F$8:$F$13,"Buffered",$H$8:$H$13,4)>0)+--(COUNTIFS($B$8:$B$13,A16,$F$8:$F$13,"Supplemented",$H$8:$H$13,4)>0)`

| Label cell | Label | Value cell | Formula | Expected |
|---|---|---|---|---:|
| A19 | Slots with four matched positions | B19 | `=COUNTIF(H8:H13,4)` | 6 |
| A20 | Distinct culture-date / position keys | B20 | `=SUM('Position map'!J8:J31)` | 24 |
| A21 | Unmatched map rows | B21 | `=COUNTIF('Position map'!I8:I31,0)` | 0 |
| A22 | Map rows matching multiple slots | B22 | `=COUNTIF('Position map'!I8:I31,">1")` | 0 |

Each of the six slot counts is 4 and agrees with the schedule. Each culture date has 12 matched positions out of 12 expected, with all three required conditions represented at four positions each.

## Sheet: Position map

A2: Aug 6 planned position map  
B4: BCE-CE-2024-TBL-011-R00 · E4: Anthony Mendoza  
B5: Aug 9, 2024 · E5: Planned positions (count)

Row 7, A:J: Culture date | Proposed service date | Reader | Position | Slot key | Condition | Status | Position number | Schedule matches | First key occurrence

A:G transcribe the Aug 6 TSV. A and B are date values. D preserves both characters. Put each position in its own row in the order shown.

| Rows | A Culture date | B Service date | C Reader | D Positions in row order | E Slot | F Condition | G Status |
|---|---|---|---|---|---|---|---|
| 8–11 | 2024-09-09 | 2024-09-10 | CE-Reader-2 | 01, 02, 03, 04 | T1 | Reference | Planned |
| 12–15 | 2024-09-09 | 2024-09-10 | CE-Reader-2 | 05, 06, 07, 08 | T2 | Buffered | Planned |
| 16–19 | 2024-09-09 | 2024-09-10 | CE-Reader-2 | 09, 10, 11, 12 | T3 | Supplemented | Planned |
| 20–23 | 2024-09-11 | 2024-09-12 | CE-Reader-2 | 01, 02, 03, 04 | L1 | Reference | Planned |
| 24–27 | 2024-09-11 | 2024-09-12 | CE-Reader-2 | 05, 06, 07, 08 | L2 | Buffered | Planned |
| 28–31 | 2024-09-11 | 2024-09-12 | CE-Reader-2 | 09, 10, 11, 12 | L3 | Supplemented | Planned |

Fill each formula through row 31.

- H8: `=VALUE(D8)`
- I8: `=IF(AND(C8="CE-Reader-2",G8="Planned"),COUNTIFS('Schedule backcheck'!$B$8:$B$13,A8,'Schedule backcheck'!$C$8:$C$13,B8,'Schedule backcheck'!$A$8:$A$13,E8,'Schedule backcheck'!$F$8:$F$13,F8,'Schedule backcheck'!$D$8:$D$13,"<="&H8,'Schedule backcheck'!$E$8:$E$13,">="&H8),0)`
- J8: `=--(COUNTIFS($A$8:A8,A8,$D$8:D8,D8)=1)`

Each map row matches one schedule slot. Each date-position pair occurs once.

## Sheet: Check note

A2: Position 05 lookup  
B4: BCE-CE-2024-TBL-011-R00 · E4: Anthony Mendoza  
B5: Aug 9, 2024 · E5: Planned matches (count)

| Cell | Entry |
|---|---|
| A7 | Position only |
| B7 | `=COUNTIF('Position map'!D8:D31,"05")` |
| A8 | 2024-09-09 / 05 |
| B8 | T2 · Buffered · Sep 10, 09:30–10:00 EDT |
| A9 | 2024-09-11 / 05 |
| B9 | L2 · Buffered · Sep 12, 13:30–14:00 EDT |
| A10 | Both cards reuse position 05. B7 returns two planned rows. Read the culture date with the two-character position before selecting the slot. |
| A11 | This check reconciles the planning records. Booking, actual receipt, measurement and authorization require their own records. Recheck the map if the proposed schedule changes. |
