# 坂本修一郎の文書の型

対象は汐葛休憩棟改修の自主点検、完成検査、是正、仮使用条件、引渡し（2025年9月8日〜30日）。普通のDOCX・XLSX・PPTXを開き、別名で保存して使う。型は組織の識別、固定の文書種別、スタイル、列見出しだけを持つ。案件の事実・文書番号・是正内容は計画と原稿から入れる。

## 一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 検査・是正報告 | 湊梁建設、坂本・西村。自主点検、完成検査、是正の報告、写真照合 | `minatoryo-inspection.docx` | A4縦、表紙なし。梁のロゴ、橙のヘッダ罫線、青灰の左寄せ見出し。写真を含めて複数ページにできる |
| 現場日報 | 湊梁建設、青木・坂本。日々の作業、立会い、未完了と翌日の段取り | `minatoryo-daily.docx` | A4縦、表紙なし。同じロゴと色、本文と小見出しを中心とする簡潔な書式 |
| 仮使用条件確認 | 湊梁建設、坂本から和田へ。使用範囲、残る是正、期限、確認者を記録 | `minatoryo-conditions.docx` | A4縦、表紙なし。共通の社外提出書式。条件を見出しで分け、必要な場合だけ表を挿入 |
| 引渡し書類送付 | 湊梁建設、坂本・太田から和田へ。書類の送付、添付一覧、照合 | `minatoryo-handover.docx` | A4縦、表紙なし。共通の梁のロゴと青灰。宛先、対象、添付、照合者を本文で整理 |
| 是正確認・受領 | 汐葛食品、和田。施主側の確認結果・受領した書類の記録 | `shiokuzu-confirmation.docx` | A4縦、表紙なし。えんじの円と菱形のロゴ、明朝、細めのえんじ罫線。施工会社の橙・梁のロゴを使わない |
| 指摘管理台帳 | 湊梁建設、坂本・西村・太田。自主点検と完成検査の指摘、是正、根拠の対応 | `minatoryo-punch-list.xlsx` | A4横、梁ロゴ、青灰白字の見出し、通り芯・内容欄を広くする。日付は実値、注記は折返し |
| 引渡し書類照合表 | 湊梁建設、太田。検査記録・写真・是正報告などの版と保管先の照合 | `minatoryo-document-index.xlsx` | A4横、同じロゴと見出し。書類名、文書番号・版、日付、作成・照合者、保管先を持つ |
| 検査・引渡し説明 | 湊梁建設、坂本。施主への経過説明が原稿にある場合 | `minatoryo-site-brief.pptx` | 16:9、白地、左上に梁ロゴと社名、橙の下罫線。青灰の大見出し。マスター1種と実レイアウト5種。型本体はスライド0枚 |
| 現場検査票の印刷原紙 | 湊梁建設、坂本・西村。持出し用の検査票、自主点検記録を直接PDFに組む | `minatoryo-field-sheet.cls` | A4縦、梁ロゴと橙の罫線、ゴシック。番号・通り芯・点検内容・対応根拠・確認者の罫線表。複数ページでは列見出しを繰り返す |

汐葛食品からのメールはEMLであり、施主のWord書式を経由しない。今回、設計室・設備供給者・他の施主の型は追加しない。外部書式を施工会社からの提出物へ流用しない。

## 組織の識別と文書番号

湊梁建設のロゴは2本の柱と水平の梁、その上の橙の細線、社名文字の組合せ。OfficeのWord・Excelは図形を画像として埋め込み、PowerPointとTeXは同じ構成を図形で持つ。色は青灰 `263D4A`、橙 `CE702B`、本文の墨 `30383C`。橙は罫線や小さな強調に限り、記録の本文は墨で読みやすくする。中規模の施工会社の現場書類として、装飾的な表紙や大きな余白は足さない。日本語・欧文ともOfficeでは Noto Sans CJK JP。TeXの日本語は同じフォント、欧文は Liberation Sans。ヘッダはロゴと社名、フッタは湊梁建設株式会社・工事部とページ番号。Excelには組織名を上段、ページ番号を印刷フッタに持たせる。

自社番号の書式は `MB-SK-YYYY-用途-連番`。用途は検査 `IN`、日報 `DR`、使用条件 `UC`、送付 `HO`、指摘台帳 `PL`、書類一覧 `DX`、説明 `BR` とする。検査票も `IN` を使う。年は文書の年、連番は用途内で付す。実際の番号は計画・事実台帳で定め、型には埋め込まない。同じ文書のWordとPDFは同じ番号、改訂は版で区別する。写真や個人メモにはこの番号を強制しない。

汐葛食品はえんじ `814454` と墨 `30383C`、日本語・欧文とも Noto Serif CJK JP。円で囲んだ菱形と社名を使い、施工会社とはフォント・色・ロゴを変える。フッタは汐葛食品株式会社・工場管理とページ番号。施主の番号は `SK-KK-YYYY-連番`、版は別に記録する。型に担当者の署名、押印、所在地、電話番号、実在の会社の識別情報は入れていない。

他personaで既存の `templates/p14-finance-controller/` のREADMEとWord・Excel・PowerPointの実物を開いて比較した。Cedar Worksの深緑と木材の階段状ロゴ、親会社の青、銀行の明朝・えんじに対して、湊梁建設は青灰・橙と柱梁のロゴを採用。汐葛食品の円・菱形ロゴも銀行の山形ロゴとは異なる。

## Wordの使い方

- 本文は `Normal` 10.5pt、`Title` 21pt、`Heading 1` 13pt、`Heading 2` 11pt、`Caption` と `Site Note` は8.5pt。自社はゴシック、施主は明朝。段落後7pt、行間1.18。A4、左右21mm、上29mm、下22mm。全種類とも独立の表紙は作らない。
- 固定の文書種別の題の後に、原稿の工事名、宛先、文書番号、版、文書日、作成・確認者を `Site Metadata` 9ptで追加する。個々の見出し・本文は原稿から追加する。型に空の記入表や仮の署名欄はない。
- 表を追加する場合は `Site Table` を指定。先頭行を見出しにし、`w:tblHeader` を付けて次ページでも繰り返す。表の見た目の設定で「最初の行」を有効にし、ヘッダセルに `Site Table Header`（白字太字9pt）、明細セルに `Site Table Body`（墨9pt）を指定する。自社は青灰、施主はえんじの見出し背景。罫線は灰色。実際の明細だけを挿入し、行高は固定しない。数値は右寄せ、文章は左寄せとする。
- 写真は幅168mm以内、縦横比を維持して配置し、Captionに通り芯・撮影日・是正前後・写真参照を原稿どおり記す。大きい写真は改ページし、キャプションとの分離を避ける。写真自体はPicturesの原本と対応させる。
- 仮使用条件では使用範囲、立入を分ける箇所、残る是正、担当・期限、施主確認を混同しない。型は条件の内容や11件の内訳を決めない。引渡し書類は太田の照合記録と対応させ、受領側の記録は施主の型で作る。

## Excelの明細と合計を作る方法

型には1行目のロゴ・社名、2〜3行目の管理ラベル、4行目の列見出しだけがある。明細行、合計行、仮の式、空の罫線行はない。スタイルはブックのNamedStyleとして保持する。既定セル書式とNamedStyleのNormalはともにNoto Sans CJK JP・10ptで、テーマフォントに依存しない。管理項目の入力先B2・G2・B3・E3・G3もこの書体・サイズを継承し、文書番号のE2のみ9ptとする。

1. `load_workbook(型)` で開き、B2に工事名、E2に文書番号、G2に版、B3に基準日、E3に作成者、G3に確認者を入れる。日付は文字列ではなく実日付を入れる。B3には `MB Date` を指定する。E2は型に設定したNoto Sans CJK JP・9pt、左寄せ・上下中央・折返しあり、2行目の高さ32ptを維持する。`MB-SK-2025-PL-001`／`MB-SK-2025-DX-001` は2行に折り返して全文を表示する。E2へ明細用スタイルを上書きしない。番号体系を変えてさらに長くする場合は、実文量に合わせて2行目の高さを増やし、繰返し印刷される全ページで末尾を確認する。
2. 原稿の実明細を5行目以降に作る。文字は `MB Text`、整数・件数は `MB Number`、日付は `MB Date`、注記は `MB Note`。Noto Sans CJK JP、本文10pt・注記9pt、上揃え・折返し。文章は左、数値は右、日付は中央。整数は `#,##0;[Red](#,##0);"—"`、日付は `yyyy-mm-dd`。未是正・未確認をゼロや架空の日付で埋めず、状態と期限を区別する。
3. 集計が原稿に必要な場合だけ、最終明細の次に合計・件数行を作る。ラベルと実数式のセルへ `MB Total` を適用する（青灰の太字、淡灰背景、二重下罫線）。ラベルは左寄せへ変更し、数式は右寄せ。指摘番号がすべて入っている場合の件数は `COUNTA(A5:A最終明細行)`、完了数は実際の状態列の値を参照する。書類照合表に意味のない合計は追加しない。数式は成形後に `bin/recalc` で結果を確認する。
4. 行高は実文量に応じて設定する。短い明細の目安は30〜42pt、長い文章にはさらに高さを確保する。列を変える場合は見出し・幅・印刷範囲も実内容に合わせる。用意した7列ではC5で固定し、A4横、幅1ページ・高さ自動、1〜4行を各ページに繰り返す。印刷範囲を `A1:G最終内容行`、オートフィルタを4行目から最終明細行まで設定する。合計行はフィルタ対象に含めない。印刷倍率で文字が小さくなる場合は内容を分ける。

## PowerPointの使い方

`Presentation(型)` を開き、`slide_layouts[0..4]` の表紙・見出しと本文・2段・表・図から選ぶ。すべて実レイアウトで、見た目だけを空スライドにコピーしたものではない。プレースホルダのidxは題0、本文または表・図の領域1、2段の右2。表紙題32pt、その他の題28pt、本文19〜20ptが初期値。日本語はNoto Sans CJK JPで、テーマの欧文・東アジア書体も同じ指定にしてある。

表・図レイアウトの領域1は汎用オブジェクトのプレースホルダ。表や図形、画像をその座標・寸法以内に追加して置き換え、元の領域は削除する。表の見出しは青灰白字、本文は墨、最小18ptを目安とし、多すぎる行は分割する。図は縦横比を守り、通り芯や凡例を付ける。表紙の副題には原稿の文書番号・版・日付・発表者を入れ、各スライド右下には実際のページ番号を追加する。共通のロゴ・社名・罫線・部門表記はマスターから継承する。発表者ノートは原稿にある場合だけ追加する。

## TeXの使い方とスキャン

`TEXINPUTS="$PWD/templates/p17-construction-project-manager:"` を渡し、当該batchのscratchに置いた `.tex` を `bin/tex <入力.tex> <出力.pdf>` で組む。

- `\documentclass{minatoryo-field-sheet}`、`\SiteHeading{題}{文書番号}{工事名}{実施日}{記録者}` で開始する。
- `\SiteSection{見出し}`、`\SiteNote{注記}` は内容がある場合だけ使う。
- `FieldTable` 環境の各行に、番号、位置（通り芯）、点検・指摘内容、対応・確認根拠、確認者の5項目を `&` で区切って入れる。行末は `\\\hline`。環境側に列見出しと繰返し設定があり、原稿の明細だけを追加する。本文幅168mmの内側に収まる列幅を設定済み。長い記録は明細単位で改ページする。
- 型の中には仮の点検内容・空の記入行・署名欄を配置していない。印刷した検査票へ現場で筆記した記録を作る際は、原稿の内容を持つ原紙を先に組み、筆記そのものはimagegen skillを読んで作る。活字の見本を手書き記録の代わりにしない。最終的な内容入り画像またはPDFを `bin/scan` に通す。
- Word・Excel・PowerPointから保存するPDFはOffice経由とする。是正確認・受領の施主PDFは施主のWordを使い、TeXの施工側ロゴを付けない。

## 見本と画像

今回の組版検査の作業先は `scratch/p17-construction-project-manager/b001/work/templates-0001/`。`examples/` に仮内容を入れた見本、`render/` に `bin/render` の画像を置く。見本は受入対象でも事実台帳の根拠でもなく、その文面・通り芯・件数・番号を計画へ転記しない。型本体には見本の本文を残していない。

| 型 | examples/以下の見本 | render/以下の画像フォルダ |
|---|---|---|
| minatoryo-inspection.docx | `minatoryo-inspection.docx`（2ページ） | `minatoryo-inspection/` |
| minatoryo-daily.docx | `minatoryo-daily.docx` | `minatoryo-daily/` |
| minatoryo-conditions.docx | `minatoryo-conditions.docx` | `minatoryo-conditions/` |
| minatoryo-handover.docx | `minatoryo-handover.docx` | `minatoryo-handover/` |
| shiokuzu-confirmation.docx | `shiokuzu-confirmation.docx` | `shiokuzu-confirmation/` |
| minatoryo-punch-list.xlsx | `minatoryo-punch-list-3.xlsx`、`minatoryo-punch-list-38.xlsx` | 同じstemの各フォルダ |
| minatoryo-document-index.xlsx | `minatoryo-document-index-3.xlsx`、`minatoryo-document-index-38.xlsx` | 同じstemの各フォルダ |
| minatoryo-site-brief.pptx | `minatoryo-site-brief.pptx`（全5レイアウト） | `minatoryo-site-brief/` |
| minatoryo-field-sheet.cls | `minatoryo-field-sheet.tex`、`minatoryo-field-sheet.pdf`（2ページ） | `minatoryo-field-sheet/` |

全9種類について、11個の内容入り見本・計23ページを `bin/render` で画像化して目視確認した。TeXは `bin/tex` で組版成功。Wordの元テーマの書体と題下罫線、PowerPointの元マスターの文字サイズ・中央揃え・影を除去し、指定書式へ修正して再確認した。PowerPointの最終見本は型のレイアウトから文字書式を継承する方法で作り直して確認した。構造検査は `structure.json` に記録し、Wordは題のみ・表の実体なし、Excelは4行まで・数式なし、PowerPointは1マスター・5レイアウト・スライド0枚、型内に見本の文面や生成ライブラリ名がないことを確認した。書式資産の制作なので、振る舞いのRed→Greenに代えて組版・画像・構造を検証した。

レンダリングの結果は作業先の `render-results.json`、各 `*.render.log`、`tex.log`。scratchは後続の受入・破棄で消えるため、再確認時には使用する型に当該実行のscratchで見本を入れて再レンダリングする。別実行のtemplates-reviewによる検査と統括の受入はこの作業の後に行われる。

## 型を使わない記録

坂本の作業中のメモ、メール、CSV、現場写真にはロゴや文書番号を一律に付けない。写真はPicturesに撮影日で整理し、位置は実在住所でなく通り芯で示す。写真・手書きの画像を制作する段階ではimagegen skillを使う。本作業の図版見本は配置確認の幾何図形であり、現場写真でも手書き画像でもない。

## 文書番号欄の修正と再検証（b001・templates-0003）

両XLSXのE2は幅14／15・折返しなしのため、規定の文書番号がF2の「版」に遮られていた。E2のフォントを9pt、折返しあり・左寄せ・上下中央、2行目の高さを32ptへ修正した。番号の入力先、明細列幅、版の欄、印刷範囲と繰返し行は維持し、仮の番号や空の明細を型に追加していない。他のWord・PowerPoint・TeXの型と受入済み成果物は変更していない。

今回の見本は `scratch/p17-construction-project-manager/b001/work/templates-0003/examples/` の `minatoryo-document-index-3.xlsx`、`minatoryo-document-index-38.xlsx`、`minatoryo-punch-list-3.xlsx`、`minatoryo-punch-list-38.xlsx`。各型と同名の見本に規定形式の番号を入力し、`bin/render` で同作業先の `render/<見本のstem>/` に全ページを画像化した。見本の文面・番号は組版検査専用であり計画や成果物に転記しない。今回も書式修正のためRed→Greenの単体テストに代えて、実際の印刷画像と構造を検証する。検証結果は同作業先の `verification.json`、実行ログは `*.render.log` に置く。

再検証では両型それぞれ3行見本が1ページ、38行見本が4ページ、計10ページとなり、繰返しヘッダの文書番号が末尾の001まで見えることを全ページの画像で確認した。PDFの文字抽出でも全10ページに規定番号の全文があることを確認した。型本体は4行までで、E2は空、数式・仮の明細はない。LibreOfficeの初回変換は中断とPDF保存エラーがあったが、作業先の絶対パスを一時領域に指定し、`GSETTINGS_BACKEND=memory` を設定した再実行で4見本すべて成功した。変更前ハッシュとの比較ではREADMEと両XLSXだけが変更されている。別実行のtemplates-reviewと統括の受入は未実施。

## 汐葛の完成検査で用いる型なしの記録

- `text`：坂本のTXT覚書と保存したEML。ロゴ・文書番号を付けない。EMLは差出人・宛先・送受信時刻と返信関係を本文と一致させる。
- `photo`：Pictures/汐葛休憩棟改修/撮影日/の指摘・是正写真。型なし。同じ指摘の前後は通り芯・背景・向きをそろえ、原稿に小黒板の文字と写る対象を指定する。
- `handwriting`：坂本の検査手帳。型なしの手書き記録で、成果物の作り方は `scan:handwriting`。原稿に全頁の筆記内容を指定し、成形時にimagegen skillを読んで画像化した後bin/scanでPDF化する。検査終了時点の記録に後の是正結果を先取りしない。
- 施主の書類受領控えは `scan:office:shiokuzu-confirmation.docx`。内容を入れた施主の原紙に手書きの受領者名・照合印を加え、bin/scanに通す。型自体に署名を足さず、Office原紙と手書き素材は当該実行のwork内に置く。

今回の計画に新しいOffice・TeXの型は不要。PDFだけを保管する引継ぎ案内・施主確認書は既存の各型から作り、配布物に不要な元Wordを増やさない。

## Excelのフォント修正と再検証（b001・templates-0007）

両XLSXを開き、E2がCalibri・9pt、B2・G2・B3・E3・G3が既定セル書式のCalibri・11ptを使うことを確認した。NamedStyleのNormalだけでは既定セル書式を変更できていなかったため、既定セル書式をNoto Sans CJK JP・10pt、E2の個別書式を同フォント・9ptへ修正し、両者のテーマ書体指定を外した。E2の折返し・左寄せ・上下中央、2行目32pt、印刷設定は維持した。XLSX内部で変更したのは `xl/styles.xml` だけで、ロゴ・セルの値・行数・NamedStyleは維持している。

今回の作業先は `scratch/p17-construction-project-manager/b001/work/templates-0007/`。修正した入力欄の書体を反映した組版検査専用の見本を以下に置いた。見本文面は計画・事実台帳へ転記しない。

| 型 | 作業先からの見本パス | 画像フォルダ |
|---|---|---|
| minatoryo-document-index.xlsx | `examples/minatoryo-document-index-3.xlsx`、`examples/minatoryo-document-index-38.xlsx` | `render/minatoryo-document-index-3/`、`render/minatoryo-document-index-38/` |
| minatoryo-punch-list.xlsx | `examples/minatoryo-punch-list-3.xlsx`、`examples/minatoryo-punch-list-38.xlsx` | `render/minatoryo-punch-list-3/`、`render/minatoryo-punch-list-38/` |

`bin/render` と `bin/office-pdf` は4見本すべて終了コード0。各型の3行見本は1ページ、38行見本は4ページで計10ページ。全ページの画像を確認し、番号の末尾001、管理項目と繰返し列見出しの表示を確認した。PDFの文字抽出でも全ページに文書番号の全文がある。型を再読込して6入力欄の書体・サイズ・テーマ指定なしを検証し、型は4行まで・入力値と数式なしを確認した。書式だけの修正のため、TDDに代えて実ファイルの書式・構造と印刷画像を検証した。

証跡は同作業先の `structure-verification.json`、`render-verification.json`、`hashes-before.json`、`final-verification.json` と各見本の `*.render.log`・`*.office-pdf.log`。Word・PowerPoint・TeXの既存型と受入済み成果物は変更していない。別実行のtemplates-reviewと統括の受入は未実施。
