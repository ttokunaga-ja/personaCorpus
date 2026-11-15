---
persona: p02-site-reliability-engineer
period: 2024-04-01..2026-09-30
timezone: Europe/London
allowed_terms: scope of incident, batch processing, batch window
photo_dirs: Pictures
---

# p02 サイト信頼性エンジニア

## 人物

- **Owen Brackenholt**、Northstar Transit Cloud Wayglass 社の SRE。入社 6 年目、当番調整役は 3 年目。
- PC は Ubuntu 24.04 の会社支給ワークステーション。OS は en-US、文書・会議・個人メモの言語は英語。
- 運用対象は Checkout と Identity。朝に前夜の引継ぎを読み、障害対応後は翌営業日までに判断経緯を文章にする。
- 報告は UTC、勤務表は英国現地時刻で書き、時間帯を省かない。
- Documents はサービス別より障害日別を好む。Desktop に調査途中の図が残り、週末に結論が出た分だけ移す。

## 組織と関係者

- **Northstar Transit Cloud Wayglass Ltd**：英国の交通事業者向け予約基盤会社、従業員 135 人。Checkout は決済受付、Identity は乗客の認証機能。月間可用性目標は 99.9%。
- 社内：
  - **Priya Fenwicke**：運用部長。信頼性向上の予算と変更延期を承認。
  - **Miles Corrow**：Checkout 責任者。障害後の設計改善を担当。
  - **Tessa Wrenholt**：Identity 責任者。権限と当番教育を担当。
- 社外：
  - **Merewick Coach Cooperative** の **Jonas Pellcroft**：運行システム窓口。
  - **Emberquay Capacity Services** の **Ruth Ashcombe**：処理能力の調査担当。
- メールは `owen@wayglass-transit.example`、運行側は `jonas@merewick-coach.example`。サービス実機の接続情報は記録しない。

## 仕事の流れ

- **2024-04**：Priya と当番 8 人の負担を調査。夜間呼出し月 46 回のうち重複通知 18 回を整理し、引継ぎ票を統一した。
- **2024-07**：Jonas から朝の予約遅延の相談。Miles と 3 日分の集計表を比較し、ピークの開始時刻を 30 分前倒しして準備する運用を合意。
- **2024-10**：Tessa と Identity の復旧訓練。目標 20 分に対し 34 分だったため、承認待ち 12 分を分離し、代行責任者を定めた。
- **2025-01**：Priya に四半期の可用性 99.94% を報告。Checkout の 2 件の障害が停止時間の 81% を占め、改善対象を絞った。
- **2025-04**：Ruth の容量試算で繁忙期需要が平時の 2.3 倍と判明。上限 2 倍の旧案を却下し、余裕を含む 2.8 倍案を採用した。
- **2025-07**：Miles と変更後障害を振り返り。確認 14 項目中 3 項目が口頭のみだったため、実施結果の報告書を必須にした。
- **2025-10**：Jonas と返金遅延 62 件を照合。予約そのものの欠損は 0 件と確認し、返金処理の説明を利用者窓口へ渡した。
- **2026-01**：Tessa と当番訓練を再実施。復旧 18 分を達成し、旧 34 分の報告書を改善前の記録として残した。
- **2026-04**：Priya と SVC-17 の変更審査。残りのエラーバジェットが少なく、機能変更 4 件を 6 月まで延期した。
- **2026-07-13**：Miles と Checkout SVC-17 の障害 INC-260713 を評価。月間エラーバジェット残率 2.4% のため、復旧後も通常変更を凍結した。
- **2026-08**：Ruth と再発防止の容量確認を実施。ピーク時余裕 28% を確認し、Priya が限定した変更再開を承認した。
- **2026-09**：Jonas へ改善報告。Checkout の問い合わせが週 17 件から 6 件へ減り、次四半期は Identity の訓練を優先する結論となった。
- 障害報告には発生・検知・復旧時刻、影響範囲、推定と確定の区別を記す。時系列の追記と訂正理由も残す。

## フォルダ構成

```text
Desktop/                       進行中の障害メモ、今日の引継ぎ
Documents/
  Incidents/<年>/<発生日>/      影響整理、時系列、事後報告
  Service-reviews/Checkout/    可用性報告、SVC-17 の判断
  Service-reviews/Identity/    訓練報告、権限点検の結果
  On-call/                    当番表、教育資料、引継ぎ文書
  Capacity/                   需要予測、費用比較、承認
  Change-decisions/           変更の採否と凍結通知
  Customer-updates/           Merewick 向けの状況説明
  Reading-notes/              文献メモ、自分用の整理
Downloads/                     監視画面から出した集計表、受信添付
Pictures/
  Reliability-charts/         傾向図、画面のスクリーンショット
  Workshop-boards/            改善会議のボード写真
Documents/Mail/                障害連絡と承認メール
Documents/Shared-drive/        部内共有文書のオフライン控え
```

## 形式の目安

| 拡張子 | 割合 |
|---|---:|
| md | 23% |
| pdf（テキスト） | 17% |
| txt | 12% |
| eml | 10% |
| csv | 10% |
| png（監視画面・傾向図） | 9% |
| docx | 6% |
| xlsx | 5% |
| pptx | 4% |
| html | 3% |
| jpg（会議写真） | 1% |

txt は人が書いた時系列メモ、csv は集計結果。音声と zip は作らない。

## 規模の目安

全体で 2,000 件。最初の区切りは 300 件で、INC-260713、容量審査、当番訓練の改善前後を含める。
障害のない月も月次報告や引継ぎがあり、障害件数で文書数を水増ししない。

## 書かないもの

- ソースコード、ノートブック、SQL、設定ファイル、pcap、コマンド列、実行手順のコード断片。
- サーバーの生ログ、実 IP、監視接続先、認証情報、Git リポジトリ。
- 実在する乗客の予約・決済記録。運用手順は担当者の判断と確認を自然文で書く。
