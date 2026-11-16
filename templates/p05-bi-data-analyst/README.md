# 吉田美佐の文書の型

織庭生活雑貨株式会社の営業企画部・分析室が使う型。b001（2025-10-20〜2025-11-10の通販取消再注文の調査と週報訂正）用に作成した。型は本文を空にしてあり、Officeファイルは開いて別名保存する。文書の事実、対象週、数式、版、作成者・確認者は原稿と計画に従う。型の見本から補わない。MTR-09の定義確定やFY26の資料には広げない。

## 一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 分析報告 | 織庭、吉田から岡田へ。原因・影響範囲・対応の説明 | `oriba-analysis.docx` | A4縦、表紙なし。織り目のロゴと社名、藍色の見出し、罫線表。本文10.5pt、題24pt |
| 計算条件・確認記録 | 織庭、吉田・石井・前田。対象・除外条件・照合根拠の記録 | `oriba-condition.docx` | 同じ本文書式、ヘッダに「計算条件・確認記録」。短い節で確認事項を並べる |
| 訂正・連絡 | 織庭、吉田から関係部署へ。正式な訂正説明を文書として添付する場合 | `oriba-notice.docx` | 同じ本文書式、ヘッダに「訂正・連絡」。表紙なしの簡潔な通知 |
| 週次指標 | 織庭、吉田。訂正前後の週報、週・指標別の影響表 | `oriba-weekly.xlsx` | A4横、藍色の見出し、白地、淡い藍の合計行、数値右寄せ。対象週／指標／訂正前／訂正額／訂正後／根拠・状態 |
| 取消再注文の照合 | 織庭、吉田・前田。取消と再注文の対応、受領表と加工後の差の確認 | `oriba-matching.xlsx` | A4横。元受注番号／再受注番号／取消日／対象週／重複額／照合結果・根拠。週報と共通の色・印刷設定 |
| 営業会議週報 | 織庭、吉田から岡田と会議参加者へ。訂正前の報告と訂正後の説明 | `oriba-weekly.pptx` | 16:9、白地、左上の織り目と社名、下端に藍の帯。1マスター、実レイアウト5種（表紙、見出しと本文、2段、表、図）。型にスライドなし |
| 分析説明PDF | 織庭、吉田。Officeを経ずにPDFとしてまとめる影響範囲・照合条件の説明 | `oriba-analysis.cls` | A4縦、同じ織り目と藍・朱、Noto Sans CJK JP／Liberation Sans。細い横線、余白21mm、ページ番号 |

## 共通の意匠と番号

- ロゴは縦2本と横2本の帯を交差させた織り目と「織庭生活雑貨株式会社」の文字。藍 `304E67`、朱 `C5614F`、本文の墨 `303A42`。白地と淡い藍 `EAF0F3` は背景。朱はロゴ・図の差分など少量に使い、赤字の負数と意味を混同しない。
- Officeの欧文・日本語は Noto Sans CJK JP。TeXの欧文は Liberation Sans、日本語は Noto Sans CJK JP。いずれも環境にあるフォントを確認した。グラフPNGにも同じ色と日本語フォントを使う。増減の意味は色だけで示さず、符号と凡例も書く。
- ヘッダに会社・文書種別、フッタに営業企画部・分析室とページ番号。Excelは用紙上端の社名と印刷ヘッダ、用紙下端の部門・ページ番号を持つ。PowerPointのページ番号は成形時に実スライドへ追加する。
- 社内文書番号は `ORI-BI-YYYYMM-用途-連番`。用途は報告・確認・連絡・週報・照合など。実際の値は計画と事実台帳で確定する。同じ文書の訂正前後は同番号で版を分け、会議提出日・訂正日・対象期間を区別する。型にはダミーの番号や日付を入れていない。
- 他personaの既存型（p14のCedar Works、親会社・銀行・仕入先）を確認した。Cedarの深緑・木材ロゴ・下部の簡素な部門表記とは、織り目、藍と朱、PowerPointの下端帯で分けた。

## Word

`Document(型)`で開く。Normal（10.5pt、段落後7pt、行間1.18）、Title（24pt）、Heading 1（15pt）、Heading 2（12pt）、Caption（9pt）を使う。いずれも日本語フォントを明示。Titleの題名下罫線は藍 `304E67` を直接指定し、テーマのaccent1も同じ藍にする。表紙は追加せず、Titleの後に文書番号・版・日付・作成／確認者、次に本文を置く。

表は必要な場合だけ追加し、`Oriba Table` を指定する。先頭行に `Oriba Table Header`（9pt・白・太字）、明細に `Oriba Table Body`（9pt）を指定。先頭行は型の表スタイルで藍背景になる。成形時に先頭行の `w:tblHeader` を設定し、複数ページで見出しを繰り返す。原稿の長さに合わせて列幅・セル余白を設定する。型に空の表や仮の本文はない。

## Excelの行・式・印刷

1. `load_workbook(型)`で開く。1行目は社名、2行目は表題。B3=文書番号、D3=版、F3=基準日、B4=作成者、D4=確認者、F4=単位・対象期間。日付や期間が長い場合は行高を増やし折り返す。単位は必ず明記する。
2. 6行目が列見出し。表題、列名と幅は原稿に合わせて変える。必要列を増減したら社名・表題の結合、印刷範囲、見出し色を同じ幅に揃える。型の最終セル行は6。7行目以降に書式だけの行、仮の合計、未確認値は置いていない。
3. 原稿の実明細のみ7行目から追加する。NamedStyleの `OR Text`（10pt、左寄せ）、`OR Amount`（10pt、右寄せ）、`OR Note`（9pt、左寄せ）を各セルに指定する。すべて上揃え・折返し。金額は `#,##0;[Red](#,##0);"—"`、率は原稿どおり `0.0%` 等、日付は実値に `yyyy-mm-dd`。番号は文字列として保持する。空欄や未確認値を0にしない。
4. 合計が意味を持つ場合だけ最終明細の次に行を追加し、ラベルと計算セルへ `OR Total`（淡藍、太字、上細線・下二重線）を指定する。ラベルは左寄せへ変更し、数値は右寄せ。原稿の `SUM` や差額式を実セルの範囲へ入れる。週報の異なる指標を単純加算しない。訂正後は原稿の式で算出し、計算結果は `bin/recalc` で確認する。
5. 注記も原稿にある場合だけ最終行の下へ追加する。折返しに応じた行高と実最終行までの印刷範囲を設定。A4横、幅1ページ・高さ自動、1〜6行を繰返し、C7で固定。数字や根拠が縮みすぎる場合は表やシートを分ける。受領時点のCSVをこの型で装飾・上書きしない。

## PowerPoint

`Presentation(型)`を開き、`slide_layouts[0..4]`を選ぶ。題はidx=0、本文または図表領域はidx=1、2段の右はidx=2。本文20pt、題28ptを継承。表紙の副題に文書番号・版・提出日・作成者を入れる。全レイアウトの共通ロゴとフッタはマスターに実装してある。

表・図のobjプレースホルダは配置の基準（左0.7、上2.0、幅11.9、高さ4.5インチ）。実表・グラフ・画像をこの内側に置き、置換後は不要なプレースホルダを削除する。表の見出しは藍に白字、明細は白または淡藍、数字は右寄せ。本文は原則20pt、注記12pt以上。各スライド右下（左12、上6.95インチ付近）へ実ページ番号を10ptで追加する。原稿の発表者メモも保持する。訂正前の資料を後から訂正済みの内容に置き換えない。

## TeX

`TEXINPUTS="$PWD/templates/p05-bi-data-analyst:" bin/tex <scratch内の入力.tex> <scratch内の出力.pdf>` を使う。

- `\documentclass{oriba-analysis}`。`\AnalysisHeading{題}{文書番号}{版}{作成者／確認者}{日付}`、`\AnalysisSection{見出し}`、`\AnalysisNote{注記}` を使用。
- 表は `\begin{AnalysisTable}{列指定}`。最初に見出し行と `\\\midrule\endhead` を置く。本文幅168mm以内に収める。金額はr列、説明はp列を使う。行間は `\\`、最終行に改行を重ねない。実際の明細・必要な合計だけを呼出し側で書く。
- 既存Office資料の配布PDFは完成したOfficeから `bin/office-pdf` で出し、TeXで別内容を作り直さない。

## 社外文書・型を使わない記録

b001の依頼には社外が発行するOffice文書・PDFがなく、社外の型は作っていない。受注システムのCSVは社内の受領表であり、取引先の帳票ではない。将来、実際に社外の文書が計画される際は、その相手の型を別途作る。

CSV・TSV、EML、吉田の作業中のTXT・Markdownメモにはロゴや文書番号を付けない。計算条件の正式な記録にはWord、手元の検討には無装飾のメモを使い分ける。グラフPNGは元データ・軸・単位・凡例を原稿に従って描き、会社の用紙画像にしない。

## 初回作成時の見本と確認先

見本は `scratch/p05-bi-data-analyst/b001/work/templates-0001/examples/`、レンダリング画像は同じworkの `render/`。すべて組版確認用の仮内容であり、成果物や事実の根拠ではない。

| 型 | examples内の見本 | render内の画像フォルダ |
|---|---|---|
| oriba-analysis.docx | `oriba-analysis.docx` | `oriba-analysis-docx/` |
| oriba-condition.docx | `oriba-condition.docx` | `oriba-condition-docx/` |
| oriba-notice.docx | `oriba-notice.docx` | `oriba-notice-docx/` |
| oriba-weekly.xlsx | `oriba-weekly-3.xlsx`、`oriba-weekly-48.xlsx` | `oriba-weekly-3-xlsx/`、`oriba-weekly-48-xlsx/` |
| oriba-matching.xlsx | `oriba-matching-3.xlsx`、`oriba-matching-48.xlsx` | `oriba-matching-3-xlsx/`、`oriba-matching-48-xlsx/` |
| oriba-weekly.pptx | `oriba-weekly.pptx`（5レイアウト各1枚） | `oriba-weekly-pptx/` |
| oriba-analysis.cls | `oriba-analysis.tex`、`oriba-analysis.pdf` | `oriba-analysis-pdf/` |

実行結果は同じworkの `render-results.json`、実行ログは型別の `.log` と `tex.log`。scratch削除後は次のbatchのscratchへ仮内容を入れて再レンダリングする。受入済み成果物は変更していない。

初回作成時のレンダリングは9見本・計21ページで成功し、全ページの画像を確認した。Wordのテーマフォント指定を除去して再確認済み。構造確認は同じworkの `structure.json` に記録した。Officeの型は作者・更新者を吉田美佐、型の日付を2025-10-01に設定している。成形した成果物には、各原稿の日付と人物を改めて設定する。別実行のtemplates-reviewと統括の受入はこれから行う。

## 題名下罫線の修正と今回の確認先（templates-0003）

別実行のレビューで、Word 3型のTitle下罫線とテーマaccent1がOffice既定の青 `4F81BD` のままで、上記の共通配色と一致しないと判明した。`oriba-analysis.docx`、`oriba-condition.docx`、`oriba-notice.docx` の下罫線を藍 `304E67` の直接指定に修正し、その罫線のthemeColor指定を除いた。テーマaccent1も `304E67` に修正した。本文・表は空のまま、ヘッダ・フッタ・段落書式など他の構成は維持した。Excel・PowerPoint・TeXの型は変更していない。

今回の見本は `scratch/p05-bi-data-analyst/b001/work/templates-0003/examples/`、画像は同workの `render/` に置く。修正した各Word型と同名のDOCX見本を作り、画像は `render/oriba-analysis/`、`render/oriba-condition/`、`render/oriba-notice/` に保存する。分析報告は長い表を含む複数ページ、確認記録と訂正連絡は各1ページで、題名下罫線・繰返し表見出し・ヘッダ／フッタを確認する。仮内容は型には含めない。

構造検査は同workの `structure.json`、レンダリング結果は `render-results.json`、実行ログは各型名の `.log`。初回の「崩れなし」の確認では罫線の配色不一致を見落としていたため、この修正の根拠としない。今回の画像確認結果は `progress/p05-bi-data-analyst/b001.templates.md` に記録する。受入済み成果物は変更せず、別実行のtemplates-reviewは修正後に行う。
