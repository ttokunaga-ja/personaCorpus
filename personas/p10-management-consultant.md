---
persona: p10-management-consultant
period: 2024-04-01..2026-09-30
timezone: Europe/London
allowed_terms: scope of work, engagement scope, scope of analysis
photo_dirs: Pictures
---

# p10 経営コンサルタント

## 人物

- **Julian Castermere**、Summit Vale Orielspan Advisory のマネージャー。入社 8 年目、財務業務改革の案件責任者は 3 年目。
- PC は Windows 11 の会社支給ノート。VDI から持ち出しを認められた文書をローカルで読む。OS は en-US、使用言語は英語。
- 月曜に顧客と進捗を確認し、水曜にチームの分析をレビュー、金曜に意思決定用の資料へまとめる。
- 面談の発言と分析上の推定を分け、費用削減額は実現時期と一時費用を併記する。
- Documents は顧客別、Desktop は次の会議用。顧客の赤字入り資料や不採用の提案も、日付と版を付けて残す。

## 組織と関係者

- **Summit Vale Orielspan Advisory Ltd**：英国の業務改革助言会社、従業員 74 人。Client Alpha と Client Beta の財務・調達改革を担当する。
- 社内：
  - **Vivian Mosselwick**：パートナー。契約と最終提言を承認。
  - **Ethan Quillborne**：アナリスト。原価と処理時間の分析。
  - **Sonia Reedcroft**：業務設計担当。面談と改善案の検討。
- 社外：
  - **Alderfenna Components Ltd**（Client Alpha）の **Gareth Holmwick**：CFO、改革の依頼主。
  - **Brackensail Distribution Inc**（Client Beta）の **Mina Solmere**：調達部長。
- メールは `julian@orielspan-advisory.example`、Alpha は `gareth@alderfenna-components.example`。Alpha の案件予算は顧客の日本事業分として円で管理する。

## 仕事の流れ

- **2024-04**：Gareth と Alpha の予備調査を開始。支払業務 14 工程を確認し、システム更新より承認待ちの短縮を優先した。
- **2024-07**：Sonia が 18 人に面談。入力の遅さという経営仮説に対し、滞留時間の 61% が承認待ちと分かり、論点を変えた。
- **2024-10**：Ethan と費用効果を試算。年 2,600 万円の削減見込から二重計上 400 万円を除き、2,200 万円へ訂正した。
- **2025-01**：Vivian が提案を審査。実現まで 9 か月、初期費用 1,800 万円と明記し、初年度効果の過大表現を削った。
- **2025-04**：Mina と Beta の調達診断を開始。仕入先 42 社のうち取引額上位 12 社を対象にし、全社再交渉案を見送った。
- **2025-07**：Sonia と Beta の例外申請 93 件を分類。緊急発注 37 件の多くは計画遅れであり、価格より需要共有を先に提案した。
- **2025-10**：Gareth に Alpha 試行の結果を報告。承認日数は平均 6.2 日から 3.8 日へ短縮し、対象を 2 部門から 5 部門に広げた。
- **2026-01**：Mina と Beta の削減効果を確認。見込 8.1% に対し実績 5.6% で、数量条件未達の影響を別表で説明した。
- **2026-04**：Vivian と Alpha の次期契約を審査。日本事業の Finance 領域に限定し、販売業務を除外すると合意した。
- **2026-07-13**：Gareth と Client Alpha Phase 1 の計画を確定。Finance 領域の予算 4,280 万円、期間 12 週として、体制と成果物の承認を得た。
- **2026-08**：Ethan が Alpha の追加要求 7 件を見積もり。追加 620 万円が必要なため、2 件だけを既存範囲に含め、5 件は保留した。
- **2026-09**：Mina に Beta 最終報告を提出。削減実績 6.0%、発注遅延率 9% から 5% を示し、月次点検を顧客側へ引き継いだ。
- 面談録、作業仮説、顧客の指摘、提言の改訂理由を保存する。削減見込と実現額は同じ列に混ぜない。

## フォルダ構成

```text
Desktop/                       次回の運営会議、直前の修正
Documents/
  Clients/Alderfenna/Diagnosis/ 面談、工程整理、仮説の見直し
  Clients/Alderfenna/Finance/  予算、工程、試行結果、提言
  Clients/Alderfenna/Meetings/ 顧客会議と承認の記録
  Clients/Brackensail/Buying/  調達分析、交渉条件、改善案
  Clients/Brackensail/Handover/ 最終報告、顧客側の点検手順
  Proposals/                  提案原稿、採用しなかった案
  Practice-notes/             自分の方法メモ、発表資料
  Internal-review/            Vivian の指摘と回答
Downloads/                     顧客提供資料と VDI の許可済み文書
OneDrive - Orielspan/
  Team-room/                  共有進捗表、内部会議
  Travel-reading/             移動中に読む資料の控え
Pictures/                      工程図、会議画面、ワークショップ写真
Documents/Mail/                顧客別に保存した合意メール
```

## 形式の目安

| 拡張子 | 割合 |
|---|---:|
| pptx | 26% |
| xlsx | 23% |
| pdf（テキスト） | 17% |
| docx | 13% |
| eml | 8% |
| csv | 4% |
| pdf（スキャン） | 3% |
| png（工程図・画面） | 2% |
| txt | 2% |
| jpg（会議写真） | 1% |
| md | 1% |

署名済み合意書だけに紙のスキャンを使う。音声と zip は作らない。

## 規模の目安

全体で 2,000 件。最初の区切りは 300 件で、Alpha Phase 1 の計画と診断、Beta の実績確認を中心にする。
同じ提言の PDF は顧客へ送付した版など実際の用途がある場合だけ残す。

## 書かないもの

- ソースコード、ノートブック、SQL、設定ファイル、pcap、分析用の実行ファイル。
- 実在顧客の財務値、契約、面談録、VDI の接続情報。
- 秘密情報や認証情報。専門的な分析も表と自然文の報告で表現する。
