# Judith Morris の文書の型

| 型 | 組織・用途 | ファイル | 見た目 |
|---|---|---|---|
| Working outline | Langley Historical Trust。Judith が章順を整理し Roger Hewitt に回覧する草稿 | [langley-outline.docx](langley-outline.docx) | A4縦。題の下に回覧情報、章順に小見出し。レビューコメントは Word の校閲欄 |
| Evidence note / Translation draft / Citation exercise | Langley Historical Trust。Judith が史料の所在と推論の限界を記し、Julie と仏語書簡の訳、Florian と引用の参照先を確認する | [langley-evidence-note.docx](langley-evidence-note.docx) | A4縦。本文、参照表または対訳表。淡い葡萄色の見出しと細い横罫 |
| Chapter manuscript | Langley Historical Trust。Judith が Edward Fairclough に提出する章稿や、阿部 由紀子に回覧する英語草稿 | [langley-chapter-manuscript.docx](langley-chapter-manuscript.docx) | A4縦。表紙なし。広めの行間と節・小節、典拠注、連続頁番号 |
| Access brief | Langley Historical Trust。Judith が Camille Laurent 宛の閲覧申請で使う照会書や添付案 | [langley-access-brief.docx](langley-access-brief.docx) | A4縦。宛先、対象史料、取扱いと受渡しの希望を一続きに読める配置 |
| Research discussion | Langley Historical Trust。Judith と研究班の章構成・典拠の検討、比較研究会の説明順の検討 | [langley-research-briefing.pptx](langley-research-briefing.pptx) | 16:9。白地の上下に細い罫線。表紙・本文・2段・表・図を収録したマスター |
| Leaf register | Langley Historical Trust。Judith が箱・請求記号・葉範囲と閲覧／複写条件を照合する表 | [langley-leaf-register.xlsx](langley-leaf-register.xlsx) | A4横印刷。組織名と管理欄、淡い葡萄色の列見出し。件数列は右揃え |
| Subscription count review | Langley Historical Trust。Judith と Julie が受領明細を再集計し、第2章の表の見出しと式を照合する | [langley-subscription-review.xlsx](langley-subscription-review.xlsx) | A4横。受領明細と見出し照合の2シート。淡い葡萄色の列見出しと繰返し管理欄 |
| Research paper | Langley Historical Trust。Judith が章の論点、仏英対訳、編集者への著者回答、比較研究会の配布稿や引用の相互点検手順を直接PDFに組む | [langley-research-paper.cls](langley-research-paper.cls) | A4縦。セリフ体、番号付きの節、典拠注、自動頁番号。対訳は出典と注を添えた左右2列 |
| Person index | Langley Historical Trust。Judith と Stefan が付録の人物同定を整理し、同じ表記の候補を典拠から引けるようにする | [langley-person-index.cls](langley-person-index.cls) | A4縦のテキストPDF。淡い葡萄色の帯、索引語・典拠・同定注の3列。列見出しを各頁に繰り返す |
| Peer review report | Ainsworth Scholarly Press。Edward が匿名査読者の意見を Judith に伝える直接 PDF | [ainsworth-peer-review.cls](ainsworth-peer-review.cls) | A4縦。開いた本を表す2枚の矩形と社名、紺と淡い青灰色の管理欄、番号付きの査読指摘 |
| Extrait du registre de circulation | Montclair Reading Archive。Isabelle Chevalier が Judith に渡す貸本台帳の閲覧用複写 | [montclair-ledger-copy.cls](montclair-ledger-copy.cls) | A4横。交差する2本の短い線と館名、黄土色の見出し、薄い砂色の複写管理欄。原紙の画像または活字の台帳を置く |
| Inventaire sommaire | Archives de Saint-Aubin-sur-Rive。Camille が閲覧室で交付する紙の箱別目録 | [saint-aubin-inventory.cls](saint-aubin-inventory.cls) | A4縦。高さの異なる3本の背表紙と仏語の所蔵館名、深緑の帯、サンセリフ体、5列の目録 |
| Correspondance imprimée | 史料中の地方読書会。書記が発行した活字刷りの書簡を紙面に組む | [reading-society-letterprint.cls](reading-society-letterprint.cls) | A5縦。墨茶1色、中央の会名と短い罫線、右寄せの発信日、字下げした本文と結び |
| Cotisations reçues | 史料中の地方読書会。会計係の会費受領明細の活字刷りをスキャン原紙にする | [reading-society-subscriptions.cls](reading-society-subscriptions.cls) | A5縦。墨茶の会名・短い罫線・会計上の葉の表示。日付、氏名、金額と支払いの内訳を同じ行に置く |

## 組織の意匠と管理欄

- Langley は葡萄色 `603346` の菱形と組織名、青銅色 `9A6C39` の細罫、墨色 `292526` の本文を使う。表の背景 `F1E8EC` は葡萄色の淡色。York と文書用途はフッタ左、頁番号は右。比較した他 persona のロゴや罫線の配置から区別し、小規模な研究組織の回覧文書らしくセリフ体と余白を中心にした。
- Langley の Word・PowerPoint・TeX は欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。Word は既定の Normal と各段落・文字・表のスタイル、テーマの欧文／和文、PowerPoint はテーマとマスター・レイアウトに指定済み。Excel は一つのセルに仏語と日本語を併記できるよう欧文を含め `Noto Serif CJK JP` を使う。ブックのテーマ、既定の Normal、見出し、管理欄の入力セル、明細の名前付きスタイルに指定済み。Excel のテーマ配色も Langley の葡萄色、青銅色、墨色にそろえる。
- Archives は深緑 `28483C`、濃い墨緑 `202923`、深緑8%の見出し帯。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。ヘッダには背表紙3本と所蔵館名、フッタには `Salle de lecture` と頁番号。活字刷りの史料は墨茶 `2D2823`、欧文 `Liberation Serif`、和文 `Noto Serif CJK JP`。史料の会名、日付、発信者は原稿で決まった情報を使う。歴史的な書簡には現代の所蔵館ロゴ・管理欄・頁番号を付けない。
- Langley の発行番号は `LHT-YYYYMMDD-用途-連番3桁`。用途は `OUT`（章構成）、`CHP`（提出する章稿）、`EVD`（典拠メモ・訳稿）、`REQ`（閲覧照会書）、`BRF`（研究説明）、`REG`（葉別台帳）、`PAP`（論点稿・対訳資料・著者回答）。Archives の現代の目録文書は `ASR-YYYYMMDD-INV-連番3桁`。目録文書の番号と各明細の史料請求記号は別に扱う。
- Ainsworth は紺 `18374D`、本文の濃い青灰 `29353C`、管理欄の淡い青灰 `E6EEF2`。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。開いた本の印と社名をヘッダに、編集部名・守秘区分・頁をフッタに置く。査読番号は `ASP-YYYY-REV-連番3桁`。扱った章稿の発行番号や著者回答の番号とは別に付ける。
- Montclair は黄土色 `825019` の印と見出し、砂色 `F1E8D7` の帯、黒鉛色 `292B2E` の本文。欧文は `Liberation Sans`、和文は `Noto Sans CJK JP`。右上に複写窓口、左下に `Copie de consultation`、右下に頁番号を置く。複写番号は `MRA-YYYYMMDD-COP-連番3桁`。複写日・担当者・元の台帳の請求記号と葉はそれぞれの欄に入れる。
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
- **Peer review report** は `\ReviewHeading{章題}{査読番号}{発行日}{編集者}{査読者表示}`、`\section`、`\ReviewPoint{指摘番号と論点}{査読意見}`。匿名なら査読者欄には `Anonymous reader` と置く。指摘見出しと本文の冒頭を同じ頁に置く。欧文の曲線引用符とアポストロフィは Liberation Sans で組む。見本では1頁目の管理欄と R05、全頁の社名・守秘表示・頁番号を確認する。
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

## 2025年10〜11月の箱14再撮影と引用の相互点検

- 再撮影の依頼・返答は Judith と Camille の仏語 EML で、依頼した範囲と受け渡し、実際の添付を記録する。再複写 PDF には綴じ目や書き込みまで確認できる原紙画像を用意して `bin/scan` に通す。活字刷りの原紙には Correspondance imprimée を使い、手書きの原紙や葉番号控えは imagegen skill の手順に従う。撮影位置 JPG、字の重なりを示す PNG、判読を保留した理由の TXT は各形式のまま保存する。再撮影の対象12葉は再複写済み8葉と翻刻保留4葉に分け、初回閲覧分と区別する。
- **Citation exercise** は Evidence note を開き、用途表示とフッタを `Citation exercise` に変える。管理欄は `EVD` の番号、日付、作成者、確認者。節を演習に合わせ、元の参照と訂正案に請求記号・葉番号・表裏・行を入れ、Florian のコメントを元の参照の run に付ける。欄を並べる場合は Word の参照表の手順を使う。
- 相互点検用の直接 PDF は Research paper で組む。確認者が参照先を開き直す手順、二人の判断記録、本文と注へ反映する条件を原稿の順に置く。研修の20箇所と葉番号の誤り4件は引用の点検記録に、転記を保留した4葉は再撮影の記録に残す。

## 2026年1〜4月の会費再集計と付録

追加する Langley の用途コードは SUB（会費集計の照合）、IDX（人物索引）。どちらも共通の発行番号規則に従う。

### 会費集計の照合ブック

1. Subscription count review を開く。両シートの B3:D3 に発行番号の全文、F3:G3 に見直し日、B4:D4 に作成者、F4:G4 に確認者を入れる。空の入力先は Noto Serif CJK JP 10 pt、折返し、インデント1。管理欄の高さ29／32 ptを基準にする。
2. Payments の7行目から、集計対象の受領明細を1支払い1行で入れる。Aは LHT Integer、B〜DとFは LHT Text、Eは LHT Francs（右寄せ・小数2桁）、Gは LHT Note。Bは請求記号・葉の表裏・行を示す。Cの19世紀の日付は文字列で保持する。未判読の金額は空欄。同じ氏名で異なる日に入った分納はそれぞれの行に残す。
3. Heading review の7行目から、重複しない集計区分を1行ずつ置く。A・Eは LHT Integer、B〜DとFは LHT Text、Gは LHT Note。Cに旧稿の見出し、Dに改題後の見出し（英語の章では Payment entries、日本語では「延べ支払件数」）、Fに1件の単位を入れる。Payments の最終明細が26行なら、E7の式は `=COUNTIF(Payments!$F$7:$F$26,B7)`。Payments のF列と Heading review のB列の区分を照合し、支払いの行を数える。1月14日に読み直した184件は支払いの記録。集計根拠CSVにも典拠・区分・支払い単位を残す。
4. 明細の高さは36 ptを目安にし、折返しに合わせて変える。AとB、EとFの間を離すため、数値の右寄せと文字列の左インデント1を維持する。Heading review の末尾に合計を出す場合は、A:Dを結合して LHT Total Label、Eに実集計行だけを足す式と LHT Total Integer を使う。注記が要る場合は直下のA:Gを結合して LHT Note。印刷とフィルタは Leaf register と同様に実使用範囲を指定し、両シートとも1〜6行を繰り返す。区分別の合計を受領明細数と突合し、bin/recalc で式、bin/render で全頁の幅・行送り・注記・合計位置を確認する。

### 表注・人物索引・会費記録

- 第2章の表注は Chapter manuscript を使う。対象の表、1件の意味、複数回支払いの扱いを書き、典拠や集計根拠CSVへの参照は LHT Note に入れる。語注はUTF-8のTXTで、原語・葉と行・会費や寄付と読む根拠を記す。
- Person index は `\IndexHeading{題}{発行番号}{日付}{作成者}{確認者}` で管理欄を入れ、PersonIndex 環境に`索引語 & 典拠と行 & 同定注 \\`の順で行を置く。索引語には原綴りと識別に必要な文脈を入れる。4月16日の人物同定57件のうち別人の可能性がある6件は、同じ表記でも典拠を添えた別項目にする。読書会の人物索引は貸本台帳の利用者索引および査読時の地名索引と区別する。検索可能なテキストPDFに組む。
- Cotisations reçues は活字刷りの受領控え用。 `\AccountHeading{会名}{会計の表題}{葉・期間などの表示}`、SubscriptionEntries 環境に`記帳順 & 受領日 & 記載名 & フラン額 & 支払いの内訳 \\`、必要に応じて `\AccountClosing{会計係の肩書または署名表記}` の順で置く。数値列の後は4 mm空ける。原紙の葉境界では改頁し、原稿に従って葉の見出しを置く。現代の管理番号は付けず、日付と仏語の金額表記は原紙のものを使う。印刷PDFを bin/scan --gray --quality 78 に通し、氏名、別々の受領日、金額と分納の内訳が読めることを全頁で確認する。手書き原紙が計画で指定された場合は imagegen skill の手順に従う。

## 2026年6〜7月の回覧経路と年記の再確認

- Montclair の複写は `\MontclairHeading{題}{複写番号}{日付}{複写担当者}{台帳の請求記号と葉}` で仏語の管理欄を入れる。原紙画像を載せる場合は `\MontclairFacsimile{画像パス}` を使い、葉が変わる位置で改頁し管理欄にその葉を示す。手書きの原紙画像は imagegen skill の手順に従う。活字で組む原紙には `MontclairPrintedLedger` 環境を使い、行番号、原紙の日付、書名、原記載の回覧先、台帳の備考を並べる。次頁にも列見出しが入り、右寄せの行番号と次の文字列の間は5 mm空く。原紙の内容と葉順を確認し、組んだ PDF に `bin/scan --gray --quality 78` を通す。
- 6月25日の照合では台帳9冊の117回覧先記録を扱い、同定不能14記録は原表記と典拠を残す。対応 CSV は1記録1行。台帳の請求記号、葉・行、原表記、照合結果が追える形にする。作業図 PNG は裏付けのある接続と途切れている箇所を示す。CSV・作業図・確認依頼 EML はそれぞれの形式で保存する。
- 訂正前の Word は Chapter manuscript を開き、対象段落の run に校閲コメントを付ける。7月8日の訂正後の直接 PDF は Research paper を使う。箱14・184葉の LA-042 の年記を1864年から1867年へ訂正した根拠を典拠注へ入れ、書き直した因果説明の2段落を直前・直後の文脈と一緒に載せる。Desktop の作業用ページは確認中の葉と箇所をすぐ引けるようにし、数字の読みと判断根拠は TXT に残す。

## 見本と画像

b007 の見本。組版と印刷の確認に用いた抜粋で、配布物の内容と管理欄は計画と原稿で確定する。

| 型と確認内容 | 見本 | レンダリング画像 |
|---|---|---|
| Working outline（Roger の校閲コメント） | [langley-outline.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-outline.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-outline/langley-outline-1.png) |
| Chapter manuscript（節構成と連続頁番号） | [langley-chapter-manuscript.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-chapter-manuscript.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-chapter-manuscript/langley-chapter-manuscript-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-chapter-manuscript/langley-chapter-manuscript-2.png) |
| Chapter manuscript（訂正前の2段落と Julie のコメント） | [langley-chapter-before-correction.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-chapter-before-correction.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-chapter-before-correction/langley-chapter-before-correction-1.png) |
| Evidence note（短い典拠表） | [langley-evidence-note.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-evidence-note.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-evidence-note/langley-evidence-note-1.png) |
| Evidence note（複数頁の典拠表） | [langley-evidence-note-long.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-evidence-note-long.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-evidence-note-long/langley-evidence-note-long-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-evidence-note-long/langley-evidence-note-long-2.png)、[3頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-evidence-note-long/langley-evidence-note-long-3.png) |
| Access brief（閲覧照会） | [langley-access-brief.docx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-access-brief.docx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-access-brief/langley-access-brief-1.png) |
| Research discussion（全5レイアウト） | [langley-research-briefing.pptx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-research-briefing.pptx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-briefing/langley-research-briefing-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-briefing/langley-research-briefing-2.png)、[3頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-briefing/langley-research-briefing-3.png)、[4頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-briefing/langley-research-briefing-4.png)、[5頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-briefing/langley-research-briefing-5.png) |
| Leaf register（短い明細） | [langley-leaf-register-short.xlsx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-leaf-register-short.xlsx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-leaf-register-short/langley-leaf-register-short-1.png) |
| Leaf register（複数頁の明細） | [langley-leaf-register-long.xlsx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-leaf-register-long.xlsx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-leaf-register-long/langley-leaf-register-long-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-leaf-register-long/langley-leaf-register-long-2.png) |
| Subscription count review（短い明細、2シート） | [langley-subscription-review-short.xlsx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-subscription-review-short.xlsx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-subscription-review-short/langley-subscription-review-short-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-subscription-review-short/langley-subscription-review-short-2.png) |
| Subscription count review（複数頁、2シート） | [langley-subscription-review-long.xlsx](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-subscription-review-long.xlsx) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-subscription-review-long/langley-subscription-review-long-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-subscription-review-long/langley-subscription-review-long-2.png)、[3頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-subscription-review-long/langley-subscription-review-long-3.png) |
| Research paper（対訳と典拠注） | [langley-research-paper.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-research-paper.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-research-paper/langley-research-paper-1.png) |
| Research paper（訂正後の2段落と前後） | [langley-revised-passages.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-revised-passages.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-revised-passages/langley-revised-passages-1.png) |
| Person index（短い索引） | [langley-person-index-short.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-person-index-short.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-person-index-short/langley-person-index-short-1.png) |
| Person index（複数頁の索引） | [langley-person-index.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/langley-person-index.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-person-index/langley-person-index-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/langley-person-index/langley-person-index-2.png) |
| Peer review report（複数頁の査読票） | [ainsworth-peer-review.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/ainsworth-peer-review.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/ainsworth-peer-review/ainsworth-peer-review-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/ainsworth-peer-review/ainsworth-peer-review-2.png) |
| Inventaire sommaire（短い目録） | [saint-aubin-inventory-short-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/saint-aubin-inventory-short-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/saint-aubin-inventory-short-scan/saint-aubin-inventory-short-scan-1.png) |
| Inventaire sommaire（複数頁の目録） | [saint-aubin-inventory-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/saint-aubin-inventory-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/saint-aubin-inventory-scan/saint-aubin-inventory-scan-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/saint-aubin-inventory-scan/saint-aubin-inventory-scan-2.png) |
| Correspondance imprimée（活字原紙のスキャン） | [reading-society-letterprint-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/reading-society-letterprint-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/reading-society-letterprint-scan/reading-society-letterprint-scan-1.png) |
| Cotisations reçues（短い明細） | [reading-society-subscriptions-short-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/reading-society-subscriptions-short-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/reading-society-subscriptions-short-scan/reading-society-subscriptions-short-scan-1.png) |
| Cotisations reçues（複数葉の明細） | [reading-society-subscriptions-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/reading-society-subscriptions-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/reading-society-subscriptions-scan/reading-society-subscriptions-scan-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/reading-society-subscriptions-scan/reading-society-subscriptions-scan-2.png) |
| Extrait du registre de circulation（短い活字抜粋） | [montclair-ledger-copy-short-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/montclair-ledger-copy-short-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/montclair-ledger-copy-short-scan/montclair-ledger-copy-short-scan-1.png) |
| Extrait du registre de circulation（複数頁の活字抜粋） | [montclair-ledger-copy-long-scan.pdf](../../scratch/p07-humanities-researcher/b007/work/templates-0002/examples/montclair-ledger-copy-long-scan.pdf) | [1頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/montclair-ledger-copy-long-scan/montclair-ledger-copy-long-scan-1.png)、[2頁](../../scratch/p07-humanities-researcher/b007/work/templates-0002/render/montclair-ledger-copy-long-scan/montclair-ledger-copy-long-scan-2.png) |
