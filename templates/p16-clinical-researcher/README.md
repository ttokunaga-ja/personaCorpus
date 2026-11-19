# かつら川臨床研究センター　研究事務局の文書様式

| 名前 | 組織・用途 | ファイル | 見た目 |
|---|---|---|---|
| 研究計画書 | センター。鈴木が田中と確認する計画の草稿や改訂稿 | [katsuragawa-protocol.docx](katsuragawa-protocol.docx) | A4縦。独立表紙に題と管理欄、次頁から目的と記録の方針 |
| 会議記録 | センター。研究事務局が読み合わせの確認事項と担当を残す | [katsuragawa-minutes.docx](katsuragawa-minutes.docx) | A4縦。独立表紙なし。題と管理欄の後に議題・確認事項・対応と担当 |
| 統計相談書 | センター。鈴木が相談の背景と確認項目を別紙で整理する | [katsuragawa-consultation.docx](katsuragawa-consultation.docx) | A4縦。独立表紙なし。質問に答える際に参照する関連資料の節を持つ |
| 分母の比較 | センター。鈴木・石川が項目ごとの対象、記録あり、欠測理由を並べる | [katsuragawa-denominators.xlsx](katsuragawa-denominators.xlsx) | A4横。淡い赤紫の管理欄ラベルと白抜きの列見出し。続頁にも管理欄と列見出し |
| 研究会議 | センター。鈴木が会議で論点・空表・手続図を示す | [katsuragawa-meeting.pptx](katsuragawa-meeting.pptx) | 16:9。上下の細罫と余白。2段レイアウトの本文領域は淡い赤紫 |
| 研究概要 | センター。鈴木が作成し田中が確認する研究概要をPDFで共有する | [katsuragawa-summary.cls](katsuragawa-summary.cls) | A4縦。右上に研究概要、題の下に管理欄と確認者。本文を選択できるPDF |
| Statistical Consultation | 吉村統計研究合同会社。後藤が統計所見を文書添付で返す場合の型 | [yoshimura-statistical-note.cls](yoshimura-statistical-note.cls) | A4縦。右上に座標軸と3つの点・社名。青い見出しと左寄せの英文管理欄、下部の青罫 |

## 共通の書式と文書管理

- センターは円の中に中心と右上の観察点を置く図形と正式名称を組み合わせる。題・見出しと主図形は赤紫 `742747`、罫線と副点はくすみ紅 `AB586C`、本文は墨 `302830`。白地と赤紫の淡色 `F7EDF1` を背景に使う。
- Officeは和文・欧文とも **Noto Serif CJK JP**。WordのNormal・見出し・表・テーマ、ExcelのNormal・既定フォント・管理入力セル・名前付きスタイル、PowerPointのテーマとレイアウトに設定済み。センターのTeXは和文 Noto Serif CJK JP、欧文 Liberation Serif。
- センターのWord・PowerPoint・TeXはヘッダに図形と組織名。Excelはシートの先頭に持つ。フッタは研究事務局と頁。吉村のTeXは和文 Noto Sans CJK JP、欧文 Liberation Sans。青 `214D81` をロゴ・題・見出し・下罫、墨 `222D3C` を本文・管理欄に使う。
- センターの低用量観察研究は `KRC-LDO-YYYYMMDD-種別-連番3桁`。種別は `PRO`（計画書）、`MIN`（会議記録）、`QRY`（相談書）、`TAB`（表）、`MTG`（スライド）、`SUM`（概要）。日付は番号を最初に割り当てた文書の作成日。改訂時は元の番号を保ち、版と当該版の作成日を更新する。同日・同種別の別文書は連番を増やす。吉村の所見は `YST-LDO-YYYYMMDD-NOT-連番3桁`。どちらも作成者は実際に文書を書いた人物。
- 4月26日の48例は計画の目安、7月18日の12項目は統計計画の確認項目数。分母・記録あり・理由付き欠測を別欄にし、記録された0と欠測を区別する。

## Word

1. `Document(型のパス)` で開く。本文の最初の段落（`Title`）に題を入れ、続く2段落（`KRC Control`）に「文書番号・版」「作成日・作成者」を記入する。日付は `YYYY-MM-DD`。計画書は最初の「研究目的」の段落でページを改める。議事録と相談書は同じ頁から続ける。
2. 本文は `Normal` 11pt、段落後6pt、行間1.18。題は25pt、`Heading 1` は15pt、`Heading 2` は12pt。型の節見出しは原稿に合わせて削除・追加し、節の間に本文を入れる。
3. 表が必要な箇所だけ `KRC Research Table` を使い、列幅を合計160mm以内にする。見出し段落は `KRC Table Header` と赤紫 `742747` のセル背景、本文段落は `KRC Table Body`。続頁に見出しを出す表は先頭行に `w:tblHeader` を設定する。数値段落を右寄せする場合は、次の文字セルに左余白を増やす。注記は `KRC Note`。

## Excel

1. `load_workbook(型のパス)` で開く。シート名とA2の題を原稿に合わせ、C3に番号、H3に版、C4に日付型の値、F4に作成者を入れる。番号のC3:F3、日付のC4:D4、作成者のF4:H4は結合済み。管理欄は折返し付き。3行目23pt・4行目25ptを基準とし、内容が長いときは行高を増やしてPDF上で全文を確かめる。
2. 6行目が見出し。明細は7行目から原稿にある行だけを作る。A/C/D/E/Gは右寄せ・整数 `KRC Integer`、B/F/Hは左寄せ・インデント1の `KRC Text`。未入力の数値セルは `None` を代入して書式を付ける。欠測理由はF列に記し、未入力の数値セルは空欄のまま使う。理由別に行を分ける場合、項目の対象数は項目内の見出しとなる行だけに置き、繰り返した分母を足さない。幅はA:Hで7/24/12/12/16/21/12/29。短い明細は24pt程度から。空表の見本は折り返す注記に合わせて48pt。折返しと行高を印刷で確認する。
3. 合計は原稿にあり、対象が重ならず加算できる列だけ作る。明細末尾の次行にラベルを置き、`KRC Total Text` を指定する。同じ行の加算できる列に実際の範囲を指す式を入れ、`KRC Total Integer` を指定する。全行に同じ分母を記す表や空表では合計を設けない。注記は必要なときだけA:Hを結合して `KRC Note` にする。
4. `print_area` はA1から実際の最終欄まで。フィルタは `A6:H<明細の末行>` に留め、合計・注記にはかけない。C7固定、1〜6行の繰返し、A4横・幅1頁・高さ自動を維持する。数式があれば再計算して出力を確認する。短い版と続頁のある版の両方で、番号全文、隣接する版・日付・氏名、数値と文字の境目を見る。

## PowerPoint

- `Presentation(型のパス)` で開き、`slide_layouts[0:5]` を順に表紙、見出しと本文、2段、表、図として使う。題はプレースホルダ0。表紙の管理欄・本文・表・図は1、2段の右側は2。表紙の題34ptと副題18pt、他頁の題28ptと本文20pt。副題には文書番号と版、日付、作成者を入れる。
- 表は `placeholders[1].insert_table(行数, 列数)`。見本は18ptで赤紫に白抜きの見出し、本文は白または淡い赤紫の地に墨。図は `insert_picture` した後に、全体が見えるようにトリミングを解除し、位置・幅・高さを明示して元の縦横比で領域内に収める。表・図の領域は左0.72、上2.03、幅11.83、高さ4.55インチ。
- 頁数は成形時に右下（左12.08、上6.99インチ）へ墨色の11ptで置く。ロゴと研究事務局の表記はマスター上にある。見出しや本文が2行になったときは、続く要素との間隔も画像で確認する。

## TeX・メール・手続図

- センターは `\documentclass{katsuragawa-summary}`、`\KRCHeading{題}{文書番号}{版}{作成日}{作成者}{確認者}`、`\KRCSection{見出し}`、`\KRCNote{注記}` を使う。確認者は実際にその版を確認した場合に記載する。
- 後藤の文書添付には `\documentclass{yoshimura-statistical-note}`、`\YSTHeading{題}{参照番号}{作成日}{作成者}{宛先}`、`\YSTSection{見出し}` を使う。返信本文だけの場合は通常の英文メールにする。
- どちらもリポジトリ直下から `TEXINPUTS="$PWD/templates/p16-clinical-researcher:" bin/tex <作業フォルダの.tex> <出力.pdf>` で組む。
- EMLは通常のテキストメール。鈴木は `mitsuko.suzuki@katsuragawa-clinical.example`、後藤は `koichi@yoshimura-statistics.example`。返信は件名と返信ヘッダを元メールに対応させ、時刻は日本時間を使う。社名は署名で表す。
- PNGの手続図は原稿の流れに応じて組む。記録票、項目別の対象の確認、集計表の各段階を箱で示し、白地にセンターの色・和文書体を使う。

## 組版見本

以下はリポジトリ直下からのパス。見本原稿・出力は `scratch/p16-clinical-researcher/b001/work/templates-0001/samples/`、画像は `scratch/p16-clinical-researcher/b001/work/templates-0001/render/` にある。12行の空表は改頁確認用の項目例。仮定付きの間隔確認見本は数値欄が埋まったときの確認に使う。

| 見本ファイル（samples/） | 画像（render/） |
|---|---|
| `katsuragawa-protocol.docx` | `katsuragawa-protocol-1.png`、`katsuragawa-protocol-2.png` |
| `katsuragawa-minutes.docx` | `katsuragawa-minutes-1.png` |
| `katsuragawa-consultation.docx` | `katsuragawa-consultation-1.png` |
| `katsuragawa-denominators-short.xlsx` | `katsuragawa-denominators-short-1.png` |
| `katsuragawa-denominators-multipage.xlsx` | `katsuragawa-denominators-multipage-1.png`、`katsuragawa-denominators-multipage-2.png` |
| `katsuragawa-denominators-spacing.xlsx` | `katsuragawa-denominators-spacing-1.png` |
| `katsuragawa-meeting.pptx` | `katsuragawa-meeting-1.png`、`katsuragawa-meeting-2.png`、`katsuragawa-meeting-3.png`、`katsuragawa-meeting-4.png`、`katsuragawa-meeting-5.png` |
| `katsuragawa-summary.pdf` | `katsuragawa-summary-1.png` |
| `yoshimura-statistical-note.pdf` | `yoshimura-statistical-note-1.png` |
| `katsuragawa-record-flow.png` | `katsuragawa-record-flow.png` |
