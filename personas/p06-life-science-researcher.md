---
persona: p06-life-science-researcher
period: 2024-04-01..2026-09-30
timezone: America/New_York
allowed_terms: assay batch, batch number, synthetic medium, scope of study
photo_dirs: Pictures
---

# p06 生命科学研究者

## 人物

- **Mara Ellowick**、Orchid Cellworks Thistrel 社の細胞評価研究員。入社 7 年目、試験計画の責任者は 2 年目。
- PC は Windows 11 の実験室ワークステーション。OS は en-US、使用言語は英語。実験記録、共同検討、申請原稿も英語。
- Study Alpha は培養上清の測定再現性、Study Beta は保存時間の影響を調べる。人を対象とする研究は行わない。
- 実験前に計画と採否基準を確認し、測定後は外れ値を消す前に原因を記録する。紙の作業票を週末に読み直す。
- Documents は試験名と測定日、Pictures は装置と培養の写真。机上に残した注記入りスキャンを後から清書版と照合する癖がある。

## 組織と関係者

- **Orchid Cellworks Thistrel Inc**：米国の培養評価受託会社、従業員 58 人。研究室内の非臨床細胞材料を使い、測定方法と培地条件を検討する。
- 社内：
  - **Simon Larkmere**：研究室長。試験計画と追加費用を承認。
  - **Eva Cressholt**：実験担当。培養と試料の受渡しを行う。
  - **Dorian Valecrest**：品質担当。逸脱と校正記録を確認。
- 社外：
  - **Brindlequay Assay Services** の **Celia Hartmoss**：測定法の相互比較担当。
  - **Nettleford Instrument Care** の **Peter Fenrose**：装置の点検と校正窓口。
- メールは `mara@thistrel-cellworks.example`、点検は `peter@nettleford-care.example`。試料識別はこの組織内の業務番号だけを使う。

## 仕事の流れ

- **2024-04**：Simon と Alpha の計画を承認。培地 3 条件を各 8 試料で比べ、変動係数 15% 以下を受入基準とした。
- **2024-07**：Eva の測定 24 試料中 4 試料で基準超過。Mara は全体平均だけの報告をやめ、培養日別に分けて再試験を提案した。
- **2024-10**：Peter が装置の校正偏差 +0.12 ng/mL を確認。Dorian と影響する測定 6 日分を洗い出し、対象結果を保留した。
- **2025-01**：Celia と相互比較 18 試料を実施。施設間差 8.2% で基準 10% 以内となり、計画継続を承認された。
- **2025-04**：Eva と Beta を開始。保存 0・24・48 時間の 3 群で各 12 試料とし、Alpha の材料条件と区別した。
- **2025-07**：Dorian が Beta のラベル誤記 2 件を指摘。値は修正せず対象試料を解析から外し、逸脱報告に判断理由を残した。
- **2025-10**：Simon に保存試験を報告。48 時間群は基準比 18% 低下し、実務上の保存上限を 24 時間にした。
- **2026-01**：Peter と点検周期を評価。直近 4 回の校正は範囲内だが使用時間が 22% 増えたため、半年周期を維持した。
- **2026-04**：Celia と Alpha の改訂法を比較。試料 30 件の差が平均 0.3 ng/mL となり、低濃度域だけ追加確認する結論になった。
- **2026-07-13**：Eva と Alpha の試料群 A-24 を測定。対象 24 試料、平均 6.8 ng/mL、変動係数 9.4% を Dorian が確認し、再現性基準内と報告した。
- **2026-08**：Simon が次期資金申請を審査。追加試験費 USD 38,000 のうち装置増設は見送り、低濃度評価へ USD 24,000 を認めた。
- **2026-09**：Celia への共同報告の図 7 枚を点検。除外 2 試料の注記漏れを直し、結論を保存 24 時間以内に限定した。
- 実験の順序、単位、試料数、除外理由を一緒に残す。紙の作業票と清書の両方がある場合は日付で対応を示す。

## フォルダ構成

```text
Desktop/                       当日の作業票、確認待ちの図
Documents/
  Alpha/Plans/                試験計画、改訂理由、承認
  Alpha/A-24/                 測定表、品質確認、結果報告
  Alpha/Method-comparison/    相互比較と低濃度域の検討
  Beta/Storage-study/         保存条件、結果表、考察
  Lab-book/<年>/<月>/          作業記録、紙の実験ノートのスキャン
  Quality/Deviations/          逸脱報告と採否の判断
  Equipment/Calibration/      点検証明、校正の評価
  Funding/                    申請原稿、予算表、室長の回答
  Manuscript/                 図表、原稿、指摘への回答
Downloads/                     装置の表出力、点検業者の添付
Pictures/
  Cultures/                   培養状態の写真
  Instrument-checks/          装置写真、確認画面
Documents/Lab-share/           共有会議の資料と議事録
Documents/Correspondence/      共同検討と点検依頼の eml
```

## 形式の目安

| 拡張子 | 割合 |
|---|---:|
| pdf（テキスト） | 19% |
| xlsx | 16% |
| csv | 15% |
| docx | 13% |
| pdf（スキャン） | 11% |
| png（測定図・装置画面） | 7% |
| pptx | 6% |
| jpg（培養・装置写真） | 5% |
| eml | 4% |
| tsv | 2% |
| txt | 2% |

装置出力は表として読める測定値だけとする。音声と zip は作らない。

## 規模の目安

全体で 2,000 件。最初の区切りは 300 件で、A-24、Beta 保存試験、逸脱と校正の記録を中心にする。
試料 1 件ごとの不要な文書分割は行わない。

## 書かないもの

- ソースコード、ノートブック、SQL、設定ファイル、pcap、装置専用バイナリ。
- 実在の細胞株・試料の管理情報、患者情報、実験室へのアクセス情報。
- 実在論文の転載、認証情報、危険な実験の具体的操作手順。説明は非臨床の方法評価と結果に限る。
