# 千田真理子の文書の型

月次締め共通の型。2025年11月の静岡工場の実地棚卸・棚卸差異調査・月末支払にも使用する。型の本文は空で、見出し・本文・表の書式と組織の識別だけを持つ。Officeの型は通常のDOCX・XLSX・PPTXなので、開いて別名保存する。受入済み成果物は変更しない。

## 型の一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 支払・差異の確認記録 | Cedar Works経理、堀口・千田。調査の下書きと修正版 | `cedar-review.docx` | A4縦、表紙なし。小さな木材ロゴ、経理財務部／確認記録のヘッダ、深緑の見出し、罫線表。修正も本文中で読める余白 |
| 締め会議議事録 | Cedar Works経理、千田。決定・責任者・期限の記録 | `cedar-minutes.docx` | A4縦、表紙なし。共通ロゴと「議事録」のヘッダ、簡潔な見出し階層 |
| 部内通知・手順連絡 | Cedar Works経理、柳瀬・千田。確認手順や部内通知 | `cedar-notice.docx` | A4縦、表紙なし。「部内通知」のヘッダ、本文中心、必要時のみ表を追加 |
| 月次報告パッケージ | Cedar Works Holdings Finance指定書式。千田が英語でDanaへ報告 | `holdings-monthly.docx` | A4縦、表紙なし。紺と青、菱形と横棒のロゴ、英語ヘッダ。日本子会社の木材ロゴは使わない |
| 勘定調整・換算ワーク | Cedar Works経理。現預金・売掛・買掛・在庫・固定資産・未払費用、支払予定と換算 | `cedar-reconciliation.xlsx` | A4横、深緑の見出し、円数値右寄せ、負数赤括弧、合計は薄緑と二重下罫線。項目、根拠、調整前、修正額、調整後、確認状況の6列 |
| 締めチェックリスト | Cedar Works経理、柳瀬・千田。工程、担当、期限、実績、確認者 | `cedar-close-checklist.xlsx` | 共通のA4横書式。工程／担当／期限／実完了／確認者／状況。架空の未完了工程を初期配置しない |
| CFO締め報告 | Cedar Works経理、千田から大庭へ。5〜8枚の報告 | `cedar-close-report.pptx` | 16:9、白地、左上の木材ロゴ、深緑の題、下部の部門表記。マスター1種と実レイアウト5種（表紙、見出しと本文、2段、表、図）。型にはスライドなし |
| 残高証明書 | みなと中央銀行横浜支店、小田切。月次照合用の証明原紙 | `minato-balance.cls` | A4縦、Noto Serif CJK JPとLiberation Serif。えんじ色の円と山形のロゴ、細い罫線、広い余白、右下の発行者。Cedarの緑は使わない |
| Ledgerline帳票 | Cedar Works経理のLedgerline出力。補助元帳、照会、集約試算表 | `ledgerline-report.cls` | A4縦、Cedarの木材ロゴと深緑。右上にシステム名、横罫線中心の密度の高い表、基準日・出力日時を区別 |
| 実地棚卸票の印刷原紙 | Cedar Works静岡工場、三枝。区画別の手書き棚卸票の原紙 | `cedar-count-sheet.cls` | A4縦、木材ロゴと深緑、Noto Sans CJK JP。品番・品名と工程・実数・単位・備考の囲み表、数えた人と確認した人の署名欄。本文幅170mm |
| 北斗合板の請求書 | 北斗合板の請求管理。紙で届く当初・訂正請求書の原紙 | `hokuto-invoice.cls` | A4縦、茶 `654B35` と墨、Noto Serif CJK JP／Liberation Serif。合板の積層を表す長方形ロゴ、中央の明朝タイトル、右寄せ管理項目、枠付き請求額、細い横罫線 |
| 遠州金物の請求書 | 遠州金物の営業管理。紙で届く当初・訂正請求書の原紙 | `enshu-invoice.cls` | A4縦、鋼青 `384D6A` と墨、Noto Sans CJK JP／Liberation Sans。六角ナットの図形ロゴ、左寄せの大きな題と太線、白抜き請求額帯、横罫線明細 |

既存の親会社と銀行の型は維持するが、b005では使用予定なし。Aoba StayとのメールはEML、画面保存はPNGであり、Officeの型は作らない。監査法人の型は追加しない。個人メモ、CSV、メールやERP画面を会社の報告書風に統一しない。

## Cedar Worksの共通書式

- ロゴは長さの違う3本の角材を階段状に並べた図形と社名。深緑 `264D3C`、黄土 `AE8244`、文字の灰緑 `59615C`。白・淡緑は背景色。文書の主体は深緑で、黄土は必要な強調・図示だけに使う。
- 日本語は Noto Sans CJK JP。Word・Excel・PowerPointの日本語と英語も同フォントに統一。Ledgerlineの欧文は Liberation Sans。Word本文10.5pt、題22pt、見出し1は14pt、見出し2は11.5pt、注記9pt。段落後7pt、行間1.15。Wordの表は `Finance Table` を指定し、先頭行を見出しとして繰り返す。表の先頭行は深緑白字。先頭行のセル内段落には `Finance Table Header`、本文セルには `Finance Table Body` を必ず指定する（いずれも9pt）。本文に不要な空表は入れていない。
- WordはNormal、Title、Heading 1、Heading 2、Captionを使用。フッタは組織・部門と自動ページ番号。本文の先頭にTitle、次に文書番号・版・基準日・作成／確認者を原稿に従って入れる。長い文書でも今回は独立した表紙を足さない。
- 自社番号は `CW-FIN-YYYYMM-用途-連番`、Ledgerlineは `CW-LL-YYYYMM-帳票-連番`。実際の番号は事実台帳に従う。同一文書の下書き・修正・PDFは同番号で版を分ける。型自体に年月やダミー番号を焼き込まない。
- PowerPointは `Presentation(型)` を開き、`slide_layouts[0..4]`から選ぶ。題はidx=0、本文／表・図の領域はidx=1、2段の右はidx=2。本文19pt、題28pt、注記12ptを下限の目安とする。表・図レイアウトのobjプレースホルダは実体の表・図に置き換え、領域外へ出さない。表は深緑白字のヘッダ、数値右寄せ。原稿が持つ文書番号・版・日時を表紙の副題へ、ページ番号を各スライド右下へ実際の枚数に合わせて追加。発表者メモは原稿どおり格納する。

## Excelの行を作る方法

型には1〜3行目の組織・管理項目と5行目の見出しだけがある。管理ラベルと入力先が継承するNormalもNoto Sans CJK JPで、テーマフォントへの依存はない。4行目は見出しとの余白で未書式。6行目以降に空セルや仮の数式・合計を置かない。ブック内のNamedStyleを使う。

1. `load_workbook(型)`で開く。B2に文書番号、D2に版、F2に基準日、B3に作成者、D3に確認者。E3の単位も原稿に合わせる。表の種類が違うときは5行目の列名と幅を変更する。未払費用など6列に収まらない表では必要な列を追加し、タイトルの結合範囲、印刷範囲も広げる。
2. 原稿の明細だけを6行目以降に追加する。テキストは `CW Text`、金額・数式は `CW Amount`、注記は `CW Note`。金額は右寄せ、文章は左寄せに設定する。全スタイルは折返し・上揃え。本文は10pt、注記9pt。日付・時刻は実値と必要な表示形式を設定する。USDは `#,##0.00;[Red](#,##0.00);"—"`、JPYは `#,##0;[Red](#,##0);"—"`。未確認の金額を0に置き換えない。
3. 集計が意味を持つ表だけ、最後の明細の次に合計行を作る。合計対象の実セルを指す `SUM` など原稿の数式を入れ、ラベル・数式セルに `CW Total` を適用。ラベル左寄せ、数値右寄せ。無関係な勘定残高を足さない。未知額の合計を0としない。差異や調整後も原稿の式を使い、計算結果の確認はLibreOfficeで行う。
4. 折返しに合わせて行高を設定し、実際の内容の最終行まで印刷範囲を設定する。A4横、幅1ページ・縦自動、1〜5行繰返し、C6で固定、ページ番号フッタを維持。必要なら行数に応じて改ページを設定し、数字が縮みすぎる場合は表を分ける。

## 親会社と銀行

親会社は紺 `233C60` と青 `5287B4`、本文Liberation Sans、日本語が必要ならNoto Sans CJK JP。表用のFinance Table、Finance Table Header、Finance Table Bodyも欧文Liberation Sans・日本語Noto Sans CJK JPとする。Wordの英語見出し・表とページ番号を利用する。番号 `CWH-MR-YYYYMM-JP-連番` を本文先頭に入れる。JPYとUSD、期末と平均のレートを明示する。これは親会社指定の書式で、日本子会社の書式と分けている。

銀行はえんじ `783B45` と墨色 `292929`。本文・表罫線・ヘッダの横罫線・支店名・フッタは墨色、ロゴと銀行名はえんじとする。番号は銀行の `横浜残証-YYMMDD-連番`。会計の社内番号を追加しない。口座は下4桁のみ。`minato-balance.cls`で内容を組んだPDFを `bin/scan` に通す。型には押印や架空の証明内容は埋め込まない。必要な押印は原稿で指定された場合だけ成形する。

## TeXの使い方

型をLuaLaTeXから見つけられるよう、`TEXINPUTS="$PWD/templates/p14-finance-controller:"`を環境に渡して `bin/tex scratch/.../document.tex scratch/.../document.pdf` を実行する。ソースと出力はそのbatchのscratchへ置く。

- 銀行：`\documentclass{minato-balance}`、`\CertificateHeading{番号}{発行日}{宛先}{基準日}`、`BalanceTable`環境内に実際の預金種類・下4桁・金額の行、`\BankSignature{担当者}`。表の最後の行には改行記号を重ねない。
- Ledgerline：`\documentclass{ledgerline-report}`、`\ReportHeading{題}{帳票番号}{基準日}{出力日時}`、`\ReportNote{注記}`、`\begin{LedgerTable}{列指定}`。先頭行に見出しを入れ、`\endhead`で継続ページに繰り返す。数値列はr、文章はpを使い、本文幅174mmの内側に収める。表には実在する明細だけを入れる。

## 棚卸票と仕入先請求書

- 棚卸集計初版・再集計、在庫調整、支払予定は `cedar-reconciliation.xlsx` を開き、原稿に従って列名・幅・式を変える。区画、品番、棚卸票参照、帳簿、実地、差異、追記照合など必要な列だけを設ける。締め工程は既存チェックリスト、CFO向け報告は既存PowerPointを使う。
- 棚卸票：`\documentclass{cedar-count-sheet}`、`\CountHeading{管理番号}{実施日}{区画}{票番号}`、`CountTable`環境、`\CountSignatures{数えた人}{確認した人}`、`\CountNote{注記}`。管理番号体系は `CW-SZ-YYYYMM-COUNT-連番`。実際の番号は計画で確定する。表の明細は原稿にある行だけを成形時に追加し、行間の罫線は `\\\hline`、最終行は末尾の改行なしとする。型に空の明細・署名・仮の品番を埋め込まない。
- `cedar-count-sheet.cls` は用紙の印刷書式であり、手書きそのものを作る型ではない。原稿が指定する筆記・署名・品番追記はimagegen skillで作る。読める文字はすべて原稿に記す。活字の検査見本を手書きの成果物として使わない。手書き画像を `bin/scan` に通して画像のみのPDFとする。
- 請求書：それぞれ `\documentclass{hokuto-invoice}` または `\documentclass{enshu-invoice}`。`\InvoiceHeading{請求書番号}{発行日}{宛先}{対象期間}{支払期日}`、`\InvoiceTotal{税込請求額}`、`InvoiceTable`環境、`\InvoiceSummary{税抜小計}{税率}{消費税}{税込合計}`、`\InvoiceNote{注記}`。明細は品名・数量・単価・金額の4列。追加行の間は `\\`、最終行は末尾改行なし。数量の単位・税区分・端数処理・改定後単価の適用根拠・当初と訂正後の関係は原稿で指定する。未確定の税額を型で推算しない。
- 北斗の番号は `HK-YYMM-連番`、遠州は `ES-YYMM-連番`、訂正再発行は同じstemに `-R1` などの版を付す方針。実際の値は計画・事実台帳で定め、自社の文書番号を付けない。組織名はpersonaどおりとし、所在地・登録番号・口座番号・押印を型で捏造しない。
- 2社のクラスとも明細は空で、合計欄も呼出し時だけ生成する。表幅は本文170mm以内。フッタは相手の会社と部署・ページ番号。原稿の実明細がある原紙PDFを `bin/scan` へ渡す。当初・訂正後は同じ相手の型で作り、金額・注記・発行日を原稿どおりに区別する。
- 写真・ホワイトボード・個人メモのための会社文書型は作らない。写真はimagegen skillで制作し、写る文字は全文を原稿に書く。

## 見本と確認

b005の検査用見本は `scratch/p14-finance-controller/b005/templates-work/examples/`、画像は `scratch/p14-finance-controller/b005/templates-work/render/` に置く。以下は型に仮内容を入れた組版検査用であり、受入済み成果物や事実台帳の根拠ではない。請求額・品番・番号等も検査用で、制作時へ転記しない。b004の削除済みscratchへの参照は置換した。

| 型 | 見本（上記examplesからの相対パス） | 画像（上記renderからの相対パス） |
|---|---|---|
| cedar-review.docx | `cedar-review.docx` | `cedar-review/` |
| cedar-minutes.docx | `cedar-minutes.docx` | `cedar-minutes/` |
| cedar-notice.docx | `cedar-notice.docx` | `cedar-notice/` |
| holdings-monthly.docx | `holdings-monthly.docx` | `holdings-monthly/` |
| cedar-reconciliation.xlsx | `cedar-reconciliation-3.xlsx`、`cedar-reconciliation-36.xlsx` | `cedar-reconciliation-3/`、`cedar-reconciliation-36/` |
| cedar-close-checklist.xlsx | `cedar-close-checklist-3.xlsx`、`cedar-close-checklist-36.xlsx` | `cedar-close-checklist-3/`、`cedar-close-checklist-36/` |
| cedar-close-report.pptx | `cedar-close-report.pptx`（全5レイアウト） | `cedar-close-report/` |
| minato-balance.cls | `minato-balance.tex`、`minato-balance.pdf` | `minato-balance/` |
| ledgerline-report.cls | `ledgerline-report.tex`、`ledgerline-report.pdf` | `ledgerline-report/` |
| cedar-count-sheet.cls | `cedar-count-sheet.tex`、`cedar-count-sheet.pdf`（活字の配置確認用） | `cedar-count-sheet/` |
| hokuto-invoice.cls | `hokuto-invoice.tex`、`hokuto-invoice.pdf` | `hokuto-invoice/` |
| enshu-invoice.cls | `enshu-invoice.tex`、`enshu-invoice.pdf` | `enshu-invoice/` |

実行記録は同scratchの `render.log` と `render-results.json`。PowerPointは全5レイアウト、Excelは少数行と複数ページの見本で確認する。scratchは受入・破棄時に削除されるため、上記は今回のレビュー中の参照である。削除後の再検証では、使用する型に仮内容を入れて当該batchのscratchへ組版・レンダリングする。受入済み成果物の見本はこの一覧に登録していない。

他personaの型は作業時点で `templates/` に存在せず、比較対象なし。

## 型なしの記録

| 対象 | 作り方 | 扱い |
|---|---|---|
| 千田の手元メモ（TXT） | `text` | 自分への短い確認メモ。ロゴ・文書番号・報告書のヘッダを付けない |
| LedgerlineからのCSV | `text` | facts.mdで指定した列と行を直接出力し、帳票番号は列の値にする。説明の前置きや会社書式の装飾を付けない |
| Outlookの保存メール（EML） | `text` | 送受信ヘッダ、本文、引用、必要な実添付を保持する。Officeの型を経由しない |
| Ledgerline画面の保存（PNG） | `screenshot` | 検索欄・結果表・状態を含む業務画面として構成し、報告書の用紙にはしない |
