# 千田真理子の文書の型

月次締め・年度監査・年度予算・経費精算・リース会計の検討で使う型。見出し・本文・表の書式と組織の識別を持つ。Officeの型は通常のDOCX・XLSX・PPTXなので、開いて別名保存する。

## 型の一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 確認記録・論点メモ | Cedar Works経理、堀口・千田。支払や差異の調査、予算審議の論点と判断記録 | `cedar-review.docx` | A4縦、表紙なし。小さな木材ロゴ、経理財務部／確認記録のヘッダ、深緑の見出し、罫線表。修正も本文中で読める余白 |
| 締め会議議事録 | Cedar Works経理、千田。決定・責任者・期限の記録 | `cedar-minutes.docx` | A4縦、表紙なし。共通ロゴと「議事録」のヘッダ、簡潔な見出し階層 |
| 部内通知・手順連絡 | Cedar Works経理、柳瀬・千田。確認手順や部内通知 | `cedar-notice.docx` | A4縦、表紙なし。「部内通知」のヘッダ、本文中心、必要時のみ表を追加 |
| 経費精算規程 | Cedar Works経理財務部、千田・柳瀬。旧規程、改定案、承認版の原本 | `cedar-expense-policy.docx` | A4縦、表紙なし。木材ロゴと「経費規程」のヘッダ、深緑の条見出し、フッタに部門名と自動ページ番号 |
| 月次報告パッケージ | Cedar Works Holdings Finance指定書式。千田が英語でDanaへ報告 | `holdings-monthly.docx` | A4縦、表紙なし。紺と青、菱形と横棒のロゴ、英語ヘッダ。日本子会社の木材ロゴは使わない |
| 勘定調整・予算ワーク | Cedar Works経理・FP&A。勘定調整、支払予定と換算、部門申請・前提・集約・修正差分・承認予算 | `cedar-reconciliation.xlsx` | A4横、深緑の見出し、円数値右寄せ、負数赤括弧、合計は薄緑と二重下罫線。用途に合わせて5行目の列を組み替える |
| 締めチェックリスト | Cedar Works経理、柳瀬・千田。工程、担当、期限、実績、確認者 | `cedar-close-checklist.xlsx` | 共通のA4横書式。工程／担当／期限／実完了／確認者／状況。架空の未完了工程を初期配置しない |
| 財務報告・審議資料 | Cedar Works経理、千田から大庭へ。CFO締め報告、予算審議草案と取締役会資料、リース会計の対応案 | `cedar-close-report.pptx` | 16:9（13⅓ × 7.5インチ、内部のサイズ種別も `screen16x9`）。白地、左上の木材ロゴ、深緑の題、下部の部門表記。マスター1種と実レイアウト5種（表紙、見出しと本文、2段、表、図）。型にはスライドなし |
| 部門説明・承認予算配布 | Cedar Works経理財務部、日下部・千田。部門向け説明と承認後の配布 | `cedar-budget.cls` | A4縦、Noto Sans CJK JP。木材ロゴ、深緑の題と表見出し、灰緑の本文、黄土の注記記号。部門名とページ番号をフッタに載せる |
| 経費申請Q&A・配布補足 | Cedar Works経理財務部、柳瀬・堀口。申請者向けの独立した配布PDF | `cedar-expense-guide.cls` | A4縦、Noto Sans CJK JP。木材ロゴと「経理財務部｜経費精算」、深緑の見出し、墨緑の本文、黄土の注記記号。横罫線表とページ番号 |
| 年度予算指示 | Cedar Works Holdings Group Finance、Dana Whitlockから日本の経理財務部へ。年度予算の前提と提出指示 | `holdings-budget.cls` | A4縦、Liberation Sans／Noto Sans CJK JP。紺と青、菱形と横棒のロゴ、青い太めの上罫、紺の英語本文、FINANCE • INTERNALとページ番号 |
| 残高証明書 | みなと中央銀行横浜支店、小田切。月次照合用の証明原紙 | `minato-balance.cls` | A4縦、Noto Serif CJK JPとLiberation Serif。えんじ色の円と山形のロゴ、細い罫線、広い余白、右下の発行者。Cedarの緑は使わない |
| Ledgerline帳票 | Cedar Works経理のLedgerline出力。補助元帳、照会、集約試算表 | `ledgerline-report.cls` | A4縦、Cedarの木材ロゴと深緑。右上にシステム名、横罫線中心の密度の高い表、基準日・出力日時を区別 |
| 実地棚卸票の印刷原紙 | Cedar Works静岡工場、三枝。区画別の手書き棚卸票の原紙 | `cedar-count-sheet.cls` | A4縦、木材ロゴと深緑、Noto Sans CJK JP。品番・品名と工程・実数・単位・備考の囲み表、数えた人と確認した人の署名欄。本文幅170mm |
| 北斗合板の請求書 | 北斗合板の請求管理。紙で届く当初・訂正請求書の原紙 | `hokuto-invoice.cls` | A4縦、茶 `654B35` と墨、Noto Serif CJK JP／Liberation Serif。合板の積層を表す長方形ロゴ、中央の明朝タイトル、右寄せ管理項目、枠付き請求額、細い横罫線 |
| 遠州金物の請求書 | 遠州金物の営業管理。紙で届く当初・訂正請求書の原紙 | `enshu-invoice.cls` | A4縦、鋼青 `384D6A` と墨、Noto Sans CJK JP／Liberation Sans。六角ナットの図形ロゴ、左寄せの大きな題と太線、白抜き請求額帯、横罫線明細 |
| 横浜なぎさタクシーの領収証 | 横浜なぎさタクシーが乗車後に交付する紙の領収証 | `yokohama-nagisa-taxi.cls` | 幅80mmの縦長レシート、黒一色、中央に発行者と領収証の題、金額は右寄せ |

既存の親会社・銀行・棚卸票・仕入先請求書の型は維持する。b006では経費精算の確認記録に `cedar-review.docx`、精算ポータルの稼働案内・設定修正の連絡に `cedar-notice.docx`、突合表・部門別経費集計・取消仕訳の確認・振込履歴の照合に `cedar-reconciliation.xlsx` を使用する。列名・幅は各原稿に合わせる。締め工程は `cedar-close-checklist.xlsx`、CFO報告と配布PDFは `cedar-close-report.pptx` を使い、PDFは完成したPPTXから直接変換する。LedgerlineのPDF帳票には `ledgerline-report.cls` を使う。精算ポータルのCSV・画面出力、千田の個人メモは会社の報告書の型を経由しない。b006に新しい社外組織の型は追加しない。Aoba StayとのメールはEML、画面保存はPNGであり、Officeの型は作らない。監査法人の型は追加しない。個人メモ、CSV、メールやERP画面を会社の報告書風に統一しない。

## Cedar Worksの共通書式

- ロゴは長さの違う3本の角材を階段状に並べた図形と社名。深緑 `264D3C` を題・見出し、黄土 `AE8244` を強調や図示、墨緑 `202924` を本文に使う。WordのNormalとFinance Table Body、ExcelのNormal・管理欄・CW Text・CW Amount、PowerPointの本文、経費申請Q&Aの本文は墨緑。灰緑 `59615C` はExcelのCW Note、PowerPointの副次表記、予算PDFの本文に使う。白・淡緑は背景色。
- 日本語は Noto Sans CJK JP。Word・Excel・PowerPointの日本語と英語も同フォントに統一。Ledgerlineの欧文は Liberation Sans。Word本文10.5pt、題22pt、見出し1は14pt、見出し2は11.5pt、注記9pt。段落後7pt、行間1.15。Wordの表は `Finance Table` を指定し、先頭行を見出しとして繰り返す。表の先頭行は深緑白字。先頭行のセル内段落には `Finance Table Header`、本文セルには `Finance Table Body` を必ず指定する（いずれも9pt）。本文に不要な空表は入れていない。
- WordはNormal、Title、Heading 1、Heading 2、Captionを使用。フッタは組織・部門と自動ページ番号。本文の先頭にTitle、次に文書番号・版・基準日または発行日・作成／確認者を原稿に従って入れる。番号、版、日付を1段落にまとめ、必要なら作成・確認者を次の段落に配置する。長い文書でも今回は独立した表紙を足さない。表の列幅は本文幅に合わせ、数値の右隣の文字セルの段落には左インデント3mmを付ける。続きページの先頭では深緑の表見出しを繰り返し、明細行の途中でページを割らない。
- 自社番号は `CW-FIN-YYYYMM-用途-連番`、Ledgerlineは `CW-LL-YYYYMM-帳票-連番`。実際の番号は事実台帳に従う。同一文書の下書き・修正・PDFは同番号で版を分ける。型自体に年月やダミー番号を焼き込まない。
- PowerPointは `Presentation(型)` を開き、`slide_layouts[0..4]`から選ぶ。題はidx=0、本文／表・図の領域はidx=1、2段の右はidx=2。本文19pt、題28pt、注記12ptを下限の目安とする。表・図レイアウトのobjプレースホルダは実体の表・図に置き換え、領域外へ出さない。表は深緑白字のヘッダ、数値右寄せ。数値の右隣の文字セルは左余白を約4.6mm（`Inches(.18)`）取る。原稿が持つ文書番号・版・日時を表紙の副題へ、ページ番号を各スライド右下へ実際の枚数に合わせて追加。発表者メモは原稿どおり格納する。

## Excelの行を作る方法

型には1〜3行目の組織・管理項目と5行目の見出しだけがある。管理ラベルと入力先が継承するNormalもNoto Sans CJK JPで、テーマフォントへの依存はない。4行目は見出しとの余白で未書式。6行目以降に空セルや仮の数式・合計を置かない。ブック内のNamedStyleを使う。

1. `load_workbook(型)`で開く。文書番号は結合したB2:C2のB2、版は結合したE2:F2のE2、基準日はB3、作成者はD3、確認者はF3に入れる。ラベルはA2=文書番号、D2=版、A3=基準日、C3=作成、E3=確認。管理欄は11pt、左インデント1・縦中央、2〜3行目の高さ26pt。文書番号は折返し・縮小を使わず1行で印刷する。初期幅はA=24、B=32、C〜E=22、F=24で、番号欄はB+C=54文字幅を確保している。基準日は実日付に `yyyy-mm-dd` を指定する。単位はセルではなく `ws.oddHeader.right.text` に原稿どおり設定する（調整表の初期値は「単位：円」、チェックリストは「月次締め工程」）。表の種類が違うときは5行目の列名と幅を変更する。列を足す場合も管理欄の結合を維持し、1行目の社名結合B1から最終列までと印刷範囲を広げる。管理欄と同じ列の幅も変わるため、Aは14以上、B+Cは54以上、D・E・Fは22以上を目安に確保し、正式番号と隣接する版・日付・氏名を埋めて印刷を確認する。収まらない番号は結合幅を広げ、管理ラベルと入力先を移設して衝突を避ける。
2. 原稿の明細だけを6行目以降に追加する。テキストは `CW Text`、金額・数式は `CW Amount`、注記は `CW Note`。金額・件数・先頭の連番は右寄せ（`CW Amount`）、文章は左寄せに設定する。`CW Text`・`CW Note`・`CW Heading` と既定Normalは左インデント1を型に設定済み。数値列の右隣の文字列はこのインデントを維持する。先頭の番号列の右隣も同じで、列を追加・並べ替え・狭めるときもインデントを0へ戻さず、文字幅と余白の分を列幅に含める。配置を上書きする場合も `Alignment(horizontal="left", vertical="top", wrap_text=True, indent=1)` とする。数値にインデントは付けない。全明細スタイルは折返し・上揃え。本文は10pt、注記9pt。日付・時刻は実値と必要な表示形式を設定する。USDは `#,##0.00;[Red](#,##0.00);"—"`、JPYは `#,##0;[Red](#,##0);"—"`。未確認の金額を0に置き換えない。
3. 集計が意味を持つ表だけ、最後の明細の次に合計行を作る。合計対象の実セルを指す `SUM` など原稿の数式を入れ、ラベル・数式セルに `CW Total` を適用。ラベルは左寄せ・インデント1へ変更し、数値は右寄せ・インデント0を維持する。無関係な勘定残高を足さない。未知額の合計を0としない。差異や調整後も原稿の式を使い、計算結果の確認はLibreOfficeで行う。
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

## b009で使う型

- 部門申請、前提、集約初稿、修正差分、承認予算は `cedar-reconciliation.xlsx` を開く。申請・差分の表では「番号／予算項目／申請額／修正差額／修正後／判断の根拠」を基に原稿に合わせる。差分は修正後から修正前を引く。前提表では値・単位・対象・根拠を分け、円/USDは数値、前年比は実値に百分率書式を適用する。単位混在の表には項目ごとの単位列を設ける。承認予算は承認した版の金額と承認日を示し、四半期の更新予測は別のファイルで扱う。
- 論点メモは `cedar-review.docx`、予算審議の草案と12月18日の取締役会資料は `cedar-close-report.pptx`。取締役会スライドと同じ内容の配布PDFが計画にある場合は、完成したPPTXから直接変換する。独立した部門説明・承認後の配布通知は `cedar-budget.cls`、親会社の英語指示は `holdings-budget.cls`。紙の部門申請・承認記録は `cedar-evidence.cls` で個別に組んで `bin/scan` に渡す。記録日時、承認者、署名の有無は各原稿に従う。
- 自社の番号は共通の `CW-FIN-YYYYMM-用途-連番`。FY2026でも年月部は発行月（12月なら `202512`）。親会社の年度指示は `CWH-BUD-FY年度-JP-連番`（例 `CWH-BUD-FY2026-JP-01`）。実際の番号・版・日付・作成者は計画と原稿に従う。
- 部門の要求、3回の修正の争点・却下案・金額差・判断日は計画で確定する。前提表と説明資料では148円/USD、木材価格全体の前年比＋6％、北斗合板の個別単価（18mm合板4,300円から4,200円、11月納品分から）を別の項目にする。北斗の覚書・11月締め・後日の予測はそれぞれの案件で扱う。
- 予算検討用実績CSV、差戻しと回答のEML、説明順のTXTは「型なしの記録」に従う。会議ホワイトボードのJPGは原稿に記した板書をimagegen skillで作る。

### 予算PDFの組み方

- 社内配布は `\documentclass{cedar-budget}`、`\BudgetHeading{題}{文書番号}{発行日}{作成者}{版}`、`\BudgetSection{見出し}`、`\BudgetNote{注記}`。表は `\begin{BudgetTable}{列指定}` を使う。本文幅170mm。3列の例は `p{38mm}>{\raggedleft\arraybackslash}p{21mm}p{91mm}`。列間には型の余白（片側7pt）を維持し、右寄せ数値と次の文字列の間に約5mmを取る。余白を含めて本文幅に収める。見出しを深緑白字にするには行頭に `\rowcolor{cedargreen}`、各見出しセルに `\textcolor{white}{見出し}` を指定する。
- 親会社は `\documentclass{holdings-budget}`、`\DirectiveHeading{Title}{Reference}{Issued}{Prepared by}`、`\DirectiveSection{Heading}`、`\DirectiveNote{Note}`。表がある場合は `\begin{DirectiveTable}{列指定}` を使う。本文幅162mm。3列の例は `p{55mm}p{36mm}p{54mm}`。発行者は親会社担当者、宛先・提出期限・指示は原稿から本文に入れる。
- 両表とも、見出しの後に `\\\midrule\endhead` を指定して継続ページで繰り返す。原稿にある明細だけを追加し、最終行の末尾の `\\` は省く。表が不要なら環境ごと省く。上記「TeXの使い方」に従ってテキストPDFへ組む。

## b010で使う型

- 不備集計と再申請照合は `cedar-reconciliation.xlsx` を開き、5行目から列を設定する。集計は費目・不備の種類と対象期間、照合は元申請番号・再申請番号・確認事項・処理状況を区別する。元申請と再申請の金額や件数を重ねて合計しない。欄・対象期間・差戻しと再申請の結論は計画と台帳に従う。
- 旧規程・改定案・最終規程は `cedar-expense-policy.docx`。条番号と条名はHeading 1、条文内の項目見出しはHeading 2、条文はNormal。共通の管理欄に加え、承認日・施行日・適用基準は原稿に該当するものを記載する。同じ改定案の版と同内容の配布PDFは同じ番号で追う。旧規程の番号が原稿にあるときは原本の番号を使う。正式規程と同内容の配布PDFは完成DOCXから `bin/office-pdf` で出す。
- 独立したQ&Aは `cedar-expense-guide.cls`。部内確認の記録は `cedar-review.docx`、議事録は `cedar-minutes.docx`、通知は `cedar-notice.docx`、改定説明は `cedar-close-report.pptx` の5レイアウトを用途に合わせて使う。説明資料ではタクシー利用理由と接待人数の内訳を別項目にし、施行前後の扱いと確認担当を原稿から示す。
- CSVは精算ポータルからの対象申請の出力、PNGは対象申請の画面、EMLは相談・差戻しのスレッド、HTMLは社内告知の保存ページの形で作る。紙の領収証は原稿で決めた発行者・証憑に合わせる。発行者の紙面とCedarの人数補記を区別し、手書きの補記と手元のJPGメモはimagegen skillで作る。紙面は `bin/scan` で画像のみのPDFへ成形する。発行者ごとの原本レイアウトに従い、Cedarロゴや社内番号を領収証の発行欄へ入れない。
- b006の移行済み64件・取消仕訳・2月18日の勘定科目勉強会、および2月締めの通常精算集計はそれぞれ既存の案件。b010の文書は今回の申請と規程改定を扱う。

- タクシー領収証は `\documentclass{yokohama-nagisa-taxi}`、`\NagisaReceipt{発行日時}{領収証番号}{領収金額}{但し書き}`。金額引数に円を付ける。中間のレシートPDFをTeXで組んでから `bin/scan` を使う。自社用の人数補記は別紙で `cedar-evidence.cls` の印字と手書きを分ける。

### 経費申請Q&Aの組み方

- `\documentclass{cedar-expense-guide}`、`\ExpenseHeading{題}{文書番号}{発行日}{作成者}{版}`、`\ExpenseSection{見出し}`、`\ExpenseNote{注記}` を使う。表が必要なら `\begin{ExpenseTable}{列指定}` を使う。本文幅は170mmで、番号を右寄せにした3列の例は `>{\raggedleft\arraybackslash}p{14mm}p{44mm}p{93mm}`。片側7ptのセル余白を保ち、番号の右隣の文字列との間隔を確保する。
- 見出し行は `\rowcolor{cedargreen}` と各セルの `\textcolor{white}{見出し}` で設定し、行末を `\\\midrule\endhead` として継続ページに繰り返す。原稿の実明細だけを入れ、最終行には末尾の `\\` を付けない。「TeXの使い方」の呼出しでPDFを組む。

## b011で使う型

- Cedar Worksの契約一覧・支払照合・判定表・条件別試算は `cedar-reconciliation.xlsx`。契約、支払実績、判定根拠、試算前提を列やシートで区別する。金額は円、利率はパーセント、期間は日付または月数で表記する。棄却した前提を含む旧版は当時の版として保存する。試算配布版は配布対象の版とシートを確定してから完成したXLSXを `bin/office-pdf` で変換する。
- 検討メモは `cedar-review.docx`、議事録は `cedar-minutes.docx`、大庭への対応案は `cedar-close-report.pptx`。採用案、試算の条件、残る確認事項を区別し、文書番号・版・日付・作成者は計画と事実台帳に従う。
- 東雲監査法人の説明資料は `shinonome-audit.cls`。岩城・能登の役割と発行日を管理欄に入れ、文書番号は今回の検討用に計画で定めた番号を使う。Cedar Worksが作る社内保管契約抜粋と確認票の印字原紙は `cedar-evidence.cls`。確認票の手書き部分と打合せ板書は原稿に基づきimagegen skillで作り、契約抜粋と確認票は `bin/scan` で画像のみのPDFにする。
- 既存支払・費用出力のCSV、照会と回答のEML、千田の再計算メモ（TXT）は「型なしの記録」に従う。

### b011のPowerPoint見本

配置確認用の表紙1枚版は `scratch/p14-finance-controller/b011/work/templates-0002/examples/cedar-close-report-cover.pptx`、5レイアウト版は `scratch/p14-finance-controller/b011/work/templates-0002/examples/cedar-close-report.pptx`。画像は同じwork内の `render/cedar-close-report-cover/` と `render/cedar-close-report/`。表紙は文書番号 `CW-FIN-202605-LEASEBRIEF-01`、0.2版、2026-05-19 15:00、千田 真理子を並べ、表では右寄せの件数と右隣の文字の間隔を確認した。見本の日付・件数は配置確認用の値。

## b010の見本と確認

b010で作成した見本は `scratch/p14-finance-controller/b010/work/templates-0002/examples/`、画像は `scratch/p14-finance-controller/b010/work/templates-0002/render/`。見本は配置確認用。申請額・発行日などの業務事実は計画と事実台帳を参照する。

| 型 | 見本（examplesからの相対パス） | 画像（renderからの相対パス） |
|---|---|---|
| cedar-review.docx | `cedar-review-short.docx`、`cedar-review-long.docx` | `cedar-review-short/`、`cedar-review-long/` |
| cedar-minutes.docx | `cedar-minutes-short.docx`、`cedar-minutes-long.docx` | `cedar-minutes-short/`、`cedar-minutes-long/` |
| cedar-notice.docx | `cedar-notice-short.docx`、`cedar-notice-long.docx` | `cedar-notice-short/`、`cedar-notice-long/` |
| cedar-expense-policy.docx | `cedar-expense-policy-short.docx`、`cedar-expense-policy-long.docx`、`cedar-expense-policy-distribution.pdf` | `cedar-expense-policy-short/`、`cedar-expense-policy-long/`、`cedar-expense-policy-distribution/` |
| cedar-expense-guide.cls | `cedar-expense-guide-short.tex`、`cedar-expense-guide-long.tex`、`cedar-expense-guide-short.pdf`、`cedar-expense-guide-long.pdf` | `cedar-expense-guide-short/`、`cedar-expense-guide-long/` |
| holdings-monthly.docx | `holdings-monthly.docx` | `holdings-monthly/` |
| cedar-reconciliation.xlsx | `cedar-reconciliation-3.xlsx`、`cedar-reconciliation-36.xlsx` | `cedar-reconciliation-3/`、`cedar-reconciliation-36/` |
| cedar-close-checklist.xlsx | `cedar-close-checklist-3.xlsx`、`cedar-close-checklist-36.xlsx` | `cedar-close-checklist-3/`、`cedar-close-checklist-36/` |
| minato-balance.cls | `minato-balance.tex`、`minato-balance.pdf` | `minato-balance/` |
| ledgerline-report.cls | `ledgerline-report.tex`、`ledgerline-report.pdf` | `ledgerline-report/` |
| cedar-count-sheet.cls | `cedar-count-sheet.tex`、`cedar-count-sheet.pdf` | `cedar-count-sheet/` |
| hokuto-invoice.cls | `hokuto-invoice.tex`、`hokuto-invoice.pdf` | `hokuto-invoice/` |
| enshu-invoice.cls | `enshu-invoice.tex`、`enshu-invoice.pdf` | `enshu-invoice/` |
| shinonome-audit.cls | `shinonome-audit.tex`、`shinonome-audit.pdf` | `shinonome-audit/` |
| cedar-evidence.cls | `cedar-evidence.tex`、`cedar-evidence.pdf`、`cedar-evidence-scan.pdf` | `cedar-evidence/`、`cedar-evidence-scan/` |
| cedar-budget.cls | `cedar-budget.tex`、`cedar-budget.pdf` | `cedar-budget/` |
| holdings-budget.cls | `holdings-budget.tex`、`holdings-budget.pdf` | `holdings-budget/` |

Excelの不備集計見本は `CW-FIN-202603-EXPCHK-01`、版1.0、基準日2026-03-13、作成堀口颯太・確認柳瀬文。締めチェックリストの見本は `CW-FIN-202602-CLOSE-01`、版1.0、基準日2026-02-28、作成千田真理子・確認柳瀬文。3行と36行で、番号と周囲の管理項目、先頭の連番および数値列と次の文字列との余白、繰返し見出し、合計がある場合は合計式、ページ番号を確認する。Wordと経費申請Q&Aは少数行と自然な改ページを含む長い表、PowerPointは5種のレイアウトを実スライドにして確認する。

他personaの既存README（p08-product-manager・p05-bi-data-analyst）を確認した。Cedarの深緑と木材ロゴは維持し、青緑の巡回経路、藍と朱の織り目とは区別している。

## b008で使う型

- 主要勘定調整・未払費用確認・JPY/USD換算は `cedar-reconciliation.xlsx`、締め工程は `cedar-close-checklist.xlsx`。残高と当月損益は別表または別シートとし、JPY・USDと適用レートの根拠を明示する。無関係な残高を総計しない。
- 締め会議は `cedar-minutes.docx`、英語報告は `holdings-monthly.docx`。英語報告のPDFは完成したDOCXから直接変換する。CFOへの5〜8枚は `cedar-close-report.pptx`。LedgerlineのテキストPDFは `ledgerline-report.cls`、必要な銀行資料の原紙は `minato-balance.cls` を使う。
- CSV、DanaとのEML、Ledgerline画面のPNG、Desktopの途中TXTは下記「型なしの記録」に従う。Desktopの確認用Excelは必要に応じて共通の調整表を使う。
- 2月通常取込の確認は1月の64件調査・取消・2月5日の設定確認・9日の引継ぎと分ける。2月固有の論点と残高増減・締め日・親会社レートは計画段階でpersonaの仕事の流れとfacts.mdに補足する。型の見本で未確定の業務事実を決めない。年度監査用の型は維持するが、今回の月次締めで監査成果物を再制作しない。

## 型なしの記録

| 対象 | 作り方 | 扱い |
|---|---|---|
| 千田の手元メモ（TXT） | `text` | 自分への短い確認メモ。ロゴ・文書番号・報告書のヘッダを付けない |
| 精算ポータル・旧精算照会からのCSV | `text` | 移行済と旧承認済の申請を装飾せず表データとして保存する。列・64行の新旧対応・金額・取得時刻はfacts.mdに従う |
| LedgerlineからのCSV | `text` | facts.mdで指定した列と行を直接出力し、帳票番号は列の値にする。説明の前置きや会社書式の装飾を付けない |
| Outlookの保存メール（EML） | `text` | 送受信ヘッダ、本文、引用、必要な実添付を保持する。Officeの型を経由しない |
| Ledgerline・精算ポータル画面の保存（PNG） | `screenshot` | 検索欄・結果表・状態を含む業務画面として構成し、報告書の用紙にはしない |

## 年度監査の追加書式（2025年12月期）

上の「監査法人の型は追加しない」はb006の対象範囲についての記録。年度監査では下記2型を追加する。Cedarの既存Office型は変更しない。

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 監査依頼・質問と受領整理 | 東雲監査法人、岩城・能登から千田・柳瀬へ。依頼リスト、質問番号、資料の受領と継続確認 | `shinonome-audit.cls` | A4縦、濃紫51465Fと墨303038、Noto Serif CJK JP／Liberation Serif。日の出と水平線の小ロゴ、上部発行者、文書番号・宛先・担当と確認者、横罫線表、発行者とページ番号。Cedarの木材ロゴを使わない |
| 検収・除却の紙控え | Cedar Works静岡工場と経理。検収記録、除却申請、補完承認記録の印字原紙 | `cedar-evidence.cls` | A4縦、Cedarの木材ロゴ・深緑、Noto Sans CJK JP。社内保管控えのヘッダ、管理番号・記録日・部署、印字項目と横罫線表。承認者・承認日時は本文から渡し、空欄承認と補完承認を区別する |

- 東雲：`\documentclass{shinonome-audit}`、`\AuditHeading{題}{番号}{発行日}{宛先}`、`\AuditContact{担当}{確認者}`、`\AuditSection{見出し}`、`AuditTable`環境（列指定を引数に取る）、`\AuditNote{注記}`。番号は `SN-CW-YYYYMM-PBC-連番`・`SN-CW-YYYYMM-Q-連番`。年度末YYYYMMを使い、実番号・期限・質問番号はfacts.mdで決める。表ヘッダと明細は原稿から追加し、`\endhead`で継続ページにヘッダを繰り返す。
- 社内紙：`\documentclass{cedar-evidence}`、`\EvidenceHeading{題}{管理番号}{記録日}{部署}`、`\EvidenceSection{見出し}`、`EvidenceTable`環境（列指定）、`\EvidenceField{項目}{値}`、`\EvidenceNote{注記}`。本文幅170mm以内に列幅と列間余白を含める。検収は品名・数量・単価・金額・日付と確認者、除却は資産の識別・故障と廃棄・申請と承認の記録を原稿どおり入れる。
- 両型に明細・空の承認欄・期限・仮署名・合計は埋め込まない。必要な空欄は個別原稿が明示した箇所だけに作る。今回は印字済みの紙控えであり手書きや印影を足さない。申請控えの大庭承認欄は空欄、補完承認は別紙に正しい2月の日付を印字する。
- TeXの探索パス・`bin/tex`の呼出しは既存と同じ。社内紙の中間PDFを当該実行のwork内に作り、`bin/scan`で画像のみのPDFへ変換する。元紙は配布物に追加しない。
- 依頼管理・残件は `cedar-close-checklist.xlsx`、買掛照合・年間資産増減・回答根拠は `cedar-reconciliation.xlsx`、回答と改善計画は `cedar-review.docx`、打合せ記録は `cedar-minutes.docx`、CFO説明は `cedar-close-report.pptx` を開いて使う。番号はfacts.mdの年度監査用を使う。
- 「b010の見本と確認」の `shinonome-audit.pdf` は質問資料の表と見出し、`cedar-evidence.pdf` は部門予算申請を例に管理項目と明細表の配置を確認する。
