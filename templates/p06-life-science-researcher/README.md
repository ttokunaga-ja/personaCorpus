# Melissa Watkins の文書の型

Bennett Cell Evaluation Inc の Cell Evaluation Laboratory が、非臨床の測定評価で使う英語の書式。Office の型は通常の DOCX・XLSX・PPTX として開き、別名で保存する。管理番号、本文、日付、人名、明細は原稿とファイル計画の値を入れる。日付は Philadelphia の現地日付（America/New_York）で、文書では `Jul 11, 2024`、機器出力などの交換用データでは元の記録形式を使う。

## 社内の型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Study plan | Bennett、Melissa Watkins が計画を起こし、Robert Nguyen が承認する。計画草案と承認版の PDF に使用 | `bennett-study-plan.docx` | US Letter 縦。表紙なし。共通ヘッダと4節で、設計・基準・承認の場所が固定 |
| Findings review | Bennett、Melissa が集計結果、培養日別・保存時間群別の解釈、次の提案を説明する | `bennett-findings.docx` | US Letter 縦。表紙なし。初見と群別確認を別の見出しに置き、根拠表の見出しを次ページにも出せる |
| Laboratory review minutes | Bennett、Melissa が Robert・Lauren らとの検討で照合した記録、判断、担当を残す | `bennett-lab-minutes.docx` | US Letter 縦。表紙なし。会議・参照記録・判断・対応の4節と横罫の対応表 |
| Culture-date comparison | Bennett、Melissa が培養日×条件で測定群を比較する | `bennett-culture-day.xlsx` | US Letter 横。淡藤の管理欄とアイリス色の列見出し。左上に社章があり、続きのページにも管理欄と見出しを印刷 |
| Interlaboratory comparison | Bennett、Melissa が Bennett と Caldwell の原記録 ID・測定値を試料単位で照合する | `bennett-facility-comparison.xlsx` | US Letter 横。Bennett の社章・管理欄・反復見出しを継承。両施設の ID と数値を隣接させ、数値の次の文字列には左インデントを取る |
| Storage study sample disposition | Bennett、Melissa が保存時間試験の試料と原記録を照合し、Anthony の採否判定を反映する | `bennett-storage-disposition.xlsx` | US Letter 横。Bennett の管理欄とアイリス色の7列見出しを反復。予定時間を右寄せし、右隣の原ラベルは字下げ。集計は最終明細の後に置く |
| Storage time group summary | Bennett、Melissa が採否確定後の試料で保存時間別の母数・平均・基準比を算出する | `bennett-storage-summary.xlsx` | US Letter 横。群別集計と解析対象の明細を2シートに分け、両シートとも Bennett の社章、淡藤の管理欄とアイリスの7列見出しを反復 |
| Laboratory director briefing | Bennett、Melissa が Robert に基準・初回測定・再集計・再試験の提案を説明する | `bennett-director-brief.pptx` | 16:9。濃い紫の左帯、上部の小さな社章、下部の細線を共有するマスターと5つのレイアウト |
| Measurement work sheet | Bennett、Lauren Brooks が測定日と原記録の参照情報を作業票に記録する。Lab-book のスキャン PDF の台紙に使用 | `bennett-bench-record.cls` | US Letter 縦。共通ヘッダ、淡藤の記録欄、アイリス色の反復見出し付きの一覧。左下に記録種別、右下にページ番号 |
| Quality hold notice | Bennett、Anthony Mendoza が保留範囲と配布・解除条件を通知する | `bennett-hold-notice.cls` | US Letter 縦。Bennett の共通ヘッダを継承し、発効日時・発行者・配布先を淡藤の枠に置く。フッタは QUALITY NOTICE |
| Quality deviation report | Bennett、Anthony が逸脱の根拠・採否・原記録の保管先を確定して発行する | `bennett-deviation-report.cls` | US Letter 縦。共通ヘッダとアイリスの節見出し。文書番号・判定日・発行者・状態を淡藤の枠に置く。左下は QUALITY DEVIATION REPORT、右下はページ番号 |
| Laboratory operating decision | Bennett、Melissa が作成し、Robert が承認した業務上限、判断根拠、適用範囲を発行する | `bennett-operating-decision.cls` | US Letter 縦。共通ヘッダ、アイリスの節見出し、淡藤の番号・決定日・作成者・承認者欄。左下は LABORATORY OPERATING DECISION、右下はページ番号 |

## 社外から受け取る型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Calibration service record | Mason Laboratory Maintenance、Christopher Medina が校正・点検の記録を発行 | `mason-service-record.cls` | US Letter 縦。Liberation Mono、墨色 `#26291F`、オリーブ `#515A34`、紙色 `#F0EDDF`。三段の白いゲージと大文字の社名を反転帯に置き、管理欄を3行に分ける |
| Interlaboratory comparison note | Caldwell Assay Services、Angela Price が測定の相互比較所見を Melissa に送付 | `caldwell-comparison-note.cls` | US Letter 縦。Liberation Serif、濃紺 `#18354B`、青緑 `#1B6473`、淡青 `#E7F1F2`。菱形の社章と社名を中央に置き、件名と識別番号を細罫の下に配置 |

Mason は `MLM-YYYY-MMDD-SV-NNN`、Caldwell は `CAS-YYYY-MMDD-CMP-NNN`。年・月日と発行側の連番を使う。Caldwell の宛先は管理欄の直後に置く。いずれも発行者の社名・番号・日付を使う。CSV や写真は取得元の形式と対応する日付を保つ。

## Bennett の共通書式

- 社章は円環と2つの点を社名と組み合わせる。Liberation Sans を使い、本文とタイトルは濃紫 `#362742`、罫線と強調はアイリス `#765387`、管理欄や集計の淡色は `#F2EAF3`。濃色の表見出しには白字を使う。
- 文書番号は `BCE-CE-YYYY-TYP-NNN-Rnn`。種別は計画 `PLN`、所見 `REV`、議事 `MIN`、集計 `TBL`、説明資料 `BRF`、作業票 `LAB`、保留通知 `HLD`、逸脱報告 `DEV`、業務上の決定 `DEC`。年と種別内の3桁連番で起票し、版は `R00` から始める。同じ計画の承認版などは連番を引き継いで版を進める。

### Word

- `Normal` は 10.5 pt、`Title` は 24 pt、`Heading 1` は 13 pt、`Heading 2` は 11 pt。タイトルの下に `Caption` で番号・日付・氏名・状態を1段で置き、各節見出しの下に `Normal` で本文を入れる。承認 PDF は承認済み DOCX の版を `bin/office-pdf` で書き出す。
- 表が必要な箇所にだけ `Bennett Evidence` で作る。見出しは `Bennett Table Heading` とアイリスの塗りで、先頭行を反復見出しにする。例の3列は 0.64 / 1.95 / 4.31 in。番号は `Bennett Table Number` で右寄せ、直後の文字列は `Bennett Table After Number` で左寄せ・左 0.13 in、残りの文字列は `Bennett Table Text`。数値列を増やす場合も同じ組合せを使う。行をページ途中で分割しない設定にする。Word のページ番号フィールドはフッタにある。
- 施設間の照合所見では Findings review の日別確認見出しを `Pair-level reconciliation` に変更し、判断の記録には Laboratory review minutes を使う。

### Excel

各ブックのテーマ、既定の `Normal`、見出し、管理欄の入力セル、明細の名前付き書式に Bennett の配色と Liberation Sans を使う。

#### 培養日別集計

1. `Culture date` の B4:C4 に文書番号、E4:G4 に作成者、B5:C5 に現地でのレビュー日、E5:G5 に指標と単位を入れる。B5 には実日付を入力し、型の表示形式 `mmm d, yyyy` を使う。結合範囲の左上セルを使う。番号欄は結合幅32と行高30 pt、折返し設定を持つ。A7:G7 は `Bennett Header`。条件列の表示名は原稿に合わせる。必要なら同じ管理欄・印刷設定を持つシートを複製し、別の指標を載せる。
2. 明細を8行目から必要数だけ作る。Aの通し番号と C:F の整数は `Bennett Number`（右寄せ・インデント1）、B の実日付は `Bennett Date`（左寄せ・インデント1）、G の注記は `Bennett Text`（左寄せ・インデント1）。この余白で A/B と F/G の境界を離す。率は比率値を `Bennett Percent` で表示する。行高は短い注記で26 ptを目安にし、折り返す行だけ広げる。件数を比較する場合は F を条件別件数の行合計とする。率や CV の横断集計は、分子・分母や基礎データから求める。
3. 集計が必要なら最終明細の直後に A:B を結合してラベルを入れ、A・G に `Bennett Total Label`、C:F に `Bennett Total` を使う。件数表では各列に明細範囲の `SUM` を入れる。注記がある場合は次の行の A:G を結合し、`Bennett Note` を適用する。集計行は32 pt、1行で収まる注記は28 ptを目安に、折返しに合わせて広げる。
4. 印刷範囲は A1 から実際の最終行の G 列まで。左2列と先頭7行を固定し、印刷では1〜7行を反復する。横幅を1ページに合わせ、縦方向は必要なページ数を使う。印刷プレビューで最終明細、集計、注記が同じページに載ることを確認する。分かれるときは、A:G が横1ページに収まる固定倍率に切り替え、最終明細を含む末尾のまとまりの直前に横改ページを入れる。タイトル行の反復を維持し、再度プレビューで確認する。短い表と複数ページの表で番号と日付の全文、隣接欄と続きの見出しも確認する。

#### 施設間照合

1. `Matched samples` の B4:C4 に文書番号、E4:G4 に作成者、B5:C5 にレビュー日、E5:G5 に測定単位を入れる。結合左上の B4、E4、B5、E5 を使う。日付は実日付とし、表示形式を `mmm d, yyyy` にする。A7:G7 はアイリス色の `Bennett Header`。
2. 対応 TSV を参照し、8行目から1組につき1行追加する。A は組番号に `Bennett Number`、B と D は各施設の原記録 ID に `Bennett Text`、C と E は各施設の測定値に `Bennett Measurement`、F と G は照合状態・参照注記に `Bennett Text`。ID は文字列で保持する。数値書式は右寄せ、文字書式は左寄せで各1文字分のインデントを持つ。測定値は小数2桁で表示し、元記録に応じて表示桁数を調整する。明細は36 ptを目安にし、注記の折返しに合わせて行高を調整する。
3. 集計が必要な場合は最終明細の直後に置く。A:B を結合し、A・D・G に `Bennett Total Label`、C の照合件数に `Bennett Total`、E・F の施設間差と基準に `Bennett Summary Percent` を使う。F列の照合済み状態を `Matched` とした表の集計例は `=COUNTIF(F8:F25,"Matched")`（18組の明細）。2025年1月16日の施設間差と基準は、確認済みの比率値 `0.082` と `0.10` を別々に入れる。Gに両欄の意味を記す。次の行を A:G で結合し、比較対象や適用範囲を `Bennett Note` で記す。集計行32 pt、注記行28 ptを目安に、全文が見える行高にする。
4. 印刷範囲は A1:G の実際の最終行までに更新する。US Letter 横で幅を1ページに合わせ、縦は必要なページ数を使う。1〜7行を各ページに反復する。短い明細と18組の明細で、管理欄の番号・日付・作成者・単位、A/B・C/D・E/F の列境界、続きのページの見出し、集計・注記を確認する。

#### 保存時間試験の採否

1. `Sample disposition` の B4、E4、B5、E5 に文書番号、作成者、レビュー日、試験名を入れる。日付は実日付にして `mmm d, yyyy` で表示する。左上セルからそれぞれ B4:C4、E4:G4、B5:C5、E5:G5 の結合範囲に表示される。
2. 8行目から、照合対象の試料ごとに1行を追加する。A は通し番号、B は受渡し票の参照、C は予定群の時間（時）、D は記録どおりのラベル、E は原データの行識別子、F は採否、G は根拠の参照とする。A・C は `Bennett Number`（右インデント1）、B と D:G は `Bennett Text`（左インデント1）。ラベルと識別子は文字列で保持する。行高は34 ptを目安に、注記が折り返す行を広げる。採否は判定済みなら `Eligible` または `Excluded` とし、途中の版で判定待ちの行は `Pending` と明記する。
3. 集計する場合は最終明細の直後に見出しを追加する。A:B を結合して `Scope`、C:G に `Planned`、`Listed`、`Eligible`、`Excluded`、`Review basis` を入れ、`Bennett Header` を適用する。次の行も A:B を結合して集計範囲を書き、A・G に `Bennett Total Label`、C:F に `Bennett Total` を適用する。C は計画の件数、D は対象範囲の通し番号の `COUNT`、E・F は同範囲の採否列を `COUNTIF` で数える。群別の集計では予定時間と採否を条件に `COUNTIF` または `COUNTIFS` を使う。判定待ちがある版ではその件数も注記する。注記は次の行の A:G を結合して `Bennett Note` を適用する。
4. 印刷範囲は A1:G の実際の最終行までとし、US Letter 横で幅を1ページに合わせる。1〜7行を各ページに反復する。少数行と全行の見本で管理欄、A/B・C/D の間隔、続頁の列見出し、集計と注記を確認する。

#### 保存時間群別の解析

1. `Group summary` と `Eligible readings` の両方で、B4 に文書番号、E4 に作成者、B5 にレビュー日、E5 に参照した解析用抽出表の名前を入れる。B5 は実日付を `mmm d, yyyy` で表示する。各入力欄は結合の左上セルを使う。
2. `Eligible readings` の8行目から、確定した解析対象 CSV の各行を転記する。A に通し番号、B に原データ行 ID、C に数値の保存時間（0、24、48）、D に照合済みラベル、E に数値の測定値（ng/mL）、F に測定日、G に受渡し記録の参照を入れる。A・C は `Bennett Number`、B・D・G は `Bennett Text`、E は `Bennett Measurement`、F は `Bennett Date`。数値とその右隣の文字・日付は各1文字分のインデントを使い、行高は34 ptを目安に全文が見えるよう調整する。原データ行 ID の重複、空の測定値、対象外の行がないことを確認する。
3. `Group summary` の8〜10行目に 0・24・48 時間の順で置く。A・C に `Bennett Number`、B・G に `Bennett Text`、D に `Bennett Measurement`、E に `Bennett Percent`、F に `Bennett Change` を適用し、行高は33 ptを目安にする。C は `COUNTIF` で時間群の件数、D は `AVERAGEIF` で同じ群の平均。例えば明細が8〜41行なら、C8 は `=COUNTIF('Eligible readings'!$C$8:$C$41,A8)`、D8 は `=IF(C8=0,"",AVERAGEIF('Eligible readings'!$C$8:$C$41,A8,'Eligible readings'!$E$8:$E$41))`。E8 は `=IF(OR(C8=0,$C$8=0,$D$8=0),"",D8/$D$8)`、F8 は `=IF(ISNUMBER(E8),E8-1,"")`。同じ式を9・10行目へコピーし、明細の絶対参照の末尾は実際の最終明細行に合わせる。比率と変化率は丸め前の平均から計算する。0時間平均を基準に、48時間群の基準比82%と変化率−18%を区別して示す。
4. 群別表の直後に A:B を結合し、`Bennett Total Label` で `Total eligible`、C に `Bennett Total` で `=SUM(C8:C10)`。集計行は30 ptを目安にする。注記がある場合は両シートとも最終行の次に A:G を結合して `Bennett Note` を適用し、折返しに応じて行高を調整する。両シートの印刷範囲を A1 から G列の最終使用行までに更新する。US Letter 横で横1ページ、縦は必要なページ数、1〜7行を各ページに反復する。両シートの印刷フッタは左に `LABORATORY RECORD`、右に `Page &P / &N` を維持する。

### PowerPoint

`BCE Cover` は題名と番号・日付・作成者を配置する。番号・日付・作成者のプレースホルダーは濃紫 `#362742` を継承する。`BCE Heading and Body` は見出しと本文、`BCE Two Columns` は全体と培養日別などを並べる用途。 `BCE Evidence Table` は表用プレースホルダーと下部の出典欄、`BCE Figure and Caption` は左側に画像、右側に説明を入れる。表はアイリス塗り・白字の見出し、濃紫の本文、必要な集計行だけ淡藤で示す。数値列は右寄せで、続く文字列セルには左余白を約 0.2 in とる。図には測定母数、凡例・単位、出典を必要に応じて入れる。日付は表紙の管理欄に記録する。スライド番号はフッタ右端の罫線下に置き、マスターの番号欄（左11.77 in、上6.96 in、幅0.96 in、右寄せ）を5つのレイアウトで継承する。成形時はスライド番号を有効にする。プログラムで成形する場合は、レイアウトの番号プレースホルダーをスライドに追加し、番号フィールド（`slidenum`）に実際のスライド番号を入れる。

### TeX とスキャン

クラスを選んで本文の `.tex` を用意する。リポジトリ直下から実行する場合は `TEXINPUTS="$PWD/templates/p06-life-science-researcher:" bin/tex input.tex output.pdf` のように型の検索先を指定する。`input.tex` と生成物には作業先のパスを使う。

- Bennett は `\BennettRecord{title}{document no.}{measurement date}{recorded by}` の後に `\BennettSection{heading}` と本文を置く。受渡し票では `\BennettRecord[Handoff date]{title}{document no.}{handoff date}{recorded by}` を使う。`BennettBenchTable` は番号・参照・確認状況・所見の4列。実際の記録行だけ追加し、各行を `\\` で閉じる。番号と参照の間に 0.10 in の追加余白が入り、見出し行が改ページ時に繰り返される。Lab-book の紙記録は中身を入れてから `bin/scan` を使う。手書きとして扱う欄の描画は成形時に行う。
- Quality hold notice は `\BennettHoldNotice{title}{document no.}{effective date and time}{issued by}{distribution}` の後に `\BennettSection{heading}` で Scope、Handling、Review を組む。必要なら本文に日別の対象表を置く。
- Quality deviation report は `\BennettDeviationReport{title}{document no.}{decision date}{issued by}{status}` の後に `\BennettSection{heading}` で Occurrence and records reviewed、Disposition、Analysis and filing を組む。食い違った記載は原稿で確定した原記録の参照とともに置き、採否・原記録の保管先・解析対象への影響を記す。テキスト層付き PDF として保存する。
- Laboratory operating decision は `\BennettOperatingDecision{title}{document no.}{decision date}{prepared by}{approved by}` の後に `\BennettSection{heading}` で Decision、Basis、Application を組む。承認された上限、群別母数と基準比、適用する業務と条件を記し、テキスト層付き PDF として保存する。
- Mason は `\MasonRecord{title}{service no.}{service date}{issued by}` と `\MasonSection{heading}` を使う。本文は左揃えで、左右 0.9 in の余白内で単語の間を使って折り返す。詳細を別ページに載せる場合は、ページ冒頭にサービス番号・装置識別・日付を再掲し、測定値を校正前・校正後に分ける。Caldwell は `\CaldwellRecord{title}{reference}{date}{from}` と `\CaldwellSection{heading}` を使う。

### 初回の施設間比較

Correspondence の送受信は EML、各施設の測定値は各自の CSV、両者の対応は TSV で保存する。Method-comparison の照合 XLSX は2つの原記録を参照できる形にし、図の PNG は原稿で確定した施設別の値と単位を使う。Angela の所見 PDF は Caldwell の型、Lab-share の討議 PPTX と決定 DOCX は Bennett の型で作る。2025年1月16日の判断は18試料の施設間差8.2%と基準10%の照合、および Robert による現条件での継続承認に限る。10月に保留した記録の処分は別の品質記録で管理する。

### 上清保存時間試験

Storage-study の計画は Study plan を開き、設計の節に 0・24・48 時間の各12試料を記す。Lab-book の受渡し票には Measurement work sheet を使い、管理欄のほかに引渡し側・受領側、原ラベルと受渡し記録の参照、押印・注記を載せる余白を本文に設ける。押印・注記を載せた紙面を `bin/scan` に通す。測定 CSV と試料対応 TSV は原記録の値・記載を保存し、Desktop の照合メモは TXT で残す。採否は Storage study sample disposition に記録し、Quality deviation report で確定した2件と根拠を指す。予定母数と解析対象数は別に記録する。

8〜10月の解析では、確定した解析対象 12・11・11 件を Storage time group summary にまとめる。比較図 PNG は同じ集計値を使い、0 h 基準と群別の解析対象数を明記する。Findings review の `Date-level review` は `Storage-group comparison` に変更し、結果・解釈・提案を分ける。室長向け説明には Laboratory director briefing を使い、採用された24時間上限と適用範囲は10月16日付の Laboratory operating decision に記す。結論は試験で確認した条件と承認範囲に限る。

## 印刷見本

見本の明細・文書番号は組版確認用。成形ではファイル計画・台帳・原稿の確定値を使う。培養日別集計では、実日付を入力した3・24・25行の見本で、最終明細・集計・注記を同じページに印刷した。保存時間群別の解析 XLSX は各群1行の短い例と34行の全件印刷で確認した。全件見本は解析対象数12・11・11、基準比82%、基準からの変化−18%を再現する組版用の測定値を使う。PPTX は表紙・本文・2段・表・図の順。草案と承認版の見本は2025年4月の保存時間試験を題材とした。

| 見本 | 元ファイル | レンダリング |
|---|---|---|
| Study plan 草案 | [DOCX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-study-plan.docx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-study-plan/bennett-study-plan-1.png) |
| Study plan 承認版 | [PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-study-approval.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-study-approval-pdf/bennett-study-approval-1.png) |
| Findings review・3行 | [DOCX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-findings-3.docx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-findings-3/bennett-findings-3-1.png) |
| Findings review・26行 | [DOCX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-findings-26.docx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-findings-26/bennett-findings-26-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-findings-26/bennett-findings-26-2.png) |
| Laboratory review minutes | [DOCX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-lab-minutes.docx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-lab-minutes/bennett-lab-minutes-1.png) |
| Culture-date comparison・3行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-culture-day-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-culture-day-3/bennett-culture-day-3-1.png) |
| Culture-date comparison・24行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-culture-day-24.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-culture-day-24/bennett-culture-day-24-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-culture-day-24/bennett-culture-day-24-2.png) |
| Culture-date comparison・25行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-culture-day-25.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-culture-day-25/bennett-culture-day-25-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-culture-day-25/bennett-culture-day-25-2.png) |
| Interlaboratory comparison・3組 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-facility-comparison-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-facility-comparison-3/bennett-facility-comparison-3-1.png) |
| Interlaboratory comparison・18組 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-facility-comparison-18.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-facility-comparison-18/bennett-facility-comparison-18-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-facility-comparison-18/bennett-facility-comparison-18-2.png) |
| Sample disposition・3行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-storage-disposition-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-disposition-3/bennett-storage-disposition-3-1.png) |
| Sample disposition・36行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-storage-disposition-36.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-disposition-36/bennett-storage-disposition-36-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-disposition-36/bennett-storage-disposition-36-2.png)、[3ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-disposition-36/bennett-storage-disposition-36-3.png) |
| Storage time group summary・明細3行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-storage-summary-short.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-short/bennett-storage-summary-short-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-short/bennett-storage-summary-short-2.png) |
| Storage time group summary・明細34行 | [XLSX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-storage-summary-full.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-full/bennett-storage-summary-full-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-full/bennett-storage-summary-full-2.png)、[3ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-full/bennett-storage-summary-full-3.png)、[4ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-storage-summary-full/bennett-storage-summary-full-4.png) |
| Laboratory director briefing・5レイアウト | [PPTX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-director-brief.pptx) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-director-brief/bennett-director-brief-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-director-brief/bennett-director-brief-2.png)、[3ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-director-brief/bennett-director-brief-3.png)、[4ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-director-brief/bennett-director-brief-4.png)、[5ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-director-brief/bennett-director-brief-5.png) |
| Measurement work sheet・3行 | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-bench-record-3.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-bench-record-3.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-bench-record-3/bennett-bench-record-3-1.png) |
| Measurement work sheet・33行 | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-bench-record-33.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-bench-record-33.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-bench-record-33/bennett-bench-record-33-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-bench-record-33/bennett-bench-record-33-2.png)、[3ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-bench-record-33/bennett-bench-record-33-3.png) |
| Measurement work sheet・スキャン | [PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-bench-scan.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-bench-scan/bennett-bench-scan-1.png) |
| Quality hold notice | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-hold-notice.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-hold-notice.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-hold-notice/bennett-hold-notice-1.png) |
| Quality deviation report | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-deviation-report.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-deviation-report.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-deviation-report/bennett-deviation-report-1.png) |
| Laboratory operating decision | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-operating-decision.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/bennett-operating-decision.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/bennett-operating-decision/bennett-operating-decision-1.png) |
| Calibration service record・要約 | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/mason-service-record.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/mason-service-record.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/mason-service-record/mason-service-record-1.png) |
| Calibration service record・明細付き | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/mason-service-record-details.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/mason-service-record-details.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/mason-service-record-details/mason-service-record-details-1.png)、[2ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/mason-service-record-details/mason-service-record-details-2.png) |
| Interlaboratory comparison note | [TEX](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/caldwell-comparison-note.tex)、[PDF](../../scratch/p06-life-science-researcher/b005/work/templates-0004/samples/caldwell-comparison-note.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b005/work/templates-0004/render/caldwell-comparison-note/caldwell-comparison-note-1.png) |
