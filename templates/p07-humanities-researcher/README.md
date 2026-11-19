# Judith Morris の文書の型

| 型 | 組織・用途 | ファイル | 見た目 |
|---|---|---|---|
| Working outline | Langley Historical Trust。Judith が章順を整理し Roger Hewitt に回覧する草稿 | [langley-outline.docx](langley-outline.docx) | A4縦。題の下に回覧情報、章順に小見出し。レビューコメントは Word の校閲欄 |
| Evidence note / Translation draft | Langley Historical Trust。Judith が史料の所在と推論の限界を記し、Julie と仏語書簡の訳を確認する | [langley-evidence-note.docx](langley-evidence-note.docx) | A4縦。本文、参照表または対訳表。淡い葡萄色の見出しと細い横罫 |
| Chapter manuscript | Langley Historical Trust。Judith が Edward Fairclough に提出する章稿や、阿部 由紀子に回覧する英語草稿 | [langley-chapter-manuscript.docx](langley-chapter-manuscript.docx) | A4縦。表紙なし。広めの行間と節・小節、典拠注、連続頁番号 |
| Access brief | Langley Historical Trust。Judith が Camille Laurent 宛の閲覧申請で使う照会書や添付案 | [langley-access-brief.docx](langley-access-brief.docx) | A4縦。宛先、対象史料、取扱いと受渡しの希望を一続きに読める配置 |
| Research discussion | Langley Historical Trust。Judith と研究班の章構成・典拠の検討、比較研究会の説明順の検討 | [langley-research-briefing.pptx](langley-research-briefing.pptx) | 16:9。白地の上下に細い罫線。表紙・本文・2段・表・図を収録したマスター |
| Leaf register | Langley Historical Trust。Judith が箱・請求記号・葉範囲と閲覧／複写条件を照合する表 | [langley-leaf-register.xlsx](langley-leaf-register.xlsx) | A4横印刷。組織名と管理欄、淡い葡萄色の列見出し。件数列は右揃え |
| Research paper | Langley Historical Trust。Judith が章の論点、仏英対訳、編集者への著者回答、比較研究会の配布稿を直接PDFに組む | [langley-research-paper.cls](langley-research-paper.cls) | A4縦。セリフ体、番号付きの節、典拠注、自動頁番号。対訳は出典と注を添えた左右2列 |
| Peer review report | Ainsworth Scholarly Press。Edward が匿名査読者の意見を Judith に伝える直接 PDF | [ainsworth-peer-review.cls](ainsworth-peer-review.cls) | A4縦。開いた本を表す2枚の矩形と社名、紺と淡い青灰色の管理欄、番号付きの査読指摘 |
| Inventaire sommaire | Archives de Saint-Aubin-sur-Rive。Camille が閲覧室で交付する紙の箱別目録 | [saint-aubin-inventory.cls](saint-aubin-inventory.cls) | A4縦。高さの異なる3本の背表紙と仏語の所蔵館名、深緑の帯、サンセリフ体、5列の目録 |
| Correspondance imprimée | 史料中の地方読書会。書記が発行した活字刷りの書簡を紙面に組む | [reading-society-letterprint.cls](reading-society-letterprint.cls) | A5縦。墨茶1色、中央の会名と短い罫線、右寄せの発信日、字下げした本文と結び |

## 組織の意匠と管理欄

- Langley は葡萄色 `603346` の菱形と組織名、青銅色 `9A6C39` の細罫、墨色 `292526` の本文を使う。表の背景 `F1E8EC` は葡萄色の淡色。York と文書用途はフッタ左、頁番号は右。比較した他 persona のロゴや罫線の配置から区別し、小規模な研究組織の回覧文書らしくセリフ体と余白を中心にした。
- Langley の Word・PowerPoint・TeX は欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。Word は既定の Normal と各段落・文字・表のスタイル、テーマの欧文／和文、PowerPoint はテーマとマスター・レイアウトに指定済み。Excel は一つのセルに仏語と日本語を併記できるよう欧文を含め `Noto Serif CJK JP` を使う。ブックのテーマ、既定の Normal、見出し、管理欄の入力セル、明細の名前付きスタイルに指定済み。Excel のテーマ配色も Langley の葡萄色、青銅色、墨色にそろえる。
- Archives は深緑 `28483C`、濃い墨緑 `202923`、深緑8%の見出し帯。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。ヘッダには背表紙3本と所蔵館名、フッタには `Salle de lecture` と頁番号。活字刷りの史料は墨茶 `2D2823`、欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。史料の会名、日付、発信者は原稿で決まった情報を使う。歴史的な書簡には現代の所蔵館ロゴ・管理欄・頁番号を付けない。
- Langley の発行番号は `LHT-YYYYMMDD-用途-連番3桁`。用途は `OUT`（章構成）、`CHP`（提出する章稿）、`EVD`（典拠メモ・訳稿）、`REQ`（閲覧照会書）、`BRF`（研究説明）、`REG`（葉別台帳）、`PAP`（論点稿・対訳資料・著者回答）。Archives の現代の目録文書は `ASR-YYYYMMDD-INV-連番3桁`。目録文書の番号と各明細の史料請求記号は別に扱う。
- Ainsworth は紺 `18374D`、本文の濃い青灰 `29353C`、管理欄の淡い青灰 `E6EEF2`。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。開いた本の印と社名をヘッダに、編集部名・守秘区分・頁をフッタに置く。査読番号は `ASP-YYYY-REV-連番3桁`。扱った章稿の発行番号や著者回答の番号とは別に付ける。
- 日付は英国式 `25 April 2024`、仏語目録は `9 juillet 2024` の順で記す。実番号・版・日付・著者は計画と原稿で確定する。改稿は同じ番号の版で追い、同じ文書をPDF保存する場合は番号・版を引き継ぐ。作成者と確認者または宛先は別の欄に置く。メール・個人メモ・歴史史料に現代の発行番号を付けない。

## Word の成形

1. `Document(型のパス)` で開く。1段落目の用途表示に続け、最初の節見出しの前へ題（`Title`）、発行番号・発行日・作成者（`LHT Register`）、必要なら宛先・回覧先・版（同じスタイル）の順に挿入する。章は `Heading 2` で分ける。本文は `Normal`、典拠注は `LHT Note` を使い、原稿にない節は外す。
2. A4縦、左26・右24・上29・下23 mm。表紙なし。本文11.5 pt・行間1.14・段落後6 pt、題26 pt、見出し15／12 pt、管理欄・注記10 pt。ヘッダに菱形と組織名、フッタに York・用途と自動 `PAGE` フィールド。
3. 表は実際の行数で追加して `LHT Reference Table` を指定し、見出しセルの段落に `LHT Table Header`、本文に `LHT Table Text`、数値に右揃えの `LHT Table Number` を付ける。目安の列幅は左から16、34、49、61 mm。見出し行の `w:trPr` に `w:tblHeader` を、各行に `w:cantSplit` を付け、行高を固定しない。表スタイルの左右145 twips（約2.6 mm）のセル余白を維持し、箱番号と次の文字列を離す。
4. 訳稿では Evidence note の用途表示とフッタの用途名を `Translation draft` とし、フッタのタブと `PAGE` フィールドを保つ。節は `Scope`、`Passages under review`、`Implication for Chapter 2` を目安にする。箇所ごとに請求記号・葉の表裏・行を小見出しで示し、対訳表（80 mm × 2列）の下に `LHT Note` で語義や判断の根拠を置く。Julie の指摘は、該当する訳語や文の run に `doc.add_comment(run, コメント, author="Julie Perrin", initials="JP")` で付け、実際の日時を設定する。章構成の回覧稿では回覧時の確認者に合わせる。印刷画像では本文の組版、DOCX の校閲欄ではコメントの位置・作成者・日時を確認する。

5. 提出用の章稿は Chapter manuscript の用途表示に続けて、題（`Title`）、発行番号・提出日・著者・宛先（`LHT Register`）を加える。`Heading 1` と `Heading 2` で本文を分け、出典の補足には `LHT Note`。本文は11.5 pt・行間1.5・段落後8 pt。表は上記の参照表の手順で入れる。提出稿とその改稿の版を管理欄で区別する。

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
- **著者回答** は同じクラスの `\ResearchHeading` に Langley の発行番号、日付、Judith、Edward 宛の情報を入れる。査読票の番号を冒頭で参照し、`\subsection{R01 · 論点}` のように査読票内の指摘番号を見出しにする。反映した変更は改稿の章節・段落と結び、対応できない指摘には不足する史料と、そのため本文でどこまで述べられるかを書く。
- **Peer review report** は `\ReviewHeading{章題}{査読番号}{発行日}{編集者}{査読者表示}`、`\section`、`\ReviewPoint{指摘番号と論点}{査読意見}`。匿名なら査読者欄には `Anonymous reader` と置く。指摘見出しと本文の冒頭を同じ頁に置く。
- **対訳資料** は同じクラスの `\ResearchHeading` に続けて、`\ParallelPassage{請求記号・葉・行}{前後を含む仏文}{対応する英訳}{訂正理由と参照先}` を箇所ごとに使う。各対は左が仏文、右が英訳、下が注。対の途中で改頁しないため、1つの対が1頁に収まる分量にする。長い引用は文の対応が追える切れ目で分け、葉と行の範囲を明示する。書簡の依頼者、宛先、支払いの受け手、会費か寄付か、参加条件に関わるかを前後の文から記す。
- **Inventaire sommaire** は `\ArchiveHeading{題}{目録番号}{発行日}{作成者}{窓口・範囲など}`。必要に応じて `ArchiveTable` 環境に `行番号 & 請求記号 & 記述 & 葉数 & 閲覧・複写条件 \\` の順で行を加える。数値2列は右寄せ、次の文字列まで4 mmの間隔があり、改頁後は列名が繰り返される。欄外の補足には `\ArchiveNote{補足}`。欠落の所在は記述できるが、所在や欠落だけから葉の中身を推定しない。
- **Correspondance imprimée** は `\LetterHeading{会名}{発信日}{呼び掛け}`、実文、`\LetterClosing{結語}{発信者または役職}`。通常の活字刷り用。手書き史料が必要な行は imagegen skill に従って紙面の画像を作り、その画像をスキャン原紙として扱う。
- 目録・書簡のスキャンは原紙を組んでから `bin/scan --gray --quality 78 <原紙.pdf> <配布用.pdf>` を通す。全頁の文字、余白、傾きと各葉の所在を確認する。

## 2025年4月29日の比較研究会

- 英語草稿は Chapter manuscript を使い、章節、引用箇所、版、阿部への回覧情報を入れる。別に組む研究会配布用のテキスト PDF は Research paper を使う。草稿をそのまま PDF 保存する行では元の発行番号と版を引き継ぐ。見本の章稿と配布稿は組版確認用の抜粋。28ページ稿の頁数・版・内容は計画と原稿で確定する。
- 付箋付きの手元稿は対応する版の章稿を印刷用 PDF に出し、原稿が指定した頁の余白に付箋を添えてから `bin/scan` に通す。頁番号や典拠を覆わず、紙面からコメント対象の節と行を特定できるようにする。手書きを指定した行は imagegen skill の手順に従う。
- 説明順を検討するスライドは Research discussion を使う。会費の定め、徴収の記録、入室・貸出の条件、実際の利用を比較し、どの組み合わせを史料で照合できるかを示す。直接の影響を検討するのに要る史料は、たとえば規則採用前に相手方の文面を読めたことが分かる日付入りの翻訳・送付記録や、受け手が規則を検討した議事録。草稿の節と報告時の問いから該当箇所を追えるようにする。
- 阿部から届く案内は、あおぞら比較文化研究室の日本語 EML と保存 HTML。メールは実際の送受信情報を持たせ、保存ページは研究室名・開催日・案内本文を中心にした日本語ページとして組む。Langley のロゴや発行番号は付けない。日本語の読書メモは著者名で始まる Markdown、当日の質疑は発言者・論点・該当節を追える UTF-8 の TXT。

## 既出の文書と保存形式

2025年1月15日の Stefan との照合は異綴り46件。同定不能8件は原綴りで検索できるようにし、地名対照CSVには原綴り、典拠（請求記号・葉の表裏・行）、照合結果、判断根拠を残す。未確定の場所は地図に置かず、人物索引とも分ける。独語の読解メモは UTF-8 の Markdown、照合 PNG は対象の字形と典拠が追えるようにする。活字刷りの書簡は Correspondance imprimée で原紙を組む。手書きの原紙を指定した行は imagegen skill の手順に従い、いずれも原紙からスキャンする。

2月12日の Edward 宛の8,000語の提出稿には Chapter manuscript、編集部から届く査読票には Peer review report、Judith の直接PDFの回答には Research paper を使う。査読指摘11件は査読票の番号で追い、史料不足で対応できない2件の根拠を明記する。異綴りの照合件数は査読指摘の番号と独立に扱う。EML は送受信ヘッダ・本文・実際に添付したファイルを保持する。

仏語転記の TXT は推定字と判読不能箇所の記号を揃え、葉と行を追える形にする。読書メモのファイル名は著者名から始める。型なしの手書き原紙は計画では `scan:manuscript.pdf` とし、指定の葉順で画像を作業フォルダ内の `manuscript.pdf` にまとめてからスキャンする。閲覧案内HTMLは所蔵館の仏語ページとして保存し、館名・背表紙の印・サイト内の案内階層を使う。閲覧机の写真は独立した JPG として扱う。

## 見本と画像

見本は組版確認用。今回の実行で現行の型を開いて作成した。配布物の値は計画と原稿で確定する。

| 型と確認内容 | 見本 | レンダリング画像 |
|---|---|---|
| Working outline（Roger のコメント） | [langley-outline.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-outline.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-outline/langley-outline-1.png) |
| Chapter manuscript（比較研究会の草稿抜粋） | [langley-chapter-manuscript.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-chapter-manuscript.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-chapter-manuscript/langley-chapter-manuscript-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-chapter-manuscript/langley-chapter-manuscript-2.png) |
| Chapter manuscript（付箋付きの手元稿） | [langley-seminar-working-copy-scan.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-seminar-working-copy-scan.pdf) | [スキャン 1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-seminar-working-copy-scan/langley-seminar-working-copy-scan-1.png)、[スキャン 2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-seminar-working-copy-scan/langley-seminar-working-copy-scan-2.png) |
| Evidence note（短い参照表） | [langley-evidence-note.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-evidence-note.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-evidence-note/langley-evidence-note-1.png) |
| Evidence note（複数頁の参照表） | [langley-evidence-note-long.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-evidence-note-long.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-evidence-note-long/langley-evidence-note-long-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-evidence-note-long/langley-evidence-note-long-2.png) |
| Translation draft（Julie のコメント） | [langley-evidence-note-translation.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-evidence-note-translation.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-evidence-note-translation/langley-evidence-note-translation-1.png) |
| Access brief | [langley-access-brief.docx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-access-brief.docx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-access-brief/langley-access-brief-1.png) |
| Research discussion（全5レイアウト） | [langley-research-briefing.pptx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-research-briefing.pptx) | [1枚目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-briefing/langley-research-briefing-1.png)、[2枚目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-briefing/langley-research-briefing-2.png)、[3枚目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-briefing/langley-research-briefing-3.png)、[4枚目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-briefing/langley-research-briefing-4.png)、[5枚目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-briefing/langley-research-briefing-5.png) |
| Leaf register（明細4行） | [langley-leaf-register-short.xlsx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-leaf-register-short.xlsx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-leaf-register-short/langley-leaf-register-short-1.png) |
| Leaf register（明細20行） | [langley-leaf-register-long.xlsx](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-leaf-register-long.xlsx) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-leaf-register-long/langley-leaf-register-long-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-leaf-register-long/langley-leaf-register-long-2.png) |
| Research paper（研究会配布稿の抜粋） | [langley-research-paper.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-research-paper.tex)、[langley-research-paper.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-research-paper.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-research-paper/langley-research-paper-1.png) |
| Research paper（1箇所の対訳） | [langley-parallel-passages-short.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-parallel-passages-short.tex)、[langley-parallel-passages-short.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-parallel-passages-short.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-parallel-passages-short/langley-parallel-passages-short-1.png) |
| Research paper（著者回答） | [langley-author-response.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-author-response.tex)、[langley-author-response.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/langley-author-response.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-author-response/langley-author-response-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/langley-author-response/langley-author-response-2.png) |
| Peer review report（11件の指摘） | [ainsworth-peer-review.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/ainsworth-peer-review.tex)、[ainsworth-peer-review.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/ainsworth-peer-review.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/ainsworth-peer-review/ainsworth-peer-review-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/ainsworth-peer-review/ainsworth-peer-review-2.png) |
| Inventaire sommaire（短い目録） | [saint-aubin-inventory-short.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory-short.tex)、[saint-aubin-inventory-short.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory-short.pdf)、[saint-aubin-inventory-short-scan.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory-short-scan.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory-short/saint-aubin-inventory-short-1.png)、[スキャン 1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory-short-scan/saint-aubin-inventory-short-scan-1.png) |
| Inventaire sommaire（複数頁の目録） | [saint-aubin-inventory.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory.tex)、[saint-aubin-inventory.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory.pdf)、[saint-aubin-inventory-scan.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/saint-aubin-inventory-scan.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory/saint-aubin-inventory-1.png)、[2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory/saint-aubin-inventory-2.png)、[スキャン 1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory-scan/saint-aubin-inventory-scan-1.png)、[スキャン 2頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/saint-aubin-inventory-scan/saint-aubin-inventory-scan-2.png) |
| Correspondance imprimée | [reading-society-letterprint.tex](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/reading-society-letterprint.tex)、[reading-society-letterprint.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/reading-society-letterprint.pdf)、[reading-society-letterprint-scan.pdf](../../scratch/p07-humanities-researcher/b004/work/templates-0002/examples/reading-society-letterprint-scan.pdf) | [1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/reading-society-letterprint/reading-society-letterprint-1.png)、[スキャン 1頁目](../../scratch/p07-humanities-researcher/b004/work/templates-0002/render/reading-society-letterprint-scan/reading-society-letterprint-scan-1.png) |
