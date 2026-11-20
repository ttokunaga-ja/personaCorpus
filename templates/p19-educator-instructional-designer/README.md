# Heather Callahan の文書の型

*Reading Conditions at Work*（条件を読む仕事術）、Rosedale 向けの *Communicating in Workplace Meetings*（職場会議の伝え方）、*Building Proposals from Evidence*（根拠から組む提案）、Ashford 紹介者向けの *Everyday Document Basics*（学び直し文書入門）で使う。文書は英語、日時は Toronto の現地日付。連絡は EML、独立採点は匿名の CSV、点差は Excel、授業の比較図は PNG で記録する。手書き提案と講評は、紙面を `bin/scan` に通して PDF にする。

## 一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| Lesson plan | Elmwood Adult Learning Cooperative、Heather。授業の進行と理解確認。速読中心の旧案、議題から発言を組む演習、Brookdale の講座別実施案など | `elmwood-lesson-plan.docx` | Letter 縦、表紙なし。青紫の題、金茶のヘッダ罫線。Learning aims、Teaching sequence、Check for understanding |
| Curriculum design decision | Elmwood、Heather から David Ashcroft。5目標の確認、授業観察の講評、改訂理由と次に確認すること | `elmwood-design-decision.docx` | 同じヘッダ、表紙なし。Context、Change and rationale、Next review。判断の比較には横罫の表 |
| Item trial review | Elmwood、Heather と Erin MacDonald。設問試行の記録、採点基準に加える答案例と注釈 | `elmwood-item-review.docx` | 同じヘッダ、表紙なし。3節の見出しと、出典や採点理由を添える注記スタイル |
| Course outcomes report | Elmwood、Heather から Glenn Fletcher。全6回の対象、提出、条件付き提案の到達、報告できる範囲を伝える | `elmwood-course-outcomes.docx` | Letter 縦、表紙なし。共通ロゴと金茶の罫線。提出と到達を比較する表 |
| Item and aim alignment | Elmwood。Heather と Erin が学習目標・設問・根拠を対応付ける。Heather と Learner-support は郵送教材と対応する授業を照合する | `elmwood-item-alignment.xlsx` | Letter 横。ロゴと印刷管理欄、青紫地に白の列見出し、明細は細い横罫。7列 |
| Outcome reconciliation | Elmwood、Heather。匿名で提出記録と評価結果を照合し、報告の分母・提出者数・到達者数を確認 | `elmwood-outcome-reconciliation.xlsx` | Letter 横。共通のロゴと管理欄、青紫の見出し。提出と条件付き提案の判定は別の列 |
| Independent marking comparison | Elmwood、Heather と Erin。匿名答案を突き合わせ、独立採点の点差と協議時の判断を記録 | `elmwood-score-comparison.xlsx` | Letter 横。共通の管理欄と7列の青紫見出し。両者の得点と絶対点差は右揃え、協議フラグと理由は左揃え |
| Seat allocation | Elmwood、Arjun Deshmukh。Brookdale の受講枠を講座別に設定し、申込区分との照合に使う | `elmwood-seat-allocation.xlsx` | Letter 横。共通の管理欄、青紫の列見出し。受講枠の数は右寄せ、区分コードは左に1文字分の余白。フッタは Corporate courses / Seat allocation |
| Classroom deck | Elmwood、Heather。一般則と例外、議題と条件によって変わる発言、説明途中の確認課題、依頼主向けの講座説明、受講者向けの課題紹介に使う | `elmwood-classroom.pptx` | 16:9、白地に3本の段形と金茶の上罫線。1マスター、5レイアウト |
| Reading handout | Elmwood、Heather。職場文書の対読と設問、オフライン閲覧の紙案内、試行答案への個別の再指導に使う直接 PDF | `elmwood-reading.cls` | Letter 縦。青紫の大見出し、金茶の淡色抜粋欄、ヘッダとページ番号 |
| Training agreement record | Elmwood、Heather。Janice と合意した研修の範囲、講座ごとの提出物と報告単位を控える直接 PDF | `elmwood-training-agreement.cls` | Letter 縦。Elmwood の配色とロゴ。題下に相手先と管理情報、下に Corporate courses とページ番号 |
| Workplace operations notice | Brookdale Service Cooperative、Janice Mitchell。授業で参照し得る社外の業務通知を直接 PDF にする型 | `brookdale-workplace-notice.cls` | Letter 縦、右上に円と2本の山形、中央題と節番号。深緑と墨のセリフ体、下罫線に部門名とページ番号 |
| Meeting agenda extract | Rosedale Workers Housing Cooperative、Glenn Fletcher。先方名義で受領した議題抜粋を直接 PDF にする型 | `rosedale-meeting-agenda.cls` | Letter 縦、家形と社名、煉瓦色の議題番号、麻色の管理帯、左揃えのサンセリフ体 |
| Course review memorandum | Fletcher Course Review Ltd、Sean Desjardins から Heather。学習目標と設問の対応についての外部査読意見を直接 PDF にする型 | `fletcher-course-review.cls` | Letter 縦、右上にチェック印と社名。セリフ体の題と本文、青緑の小見出し、淡い青灰色の宛先・発行欄、右下のページ番号 |

Brookdale 発行の業務通知には深緑の型、Rosedale 発行の議題には煉瓦色の型を使う。Janice からの条件確認は受信 EML として保存する。Heather が編集・作成した合意の控え、演習、報告は Elmwood の型を使う。匿名の申込区分は CSV、講座案内の控えは HTML で保存する。

## 組織ごとの書式

- 3本の高さの違う矩形と組織名をロゴとする。青紫 `#393967`、金茶 `#A87323`、本文の墨色 `#29283A`。TeX の引用欄には金茶の淡色を使う。
- 欧文の本文・題・表は Lato。Word と PowerPoint の東アジア書体および TeX の和文は Noto Sans CJK JP。Excel の Normal と入力用書式は英語資料のため Lato を明示する。和文セルが必要なら当該セルの書体を Noto Sans CJK JP に設定する。
- 編さんする文書の番号は `ELM-COURSE-YYYY-KIND-NNN`。COURSE は CRW（Reading Conditions at Work）、CWM（Communicating in Workplace Meetings）、BPE（Building Proposals from Evidence）、EDB（Everyday Document Basics／学び直し文書入門）。YYYY は作成時の西暦、KIND は LP（授業案）、DD（設計判断）、IR（設問試行・採点例）、MAP（目標対応表・教材と授業の照合表）、SLD（スライド）、HO（配布資料）、REC（報告数値の照合）、RPT（講座成果報告）、MOD（独立採点の比較）、CAP（受講枠）、AGR（合意の控え）。複数講座を含む Brookdale 向け合意の控えと依頼主向け説明では COURSE の位置に取引先略称 BSC を使う。NNN は同講座・同年・同種内での3桁の通番。表示日付は `DD Mon YYYY`。Word・TeX の題下、PPTX の表紙、XLSX の管理欄に文書番号全文・日付・作成者を記す。
- Brookdale 通知は `BSC-OPS-YYYY-NNN`（年別の業務通知通番）。日付書式は同じ。深緑 `#245747`、墨 `#28322E`、欧文 Liberation Serif、和文 Noto Serif CJK JP を使う。
- Rosedale 議題の番号は `RWHC-AGD-YYYY-NNN`（年別の議題通番）。煉瓦色 `#873F38`、墨色 `#2D3439`、麻色 `#F3ECE1`。欧文 Liberation Sans、和文 Noto Sans CJK JP。表示日付は `DD Mon YYYY`。題下の帯に文書番号全文・日付・作成者を記す。
- Fletcher の査読意見の番号は `FCR-REV-YYYY-NNN`（年別の査読通番）。青緑 `#20565E`、墨色 `#26333A`、淡い青灰色 `#E7EFF0`。題と本文は Liberation Serif、社名・節見出し・管理欄とフッタは Liberation Sans。和文はそれぞれ Noto Serif CJK JP と Noto Sans CJK JP。題下の帯に文書番号全文・`DD Mon YYYY` の発行日・査読者・宛先を記す。

## Word

型を開き、先頭の Title と Elmwood Document Details の空段落を埋める。用意された Heading 1 の下に Normal で本文を置き、必要な小節は Heading 2。出典の補足は Elmwood Source Note。別紙の表紙を挟まず第1頁に題と管理欄を置く。ヘッダに社名、フッタに文書種別と自動ページ番号。講座成果報告では Training scope の冒頭に報告先を置き、Submission and assessment results で分母・提出数・到達数を明示する。Interpretation and follow-up には判断の範囲と次の対応を書く。

表が必要な文書は中身に合わせて行と列を作り、Elmwood Table を指定する。先頭行に繰り返し見出しを指定し、セルは Elmwood Table Heading、青紫の塗り `393967`。本文セルは Elmwood Table Text とし、列幅は文章量に合わせる。右寄せ数値の次に左寄せ文字が来る表では、左インデント 11 pt の Elmwood Table Label を文字セルに指定する。

採点例では Item trial review の節見出しを Independent reads、Annotated examples、Agreement for the next read など、原稿に合わせて変更する。匿名の答案抜粋を置き、Elmwood Source Note でどの記述に得点を与えるかを直後に示す。旧講評（2025年7月10日、`ELM-BPE-2025-IR-004 v1.0`）と訂正版（2025年7月14日、`ELM-BPE-2025-IR-004 v1.1`）には同じ型を開き、原稿に合わせて見出しを設定する。旧講評の満点判断、示された根拠が0件だったこと、次に何を確かめるかを区別し、訂正後の得点欄は設けない。

11月の授業観察の講評は Curriculum design decision を開き、節を Observed sequence、Agreement on 10 November、What to look for next に変更する。2回の観察で見た進行、35分の説明に15分ごとの確認を入れる合意、次に見ることを節ごとに記す。紙の手控えは、観察ごとの記入内容が読める紙面を作り、`bin/scan` で保管用 PDF にする。

Brookdale の講座別実施案は Lesson plan を講座ごとに開く。Learning aims に対象と定員、Teaching sequence に教材と講師の進行、Check for understanding に提出物・確認する観点・講座別の報告項目を置く。

## Excel

各型で `A2` に題、`C3:D3` に番号、`G3` に作成者、`C4:D4` に日付の実値、`G4` に講座名を入れる。結合範囲には左上セルから入力する。これらの入力先は書式済み。1〜6行は各印刷ページで反復し、列見出しと管理欄を読める状態にする。

### Postal packet alignment

1. `elmwood-item-alignment.xlsx` を開き、シート名を Postal packet alignment、フッタ左を Learner support / Postal packets とする。管理欄には MAP の文書番号（例 `ELM-EDB-2025-MAP-001`）、日付、作成者、講座名 Everyday Document Basics を入れる。A〜G の列幅は順に 10、26、27、34、12、28、29。6行目の見出しは Session、Enclosure、Document / version、In-class reference、Sheets / set、At-home check、Packing note とし、既存の `ELM Header` のまま高さを32 ptにする。
2. 7行目から、対応する回の番号と封入物の組み合わせごとに1明細を作る。A は授業回（1〜8）、B は封入物名、C は配布資料の番号と版、D は授業案・投影資料で同じ箇所を見つけるための節・欄・設問、E は1組当たりの物理的な枚数、F は自宅での答えの確かめ方や参照箇所、G は印刷面や綴じ順など封入時の注意。冊子内の設問と解答が同じ紙に載る場合は1明細にまとめる。授業回をまたいで一つの表にまとめ、回順・封入順に並べる。
3. A・E は整数・右寄せの `ELM Count`、B〜D・F・G は左インデントと折り返し付きの `ELM Text`。明細の見本は48 pt。紙の枚数は原稿や印刷方法から確定した値を入れる。必要なら最終明細の次の行の A に `COUNTA(B7:B最終明細行)` を入れて `ELM Total count`、B:G を結合して集計の対象を `ELM Total text` で記す（26 pt）。この件数は照合した明細数。授業回の番号や回ごとの封入枚数を足し合わせる集計は設けない。
4. 注記は実明細または集計の次の行で A:G を結合し、`ELM Note` を適用する。見本は30 pt。印刷範囲を最終使用行まで伸ばし、Letter 横、横1頁・縦自動、1〜6行の反復を維持する。数式を使うときは `bin/recalc` で計算結果を保存する。全ページで文書番号・作成者・講座名、A/B と E/F の列境界、末尾の注記を確認する。

### Seat allocation

1. 受講枠は講座ごとに型を開く。CAP の文書番号を使い、`G4` に講座名を入れる。7行目から受講枠の区分ごとに1行置く。A・E は右寄せの整数 `ELM Count`、B〜D・F・G は左インデントと折り返し付きの `ELM Text`。見本の行高は36 pt。
2. 最終明細の次の行で A:D を結合し、`ELM Total text` で Planned places と記す。E は `ELM Total count` で実明細の `SUM`。G に `ELM Total text` で集計範囲を添える。見本の合計行は26 pt。
3. 最終行の A:G を結合して `ELM Note` で枠の基準を補足する。見本は25 pt。最終行まで印刷範囲を伸ばして `bin/recalc` で再計算する。短い見本の合計は24枠、複数ページの見本は36枠。各ページで管理欄、列見出し、A/B と E/F の間隔、最終ページで講座別の枠数を確認する。申込状況と出席・提出・到達は別の記録で扱う。

### Alignment

1. データは7行目から必要行だけ作る。A と E に整数の `ELM Count`、F に割合（例 `0.92`）の `ELM Rate`、B〜D と G に折返しの `ELM Text`。右寄せ数値と左寄せ文字には各1段のインデントが入っている。未集計の値は空欄にする。行高は内容量に応じて明示し、見本は3明細で46 pt、16明細で61 pt。
2. 件数が必要なときは最終明細の次の行に実データ範囲の数式を置く。見本の A セルは `COUNTA(B7:B最終行)`、書式は `ELM Total count`。B:G を結合し、何を数えたかを書いて `ELM Total text` を適用する。集計対象に意味がある箇所だけで合計する。正答率は足さない。集計行は見本で26 pt。
3. 注記が必要なら次の行の A:G を結合し、`ELM Note` を付け、折り返しに応じて行高を指定する（見本は25 pt）。最終使用行まで印刷範囲を伸ばす。Letter 横、横1頁、縦は自動。数式を入れたら `bin/recalc` で計算結果を保存する。印刷プレビューで番号と作成者、講座名、末尾の集計と注記、数値から文字への列境界を確認する。

Planning の外部査読対応表では E6 を Review finding、F6 を Revision status に変更し、E・F を文字列の `ELM Text` にする。見本の列幅は E が24、F が19、行高は54 pt。B に設問番号、C に目標、D に改訂後の問い、G に答案で確認する根拠を書く。対応表の件数を表示するときは、収録した設問の範囲を明記する。

比較根拠を示す順序の検討では、A〜G の見出しを順に Order、Evidence、Comparison aim、Question to ask、Detail to withhold、Teaching move、What to listen for に設定する。7行目から根拠1件につき1行。A は `ELM Count`、B〜G は `ELM Text`、見本の列幅は E が24、F が19、行高は54 pt。必要なら Alignment と同じ手順で実明細の件数、配布時の注記を加え、印刷範囲を指定して再計算する。

### Reading and discussion timing

`elmwood-item-alignment.xlsx` を使い、管理欄には MAP の文書番号を入れる。A〜G の幅は順に 8、16、40、12、12、12、44。6行目の見出しは No.、Phase、Activity、Before (min)、Revised (min)、Change (min)、Prompt / decision とし、高さは31 pt。

1. 7行目から活動ごとに1行置く。A・D〜F は右寄せの `ELM Count`、B・C・G は左インデントと折り返し付きの `ELM Text`。追加する活動は旧配分を0分、削る活動は改訂後を0分とする。F7 の `=E7-D7` を最終明細まで繰り下げる。見本は3明細で46 pt、16明細で54 pt。
2. 最終明細の次の行は A:C を結合して集計名を置き、D〜F は実明細範囲の `SUM`。A・G の文字列は `ELM Total text`、D〜F は `ELM Total count`。G に討議への振替時間を書ける。見本の集計行は26 pt。
3. 注記は A:G を結合して `ELM Note` を適用し、旧版30ページと短縮版18ページの関係を記す。Alignment と同じ手順で印刷範囲を更新し、再計算する。見本の注記は25 pt。複数ページの場合は各ページの管理欄・見出しと最終ページの合計、F の数値と G の文字の間隔を確認する。

11月の観察と次回案の対照では列見出しを Observed (min)、Planned (min)、Change (min) にする。観察で計っていない値は空欄にし、差は両方に値がある行だけ `=IF(COUNT(D7:E7)=2,E7-D7,"")` で出す。集計は範囲がそろっている場合に入れる。注記に観察日と計測の範囲を示す。

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

11月の講師用スライドでは、説明開始から15分と30分を目安に確認課題を挟む。Heading and body で最初に答えさせる問いを投影し、Two columns や Diagram で次の確認に使うケースを示す。

Brookdale の依頼主向け説明では、マスターのフッタを Corporate courses / Sponsor briefing に変更し、講座ごとに定員、採点観点を示した成果例、実施後に報告する項目を置く。受講者向け課題紹介は講座別に分け、課題で使う資料、判断して書く内容、提出物を示す。条件を読む講座は適用条項と記録上の事実を添える判断メモ、提案の講座は代案との比較と未確認時の対応を添える提案文とする。

## TeX

`elmwood-reading` は `\ElmHandout{題}{講座}{文書番号}{日付}{作成者}`、`\ElmHeading{見出し}`、`ElmExtract` 環境、`\ElmSource{資料名}`。抜粋欄はページをまたがないため、長い引用は原資料の段落境界で分ける。本人返却用は最初の節に宛名を置き、答えのどこに根拠を足すかを記す。授業配布用は無記名にし、本人返却用のコメントを転記しない。手書き答案と紙への書き込みは、記入した紙面を `bin/scan` に通して保管する。

旧30ページ版と短縮18ページ版は同じ `elmwood-reading` を使い、版ごとに一つの PDF にまとめる。保持した問いの後に、本文のどの条件を根拠にしたかを討議に持ち寄る指示を置ける。

`elmwood-training-agreement` は `\ElmAgreement{題}{相手先}{文書番号}{日付}{作成者}` と `\ElmHeading{見出し}` を使う。共通の Elmwood 書式を読み込むため、`elmwood-reading.cls` と同じ場所に置く。両講座に別々の節を置き、定員、提出物、確認する到達基準、実施後に報告する集計単位が分かるようにする。4月7日の合意の控えはその時点の予定として記す。

`brookdale-workplace-notice` は `\BrookNotice{題}{文書番号}{日付}{作成者}` と `\BrookClause{節番号と見出し}{本文}`。型のあるディレクトリを `TEXINPUTS` に加えるか、作業先の TeX 原稿と同じ位置に型をコピーし、`bin/tex` で組む。

`rosedale-meeting-agenda` は `\RoseAgenda{題}{文書番号}{日付}{作成者}` と `\RoseItem{議題番号と見出し}{本文}`。議題を独立した節に分ける。`bin/tex` で組む。

`fletcher-course-review` は `\FletcherReview{題}{文書番号}{日付}{査読者}{宛先}` と `\FletcherFinding{見出し}{本文}`。Sean の査読範囲、指摘、求める改訂を記載する。Elmwood の改訂記録は設計判断や目標対応表に置く。`bin/tex` で組む。

## 学び直し文書入門の授業と郵送教材

- 2月4日の計画は Ashford 紹介18人、全8回、欠席時は紙教材を郵送する。Bernadette McLean からの紹介時の質問と Heather の回答は EML で保存する。相談員が紹介時に伝えることは、参加者が授業で案内の何を読むかと、欠席時にどう教材を使うかが分かるようにする。
- 授業案は Lesson plan を使う。Teaching sequence に、案内状から申込欄まで読ませる順序と、口頭で補う指示の場所を記す。Classroom deck ではまず案内文を示し、対応する語句を探してから申込欄を開く。授業の理解確認では持参物・集合場所の答えと、そう読める箇所の両方を尋ねる。
- 郵送教材は Reading handout を使う。授業と同じ案内の抜粋を置き、口頭で指していた参照先を設問のそばに書き添える。持参物・集合場所を自分で探す問いの後に、見直す見出しと照合する文言を示す。参照先を手書きした試し刷りは紙面を `bin/scan` に通す。Learner-support の照合表には郵送版の番号・版と授業内の該当箇所を記す。

## 夏期の配布と授業

- 紙の閲覧案内は Reading handout を開き、Learner-support に渡す一枚にまとめる。HO の文書番号を使い、接続中に保存する手順、通信を切って開き直す手順、開けない場合の紙教材の受け取り先を順に示す。
- 対読教材は同じ Reading handout を使い、公開掲示と係員向け指示の発行元・対象者を各抜粋の下に記す。時刻や承認の有無は受付記録で示す。Brookdale が発行した通知そのものを保存する場合は Workplace operations notice を使う。
- 授業案は Lesson plan、投影は Classroom deck。冒頭で端末または紙で教材にアクセスできるかを確認し、討議では通常の掲示と係員向け指示を比較する。理解確認には、適用条項と記録を挙げて案内を決める問いを使う。
- 7月6日に Arjun と確認した受講者36人中4人の未設定は、配布状況の CSV と連絡の EML で扱う。受講前に紙案内を渡し、アクセス状況と授業での読解の確認は別々に記録する。
