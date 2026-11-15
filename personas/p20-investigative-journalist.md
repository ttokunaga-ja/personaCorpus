---
persona: p20-investigative-journalist
period: 2024-04-01..2026-09-30
allowed_terms: FOIA batch, scope of request, scope of investigation
photo_dirs: Pictures
---

# p20 調査報道記者

## 人物

- **Miriam Thornequay**、Civic Ledger Fernwake の調査報道記者。入社 9 年目、公共調達と行政記録の取材を担当。
- PC はストレージ暗号化された macOS ノート。OS は en-US、使用言語は英語。取材メモ、照会、記事も英語で書く。
- 朝に資料請求と回答期限を点検し、午後に面談、夕方に事実と未確認情報を分けて整理する。
- Story Alpha は道路補修契約、Story Beta は公共図書室の設備調達。疑念から結論を決めず、反証と取材先の回答も検討する。
- Documents は記事別、録音は Music、写真は Pictures。Desktop に確認待ちの段落が残り、編集者に落とされた構成案も保管する。

## 組織と関係者

- **Civic Ledger Fernwake Cooperative**：米国の地域調査報道編集部、職員 23 人。取材対象の自治体は **Fenmarsh Borough**、契約先は **Rookelmere Paving Inc**。いずれも独自の設定。
- 社内：
  - **Cora Windermere**：編集長。掲載と訂正を承認。
  - **Simonette Ashenquill**：ファクトチェック担当。出典と引用を照合。
  - **Hector Larkvale**：データ担当。金額と契約日を集計。
- 社外：
  - **Fenmarsh Borough** の **Edith Pellmoss**：公文書公開の担当。
  - **Rookelmere Paving Inc** の **Graham Norrick**：工事契約について回答する広報担当。
  - **Fenmarsh Borough** の **Lena Orrowell**：元調達担当。記名面談に同意した情報源。
- メールは `miriam@fernwake-ledger.example`、公開請求は `edith@fenmarsh-borough.example`。私用連絡先や匿名者の特定情報は作らない。

## 仕事の流れ

- **2024-04**：Cora と Alpha の企画を検討。道路補修 18 契約の増額を調べるが、不正を前提にしないと取材方針を決めた。
- **2024-07**：Edith に契約と変更記録 6 種を請求。範囲が広すぎると回答され、対象年度と道路区画を絞って再請求した。
- **2024-10**：Hector と受領 124 ページを整理。契約額の差 USD 310,000 は税の扱いによると判明し、当初の増額仮説を撤回した。
- **2025-01**：Lena に 55 分の面談。承認日と作業開始日の逆転 4 件を聞き取り、発言だけで確定せず公文書で確認するとした。
- **2025-04**：Simonette が草稿の出典 43 件を点検。裏付けが一つしかない主張 7 件を保留し、掲載予定を 2 週間延期した。
- **2025-07**：Graham に 12 問を送付。追加工事 3 件の必要性に回答があり、説明の引用と記録上の確認を分けて記事に入れた。
- **2025-10**：Cora と Beta の図書室設備調達を企画。見積 9 件の比較に絞り、Alpha の工事費と合算しない方針を確認した。
- **2026-01**：Edith から Beta の資料 76 ページを受領。欠落する議事録 2 回分を再請求し、推測の会議要旨は作らなかった。
- **2026-04**：Hector と Alpha の変更契約を再計算。増額率は当初案 22% ではなく 16.4% となり、図と本文の両方を直した。
- **2026-08-19**：Simonette が Alpha の主要事実 28 件を確認。25 件を確認済み、3 件を留保とし、公開判断の資料にした。
- **2026-08-22**：Edith から Story Alpha 2026 の FOIA batch 17 を受領。追加 93 ページを既存資料と照合し、留保 3 件中 2 件を解消した。
- **2026-09**：Cora が Alpha の掲載稿 3,200 語を承認。残る 1 件は本文から外し、Graham の回答と確認日を併記。Beta は取材継続とした。
- 記事草稿、取材ノート、逐語録、請求のやり直し、落とした段落も残す。録音と引用は対応する箇所を確認できるようにする。

## フォルダ構成

```text
Desktop/                       確認待ちの段落、次の面談の質問
Documents/
  Stories/Road-contracts/Requests/ 公開請求、範囲の修正、回答
  Stories/Road-contracts/Records/ 受領文書、ページごとの読書メモ
  Stories/Road-contracts/Checks/ 出典表、計算、確認の留保
  Stories/Road-contracts/Drafts/ 草稿、編集、掲載稿
  Stories/Library-purchases/   Beta の請求、比較、取材メモ
  Interviews/                 面談質問、逐語録、引用の確認
  Pitches/                    企画案、却下案、編集会議
  Desk-notes/                 自分の作業メモ、研修
Downloads/                     公開担当から届いた文書の受領控え
Music/Interviews/              記名面談の音声
Pictures/Reporting/            工事箇所や掲示物の取材写真
Pictures/Charts/               金額の図、文書照合の画面
Documents/Editorial-share/     編集部共有資料のオフライン控え
Documents/Mail/                請求、質問、回答、編集承認
```

## 形式の目安

| 拡張子 | 割合 |
|---|---:|
| pdf（テキスト） | 22% |
| pdf（スキャン） | 17% |
| docx | 16% |
| txt | 11% |
| eml | 10% |
| jpg（取材写真） | 8% |
| xlsx | 5% |
| csv | 4% |
| png（図・照合画面） | 3% |
| html | 2% |
| m4a | 2% |

音声は取材の m4a 2% だけ、zip は作らない。html は保存した公表文の独自の文面とする。

## 規模の目安

全体で 2,000 件。最初の区切りは 300 件で、Alpha の FOIA batch 17 と事実確認、掲載稿、Beta の請求を中心にする。
同じ公開資料のページを理由なく独立したファイルへ切り分けない。

## 書かないもの

- ソースコード、ノートブック、SQL、設定ファイル、pcap、スクレイピングの実装。
- 実在情報源の声・顔・連絡先、実際の未公開資料、匿名者の識別情報や位置情報。
- 暗号鍵、認証情報、実在の記事転載、動画。録音と文書はこの取材設定から独自に作る。
