---
path: Documents/上清測定再現性試験/Plans/Repeat coverage comparison.xlsx
kind: bennett-culture-day.xlsx
author: Melissa Watkins
derived_from:
---

# Sheet: Slots

A2: Repeat reservation coverage — slot check  
B4: BCE-CE-2024-TBL-010-R01  
E4: Melissa Watkins  
B5: Aug 2, 2024  
E5: CE-Reader-2 | Assignable positions

For Anthony Mendoza and Robert Nguyen. Slot keys, dates, times, conditions and proposed counts are carried forward from BCE-CE-2024-TBL-010-R00. Staff windows are from Lauren Brooks's Aug 2 coverage sheet, BCE-CE-2024-LAB-008-R00.

| Row | A: Slot key | B: Culture date | C: Service date | D: Start (EDT) | E: End (EDT) | F: Condition | G: Proposed positions | H: Staff from (EDT) | I: Staff until (EDT) | J: Full slot staffed | K: Assignable positions |
|---:|---|---|---|---|---|---|---:|---|---|---|---:|
| 8 | T1 | 2024-09-09 | 2024-09-10 | 09:00 | 09:30 | Reference | 4 | 08:45 | 10:45 | =AND(D8>=H8,E8<=I8,ROUND((E8-D8)*1440,0)=30) | =IF(J8,G8,0) |
| 9 | T2 | 2024-09-09 | 2024-09-10 | 09:30 | 10:00 | Buffered | 4 | 08:45 | 10:45 | =AND(D9>=H9,E9<=I9,ROUND((E9-D9)*1440,0)=30) | =IF(J9,G9,0) |
| 10 | T3 | 2024-09-09 | 2024-09-10 | 10:00 | 10:30 | Supplemented | 4 | 08:45 | 10:45 | =AND(D10>=H10,E10<=I10,ROUND((E10-D10)*1440,0)=30) | =IF(J10,G10,0) |
| 11 | E1 | 2024-09-11 | 2024-09-12 | 09:00 | 09:30 | Buffered | 4 | 10:00 | 10:45 | =AND(D11>=H11,E11<=I11,ROUND((E11-D11)*1440,0)=30) | =IF(J11,G11,0) |
| 12 | E2 | 2024-09-11 | 2024-09-12 | 09:30 | 10:00 | Supplemented | 4 | 10:00 | 10:45 | =AND(D12>=H12,E12<=I12,ROUND((E12-D12)*1440,0)=30) | =IF(J12,G12,0) |
| 13 | E3 | 2024-09-11 | 2024-09-12 | 10:00 | 10:30 | Reference | 4 | 10:00 | 10:45 | =AND(D13>=H13,E13<=I13,ROUND((E13-D13)*1440,0)=30) | =IF(J13,G13,0) |
| 14 | L1 | 2024-09-11 | 2024-09-12 | 13:00 | 13:30 | Reference | 4 | 12:45 | 14:45 | =AND(D14>=H14,E14<=I14,ROUND((E14-D14)*1440,0)=30) | =IF(J14,G14,0) |
| 15 | L2 | 2024-09-11 | 2024-09-12 | 13:30 | 14:00 | Buffered | 4 | 12:45 | 14:45 | =AND(D15>=H15,E15<=I15,ROUND((E15-D15)*1440,0)=30) | =IF(J15,G15,0) |
| 16 | L3 | 2024-09-11 | 2024-09-12 | 14:00 | 14:30 | Supplemented | 4 | 12:45 | 14:45 | =AND(D16>=H16,E16<=I16,ROUND((E16-D16)*1440,0)=30) | =IF(J16,G16,0) |

Take all four proposed positions only when Lauren can cover the full half-hour. On Thursday morning her standing handoff keeps her until 10:00. E1 (Buffered) and E2 (Supplemented) are uncovered. E2 ends at 10:00, just as Lauren becomes available. E3 (Reference) starts at 10:00 and is staffed through its 10:30 end. All three afternoon slots are staffed.

# Sheet: Options

A2: Assignable positions by option and culture date  
B4: BCE-CE-2024-TBL-010-R01  
E4: Melissa Watkins  
B5: Aug 2, 2024  
E5: CE-Reader-2 | Assignable positions

For Anthony Mendoza and Robert Nguyen. Compare the two Thursday alternatives against the same Tuesday visit.

| Row | A: Option | B: Culture date | C: Reference | D: Buffered | E: Supplemented | F: Total | G: Included slots |
|---:|---|---|---:|---:|---:|---:|---|
| 8 | Early | 2024-09-09 | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B8,Slots!$F$8:$F$10,C$7) | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B8,Slots!$F$8:$F$10,D$7) | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B8,Slots!$F$8:$F$10,E$7) | =SUM(C8:E8) | T1–T3 |
| 9 | Early | 2024-09-11 | =SUMIFS(Slots!$K$11:$K$13,Slots!$B$11:$B$13,$B9,Slots!$F$11:$F$13,C$7) | =SUMIFS(Slots!$K$11:$K$13,Slots!$B$11:$B$13,$B9,Slots!$F$11:$F$13,D$7) | =SUMIFS(Slots!$K$11:$K$13,Slots!$B$11:$B$13,$B9,Slots!$F$11:$F$13,E$7) | =SUM(C9:E9) | E1–E3 |
| 10 | Split | 2024-09-09 | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B10,Slots!$F$8:$F$10,C$7) | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B10,Slots!$F$8:$F$10,D$7) | =SUMIFS(Slots!$K$8:$K$10,Slots!$B$8:$B$10,$B10,Slots!$F$8:$F$10,E$7) | =SUM(C10:E10) | T1–T3 |
| 11 | Split | 2024-09-11 | =SUMIFS(Slots!$K$14:$K$16,Slots!$B$14:$B$16,$B11,Slots!$F$14:$F$16,C$7) | =SUMIFS(Slots!$K$14:$K$16,Slots!$B$14:$B$16,$B11,Slots!$F$14:$F$16,D$7) | =SUMIFS(Slots!$K$14:$K$16,Slots!$B$14:$B$16,$B11,Slots!$F$14:$F$16,E$7) | =SUM(C11:E11) | L1–L3 |
| 12 | Early total | | | | | =SUM(F8:F9) | Tuesday + Thursday morning |
| 13 | Split total | | | | | =SUM(F10:F11) | Tuesday + Thursday afternoon |

Early has 16 assignable positions. Tuesday has 4 Reference, 4 Buffered and 4 Supplemented. Thursday has 4 Reference, 0 Buffered and 0 Supplemented. Split has 24 assignable positions, with four in each condition on both days. Booking choice is open. Lauren's coverage confirmation establishes capacity only. Run authorization remains pending with Robert.

## セル指定（本文外）

両シートの管理欄は型の結合セルを使い、見出しは7行目に置く。日付と時刻は Excel の日付値と時刻値で入力する。Slots の追加列 H:K も印刷範囲に含める。両シートで型の印刷フッタを引き継ぐ。
