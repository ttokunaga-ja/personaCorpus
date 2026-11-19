# Judith Morris の文書の型

| 型 | 組織・用途 | ファイル | 見た目 |
|---|---|---|---|
| Working outline | Langley Historical Trust。Judith が章順を整理し Roger Hewitt に回覧する草稿 | [langley-outline.docx](langley-outline.docx) | A4縦。題の下に回覧情報、章順に小見出し。レビューコメントは Word の校閲欄 |
| Evidence note / Translation draft | Langley Historical Trust。Judith が史料の所在と推論の限界を記し、Julie と仏語書簡の訳を確認する | [langley-evidence-note.docx](langley-evidence-note.docx) | A4縦。本文、参照表または対訳表。淡い葡萄色の見出しと細い横罫 |
| Access brief | Langley Historical Trust。Judith が Camille Laurent 宛の閲覧申請で使う照会書や添付案 | [langley-access-brief.docx](langley-access-brief.docx) | A4縦。宛先、対象史料、取扱いと受渡しの希望を一続きに読める配置 |
| Research discussion | Langley Historical Trust。Judith と研究班の章構成・典拠の検討 | [langley-research-briefing.pptx](langley-research-briefing.pptx) | 16:9。白地の上下に細い罫線。表紙・本文・2段・表・図を収録したマスター |
| Leaf register | Langley Historical Trust。Judith が箱・請求記号・葉範囲と閲覧／複写条件を照合する表 | [langley-leaf-register.xlsx](langley-leaf-register.xlsx) | A4横印刷。組織名と管理欄、淡い葡萄色の列見出し。件数列は右揃え |
| Research paper | Langley Historical Trust。Judith が章の論点や、前後の文を含む仏文と英訳を直接PDFに組む | [langley-research-paper.cls](langley-research-paper.cls) | A4縦。セリフ体、番号付きの節、典拠注、自動頁番号。対訳は出典と注を添えた左右2列 |
| Inventaire sommaire | Archives de Saint-Aubin-sur-Rive。Camille が閲覧室で交付する紙の箱別目録 | [saint-aubin-inventory.cls](saint-aubin-inventory.cls) | A4縦。高さの異なる3本の背表紙と仏語の所蔵館名、深緑の帯、サンセリフ体、5列の目録 |
| Correspondance imprimée | 史料中の地方読書会。書記が発行した活字刷りの書簡を紙面に組む | [reading-society-letterprint.cls](reading-society-letterprint.cls) | A5縦。墨茶1色、中央の会名と短い罫線、右寄せの発信日、字下げした本文と結び |

## 組織の意匠と管理欄

- Langley は葡萄色 `603346` の菱形と組織名、青銅色 `9A6C39` の細罫、墨色 `292526` の本文を使う。表の背景 `F1E8EC` は葡萄色の淡色。York と文書用途はフッタ左、頁番号は右。比較した他 persona のロゴや罫線の配置から区別し、小規模な研究組織の回覧文書らしくセリフ体と余白を中心にした。
- Langley の Word・PowerPoint・TeX は欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。Word は既定の Normal と各段落・文字・表のスタイル、テーマの欧文／和文、PowerPoint はテーマとマスター・レイアウトに指定済み。Excel は一つのセルに仏語と日本語を併記できるよう欧文を含め `Noto Serif CJK JP` を使う。ブックのテーマ、既定の Normal、見出し、管理欄の入力セル、明細の名前付きスタイルに指定済み。Excel のテーマ配色も Langley の葡萄色、青銅色、墨色にそろえる。
- Archives は深緑 `28483C`、濃い墨緑 `202923`、深緑8%の見出し帯。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。ヘッダには背表紙3本と所蔵館名、フッタには `Salle de lecture` と頁番号。活字刷りの史料は墨茶 `2D2823`、欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。史料の会名、日付、発信者は原稿で決まった情報を使う。歴史的な書簡には現代の所蔵館ロゴ・管理欄・頁番号を付けない。
- Langley の発行番号は `LHT-YYYYMMDD-用途-連番3桁`。用途は `OUT`（章構成）、`EVD`（典拠メモ・訳稿）、`REQ`（閲覧照会書）、`BRF`（研究説明）、`REG`（葉別台帳）、`PAP`（論点稿・対訳資料）。Archives の現代の目録文書は `ASR-YYYYMMDD-INV-連番3桁`。目録文書の番号と各明細の史料請求記号は別に扱う。
- 日付は英国式 `25 April 2024`、仏語目録は `9 juillet 2024` の順で記す。実番号・版・日付・著者は計画と原稿で確定する。改稿は同じ番号の版で追い、同じ文書をPDF保存する場合は番号・版を引き継ぐ。作成者と確認者または宛先は別の欄に置く。メール・個人メモ・歴史史料に現代の発行番号を付けない。

## Word の成形

1. `Document(型のパス)` で開く。1段落目の用途表示に続け、最初の節見出しの前へ題（`Title`）、発行番号・発行日・作成者（`LHT Register`）、必要なら宛先・回覧先・版（同じスタイル）の順に挿入する。章は `Heading 2` で分ける。本文は `Normal`、典拠注は `LHT Note` を使い、原稿にない節は外す。
2. A4縦、左26・右24・上29・下23 mm。表紙なし。本文11.5 pt・行間1.14・段落後6 pt、題26 pt、見出し15／12 pt、管理欄・注記10 pt。ヘッダに菱形と組織名、フッタに York・用途と自動 `PAGE` フィールド。
3. 表は実際の行数で追加して `LHT Reference Table` を指定し、見出しセルの段落に `LHT Table Header`、本文に `LHT Table Text`、数値に右揃えの `LHT Table Number` を付ける。目安の列幅は左から16、34、49、61 mm。見出し行の `w:trPr` に `w:tblHeader` を、各行に `w:cantSplit` を付け、行高を固定しない。表スタイルの左右145 twips（約2.6 mm）のセル余白を維持し、箱番号と次の文字列を離す。
4. 訳稿では Evidence note の用途表示とフッタの用途名を `Translation draft` とし、フッタのタブと `PAGE` フィールドを保つ。節は `Scope`、`Passages under review`、`Implication for Chapter 2` を目安にする。箇所ごとに請求記号・葉の表裏・行を小見出しで示し、対訳表（80 mm × 2列）の下に `LHT Note` で語義や判断の根拠を置く。Julie の指摘は、該当する訳語や文の run に `doc.add_comment(run, コメント, author="Julie Perrin", initials="JP")` で付け、実際の日時を設定する。章構成の回覧稿では回覧時の確認者に合わせる。印刷画像では本文の組版、DOCX の校閲欄ではコメントの位置・作成者・日時を確認する。

## Excel の成形

1. `load_workbook(型のパス)` で開く。1行目はロゴと組織名、2行目は題、3〜4行目は管理欄、6行目は列見出し。B3:D3 に発行番号の全文、F3:G3 に実日付、B4:D4 に作成者、F4:G4 に対象箱・章などの範囲を入れる。結合の左端セルに入力する。入力先は10 pt・折返し・インデント1。3〜4行の高さ29／32 ptと結合幅を保ち、長い範囲名に必要なら行高を増やす。
2. 7行目から、原稿にある明細だけ追加する。A（箱番号）とE（葉数）は `LHT Integer`（右寄せ・インデント1・`#,##0`）。B（請求記号）・C（葉番号／範囲）・D（史料群）・F（閲覧／複写条件）は `LHT Text`（左寄せ・インデント1・折返し）。G（所在や読解の注）は `LHT Note`（9 pt斜体）。請求記号や表記に意味のある葉番号は文字列で保持する。未確認の値を0で埋めない。Aの右のB、Eの右のFはインデントを保つ。見本の明細は36 ptで、実際の折返しに応じて高さを変える。
3. 葉数を集計する場合は、明細の直後に必要な集計行だけ作る。A:D を結合してラベルに `LHT Total Label`、Eの数式に `LHT Total Integer` を指定する。行高は23 ptを目安にする。例として F列で `Copy permitted` と `Missing` を管理する場合は、各語を条件に実明細範囲だけを指す `SUMIF` でE列の葉数を別々に加える。複写可能と複写実施は別の状態として扱う。注記は必要なときだけ次の行のA:Gを結合し `LHT Note` を使う。
4. 印刷はA4横・幅1頁・縦自動で、1〜6行を繰り返し、C7で画面を固定する。フィルタ範囲は A6:G最終明細行、印刷範囲は A1:G最終使用行とし、集計・注記はフィルタから外す。最終頁に集計だけが残るときは、本文の可読性を保って行高や改頁を調整し、集計と直前の明細を一緒に読めるようにする。数式は `bin/recalc` で確認し、`bin/render` の全頁で横幅・見出し・番号・フッタ・集計位置を確認する。

## PowerPoint の成形

`Presentation(型のパス)` から次の実レイアウトを使う。型はスライド0枚。題は idx 0。表紙題43 pt、他の題32 pt、本文20 pt。組織名と罫線、York と用途はマスターにある。

| index | レイアウト | 使い方 |
|---:|---|---|
| 0 | Cover | 副題 idx 1 に発行番号、日付・作成者、必要なら会議名・版 |
| 1 | Heading and text | 本文 idx 1 に要点 |
| 2 | Two columns | 左 idx 1、右 idx 2 に比較する観点 |
| 3 | Evidence table | `placeholders[1].insert_table(行数, 列数)` で実表を置く |
| 4 | Figure and caption | idx 1 に図、idx 2 に説明。ネイティブ図形で描く場合は idx 1 の枠内に配置し、その空プレースホルダを外す |

表は幅11.83 inch の範囲で列幅を配分し、見出しを淡い葡萄色の地に葡萄色の太字、本文を墨色20 ptで入れる。数値右・文字左、表のセル左右余白は0.17 inchを目安とする。図の枠は左0.75・上2.05、幅8.1・高さ4.55 inch。右下の左11.92・上7.09、幅0.66 inchの枠に、11 pt・葡萄色・右揃えでスライド番号を入れる。

## TeX の成形

リポジトリ直下から `TEXINPUTS="$PWD/templates/p07-humanities-researcher:" bin/tex <作業フォルダ内の原稿.tex> <同フォルダ内の出力.pdf>`。対応するクラス名を `\documentclass` に指定する。

- **Research paper** は `\ResearchHeading{題}{発行番号}{日付}{著者}{回覧先など}`、`\section{章の題}`、必要に応じて `\SourceNote{典拠注}`。表題・節・ヘッダ・フッタに葡萄色、上下罫線に青銅色、本文に墨色が適用される。章ごとの問いは原稿の順に組む。
- **対訳資料** は同じクラスの `\ResearchHeading` に続けて、`\ParallelPassage{請求記号・葉・行}{前後を含む仏文}{対応する英訳}{訂正理由と参照先}` を箇所ごとに使う。各対は左が仏文、右が英訳、下が注。対の途中で改頁しないため、1つの対が1頁に収まる分量にする。長い引用は文の対応が追える切れ目で分け、葉と行の範囲を明示する。書簡の依頼者、宛先、支払いの受け手、会費か寄付か、参加条件に関わるかを前後の文から記す。
- **Inventaire sommaire** は `\ArchiveHeading{題}{目録番号}{発行日}{作成者}{窓口・範囲など}`。必要に応じて `ArchiveTable` 環境に `行番号 & 請求記号 & 記述 & 葉数 & 閲覧・複写条件 \\` の順で行を加える。数値2列は右寄せ、次の文字列まで4 mmの間隔があり、改頁後は列名が繰り返される。欄外の補足には `\ArchiveNote{補足}`。欠落の所在は記述できるが、所在や欠落だけから葉の中身を推定しない。
- **Correspondance imprimée** は `\LetterHeading{会名}{発信日}{呼び掛け}`、実文、`\LetterClosing{結語}{発信者または役職}`。通常の活字刷り用。手書き史料が必要な行は imagegen skill に従って紙面の画像を作り、その画像をスキャン原紙として扱う。
- 目録・書簡のスキャンは原紙を組んでから `bin/scan --gray --quality 78 <原紙.pdf> <配布用.pdf>` を通す。全頁の文字、余白、傾きと各葉の所在を確認する。

## 保存形式と今回の範囲

2024年9〜10月の書簡の作業では、10月24日に Julie と照合した32通のうち訳を直した7箇所を追う。旧訳が参加条件の読みを変えた箇所は、第2章の考察草稿に訂正理由と該当する請求記号・葉・行を残す。7月の初回複写分の再収録と、2026年の支払件数の訂正は今回の範囲外。箱別の所在、請求記号、葉範囲の個別値は計画・原稿・事実の台帳に従う。

仏語転記は UTF-8 の TXT にまとめ、葉と行を追える形にし、推定字と判読不能箇所の記号を揃える。照合PNGには読み比べる行と参照先を並べ、読書メモMDのファイル名は著者名から始める。筆跡を読む書簡束の原紙は手書き史料の手順で作る。 型なしの手書き原紙は計画では `scan:manuscript.pdf` とし、原稿指定の葉順で画像を作業フォルダ内の `manuscript.pdf` にまとめてからスキャンする。活字刷りの書簡の型は印刷物に使う。

閲覧案内HTMLは所蔵館の仏語ページとして保存する。深緑の館名・背表紙の印とサイト内の案内階層を使い、印刷目録の項目を貼り付けない。EMLは実際の送受信ヘッダと本文、必要な引用・添付で作る。欠落CSVは所在を検索できる実データと列名のみを入れる。JPGは閲覧机の独立した記録として扱う。PPTX・XLSXの型は、計画に該当する成果物がある場合に使う。

## 見本と画像

今回の作業フォルダは `scratch/p07-humanities-researcher/b002/work/templates-0002/`。見本は成形の確認用。見本の引用・葉番号・管理欄の値は計画や原稿に転記しない。

| 型と確認内容 | 見本 | レンダリング画像 |
|---|---|---|
| Working outline | [langley-outline.docx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-outline.docx) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-outline/) |
| Evidence note（短い参照表） | [langley-evidence-note-short.docx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-evidence-note-short.docx) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-evidence-note-short/) |
| Translation draft（Julie のコメント付き） | [langley-evidence-note-long.docx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-evidence-note-long.docx) | [画像（2頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-evidence-note-long/) |
| Access brief | [langley-access-brief.docx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-access-brief.docx) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-access-brief/) |
| Research discussion（全レイアウト） | [langley-research-briefing.pptx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-research-briefing.pptx) | [画像（5頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-research-briefing/) |
| Leaf register（短い台帳） | [langley-leaf-register-short.xlsx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-leaf-register-short.xlsx) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-leaf-register-short/) |
| Leaf register（複数頁） | [langley-leaf-register-long.xlsx](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-leaf-register-long.xlsx) | [画像（4頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-leaf-register-long/) |
| Research paper（章の論点） | [langley-research-paper.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-research-paper.tex)、[langley-research-paper.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-research-paper.pdf) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-research-paper/) |
| 対訳資料（1箇所） | [langley-parallel-passages-short.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-parallel-passages-short.tex)、[langley-parallel-passages-short.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-parallel-passages-short.pdf) | [画像（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-parallel-passages-short/) |
| 対訳資料（7箇所） | [langley-parallel-passages-long.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-parallel-passages-long.tex)、[langley-parallel-passages-long.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/langley-parallel-passages-long.pdf) | [画像（2頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/langley-parallel-passages-long/) |
| Inventaire sommaire（短い目録） | [saint-aubin-inventory-short.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-short.tex)、[saint-aubin-inventory-short.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-short.pdf)、[saint-aubin-inventory-short-scan.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-short-scan.pdf) | [原紙（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/saint-aubin-inventory-short/)、[スキャン（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/saint-aubin-inventory-short-scan/) |
| Inventaire sommaire（複数頁） | [saint-aubin-inventory-long.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-long.tex)、[saint-aubin-inventory-long.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-long.pdf)、[saint-aubin-inventory-long-scan.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/saint-aubin-inventory-long-scan.pdf) | [原紙（2頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/saint-aubin-inventory-long/)、[スキャン（2頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/saint-aubin-inventory-long-scan/) |
| Correspondance imprimée | [reading-society-letterprint.tex](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/reading-society-letterprint.tex)、[reading-society-letterprint.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/reading-society-letterprint.pdf)、[reading-society-letterprint-scan.pdf](../../scratch/p07-humanities-researcher/b002/work/templates-0002/examples/reading-society-letterprint-scan.pdf) | [原紙（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/reading-society-letterprint/)、[スキャン（1頁）](../../scratch/p07-humanities-researcher/b002/work/templates-0002/render/reading-society-letterprint-scan/) |
