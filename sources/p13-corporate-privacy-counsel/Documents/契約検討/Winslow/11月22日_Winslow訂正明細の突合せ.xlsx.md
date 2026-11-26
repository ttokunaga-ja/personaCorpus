---
path: Documents/契約検討/Winslow/11月22日_Winslow訂正明細の突合せ.xlsx
kind: toto-decision-comparison.xlsx
author: 高田 美奈子
derived_from:
---
# シート：訂正明細の突合せ

題：Winslow訂正明細の突合せ
文書番号：TDS-LG-202411-CMP-002
基準日：2024-11-22
作成者：高田 美奈子
確認者：村田 幸一／版：1.0
照合資料：WR-202411-SCH-001 Rev A・Rev B、WR-202411-FLW-001 Rev A

| セル行 | A No. | B 工程 | C Rev A 宛先 | D Rev B 宛先 | E 工程説明の対応 | F 記載差分 | G 照合結果 | H 注記 |
|---|---:|---|---|---|---|---|---|---|
| 7 | 1 | 取込対象の受領 | Image Intake | Image Intake | 取込の入口 | なし | 据置き | 受領先 |
| 8 | 2 | 画像取込 | Image Intake | Image Intake | Image Intake | なし | 据置き | 取込担当 |
| 9 | 3 | 通常引渡し | Winslow Quality Review | Winslow Quality Review | 今回案：取込→品質確認 | なし | 経路据置き | 補正を経由しない |
| 10 | 4 | Winslowの品質確認 | Winslow Quality Review | Winslow Quality Review | 品質確認 | なし | 据置き | 確認担当 |
| 11 | 5 | 取込不良の差戻し | Image Intake | Image Intake | 今回案：品質確認→取込 | なし | 経路据置き | 差戻し先 |
| 12 | 6 | 再取込完了後の返却 | 傾き補正担当 | Winslow Quality Review | 今回案：再取込後→品質確認 | あり | 宛先訂正 | 画像・指示票の補正担当への受渡しを除外 |
| 13 | 7 | 索引照合の参照 | 既存審査を参照 | 既存審査を参照 | 対象外 | なし | 据置き | 本明細で開始を求めない |

集計セル B14「照合行数」、C14 `=COUNT(A7:A13)`（7）。E14「記載差分件数」、F14 `=COUNTIF(F7:F13,"あり")`（1）。件数は明細の記載差分であり、承認件数ではない。

状態：訂正明細Rev B受領済み。第6行は11月15日の回答と一致。補正担当の事前承認は未了、本番データ提供・開始は保留。画像取込と索引照合の開始状態はこの照合の対象外。
