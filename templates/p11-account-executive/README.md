# Michelle Whitaker の文書の型

Office の型は通常の DOCX・XLSX・PPTX。開いて別名で保存する。米国英語、用紙は US Letter、金額は USD、記録する日時は America/Denver。JDH から受け取った購買文書には JDH の型を使う。

| 名前 | 組織・利用者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Adoption Review | Halstead の Michelle。利用明細を確認し、増席を見送る理由と後続対応を記録する | `halstead-adoption-review.docx` | 縦、表紙なし。青緑の題、煉瓦色の太い横罫、ベースライン・判断・後続対応の見出し |
| Workshop Record | Halstead の Michelle と Divya。改善会の決定、部門ごとの担当、確認時期を記録する | `halstead-workshop-record.docx` | 縦、表紙なし。決定・部門担当・確認時期の見出し。表で役割と宿題を対応づける |
| Scope of Services | Halstead の Michelle が JDH に渡す運用支援範囲。Jonathan が契約条件との整合を確認する | `halstead-support-scope.docx` | 縦、表紙なし。支援範囲・双方の責任・運用レビューの見出し |
| Department Adoption | Halstead の Michelle。部門別の契約席・稼働席・利用率と確認事項を集計する | `halstead-adoption.xlsx` | 横。上部にロゴ・管理欄、青緑の列見出し、青灰色の明細と集計帯 |
| Adoption Workshop | Halstead の Michelle と Divya。顧客の改善会でベースライン、担当、確認方法を共有する | `halstead-workshop.pptx` | 16:9。左の青緑の縦罫、小型ロゴ、下の煉瓦色の罫。マスター1つ、レイアウト5種、スライド0枚 |
| Renewal Terms | Halstead の Michelle。年間更新の期間、席数、対価、支援範囲の参照先を記録する | `halstead-renewal.cls` | LuaLaTeX、縦。青緑の表見出し、右寄せの数値、青灰色の集計帯。続きページでも表見出しが出る |
| Purchasing Correspondence | JDH Route Services Inc の Janet。購買側の確認事項を Halstead に返す | `jdh-purchasing-confirmation.docx` | 縦、表紙なし。右上の経路マークと社名、茶色の見出し、黄土色の二重罫、生成りの表帯 |

CSV は利用明細を出力した時点の列定義、EML は差出人と宛先が分かるメール、訪問 JPG は許可された掲示物の写真として扱う。

## 共通の体裁と文書番号

- Halstead のロゴは、支柱から分岐する青緑の横棒と煉瓦色の端点に社名を添える。題・見出し・フッタは青緑 `133D45`、罫や注意は煉瓦色 `AF5636`、本文は墨 `26353B`。白と青灰 `EBF2F1` は背景。欧文は Liberation Sans、和文は Noto Sans CJK JP。Excel のセルは Liberation Sans を基本とし、和文セルを追加する場合は Noto Sans CJK JP を指定する。
- JDH のロゴは枠内の山形と道路線に社名を添える。焦茶 `653827`、黄土 `916232`、本文の墨茶 `352E2A`、生成りの背景 `F5ECE2`。欧文は Liberation Serif、和文は Noto Sans CJK JP。ヘッダと購買フッタは右寄せとし、フッタにページ番号を入れる。
- Halstead は `HBC-顧客略号-YYYYMMDD-用途-NN`。用途は `ADP`（利用レビュー・集計）、`WRK`（改善会）、`SVC`（支援範囲）、`REN`（更新条件）。同一発行日・用途内で文書ごとに2桁の番号を割り振る。別文書では固有番号を使い、同じ文書を別形式に出すときは番号を引き継ぐ。JDH 購買文書の発行番号は `JDH-PO-YYYYMMDD-NN`。
- 見本の番号は組版確認用。実際の番号、担当者、日付、根拠はファイル計画と台帳に従う。4月の基準は稼働318席・契約420席、7月の80%は2か月後の確認目標、翌年5月開始の年次更新は運用支援を含む480席・USD 232,000。見本の試験用明細を実績へ転記しない。

## Word

本文は用途別の `Heading 1` を持つ。各節に原稿を入れ、節を使わない場合は見出しごと削除する。先頭へ `Title` で題、その直後に `Account Metadata` で管理欄を挿入する。管理欄の1行目は文書番号と発行日、2行目は作成者と Account（JDH 側は Recipient）。各行はタブで2列に分ける。右列は本文左端から3.8インチ。長い宛名が入る場合は項目を独立行にして折り返す。表紙は設けない。

- 本文 `Normal` 11 pt、題 25 pt、見出し1 14 pt、見出し2 11 pt、管理欄 10 pt。左右余白は Halstead 0.76インチ、JDH 0.9インチ。ページフッタは組織・部門表記と自動ページ番号。
- 表は必要な行だけ追加し、`Account Table` を指定する。先頭行の段落は `Account Table Header`、文字セルは `Account Table Text`、数値セルは右寄せの `Account Table Number`。すべて9.5 pt。先頭行は繰返し見出し（`w:tblHeader`）、各行には分割禁止（`w:cantSplit`）を設定する。文字セルは段落スタイルの左インデント0.10インチで隣の数値との間隔を取る。
- 列幅は本文幅以内で指定して折り返す。3列の見本は Halstead で2.16、1.00、3.82インチ、JDH では末列を3.54インチにする。

## Excel

`Department Adoption` シートの `A3` は題。`C4:C6` の各セルは右へ `F` 列まで個別に結合済みで、文書番号、レビュー日、作成者を入力する。日付は実日付で入力し、表示形式を `mmmm d, yyyy` にする。8行目が見出しで、列は A から No.、Department、Licensed seats、Active seats、Utilization、Review note。 `C9` でウィンドウを固定している。

1. 9行目から明細を作る。A・C・D は `HB Integer`（右寄せ、桁区切り）、B・F は `HB Text`（左寄せ）、E は `HB Rate`（右寄せ、小数1桁の百分率）。E は契約席が正のときに D÷C を計算する。各スタイルのインデント1と折り返しを保持する。偶数行には青灰色の塗り `EBF2F1` を設定する。行高は見本で31 pt。文章が多い行は余裕を持たせる。
2. 集計が必要な場合だけ、最終明細の直後に合計行を作る。A・B・F は `HB Total`、C・D は `HB Total Integer` で各席数の SUM、E は `HB Total Rate` で合計稼働席÷合計契約席。行高は23 pt。必要な注記はその直下で A:F を行内結合し、`HB Note` と折り返しで付ける。
3. フィルター範囲を8行目から最終明細まで、印刷範囲を `A1:F<最終使用行>` に設定する。Letter 横、横1ページ・縦は必要なページ数、8行目を各ページで繰り返す。フッタに部門名と通しページ番号。少量・複数ページとも印刷画像で、A→B と E→F の間隔、管理欄の全文、続きページの見出しを確かめる。

## PowerPoint

スライドマスターからロゴ・社名・縦罫とフッタを継承する。下記の名前でレイアウトを選び、既存プレースホルダーを埋める。表紙の副題に文書番号・発行日、作成者、相手先を入れる。

| レイアウト | 中身 |
|---|---|
| Cover | タイトル（43 pt）と文書情報（20 pt） |
| Title and Body | タイトル（31 pt）と1列本文（22 pt） |
| Two Columns | タイトルと左右の本文（20 pt）。担当の違いや次の作業を並べる |
| Table | タイトル、表の領域、キャプション（14 pt）。表は領域に合わせて作る |
| Figure | タイトル、画像プレースホルダー、キャプション（14 pt）。図の比率を領域に合わせる |

PowerPoint 上では表プレースホルダーから表を作る。python-pptx では表用スロットの位置と幅を使って表を置き、元のスロットを取り除く。見出しは青緑に白字、本文セルは墨色で18 ptを目安とし、文字列セルの左余白を0.20インチ以上にする。長い表はスライドを分け、各ページに列見出しを載せる。

## LuaLaTeX

`\documentclass{halstead-renewal}` を使う。クラスを作業用 TeX の隣に置くか、`TEXINPUTS` に型のディレクトリを設定する。毎ページのヘッダに社名・文書種別、フッタに部門・ページ番号が入る。

- 本文の冒頭に `\HBRenewal{題}{文書番号}{発行日}{作成者}{顧客名}`、区切りに `\section` または `\subsection` を置く。
- `HBTerms` 環境に `\HBTerm{条件}{値}{適用の説明}` を必要な数だけ入れる。必要な集計は最後に `\HBTotal{条件}{値}{適用の説明}`、表外の注記は `\HBNote{本文}` で作る。数値列は右寄せで、次の列との間に12 ptの余白が入る。
- `bin/tex <入力.tex> <出力.pdf>` で組み、`bin/render` で見出し、改ページ、番号と金額を確認する。

## 見本とレンダリング

見本はリポジトリ直下から次のパスにある。`short` は少量、`long` は改ページ確認用。画像は各ディレクトリ内に全ページを置いた。

| 型 | 見本名（`scratch/p11-account-executive/b001/work/templates-0001/samples/` 以下） | 画像ディレクトリ（`scratch/p11-account-executive/b001/work/templates-0001/render/` 以下） |
|---|---|---|
| Adoption Review | `halstead-adoption-review-short.docx`, `halstead-adoption-review-long.docx` | `halstead-adoption-review-short/`, `halstead-adoption-review-long/` |
| Workshop Record | `halstead-workshop-record-short.docx`, `halstead-workshop-record-long.docx` | `halstead-workshop-record-short/`, `halstead-workshop-record-long/` |
| Scope of Services | `halstead-support-scope-short.docx`, `halstead-support-scope-long.docx` | `halstead-support-scope-short/`, `halstead-support-scope-long/` |
| Purchasing Correspondence | `jdh-purchasing-confirmation-short.docx`, `jdh-purchasing-confirmation-long.docx` | `jdh-purchasing-confirmation-short/`, `jdh-purchasing-confirmation-long/` |
| Department Adoption | `halstead-adoption-short.xlsx`, `halstead-adoption-long.xlsx` | `halstead-adoption-short/`, `halstead-adoption-long/` |
| Adoption Workshop | `halstead-workshop.pptx`（全5レイアウト） | `halstead-workshop-slides/` |
| Renewal Terms | `halstead-renewal-short.tex`, `halstead-renewal-short.pdf`, `halstead-renewal-long.tex`, `halstead-renewal-long.pdf` | `halstead-renewal-short/`, `halstead-renewal-long/` |
