# Heather Callahan の文書の型

*Reading Conditions at Work*（条件を読む仕事術）、Rosedale 向けの *Communicating in Workplace Meetings*（職場会議の伝え方）、*Building Proposals from Evidence*（根拠から組む提案）で使う。文書は英語、日時は Toronto の現地日付。連絡は EML、独立採点は匿名の CSV、点差は Excel、授業の比較図は PNG で記録する。手書き提案と講評は、紙面を `bin/scan` に通して PDF にする。

## 一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| Lesson plan | Elmwood Adult Learning Cooperative、Heather。授業の進行と理解確認。速読中心の旧案、議題から発言を組む演習など | `elmwood-lesson-plan.docx` | Letter 縦、表紙なし。青紫の題、金茶のヘッダ罫線。Learning aims、Teaching sequence、Check for understanding |
| Curriculum design decision | Elmwood、Heather から David Ashcroft。5目標の確認、改訂理由と次に確認すること | `elmwood-design-decision.docx` | 同じヘッダ、表紙なし。Context、Change and rationale、Next review。判断の比較には横罫の表 |
| Item trial review | Elmwood、Heather と Erin MacDonald。設問試行の記録、採点基準に加える答案例と注釈 | `elmwood-item-review.docx` | 同じヘッダ、表紙なし。3節の見出しと、出典や採点理由を添える注記スタイル |
| Course outcomes report | Elmwood、Heather から Glenn Fletcher。全6回の対象、提出、条件付き提案の到達、報告できる範囲を伝える | `elmwood-course-outcomes.docx` | Letter 縦、表紙なし。共通ロゴと金茶の罫線。提出と到達を比較する表 |
| Item and aim alignment | Elmwood、Heather と Erin。学習目標・設問・根拠の読み取りを対応付け、授業で根拠を示す順序を検討する | `elmwood-item-alignment.xlsx` | Letter 横。ロゴと印刷管理欄、青紫地に白の列見出し、明細は細い横罫。7列 |
| Outcome reconciliation | Elmwood、Heather。匿名で提出記録と評価結果を照合し、報告の分母・提出者数・到達者数を確認 | `elmwood-outcome-reconciliation.xlsx` | Letter 横。共通のロゴと管理欄、青紫の見出し。提出と条件付き提案の判定は別の列 |
| Independent marking comparison | Elmwood、Heather と Erin。匿名答案を突き合わせ、独立採点の点差と協議時の判断を記録 | `elmwood-score-comparison.xlsx` | Letter 横。共通の管理欄と7列の青紫見出し。両者の得点と絶対点差は右揃え、協議フラグと理由は左揃え |
| Classroom deck | Elmwood、Heather。一般則と例外、議題と条件によって変わる発言を投影し、討議に使う | `elmwood-classroom.pptx` | 16:9、白地に3本の段形と金茶の上罫線。1マスター、5レイアウト |
| Reading handout | Elmwood、Heather。職場文書と設問の配布、試行答案への個別の再指導に使う直接 PDF | `elmwood-reading.cls` | Letter 縦。青紫の大見出し、金茶の淡色抜粋欄、ヘッダとページ番号 |
| Workplace operations notice | Brookdale Service Cooperative、Janice Mitchell。授業で参照し得る社外の業務通知を直接 PDF にする型 | `brookdale-workplace-notice.cls` | Letter 縦、右上に円と2本の山形、中央題と節番号。深緑と墨のセリフ体、下罫線に部門名とページ番号 |
| Meeting agenda extract | Rosedale Workers Housing Cooperative、Glenn Fletcher。先方名義で受領した議題抜粋を直接 PDF にする型 | `rosedale-meeting-agenda.cls` | Letter 縦、家形と社名、煉瓦色の議題番号、麻色の管理帯、左揃えのサンセリフ体 |
| Course review memorandum | Fletcher Course Review Ltd、Sean Desjardins から Heather。学習目標と設問の対応についての外部査読意見を直接 PDF にする型 | `fletcher-course-review.cls` | Letter 縦、右上にチェック印と社名。セリフ体の題と本文、青緑の小見出し、淡い青灰色の宛先・発行欄、右下のページ番号 |

Brookdale は読解素材の発行元として使う場合の型。見本の通知は書式確認用で、この講座の設計や7月の試行に Brookdale が参加した記録にはしない。Rosedale 発行の資料には Rosedale の型を使う。Heather が編集・作成した演習や報告は Elmwood の型を使う。

## 組織ごとの書式

- 3本の高さの違う矩形と組織名をロゴとする。青紫 `#393967`、金茶 `#A87323`、本文の墨色 `#29283A`。TeX の引用欄には金茶の淡色を使う。
- 欧文の本文・題・表は Lato。Word と PowerPoint の東アジア書体および TeX の和文は Noto Sans CJK JP。Excel の Normal と入力用書式は英語資料のため Lato を明示する。和文セルが必要なら当該セルの書体を Noto Sans CJK JP に設定する。
- 編さんする文書の番号は `ELM-COURSE-YYYY-KIND-NNN`。COURSE は CRW（Reading Conditions at Work）、CWM（Communicating in Workplace Meetings）、BPE（Building Proposals from Evidence）。YYYY は作成時の西暦、KIND は LP（授業案）、DD（設計判断）、IR（設問試行・採点例）、MAP（目標対応表）、SLD（スライド）、HO（配布資料）、REC（報告数値の照合）、RPT（講座成果報告）、MOD（独立採点の比較）。NNN は同講座・同年・同種内での3桁の通番。表示日付は `DD Mon YYYY`。Word・TeX の題下、PPTX の表紙、XLSX の管理欄に文書番号全文・日付・作成者を記す。
- Brookdale 通知は `BSC-OPS-YYYY-NNN`（年別の業務通知通番）。日付書式は同じ。深緑 `#245747`、墨 `#28322E`、欧文 Liberation Serif、和文 Noto Serif CJK JP を使う。
- Rosedale 議題の番号は `RWHC-AGD-YYYY-NNN`（年別の議題通番）。煉瓦色 `#873F38`、墨色 `#2D3439`、麻色 `#F3ECE1`。欧文 Liberation Sans、和文 Noto Sans CJK JP。表示日付は `DD Mon YYYY`。題下の帯に文書番号全文・日付・作成者を記す。
- Fletcher の査読意見の番号は `FCR-REV-YYYY-NNN`（年別の査読通番）。青緑 `#20565E`、墨色 `#26333A`、淡い青灰色 `#E7EFF0`。題と本文は Liberation Serif、社名・節見出し・管理欄とフッタは Liberation Sans。和文はそれぞれ Noto Serif CJK JP と Noto Sans CJK JP。題下の帯に文書番号全文・`DD Mon YYYY` の発行日・査読者・宛先を記す。

## Word

型を開き、先頭の Title と Elmwood Document Details の空段落を埋める。用意された Heading 1 の下に Normal で本文を置き、必要な小節は Heading 2。出典の補足は Elmwood Source Note。別紙の表紙を挟まず第1頁に題と管理欄を置く。ヘッダに社名、フッタに文書種別と自動ページ番号。講座成果報告では Training scope の冒頭に報告先を置き、Submission and assessment results で分母・提出数・到達数を明示する。Interpretation and follow-up には判断の範囲と次の対応を書く。

表が必要な文書は中身に合わせて行と列を作り、Elmwood Table を指定する。先頭行に繰り返し見出しを指定し、セルは Elmwood Table Heading、青紫の塗り `393967`。本文セルは Elmwood Table Text とし、列幅は文章量に合わせる。右寄せ数値の次に左寄せ文字が来る表では、左インデント 11 pt の Elmwood Table Label を文字セルに指定する。

採点例では Item trial review の節見出しを Independent reads、Annotated examples、Agreement for the next read など、原稿に合わせて変更する。匿名の答案抜粋を置き、Elmwood Source Note でどの記述に得点を与えるかを直後に示す。7月14日の旧講評と訂正理由には同じ型を開き、原稿に合わせて見出しを設定する。旧講評の満点判断、示された根拠が0件だったこと、次に何を確かめるかを区別し、訂正後の得点欄は設けない。

## Excel

各型で `A2` に題、`C3:D3` に番号、`G3` に作成者、`C4:D4` に日付の実値、`G4` に講座名を入れる。結合範囲には左上セルから入力する。これらの入力先は書式済み。1〜6行は各印刷ページで反復し、列見出しと管理欄を読める状態にする。

### Alignment

1. データは7行目から必要行だけ作る。A と E に整数の `ELM Count`、F に割合（例 `0.92`）の `ELM Rate`、B〜D と G に折返しの `ELM Text`。右寄せ数値と左寄せ文字には各1段のインデントが入っている。未集計の値は空欄にする。行高は内容量に応じて明示し、見本は3明細で46 pt、16明細で61 pt。
2. 件数が必要なときは最終明細の次の行に実データ範囲の数式を置く。見本の A セルは `COUNTA(B7:B最終行)`、書式は `ELM Total count`。B:G を結合し、何を数えたかを書いて `ELM Total text` を適用する。集計対象に意味がある箇所だけで合計する。正答率は足さない。集計行は見本で26 pt。
3. 注記が必要なら次の行の A:G を結合し、`ELM Note` を付け、折り返しに応じて行高を指定する（見本は25 pt）。最終使用行まで印刷範囲を伸ばす。Letter 横、横1頁、縦は自動。数式を入れたら `bin/recalc` で計算結果を保存する。印刷プレビューで番号と作成者、講座名、末尾の集計と注記、数値から文字への列境界を確認する。

Planning の外部査読対応表では E6 を Review finding、F6 を Revision status に変更し、E・F を文字列の `ELM Text` にする。見本の列幅は E が24、F が19、行高は54 pt。B に設問番号、C に目標、D に改訂後の問い、G に答案で確認する根拠を書く。対応表の件数を表示するときは、収録した設問の範囲を明記する。

比較根拠を示す順序の検討では、A〜G の見出しを順に Order、Evidence、Comparison aim、Question to ask、Detail to withhold、Teaching move、What to listen for に設定する。7行目から根拠1件につき1行。A は `ELM Count`、B〜G は `ELM Text`、見本の列幅は E が24、F が19、行高は54 pt。必要なら Alignment と同じ手順で実明細の件数、配布時の注記を加え、印刷範囲を指定して再計算する。

### Reconciliation

1. 7行目から対象者ごとに1明細を置く。A は順番、B は匿名の照合番号、C は提出の根拠、D は評価の根拠、G は照合時の注記。A・E・F を `ELM Count`、B〜D・G を `ELM Text` とする。E は提出ありを1、なしを0とする。F は提出のうち到達を1、未到達を0、未提出で判定の根拠がない場合は空欄とする。右寄せ数値と文字列の間はスタイルのインデントを維持する。見本は明細35 pt。
2. 最終明細の次の行に集計行を作る。A に `COUNT(A7:A最終明細行)`、E・F にそれぞれの実明細範囲の `SUM`。A・E・F は `ELM Total count`。B:D は結合し、分母の対象を書いて `ELM Total text` を付ける。G に補足を入れる場合も `ELM Total text`。見本は集計行26 pt。
3. 注記、印刷範囲、再計算は Alignment と同じ手順で行う。3明細の見本は提出・到達、提出・未到達、未提出の3例。24明細の見本では24・22・17が別々に読み取れることを最終ページで照合する。

### Independent marking comparison

1. 7行目から答案ごとに1行作る。A は順番、B は匿名答案番号、C と D は独立採点 CSV の Heather と Erin の得点。A・C〜E は整数・右寄せの `ELM Count`、B・F・G は左インデントと折り返し付きの `ELM Text`。G は協議で確認する根拠や判断。見本の行高は3明細で38 pt、18明細で48 pt。
2. E7 に `=IF(COUNT(C7:D7)=2,ABS(C7-D7),"")`、F7 に `=IF(ISNUMBER(E7),IF(E7>=3,"Discuss",""),"")` を入れ、最終明細まで行参照を繰り下げる。2人分が確定した答案だけに点差を表示する。採点尺度は当時の評価基準に従う。
3. 最終明細の次の行は、A に E 列の実明細範囲の `COUNT`、F に F 列の実明細範囲の `COUNTIF`（条件は `"Discuss"`）。A・F に `ELM Total count`、B:E を結合して両者が採点した答案数のラベルを、G に3点以上の答案数のラベルを置いて `ELM Total text` を設定する。見本は26 pt。注記、印刷範囲、再計算は Alignment と同じ手順。18明細の見本は集計の18件・7件を最終ページで確認する。

## PowerPoint

スライドのない型を開き、`Cover`（表紙）、`Heading and body`（見出しと本文）、`Two columns`（比較）、`Table`（表）、`Diagram`（図と補足）から挿入する。社名・ロゴ・上下の罫線とフッタ文言はマスターにある。表紙の副題枠に講座名と文書情報を置き、右下に通し番号を置く。

表は表枠の位置と幅を使い、青紫の見出し行に白の Lato、明細に墨色の Lato を設定する。図は画像用枠に収めるか、その枠の位置に図形を組む。一般条件、例外、判断が変わる場面を順に示すときは、段階ごとにスライドを加える。

講師協議では Two columns に匿名答案の比較箇所だけを並べ、どの根拠を両者が読んだかを続くスライドで示す。授業で根拠を一つ外すときは、比較レイアウトに3件がある場合と残り2件の場合を置く。残りの根拠から言えることと、保留する判断を問い、学習者の名前は配布用・投影用の両方から外す。

## TeX

`elmwood-reading` は `\ElmHandout{題}{講座}{文書番号}{日付}{作成者}`、`\ElmHeading{見出し}`、`ElmExtract` 環境、`\ElmSource{資料名}`。抜粋欄はページをまたがないため、長い引用は原資料の段落境界で分ける。本人返却用は最初の節に宛名を置き、答えのどこに根拠を足すかを記す。授業配布用は無記名にし、本人返却用のコメントを転記しない。手書き答案と紙への書き込みは、記入した紙面を `bin/scan` に通して保管する。

`brookdale-workplace-notice` は `\BrookNotice{題}{文書番号}{日付}{作成者}` と `\BrookClause{節番号と見出し}{本文}`。型のあるディレクトリを `TEXINPUTS` に加えるか、作業先の TeX 原稿と同じ位置に型をコピーし、`bin/tex` で組む。

`rosedale-meeting-agenda` は `\RoseAgenda{題}{文書番号}{日付}{作成者}` と `\RoseItem{議題番号と見出し}{本文}`。議題を独立した節に分ける。`bin/tex` で組む。

`fletcher-course-review` は `\FletcherReview{題}{文書番号}{日付}{査読者}{宛先}` と `\FletcherFinding{見出し}{本文}`。Sean の査読範囲、指摘、求める改訂を記載する。Elmwood の改訂記録は設計判断や目標対応表に置く。`bin/tex` で組む。

## 見本

今回の作業フォルダに保存した見本。答案番号・個々の得点・設問番号・文書番号は書式と印刷範囲の確認用で、実際の制作には計画と事実の台帳を使う。

| 型 | 見本 |
|---|---|
| Lesson plan | [What changes when one piece of evidence is removed?](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-lesson-plan.docx) |
| Curriculum design decision | [Read the condition before deciding](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-design-decision.docx) |
| Item trial review | [A correct conclusion without supporting evidence](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-item-review.docx)、[Evidence that earns credit](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-rubric-annotations.docx) |
| Course outcomes report | [Workplace meetings training results](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-course-outcomes.docx) |
| Item and aim alignment | [3明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-item-alignment-3.xlsx)、[16明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-item-alignment-16.xlsx)、[査読対応](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-alignment-review.xlsx)、[根拠の提示順](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-evidence-order.xlsx) |
| Outcome reconciliation | [3明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-outcome-reconciliation-3.xlsx)、[24明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-outcome-reconciliation-24.xlsx) |
| Independent marking comparison | [3明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-score-comparison-3.xlsx)、[18明細](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-score-comparison-18.xlsx) |
| Classroom deck | [5レイアウト](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-classroom.pptx)、[匿名答案の協議](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-moderation-discussion.pptx) |
| Reading handout | [What evidence supports the decision?](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-reading.pdf)、[Which collection slot would you propose?](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-conditions-exercise.pdf)、[本人への再指導](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/elmwood-return-guidance.pdf) |
| Workplace operations notice | [Friday collection of reserved items](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/brookdale-workplace-notice.pdf) |
| Meeting agenda extract | [Maintenance handover discussion](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/rosedale-meeting-agenda.pdf) |
| Course review memorandum | [Building Proposals from Evidence](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/samples/fletcher-course-review.pdf) |

[全ページのレンダリング](../../scratch/p19-educator-instructional-designer/b004/work/templates-0002/render)
