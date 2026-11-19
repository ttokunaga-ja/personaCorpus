# Vanessa Nguyen の文書の型

## 型の一覧

| 名前 | 組織・使う人と用途 | ファイル | 見た目 |
|---|---|---|---|
| Annotation Guidelines | Mercer、Vanessa Nguyen・Fatima Rahman。受付文の正解ラベル判定基準、旧版・改訂版、判定可能／保留の対例 | `mercer-label-guide.docx` | US Letter 縦、独立表紙なし。研究室のヘッダ、節見出し、対例を並べられる横罫線表 |
| Research Memorandum | Mercer、Vanessa Nguyen。評価条件、仮説の採否、Michael Serrano への判断材料、Eric Sandoval に送る前の考察と原稿 | `mercer-research-note.docx` | US Letter 縦、独立表紙なし。題と発行情報に続けて、論点を短い節に分ける |
| Lab Meeting Record | Mercer、Vanessa Nguyen・Fatima Rahman。研究会で確認した根拠、決定、次に確かめる点 | `mercer-lab-record.docx` | US Letter 縦、独立表紙なし。共通のヘッダ、下部にページ番号 |
| Lab Meeting Deck | Mercer、Vanessa Nguyen。研究会で境界例、評価の母数、論点を説明 | `mercer-lab-meeting.pptx` | 16:9、白地にプラム色の題、細い上罫線と黄土色の下部マーク。1マスター・5レイアウト |
| Adjudication Worksheet | Mercer、Vanessa Nguyen・Fatima Rahman・Joshua Ramirez。判定結果、案件と分割の対応、台帳の出所、原稿の比較条件を照合 | `mercer-adjudication.xlsx` | US Letter 横。白地、プラム色の列見出し、適用範囲・発行情報を印刷時に各頁に再掲 |
| Research Brief | Mercer、Vanessa Nguyen。研究計画、コレクション説明、再評価報告、誤分類の対例を直接 PDF に組む | `mercer-research-brief.cls` | US Letter 縦、独立表紙なし。Mercer の社名、二重罫線、番号・発行日・作成者、薄紫の表見出し |
| Independent Methods Review | Hartwell Evaluation Institute、Nicole Foster。独立した方法レビューの所見と対応の依頼。受領した PDF はこの Word 型から `bin/office-pdf` で出力 | `hartwell-methods-review.docx` | US Letter 縦、独立表紙なし。短くなる3本の水平線と機関名を中央に置き、青緑の単線とセリフ体を使う |

## 2025年の短文評価の撤回と再評価

- 1月23日の指摘は Hartwell の型で組む。照合表には案件キーと分割を記録し、集計の単位を見出しに書く。指摘の84件は文書数。案件数を載せる場合は案件キーの重複を除いて数える。
- 4月17日の再評価報告は Mercer の Research Brief に組む。旧 macro-F1 0.851 には旧分割と撤回理由、新しい 0.832 には案件単位の分割と採点条件を添える。比較図でも旧値を撤回済みと明記する。
- 5月23日の台帳点検は Lab Meeting Deck を使う。点検した96件と出所不明で対象外にした4件を区別し、モデルの優劣は保留と記録する。

## 2026年の誤分類読み合わせと初稿点検

- 1月12日の読み違いの対例は Research Brief を使う。対の文面を同じ行に置き、否定がどの動作に掛かるかを本文で説明する。160件は Nicole Foster と読み合わせた誤分類の件数。否定表現に関わる41件を独立して示し、残る119件は文面ごとの根拠で扱う。
- 読み合わせ前の考察と書き直した初稿は Research Memorandum を使い、各文書にその時点の考察を残す。同じ文書の改版なら系列番号を保ち、Issued と版を更新する。4月23日の初稿点検は Eric Sandoval への提出を3週間延ばし、抜けていた比較条件6項目を Adjudication Worksheet の1シートにまとめる。
- Vanessa がまとめる文書には Mercer の型を使う。Nicole 自身の書面が原稿にある場合は Hartwell の型を使う。読み合わせ結果の CSV・TSV、棄却した解釈の MD、ボード写真、提出時期を伝える EML はそれぞれの形式で保存する。

## 組織・文書番号

- **Mercer**。右上へ上がる3つの正方形と社名。プラム `#593875` を題、節、罫線に、黄土 `#A56720` を3つ目の正方形や短い罫線に、墨 `#242331` を本文に使う。淡いプラム `#EFE9F4` は表見出しや集計の背景。英字は Liberation Sans、日本語が必要な箇所は Noto Sans CJK JP。成果物の文面は英語。
- **Hartwell**。幅の違う水平線3本と機関名。青緑 `#245E65`、本文の濃紺 `#233746`、青緑の淡色 `#E6F0EF`。英字は Liberation Serif、日本語が必要な箇所は Noto Serif CJK JP。
- Mercer の短文受付意図分類は `MDR-STI-YYYYMMDD-TT-NN-RNN`。日付は系列の初版発行日、`TT` は文書種別、`NN` は同日・同種内の枝番。改版では末尾の版を進め、Issued 欄はその版の発行日にする。種別は研究計画 `RP`、文書集説明 `DD`、判定基準 `GD`、研究メモ `RM`、研究会記録 `LR`、研究会スライド `LM`、照合表 `AW`。例 `MDR-STI-20240513-GD-01-R02`（初版 May 13, 2024、改訂版の Issued は July 18, 2024）。
- Hartwell は同じ日付・枝番・改版の考え方で `HEI-YYYYMMDD-MR-NN-RNN`。例 `HEI-20250123-MR-01-R01`。どの型でも欄は Document ID、Issued、Author。本文の日付は英語の月名、時刻を併記する文書では America/Los_Angeles を使う。

## Word

- 型の題を原稿の題に置き換え、その直下に `Record` で Document ID、Issued、Author を入力する。本文は `Normal` 11 pt、題は `Title` 27 pt、節は `Heading 1` 15 pt・`Heading 2` 12 pt。図表注は `Caption`。左右 .86 in。ヘッダとページ番号のフッタを使う。
- 表がある原稿では、実データの行だけを挿入して `Research Table` を適用する。「見出し行」を有効にし、見出し段落は `Table Labels`、文字は `Table Text`、右寄せの番号・件数は `Table Number` にする。列の左右に 144 twips の表セル余白が入る。見出しを各頁に繰り返す。対例の行はページ内に収め、番号／判定できる例／保留する例には .68／2.94／3.16 in を起点に内容に応じた幅を使う。1行に対の両方を入れる。
- 旧版にも同じ型を使い、原稿に記録された版・日付・その時点の判定基準をそれぞれ入れる。既定スタイル、表用段落、テーマは組織の書体に設定済み。

## Excel

1. `Adjudication` シートの **B3:C3**（Document ID）、**E3:F3**（Issued）、**B4:C4**（Author）、**E4:F4**（Scope）は結合済み。各左上に実値を入れる。日付は日付型で `mmm d, yyyy`。管理欄は24 pt高で折り返す。
2. A2 の題と5行目の列名を用途に合わせて置き換え、既存の題の書式と `Column Label` を維持する。案件の照合では Ref.／Case key／Partitions／Documents／Disposition／Evidence / notes を起点にする。6行目から実データの明細を作る。A列の参照番号、D列の件数は `Detail Count`（整数 `#,##0`、右寄せ）、B・C・E・F列は `Detail Text`（左寄せ、折返し、左インデント1）。見本の明細行は27 pt。文字がさらに折り返す場合は行高を上げる。参照番号の隣の文字列と件数の隣の措置が離れて見えるように、文字列の左インデントを維持する。
3. 合算に意味がある場合、最後の明細行の次に A:C を結合して集計名を置く。件数は D列に実範囲を参照する `SUM` を入れ `Total Count`、その他の集計セルは `Total Text`。見本の集計は24 pt高。注記は次の行の A:F を結合し `Print Note`、見本は27 pt。母数と集計範囲の関係や保留の扱いをここに書く。集計・注記の行は必要な場合に作る。
4. 印刷範囲を A1:F の最終使用行へ広げる。US Letter 横、幅1頁・高さ自動、1〜5行を各頁に再掲する。左下に研究部門名、右下に頁番号。行高を調整した後に全頁で番号・見出し・集計・注記を確認する。Excel の Normal、未入力の管理セル、明細・集計の名前付きスタイルは Liberation Sans。日本語の補助フォントはテーマに設定している。

条件表では A:F の見出しを Ref.／Comparison condition／Evidence source／Status／Draft location／What to verify とし、A列に `Detail Count`、B:F列に `Detail Text` を使う。長めの説明がある場合は39 ptを起点に、折返した末尾が出るまで行高を広げる。6項目を6〜11行に記入し、必要な注記があれば12行目の A:F を結合する。

## PowerPoint

型はスライド0枚。スライドの寸法は 13⅓ × 7.5 in（16:9）で、ファイル内部のスライド種別も `screen16x9`。挿入するスライドの用途に合わせて5つのレイアウトを選ぶ。タイトルは左寄せ。共通ロゴ・社名・上罫線・フッタはマスターから表示される。

保存時は、ファイルの作成者・更新者と作成・更新日時にも原稿の担当者と計画の日時を設定する。

| レイアウト | 入力先 |
|---|---|
| `Cover` | 48 pt の題と19 ptの管理欄。ID、Issued、Author を一行で入れる |
| `Heading and Body` | 33 pt の題、23 pt の本文欄 |
| `Two Columns` | 題、21 pt の左右独立した本文欄。判定できる例と保留例を並べる |
| `Table` | 題、表プレースホルダ（idx 1）、表注（idx 2）。表は `insert_table`。先頭行はプラムに白字、明細は淡色に墨、見本は19 pt。左右のセル余白を各 .16 in 取り、数値を右寄せ、文字を左寄せにする |
| `Figure` | 題、画像プレースホルダ（idx 1）、右のキャプション（idx 2）。画像は `insert_picture`。16:9 の図版を用意し、図が切れないことを確認する |

## TeX

`\documentclass{mercer-research-brief}` とし、本文の先頭を `\MercerBrief{題}{Document ID}{Issued}{Author}` で組む。節は `\section` と `\subsection`。表は `longtable` にし、`\MercerTableHead{...}` と `\endhead` で淡色の見出しを各頁で繰り返す。文字列列は `L{幅}`、件数列は `R{幅}`。列の左右に 7 pt ずつ余白を取る。本文・セクション・管理欄・ヘッダ・フッタの各色はクラスで適用している。型を組版原稿と同じディレクトリに置くか、型のディレクトリを `TEXINPUTS` に含め、`bin/tex 原稿.tex 出力.pdf` で組む。

英語の曲線アポストロフィと引用符は原稿どおり UTF-8 で入力できる。クラスは U+2018、U+2019、U+201C、U+201D を Liberation Sans で組み、英単語の内部に和欧文間隔が入るのを防ぐ。

仮説メモは Markdown、食い違いのデータ交換は CSV、研究会のボードは写真を使う。判定データを確認・印刷する必要がある場合は Excel の照合表を使い、CSV には印刷用の題や注記行を追加しない。

## 見本とレンダリング

見本の入力ファイルは [examples](../../scratch/p04-ml-research-engineer/b004/work/templates-0002/examples/) に、`bin/render` の各頁は [render](../../scratch/p04-ml-research-engineer/b004/work/templates-0002/render/) にある。各入力の画像は render 内の拡張子を除いた同名のディレクトリにある。Hartwell の PDF の画像は `render/hartwell-pdf/` にある。

| 型 | 確認に使った入力 |
|---|---|
| Annotation Guidelines | `mercer-label-guide-short.docx`、`mercer-label-guide-long.docx`（対例表の短い版と複数頁版） |
| Research Memorandum | `mercer-research-note-earlier.docx`、`mercer-research-note.docx`（同じ系列番号で考察の改版を確認） |
| Lab Meeting Record | `mercer-lab-record.docx`（1月12日の読み合わせ） |
| Adjudication Worksheet | `mercer-adjudication-short.xlsx`（明細1行）、`mercer-adjudication-long.xlsx`（明細28行と集計）、`mercer-adjudication-conditions.xlsx`（条件表の6行） |
| Lab Meeting Deck | `mercer-lab-meeting.pptx`（5レイアウト。図版は `january-reading-counts.png`） |
| Research Brief | `mercer-research-brief-short.pdf`、`mercer-research-brief-long.pdf`（対例の短い版と複数頁版）。組版元は同名の `.tex` |
| Independent Methods Review | `hartwell-methods-review.docx` と、そこから出力した `hartwell-methods-review.pdf`（PDFからの画像は `render/hartwell-pdf/`） |

見本の対例、明細の識別子、条件の具体的な文言は書式確認用。成形時は各原稿の内容を使う。
