# Jennifer Salazar の文書の型

## 型の一覧

| 名前 | 組織・担当と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 観察計画 | Carver Workplace Research LLC、Jennifer。配車の引継ぎ場面、観察する証拠、面談手順を英語で共有 | `carver-observation-plan.docx` | Letter 縦、表紙なし。段違いの三本線と社名、煉瓦色の小見出し。四つの場面と観察点を本文中の表で対応させる |
| 面談進行台本 | Carver、金子・Jennifer。日本語面談の発話、進行者への指示、聞き返しを区別 | `carver-moderator-guide.docx` | Letter 縦、表紙なし。区間と進行の合図に煉瓦色の太字を使用 |
| 調査判断記録 | Carver、Jennifer・金子。観察の範囲や質問の採否について判断・根拠・未決事項を記録 | `carver-research-decisions.docx` | Letter 縦、表紙なし。短い英語の見出しと、必要に応じて根拠の表を使用 |
| 日英質問対照 | Carver、Jennifer・金子。並べた質問を読み合わせ、言い直しや採否を照合 | `carver-prompt-review.xlsx` | Letter 横。日英の列を同幅に取り、黒鉛色の列見出しと煉瓦色の管理ラベルを配置 |
| 場面共有スライド | Carver、Jennifer。引継ぎの四場面や観察する証拠を共有 | `carver-field-brief.pptx` | 16:9、白地。小さな社名と図形ロゴを上に、ページの区切り線を下に置く。マスター1種・レイアウト5種 |
| 質問試行用紙 | Carver、金子。読み合わせに使う旧質問を余白のある紙面に組む | `carver-pilot-questionnaire.cls` | Letter 縦。麻色の管理欄、煉瓦色の区間見出し、番号付きの日英対。手で削除や言い直しを書き込める質問間隔 |
| 配送計画画面の説明 | Oakridge Dispatch Cooperative、Christine。Dispatch Systems から渡された画面説明 PDF | `oakridge-screen-reference.cls` | Letter 縦。分岐する配送路の図形と社名、濃紺のセリフ体の題、氷青色の管理欄と表見出し |

4月16日の合意と7月10日の試行が対象。見本の画面項目・質問文・文書番号は組版確認に用いる。実ファイルには計画と原稿で定めた内容を入れる。メールは実際の差出人・宛先・日時を持つ EML にし、言い直しの個人メモは TXT、画面への注釈は原稿に沿った PNG にする。

## Carver の書式

- 小規模な業務調査会社の社内資料として、煉瓦色 `B3463C` の段違いの三本線と正式社名を組み合わせる。黒鉛色 `252A35` を本文と表の濃色面、麻色 `ECE5DA` を補助面と罫線に使う。地色は白。Word・Excel のロゴは埋め込み画像、PowerPoint はマスター上の図形と文字、TeX は TikZ と文字。
- 日本語は Noto Sans CJK JP。Word・PowerPoint・TeX の欧文は Liberation Sans。Excel は見出しと本文・数値が混植になるため同一の Noto Sans CJK JP をブックの既定とし、英文だけの大見出し・ブランドと印刷フッタに Liberation Sans を使う。
- 文書番号は `CWR-DR-YYYYMMDD-用途-NN`。配車の見通し調査は `DR`。用途は `OBS`（観察計画）、`MOD`（進行台本）、`DEC`（判断記録）、`XLT`（対照表）、`BRF`（共有スライド）、`QST`（質問試行用紙）。日付部分と2桁の枝番は計画で定め、同じ文書の改訂は版で表す。作成日と担当者を文書番号とともに記載する。ファイルの作成者・更新者・日時は実際の担当者と計画の日時を使う。時刻の基準は America/Chicago。
- Word・質問用紙のヘッダに組織名と用途、フッタに Research Practice と頁番号。PowerPoint は組織名・研究部門名・罫線をマスターで共用。Excel は紙面上部にロゴと題を置き、印刷フッタに組織名と現在頁・総頁を出す。

## Word の成形

`Document(型のパス)` で開く。先頭の見出しの手前に `Title` の題、`CWR Metadata` の文書番号・日付・作成者を入れる。既存見出しの直後へ原稿の本文を配置し、使わなかった見出しを取り除く。進行の合図は `CWR Cue`、話す文と進行上の注記は別の段落にする。

- Letter 縦、左右0.86インチ、上1.16・下0.8インチ。Normal 11pt、Title 27pt、Heading 1 は14pt、Heading 2 は11pt、管理欄9.5pt、進行の合図10pt。段落の行間は1.16。表紙を作らず1ページ目に本文を続ける。
- 実表に `CWR Evidence Table` を設定し、先頭行は黒鉛色 `252A35` のセル塗りと `CWR Table Header`（白太字10pt）を使う。LibreOffice での印刷にもセル塗りが反映されるよう、先頭セル群に同色の `w:shd` を明示する。明細は `CWR Table Body`（10pt）にする。先頭行に `w:tblHeader`、明細に `w:cantSplit` を付け、実際の文量に合わせて列幅を決める。

## Excel の成形

`load_workbook(型のパス)` で開く。シート名は `Prompt review`。1行目にロゴ、2行目に題、3・4行目に管理欄、6行目に列見出し。明細・集計・注記は7行目から成形時に作る。

1. `B3:D3` の結合セルへ番号全文、`F3` へ版、`B4:C4` へ実日付（`yyyy-mm-dd`）、`E4:F4` へ作成者を入れる。3・4行目は24pt。入力先は空の状態で Noto Sans CJK JP 10pt、黒鉛色、左寄せ・インデント1、縦中央の設定を持つ。結合と幅を保ち、長い版名を使用するときは左右の管理欄をすべて埋めて印刷範囲内に収まるよう調整する。
2. A列の順番は `CWR Integer`（整数・右寄せ）、B〜E列の文字は `CWR Text`（左寄せ・インデント1）、F列の聞き手向け注記は `CWR Note`（9pt、左寄せ・インデント1）。列幅は A〜F の順に10、22、34、34、17、31。見本は明細42pt。日英を読める長さまで折り返し、長い行の高さは個別に増やす。先頭の番号列から区間名が離れて読めるよう、B列のインデントを維持する。
3. 件数を表示する場合は明細直後に24ptの行を1つ作り、A列へ `=COUNT(A7:A最終明細行)`、B列へその対象が分かるラベルを入れ、A〜F に `CWR Total` を適用する。B列だけは `CWR Text` の位置揃えをコピーして左寄せ・インデント1にする。集計行は麻色・太字・煉瓦色の上線と黒鉛色の二重下線。元の番号や区間の順序を合計しない。原稿に注記がある場合だけ、次の行の A:F を結合し `CWR Note` で記載する。
4. `auto_filter.ref` は `A6:F最終明細行`、`print_area` は `A1:F最終使用行`。Letter 横、幅1頁・高さ自動、1〜6行繰返し、ペインは C7。左右0.32インチ、上0.34・下0.55インチ。実データ行数および内容に合わせてプレビューを確認し、数式を入れた成果物は `bin/recalc` 後に `bin/render` で全ページを開く。

## PowerPoint の成形

`Presentation(型のパス)` で開き、対応するレイアウトからスライドを追加する。型は0枚。文字は英語 Liberation Sans、日本語 Noto Sans CJK JP、黒鉛色。マスター上部にロゴ、下部に煉瓦色の線と部門名を持つ。頁番号は成形時にフッタ行の右端（左12.13、上7.08インチ、右寄せ9pt）へ配置する。

| Index | レイアウト名 | 使い方 |
|---|---|---|
| 0 | Cover | 題46ptと副題21pt。副題に文書番号・作成日・作成者を入れる |
| 1 | Heading and text | 題32ptと本文22pt |
| 2 | Two columns | 題32pt、左右本文21pt。引継ぎを渡す側／受ける側などを対比する |
| 3 | Table | 題32pt、placeholder 1 に `insert_table`。18ptを目安に黒鉛色の見出しと白文字、明細は麻色と白を交互にする |
| 4 | Diagram | 題32pt、左の placeholder 1 に図、右の placeholder 2 に18ptの説明。図枠は8.2 × 4.5インチ。異なる縦横比の画像には余白を足して図全体を残す |

## TeX の成形

実行時は `TEXINPUTS="$PWD/templates/p09-ux-researcher:" bin/tex 原稿.tex 出力.pdf` とする。1ページ目から題と本文を記載する。

- Carver は `\documentclass{carver-pilot-questionnaire}`。本文冒頭に `\CarverDocument{題}{文書番号}{日付}{作成者}`、セクションに `\CarverHeading{見出し}`、設問に `\CarverQuestion{番号}{質問}{対訳}` を置く。単言語の設問は対訳に空の引数を渡す。旧質問票を保存する際は、旧版の本文を組み、原稿で定めた箇所に書込みを載せてから `bin/scan` で画像だけの PDF にする。
- Oakridge は `\documentclass{oakridge-screen-reference}`。濃紺 `183D52` と氷青 `D9E9EE`、配送経路の分岐図形を使う。欧文の表題・本文は Liberation Serif、管理欄・見出し・柱は Liberation Sans。和文がある場合はそれぞれ Noto Serif CJK JP と Noto Sans CJK JP。上部に社名と Dispatch Systems、下部に Screen Reference と頁を出す。
- Oakridge の番号は `ODC-DSP-YYYYMMDD-REF-NN`。本文冒頭に `\OakDocument{題}{番号}{日付}{作成者}` を置く。受領日を発行日に置き換えず、発行元の担当者と日付を記載する。節は `\OakHeading{見出し}`。表の先頭行は `\OakTableHeader` を使い、長文は `tabularx` で折り返す。
