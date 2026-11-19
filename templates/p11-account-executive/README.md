# Michelle Whitaker の文書の型

Office の型は通常の DOCX・XLSX・PPTX。開いて別名で保存する。米国英語、用紙は US Letter、金額は USD、記録する日時は America/Denver。受領した購買文書は JDH の型、提案依頼書は Aldridge の型、試行結果報告は Hollis の型を使う。

| 名前 | 組織・利用者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Adoption Review | Halstead の Michelle。利用明細を確認し、増席を見送る理由と後続対応を記録する | `halstead-adoption-review.docx` | 縦、表紙なし。青緑の題、煉瓦色の太い横罫、ベースライン・判断・後続対応の見出し |
| Workshop Record | Halstead の Michelle と Divya。改善会の決定、部門ごとの担当、確認時期を記録する | `halstead-workshop-record.docx` | 縦、表紙なし。決定・部門担当・確認時期の見出し。表で役割と宿題を対応づける |
| Scope of Services | Halstead の Michelle が JDH や Merritt に渡す運用支援範囲。Jonathan が契約条件との整合を確認する | `halstead-support-scope.docx` | 縦、表紙なし。支援範囲・双方の責任・運用レビューの見出し |
| Department Adoption | Halstead の Michelle。部門別の契約席・稼働席・利用率を集計し、Divya が Waverley の導入負荷表にも使う | `halstead-adoption.xlsx` | 横。上部にロゴ・管理欄、青緑の列見出し、青灰色の明細と集計帯 |
| Adoption Workshop / Proposal Briefing | Halstead の Michelle と Divya の改善会。Michelle が Ahmed への提案説明と Waverley の導入・更新面談にも使う | `halstead-workshop.pptx` | 16:9。左の青緑の縦罫、小型ロゴ、下の煉瓦色の罫。マスター1つ、レイアウト5種、スライド0枚 |
| RFP Cross-Reference | Halstead の Michelle が、Ahmed の要求番号と回答箇所を結び、Allison が確認した未対応条件を追う | `halstead-rfp-matrix.xlsx` | 横。青緑の列見出しと青灰の集計帯。必須区分、回答状態、参照箇所を別の列で扱う |
| Conditional Proposal Response | Halstead の Michelle が、Donald の承認範囲で回答と例外を Ahmed に提示する | `halstead-rfp-response.cls` | LuaLaTeX、縦、表紙なし。青緑の社名と煉瓦色の罫。例外の判定と回答箇所を並べる表を持つ |
| Contract Terms Review | Halstead の Marisol と Jonathan が条項を確認し、Michelle が責任上限の対案を Ahmed に返す | `halstead-terms-redline.docx` | 縦、表紙なし。煉瓦色の取消線と下線で条項の削除案と追加案を示す |
| Renewal Terms | Halstead の Michelle。年間更新の期間、席数、対価、支援範囲の参照先を記録する | `halstead-renewal.cls` | LuaLaTeX、縦。青緑の表見出し、右寄せの数値、青灰色の集計帯。続きページでも表見出しが出る |
| Purchasing Correspondence | JDH Route Services Inc の Janet。購買側の確認事項を Halstead に返す | `jdh-purchasing-confirmation.docx` | 縦、表紙なし。右上の経路マークと社名、茶色の見出し、黄土色の二重罫、生成りの表帯 |
| Request for Proposal | Aldridge Service Group の Ahmed。提案依頼を発行し、要求番号と優先区分を回答者に渡す | `aldridge-rfp.cls` | LuaLaTeX、縦、表紙なし。右上に3本の段違いの柱と社名。紫の細罫、セリフ体と節番号、淡紫の表見出し |
| Initial Adoption Terms | Halstead の Michelle。新規導入の席数と対価を注文条件として記録する | `halstead-adoption-terms.cls` | LuaLaTeX、縦、表紙なし。Halstead のロゴと煉瓦色の罫を継承し、ヘッダに ADOPTION TERMS。続きページにも青緑の表見出し |
| Pilot Results | Hollis Medical Supply Inc の Lawrence。IT が確認した試行の対象と週次利用結果を Halstead に送る | `hollis-pilot-result.cls` | LuaLaTeX、縦、表紙なし。左上に段違いの青い正方形と社名、右上に INFORMATION TECHNOLOGY。淡青の表見出し、右寄せの席数、IT Operations のフッタ |

CSV は利用明細を出力した時点の列定義、EML は差出人と宛先が分かるメール、訪問 JPG は許可された掲示物の写真として扱う。

Hollis の訪問用 PPTX は Adoption Workshop / Proposal Briefing を用いる。構成図 PNG と Sales-materials の保存 HTML は Halstead の青緑、煉瓦色、本文色を使い、試行で説明する機能と対象範囲を示す。審査質問の印刷原紙には Workshop Record の見出しを質問票に合わせて使う。紙への書き込みを載せた原紙から `bin/scan` で画像だけの PDF にする。

2024年8月の提案は30席の試行を条件とする。2025年3月18日に Lawrence が報告した週次利用は試行の30席中24席。審査用 XLSX には試行の集計単位と出典を置く。Jonathan の条件確認後に成立した180席・USD 82,000の導入契約は注文条件として記載する。

## Waverley と Merritt

Waverley Commercial Storage Inc の導入では、Divya が Department Adoption を下記の Implementation Load に組み替え、120席・USD 54,000 の導入を6週で進められるかを確認する。導入説明と2026年6月11日の更新面談は、それぞれ Adoption Workshop / Proposal Briefing で作る。更新面談では利用率74%を根拠として、120席・USD 54,000 の据え置きを示す。利用内訳の CSV は集計日と列の意味を保ち、会議室の JPG は撮影の許可範囲に合わせる。

Merritt Equipment Repair Inc の2025年11月の更新では、Scope of Services に支援の範囲と年40時間の取扱いを記し、顧客向けの Renewal Terms に年額 USD 72,000 と支援範囲の参照先を載せる。Nasrin の追加支援の要望、期限付き旧案、Donald が無償追加10時間を却下した経緯は Pricing-approvals の社内記録と EML に残す。旧案を PDF にする場合は Renewal Terms を開き、題に社内案と分かる表示を置き、発行日とは別に有効期限を記す。Waverley と Merritt の文書は顧客別に作る。

## 共通の体裁と文書番号

- Halstead のロゴは、支柱から分岐する青緑の横棒と煉瓦色の端点に社名を添える。題・見出し・フッタは青緑 `133D45`、罫や注意は煉瓦色 `AF5636`、本文は墨 `26353B`。白と青灰 `EBF2F1` は背景。欧文は Liberation Sans、和文は Noto Sans CJK JP。Excel のセルは Liberation Sans を基本とし、和文セルを追加する場合は Noto Sans CJK JP を指定する。
- JDH のロゴは枠内の山形と道路線に社名を添える。焦茶 `653827`、黄土 `916232`、本文の墨茶 `352E2A`、生成りの背景 `F5ECE2`。欧文は Liberation Serif、和文は Noto Sans CJK JP。ヘッダと購買フッタは右寄せとし、フッタにページ番号を入れる。
- Aldridge は、右上に高さの異なる紫の3本柱と社名を置く。題・見出しとフッタは紫紺 `343454`、細罫と部門ラベルは紫 `575483`、本文は墨紫 `2C2B34`、表帯は淡紫 `EEEDF5`。欧文本文は Liberation Serif、社名・部門ラベルとフッタは Liberation Sans、和文は Noto Serif CJK JP。フッタに Business Planning と通しページ番号を入れる。
- Hollis は、段違いの紺と青の正方形を左上の社名に添える。本文・題・フッタは紺 `243752`、節見出し・細い横罫・IT 部門名は青 `3B6190`、表見出しの背景は淡青 `E9EFF8`。欧文は DejaVu Sans、和文は Noto Sans CJK JP。全ページの左下に `Hollis Medical Supply Inc / IT Operations`、右下に `Page` と通しページ番号を入れる。
- Halstead は `HBC-顧客略号-YYYYMMDD-用途-NN`。用途は `ADP`（利用レビュー・集計）、`IMP`（導入負荷）、`WRK`（改善会）、`SVC`（支援範囲）、`REN`（更新条件）、`MAT`（要求と回答の対応表）、`BRF`（顧客説明）、`RFP`（提案回答）、`LEG`（条項の修正案）、`ORD`（新規注文条件）。Aldridge の略号は `ALD`、Hollis は `HOL`、Waverley は `WAV`、Merritt は `MER`。同一発行日・用途内で文書ごとに2桁の番号を割り振る。別文書では固有番号を使い、同じ文書を別形式に出すときは番号を引き継ぐ。JDH 購買文書は `JDH-PO-YYYYMMDD-NN`、Aldridge の提案依頼は `ASG-RFP-YYYYMMDD-NN`、Hollis IT の試行結果は `HMS-IT-YYYYMMDD-NN`。
- 見本の番号は組版確認用。実際の番号、担当者、日付、根拠はファイル計画と台帳に従う。4月の基準は稼働318席・契約420席、7月の80%は2か月後の確認目標、翌年5月開始の年次更新は運用支援を含む480席・USD 232,000。見本の試験用明細を実績へ転記しない。

## Word

本文は用途別の `Heading 1` を持つ。各節に原稿を入れ、節を使わない場合は見出しごと削除する。先頭へ `Title` で題、その直後に `Account Metadata` で管理欄を挿入する。管理欄の1行目は文書番号と発行日、2行目は作成者と Account（JDH 側は Recipient）。各行はタブで2列に分ける。右列は本文左端から3.8インチ。長い宛名が入る場合は項目を独立行にして折り返す。表紙は設けない。

- 本文 `Normal` 11 pt、題 25 pt、見出し1 14 pt、見出し2 11 pt、管理欄 10 pt。左右余白は Halstead 0.76インチ、JDH 0.9インチ。ページフッタは組織・部門表記と自動ページ番号。
- 表は必要な行だけ追加し、`Account Table` を指定する。先頭行の段落は `Account Table Header`、文字セルは `Account Table Text`、数値セルは右寄せの `Account Table Number`。すべて9.5 pt。先頭行は繰返し見出し（`w:tblHeader`）、各行には分割禁止（`w:cantSplit`）を設定する。文字セルは段落スタイルの左インデント0.10インチで隣の数値との間隔を取る。
- 列幅は本文幅以内で指定して折り返す。3列の見本は Halstead で2.16、1.00、3.82インチ、JDH では末列を3.54インチにする。
- Contract Terms Review は審査段階、対象条項、未決の論点を記録する。削除する原文の文字列に `HB Proposed Deletion`、追加する文字列に `HB Proposed Insertion` を適用する。取消線と下線は PDF にも表示される固定の文字書式。条項から分ける審査メモがある場合は、見出しで区切って続きページに置ける。

## Excel

`Department Adoption` シートの `A3` は題。`C4:C6` の各セルは右へ `F` 列まで個別に結合済みで、文書番号、レビュー日、作成者を入力する。日付は実日付で入力し、表示形式を `mmmm d, yyyy` にする。8行目が見出しで、列は A から No.、Department、Licensed seats、Active seats、Utilization、Review note。 `C9` でウィンドウを固定している。

1. 9行目から明細を作る。A・C・D は `HB Integer`（右寄せ、桁区切り）、B・F は `HB Text`（左寄せ）、E は `HB Rate`（右寄せ、小数1桁の百分率）。E は契約席が正のときに D÷C を計算する。各スタイルのインデント1と折り返しを保持する。偶数行には青灰色の塗り `EBF2F1` を設定する。行高は見本で31 pt。文章が多い行は余裕を持たせる。
2. 集計が必要な場合だけ、最終明細の直後に合計行を作る。A・B・F は `HB Total`、C・D は `HB Total Integer` で各席数の SUM、E は `HB Total Rate` で合計稼働席÷合計契約席。行高は23 pt。必要な注記はその直下で A:F を行内結合し、`HB Note` と折り返しで付ける。
3. フィルター範囲を8行目から最終明細まで、印刷範囲を `A1:F<最終使用行>` に設定する。Letter 横、横1ページ・縦は必要なページ数、8行目を各ページで繰り返す。フッタに部門名と通しページ番号。少量・複数ページとも印刷画像で、A→B と E→F の間隔、管理欄の全文、続きページの見出しを確かめる。

Hollis の審査用には B8〜D8 を `Pilot cohort`、`Pilot seats`、`Weekly active` に置き換える。3月18日受領分の見本は9行目に1集計単位を記載し、E9 で利用率を計算する。10行目の A:F を結合して `HB Note` を適用し、試行結果の出典を記載する。同じ試行席を含む異なる週の行を足し上げない。

`RFP Cross-Reference` も `A3` が題、`C4:C6` が文書番号・発行日・作成者、8行目が見出し、`C9` で固定。A 列は依頼書の要求番号、B〜F は要求内容、優先区分、回答状態、回答文書の節と項目番号、留保または次の確認事項。回答文書の正式番号は表外の注記に置く。

1. 9行目から A は `HB Integer`、B〜F は `HB Text`。偶数行に青灰色を設定し、見本は35 pt。番号の直後の文字列は左インデント1で離す。要求文や留保が3行を超える場合は行高を上げて印刷する。
2. 件数の集計を付ける場合は最終明細の直後に、A に要求番号の COUNT、C に必須かつ例外の COUNTIFS、B・D にそれぞれの集計名を置く。A・C は `HB Total Integer`、B・D〜F は `HB Total`。行高の目安は29 pt。続く注記は A:F を1行で結合し `HB Note` を適用する。
3. フィルターと印刷範囲は Department Adoption と同じ手順で設定する。管理欄の幅と折返しを保ち、続きページの見出しと A→B、集計行の A→B と C→D の間隔を見る。

### Implementation Load

`halstead-adoption.xlsx` を開き、シート名を `Implementation Load`、A3 を表題、A5 を `As of` に変更する。C4:C6 の文書番号・基準日・作成者の書式と結合を保つ。A 列の幅を10にし、8行目の6列を `Week`、`Workstream`、`Demand (h)`、`Available (h)`、`Headroom (h)`、`Dependency or handoff` に置き換える。

1. 9行目から、週と作業ごとに必要な明細だけを入れる。A・C:E は右寄せの `HB Integer`（単位は整数時間、A は週番号）、B・F は左寄せの `HB Text`。E は D−C。負の値を表示し、空きが足りない週を確認する。偶数行に青灰色 `EBF2F1` を塗り、インデント1と折り返しを保つ。行高の目安は31 pt。
2. 作業ごとに重複なく確保した時間を足し上げる場合は、明細の直後を23 ptの集計行にする。B に `Total`、C・D に各列の SUM、E に合計行の D−C を入れる。A・B・F は `HB Total`、C:E は `HB Total Integer`。注記には A:F を行内結合して `HB Note` を適用し、折り返して読める行高を指定する。
3. 明細だけをフィルター範囲に含め、管理欄から最終使用行までを印刷範囲にする。Letter 横と8行目の繰り返しを継承し、A→B と E→F の間隔、管理欄の全文、続きページの列見出しを確認する。

XLSX の内部プロパティは最終保存後に確定する。通常の batch では進行役が `bin/stamp` を実行する。見本を単独で作った場合は `bin/stamp <xlsx> --author <作成者> --date <日付と時刻> --timezone America/Denver` を最終保存後に実行する。`docProps/app.xml` の Application が Microsoft Excel、`docProps/core.xml` の作成者と更新者および作成・更新日時が文書の作成者と日付に合うことを確認する。

## PowerPoint

スライドマスターからロゴ・社名・縦罫とフッタを継承する。下記の名前でレイアウトを選び、既存プレースホルダーを埋める。表紙の副題に文書番号・発行日、作成者、相手先を入れる。

| レイアウト | 中身 |
|---|---|
| Cover | タイトル（43 pt）と文書情報（20 pt） |
| Title and Body | タイトル（31 pt）と1列本文（22 pt） |
| Two Columns | タイトルと左右の本文（20 pt）。担当の違いや次の作業を並べる |
| Table | タイトル、表の領域、キャプション（14 pt）。表は領域に合わせて作る |
| Figure | タイトル、画像プレースホルダー、キャプション（14 pt）。図の比率を領域に合わせる |

PowerPoint 上では表プレースホルダーから表を作る。python-pptx では表用スロットの位置と幅を使って表を置き、元のスロットを取り除く。見出しは青緑に白字、本文セルは墨色で18 ptを目安とする。数値セルは右寄せにし、文字列セルの左余白を0.20インチ以上にする。長い表はスライドを分け、各ページに列見出しを載せる。

## LuaLaTeX

`\documentclass{halstead-renewal}` を使う。クラスを作業用 TeX の隣に置くか、`TEXINPUTS` に型のディレクトリを設定する。毎ページのヘッダに社名・文書種別、フッタに部門・ページ番号が入る。

- 本文の冒頭に `\HBRenewal{題}{文書番号}{発行日}{作成者}{顧客名}`、区切りに `\section` または `\subsection` を置く。
- `HBTerms` 環境に `\HBTerm{条件}{値}{適用の説明}` を必要な数だけ入れる。必要な集計は最後に `\HBTotal{条件}{値}{適用の説明}`、表外の注記は `\HBNote{本文}` で作る。数値列は右寄せで、次の列との間に12 ptの余白が入る。
- `bin/tex <入力.tex> <出力.pdf>` で組み、`bin/render` で見出し、改ページ、番号と金額を確認する。

Halstead の回答は `\documentclass{halstead-rfp-response}` と `\HBResponse{題}{文書番号}{発行日}{作成者}{顧客名}` を使う。クラスの検索パスに `halstead-renewal.cls` も置く。原稿に従って回答範囲と留保を節にし、`HBExceptions` 環境には `\HBException{要求番号と項目}{回答状態}{留保または回答参照}` を並べる。続きページに表見出しを繰り返す。表外の補足には `\HBNote{本文}` を使う。

Aldridge の発行文書は `\documentclass{aldridge-rfp}` と `\ASGRequest{題}{依頼番号}{発行日}{発行者}{宛先}` を使う。回答方法と要求事項を `\section` で分け、`ASGRequirements` 環境に `\ASGRequirement{要求番号}{要求内容}{優先区分}` を並べる。番号は右寄せで、その右に12 ptの列間余白が入る。ページをまたいでも要求番号を通し、表見出しを繰り返す。

Halstead の新規導入条件は `\documentclass{halstead-adoption-terms}` と `\HBAdoptionTerms{題}{文書番号}{発行日}{作成者}{顧客名}` を使う。検索パスに `halstead-renewal.cls` も置く。条件、値、適用範囲を `HBTerms` と `\HBTerm` で並べ、契約金額を別の集計帯に置く場合は `\HBTotal` を使う。試行結果を参照する場合は表外の `\HBNote` に記載する。

Hollis IT の報告は `\documentclass{hollis-pilot-result}` と `\HMSPilotResult{題}{報告番号}{発行日}{作成者}{宛先}` を使う。対象、結果、審査での扱いを `\section` で分ける。`HMSResults` 環境に `\HMSResult{指標}{席数}{集計の根拠}` を必要な数だけ置く。数値は右寄せ、次の説明列まで15 pt。続きページにも列見出しを繰り返す。

## 見本とレンダリング

見本と画像は b004 の今回のテンプレート作業フォルダにある。`short` は少量、`long` は改ページ確認用。画像は各ディレクトリ内に全ページを置いた。見本の番号、明細、判定は組版確認用。制作時は計画・原稿・事実の台帳に従う。

| 型 | 見本名（`scratch/p11-account-executive/b004/work/templates-0002/samples/` 以下） | 画像ディレクトリ（`scratch/p11-account-executive/b004/work/templates-0002/render/` 以下） |
|---|---|---|
| Adoption Review | `halstead-adoption-review-short.docx`, `halstead-adoption-review-long.docx` | `halstead-adoption-review-short/`, `halstead-adoption-review-long/` |
| Workshop Record | `halstead-workshop-record-short.docx`, `halstead-workshop-record-long.docx` | `halstead-workshop-record-short/`, `halstead-workshop-record-long/` |
| Scope of Services | `halstead-support-scope-short.docx`, `halstead-support-scope-long.docx` | `halstead-support-scope-short/`, `halstead-support-scope-long/` |
| Purchasing Correspondence | `jdh-purchasing-confirmation-short.docx`, `jdh-purchasing-confirmation-long.docx` | `jdh-purchasing-confirmation-short/`, `jdh-purchasing-confirmation-long/` |
| Department Adoption | `halstead-adoption-short.xlsx`, `halstead-adoption-long.xlsx`, `halstead-hollis-pilot-review.xlsx`（Hollis 1件） | `halstead-adoption-short/`, `halstead-adoption-long/`, `halstead-hollis-pilot-review/` |
| Implementation Load | `halstead-waverley-implementation-load-short.xlsx`, `halstead-waverley-implementation-load-long.xlsx` | `halstead-waverley-implementation-load-short/`, `halstead-waverley-implementation-load-long/` |
| Adoption Workshop / Proposal Briefing | `halstead-workshop.pptx`（Waverley の導入説明、全5レイアウト）、`halstead-waverley-renewal-briefing.pptx`（更新面談） | `halstead-workshop/`, `halstead-waverley-renewal-briefing/` |
| 訪問用スライドの構成図 | `visit-diagram.png`（Figure レイアウトの挿入図） | `visit-diagram/` |
| Renewal Terms | `halstead-renewal-short.tex`, `halstead-renewal-short.pdf`, `halstead-renewal-long.tex`, `halstead-renewal-long.pdf` | `halstead-renewal-short/`, `halstead-renewal-long/` |
| Renewal Terms（Merritt） | `halstead-merritt-renewal.tex`, `halstead-merritt-renewal.pdf` | `halstead-merritt-renewal/` |
| RFP Cross-Reference | `halstead-rfp-matrix-short.xlsx`（6件）、`halstead-rfp-matrix-long.xlsx`（56件） | `halstead-rfp-matrix-short/`, `halstead-rfp-matrix-long/` |
| Conditional Proposal Response | `halstead-rfp-response-short.tex`, `halstead-rfp-response-short.pdf`, `halstead-rfp-response-long.tex`, `halstead-rfp-response-long.pdf` | `halstead-rfp-response-short/`, `halstead-rfp-response-long/` |
| Contract Terms Review | `halstead-terms-redline-short.docx`, `halstead-terms-redline-long.docx` | `halstead-terms-redline-short/`, `halstead-terms-redline-long/` |
| Request for Proposal | `aldridge-rfp-short.tex`, `aldridge-rfp-short.pdf`, `aldridge-rfp-long.tex`, `aldridge-rfp-long.pdf` | `aldridge-rfp-short/`, `aldridge-rfp-long/` |
| Initial Adoption Terms | `halstead-adoption-terms-short.tex`, `halstead-adoption-terms-short.pdf`, `halstead-adoption-terms-long.tex`, `halstead-adoption-terms-long.pdf` | `halstead-adoption-terms-short/`, `halstead-adoption-terms-long/` |
| Pilot Results | `hollis-pilot-result-short.tex`, `hollis-pilot-result-short.pdf`, `hollis-pilot-result-long.tex`, `hollis-pilot-result-long.pdf` | `hollis-pilot-result-short/`, `hollis-pilot-result-long/` |
