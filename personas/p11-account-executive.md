---
persona: p11-account-executive
period: 2024-04-01..2026-09-30
timezone: America/Denver
allowed_terms: scope of proposal, scope of services, batch invoicing
photo_dirs: Pictures
---

# p11 アカウント営業

## 人物

- **Rachel Pennelgrove**、Northstar Cloud Bellwisp 社のアカウント営業。入社 5 年目、大口顧客担当は 2 年目。
- PC は Windows 11 の出張営業ノート。OS は en-US、使用言語は英語。見積通貨は米ドル。
- 訪問前に前回の約束を読み、訪問後は当日中に依頼、担当、期限をメールで確認する。口頭の値引きを確約しない。
- 更新と新規提案を別々に管理する。顧客が言った予算と自社の予測金額を分けて書く。
- Desktop には今週訪問する顧客の資料、Documents には取引先名でまとめた履歴。機内で直した版に `_travel` が残ることがある。

## 組織と関係者

- **Northstar Cloud Bellwisp Inc**：米国の法人向け拠点連絡サービス会社、従業員 190 人。年間契約でアカウント数と支援時間を販売する。サービス名は Bellwisp Relay。
- 社内：
  - **Dennis Woldmere**：営業部長。価格例外と売上予測を承認。
  - **Asha Lindervale**：導入設計担当。移行条件と工数を評価。
  - **Cole Harrowfen**：契約管理担当。条項と署名の確認。
- 社外：
  - **Mossquay Routeworks Inc**（Account Alpha）の **Lydia Cairnwell**：購買責任者。
  - **Briarcombe Service Group**（Account Beta）の **Omar Thistwick**：事業企画責任者。
- メールは `rachel@bellwisp-cloud.example`、Alpha は `lydia@mossquay-routeworks.example`。価格表の有効期限を送信時に確認する。

## 仕事の流れ

- **2024-04**：Lydia と利用 420 席の契約を点検。稼働 318 席だったため、増席提案を取り下げ定着支援を先にした。
- **2024-07**：Asha と Alpha の利用改善会を実施。対象 4 部門で担当者を決め、2 か月後の利用率 80% を確認目標にした。
- **2024-10**：Omar から Beta の提案依頼 56 項目を受領。必須条件の 6 項目が未対応で、条件付き回答にすると Dennis が承認した。
- **2025-01**：Cole と Beta の契約案を検討。責任上限なしの条項 1 件を受け入れず、年間契約額を上限とする再提案を送った。
- **2025-04**：Lydia と Alpha を更新。USD 232,000、利用 480 席とし、未利用席の一括増加は行わなかった。
- **2025-07**：Dennis が出張提案を審査。訪問費 USD 6,400 に対して 2 社同日訪問案を採り、単独訪問案を中止した。
- **2025-10**：Omar が Beta の予算凍結を連絡。USD 180,000 の案件を当期予測から外し、失注とは区別して保留扱いにした。
- **2026-01**：Asha と Alpha の拡張工数を試算。移行 9 週が必要で、顧客希望の 4 週は約束しないと決めた。
- **2026-04**：Lydia が更新上限 USD 250,000 を提示。値引きより支援範囲の整理を選び、3 案の比較表を作成した。
- **2026-07-17**：Lydia と Account Alpha の年次更新を合意。USD 248,000、520 席、支援年 60 時間で、Cole が署名条件を確認した。
- **2026-08**：Omar と Beta の再提案を協議。初期 200 席、USD 96,000 の小規模案に変え、予測は確度 40% と報告した。
- **2026-09**：Dennis と四半期予測を確定。Alpha 更新は実績、Beta は未契約として分け、上振れ見込 USD 96,000 を基本計画に含めなかった。
- 顧客別に提案、回答、価格承認、契約、訪問メモをつなげる。不採用の価格案にも有効期限と見送り理由を残す。

## フォルダ構成

```text
Desktop/                       今週の訪問説明、送信前の見積
Documents/
  Accounts/Mossquay/Renewals/ 更新提案、価格比較、署名済み契約
  Accounts/Mossquay/Adoption/ 利用状況、支援計画、顧客会議
  Accounts/Briarcombe/RFP/    質問、条件付き回答、適合表
  Accounts/Briarcombe/Restart/ 保留後の小規模提案
  Forecast/<年>/<四半期>/      予測、確度の理由、部長への説明
  Pricing-approvals/          価格例外、却下案、有効期限
  Travel/<年>/                訪問予定、旅費申請、訪問メモ
Downloads/                     顧客の依頼書、署名済み添付
OneDrive - Bellwisp/
  Sales-materials/            サービス説明と利用例の控え
  My-meeting-packs/           出張用の資料
Pictures/Visits/               許可された会議室と掲示物の写真
Pictures/Proposal-diagrams/    利用構成図、説明画面
Documents/Mail/                取引先ごとの約束と価格承認
```

## 形式の目安

| 拡張子 | 割合 |
|---|---:|
| eml | 24% |
| pptx | 19% |
| pdf（テキスト） | 16% |
| xlsx | 13% |
| docx | 12% |
| pdf（スキャン） | 5% |
| csv | 4% |
| jpg（訪問写真） | 2% |
| png（構成図・画面） | 2% |
| txt | 2% |
| html | 1% |

csv は商談や利用状況の表出力、写真は顧客の個人を写さない。音声と zip は作らない。

## 規模の目安

全体で 2,000 件。最初の区切りは 300 件で、Alpha の USD 248,000 更新と Beta の保留・再提案を中心にする。
受信メールと添付は役割が異なる場合にだけ両方を残す。

## 書かないもの

- ソースコード、ノートブック、SQL、設定ファイル、pcap、サービス実行環境。
- 実在顧客の連絡先、実際の CRM 出力、カード情報、署名画像。
- 認証情報と秘密の契約。架空の条項と顧客設定で取引の流れを表す。
