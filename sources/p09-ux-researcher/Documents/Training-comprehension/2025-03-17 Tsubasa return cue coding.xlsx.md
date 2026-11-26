---
path: Documents/Training-comprehension/2025-03-17 Tsubasa return cue coding.xlsx
kind: carver-field-coding.xlsx
author: Brian Chen
derived_from:
---
# Return cue coding

シート：Return cues  
B3:D3: CWR-TC-20250317-DEC-01  
F3: v1.0  
B4:C4: 2025-03-17  
E4:F4: Brian Chen

6行目の列は Scene、Jennifer · first pass、Brian · first pass、Agreed cue、First pass match、Observation behind the call。保存回答への帰着は本人が既入力を確かめた場面、再入力は冒頭の空欄から入力し直し観察中に保存回答を見つけなかった場面。

| 行 | A: Scene | B: Jennifer · first pass | C: Brian · first pass | D: Agreed cue | E: First pass match | F: Observation behind the call |
|---|---|---|---|---|---|---|
| 7 | TCS-01 | Saved answer | Saved answer | Saved answer | =IF(B7=C7,"Agree","Review") | Opened 取次ぎ／折り返し先 and confirmed the entry. |
| 8 | TCS-02 | Saved answer | Saved answer | Saved answer | =IF(B8=C8,"Agree","Review") | Passed 折り返し先 and confirmed the entry at 対応期限. |
| 9 | TCS-03 | Re-entry | Re-entry | Re-entry | =IF(B9=C9,"Agree","Review") | Re-entered 受付時刻; saved answer not found during search. |
| 10 | TCS-04 | Saved answer | Saved answer | Saved answer | =IF(B10=C10,"Agree","Review") | Opened 初動／受付時刻 and confirmed the entry. |
| 11 | TCS-05 | Re-entry | Saved answer | Saved answer | =IF(B11=C11,"Agree","Review") | Opened 初動 first but entered nothing; moved to 取次ぎ and confirmed 折り返し先. |
| 12 | TCS-06 | Re-entry | Re-entry | Re-entry | =IF(B12=C12,"Agree","Review") | Entered 受付時刻 again; saved answer not found during search. |
| 13 | =COUNTIF(E7:E12,"Agree") | First pass agree |  |  |  |  |
| 14 | =COUNTIF(E7:E12,"Review") | Review |  |  |  |  |
| 15 | =COUNTIF(D7:D12,"Saved answer") | Agreed: saved answer |  |  |  |  |
| 16 | =COUNTIF(D7:D12,"Re-entry") | Agreed: re-entry |  |  |  |  |

初回一致5、要照合1、確定は帰着4・再入力2。05の初回コードは変更せず、入力の有無と最終到達を照合して確定した。
