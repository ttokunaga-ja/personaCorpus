# Melissa Watkins の文書の型

Bennett Cell Evaluation Inc の Cell Evaluation Laboratory が、非臨床の測定評価で使う英語の書式。Office の型は通常の DOCX・XLSX・PPTX として開き、別名で保存する。管理番号、本文、日付、人名、明細は原稿とファイル計画の値を入れる。日付は Philadelphia の現地日付（America/New_York）で、文書では `Jul 11, 2024`、機器出力などの交換用データでは元の記録形式を使う。

## 社内の型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Study plan | Bennett、Melissa Watkins が計画を起こし、Robert Nguyen が承認する。計画草案と承認版の PDF に使用 | `bennett-study-plan.docx` | US Letter 縦。表紙なし。共通ヘッダと4節で、設計・基準・承認の場所が固定 |
| Findings review | Bennett、Melissa が集計結果、培養日別・保存時間群別の解釈、次の提案を説明する | `bennett-findings.docx` | US Letter 縦。表紙なし。初見と群別確認を別の見出しに置き、根拠表の見出しを次ページにも出せる |
| Laboratory review minutes | Bennett、Melissa が Robert・Lauren らとの検討で照合した記録、判断、担当を残す | `bennett-lab-minutes.docx` | US Letter 縦。表紙なし。会議・参照記録・判断・対応の4節と横罫の対応表 |
| Culture-date comparison | Bennett、Melissa が培養日×条件で測定群を比較する | `bennett-culture-day.xlsx` | US Letter 横。淡藤の管理欄とアイリス色の列見出し。左上に社章があり、続きのページにも管理欄と見出しを印刷 |
| Equipment review | Bennett、Melissa と Anthony が装置ごとの校正履歴、稼働時間、点検予定を整理する | `bennett-equipment-review.xlsx` | US Letter 横。Bennett の社章、淡藤の文書管理欄、アイリス色の7列見出し。日付・校正記録番号・単位を横並びで確認できる |
| Interlaboratory comparison | Bennett、Melissa が Bennett と Caldwell の原記録 ID・測定値を試料単位で照合し、改訂法の結果を濃度帯別にまとめる | `bennett-facility-comparison.xlsx` | US Letter 横。Bennett の社章・管理欄・反復見出しを継承。両施設の ID と数値を隣接させ、数値の次の文字列には左インデントを取る |
| Repeatability summary | Bennett、Melissa Watkins が Lauren Brooks の測定出力から単一ランの件数・平均・標準偏差・CV を集計し、Anthony Mendoza の確認記録を参照する | `bennett-repeatability-summary.xlsx` | US Letter 横。Run summary と Run readings の2シート。Bennett の管理欄、アイリスの列見出し、右寄せの測定値と左インデント付きの識別子 |
| Storage study sample disposition | Bennett、Melissa が保存時間試験の試料と原記録を照合し、Anthony の採否判定を反映する | `bennett-storage-disposition.xlsx` | US Letter 横。Bennett の管理欄とアイリス色の7列見出しを反復。予定時間を右寄せし、右隣の原ラベルは字下げ。集計は最終明細の後に置く |
| Storage time group summary | Bennett、Melissa が採否確定後の試料で保存時間別の母数・平均・基準比を算出する | `bennett-storage-summary.xlsx` | US Letter 横。群別集計と解析対象の明細を2シートに分け、両シートとも Bennett の社章、淡藤の管理欄とアイリスの7列見出しを反復 |
| Figure and caption register | Bennett、Melissa が Angela 向け共同報告の図・凡例・母数と本文参照を照合する | `bennett-figure-register.xlsx` | US Letter 横。社章と淡藤の管理欄、アイリスの7列見出しを各ページに反復。母数の右隣に単位を字下げして表示 |
| Laboratory director briefing | Bennett、Melissa が Robert に基準・初回測定・再集計・再試験の提案を説明し、Angela との共同検討では図と改稿箇所を並べる | `bennett-director-brief.pptx` | 16:9。濃い紫の左帯、上部の小さな社章、下部の細線を共有するマスターと5つのレイアウト |
| Joint report | Bennett、Melissa が組版して Caldwell の Angela Price に共同検討用の報告を渡す | `bennett-joint-report.cls` | US Letter 縦。表紙なし。Bennett の社章、淡藤の管理欄、アイリス色の番号付き節見出しと図番号。左下に JOINT REPORT、右下にページ番号 |
| Measurement work sheet | Bennett、Lauren Brooks が測定日と原記録の参照情報を作業票に記録する。Lab-book のスキャン PDF の台紙に使用 | `bennett-bench-record.cls` | US Letter 縦。共通ヘッダ、淡藤の記録欄、アイリス色の反復見出し付きの一覧。左下に記録種別、右下にページ番号 |
| Quality hold notice | Bennett、Anthony Mendoza が保留範囲と配布・解除条件を通知する | `bennett-hold-notice.cls` | US Letter 縦。Bennett の共通ヘッダを継承し、発効日時・発行者・配布先を淡藤の枠に置く。フッタは QUALITY NOTICE |
| Quality deviation report | Bennett、Anthony が逸脱の根拠・採否・原記録の保管先を確定して発行する | `bennett-deviation-report.cls` | US Letter 縦。共通ヘッダとアイリスの節見出し。文書番号・判定日・発行者・状態を淡藤の枠に置く。左下は QUALITY DEVIATION REPORT、右下はページ番号 |
| Quality confirmation | Bennett、Anthony Mendoza が測定集計の照合結果と確認範囲を発行する | `bennett-quality-confirmation.cls` | US Letter 縦。Bennett の社章・節見出し、淡藤の番号・確認日・確認者・ラン欄、QUALITY CONFIRMATION とページ番号のフッタ |
| Laboratory operating decision | Bennett、Melissa が作成し、Robert が承認した業務上限、判断根拠、適用範囲を発行する | `bennett-operating-decision.cls` | US Letter 縦。共通ヘッダ、アイリスの節見出し、淡藤の番号・決定日・作成者・承認者欄。左下は LABORATORY OPERATING DECISION、右下はページ番号 |

## 社外から受け取る型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Calibration service record | Mason Laboratory Maintenance、Christopher Medina が校正・点検の記録を発行 | `mason-service-record.cls` | US Letter 縦。Liberation Mono、墨色 `#26291F`、オリーブ `#515A34`、紙色 `#F0EDDF`。三段の白いゲージと大文字の社名を反転帯に置き、管理欄を3行に分ける |
| Interlaboratory comparison note | Caldwell Assay Services、Angela Price が測定の相互比較所見を Melissa に送付 | `caldwell-comparison-note.cls` | US Letter 縦。Liberation Serif、濃紺 `#18354B`、青緑 `#1B6473`、淡青 `#E7F1F2`。菱形の社章と社名を中央に置き、件名と識別番号を細罫の下に配置 |

Mason は `MLM-YYYY-MMDD-SV-NNN`、Caldwell は `CAS-YYYY-MMDD-CMP-NNN`。年・月日と発行側の連番を使う。Caldwell の宛先は管理欄の直後に置く。いずれも発行者の社名・番号・日付を使う。CSV や写真は取得元の形式と対応する日付を保つ。

## Bennett の共通書式

- 社章は円環と2つの点を社名と組み合わせる。Liberation Sans を使い、本文とタイトルは濃紫 `#362742`、罫線と強調はアイリス `#765387`、管理欄や集計の淡色は `#F2EAF3`。濃色の表見出しには白字を使う。
- 文書番号は `BCE-CE-YYYY-TYP-NNN-Rnn`。種別は計画 `PLN`、所見 `REV`、議事 `MIN`、集計・照合表 `TBL`、説明資料 `BRF`、共同報告 `RPT`、作業票 `LAB`、保留通知 `HLD`、逸脱報告 `DEV`、品質確認 `QCF`、業務上の決定 `DEC`。年と種別内の3桁連番で起票し、版は `R00` から始める。同じ文書の改訂では連番を引き継いで版を進める。

### Word

- `Normal` は 10.5 pt、`Title` は 24 pt、`Heading 1` は 13 pt、`Heading 2` は 11 pt。タイトルの下に `Caption` で番号・日付・氏名・状態を1段で置き、各節見出しの下に `Normal` で本文を入れる。承認 PDF は承認済み DOCX の版を `bin/office-pdf` で書き出す。
- 表が必要な箇所にだけ `Bennett Evidence` で作る。見出しは `Bennett Table Heading` とアイリスの塗りで、先頭行を反復見出しにする。例の3列は 0.64 / 1.95 / 4.31 in。番号は `Bennett Table Number` で右寄せ、直後の文字列は `Bennett Table After Number` で左寄せ・左 0.13 in、残りの文字列は `Bennett Table Text`。数値列を増やす場合も同じ組合せを使う。行をページ途中で分割しない設定にする。Word のページ番号フィールドはフッタにある。
- 施設間の照合所見では Findings review の日別確認見出しを `Pair-level reconciliation` に変更し、判断の記録には Laboratory review minutes を使う。

### Excel

各ブックのテーマ、既定の `Normal`、見出し、管理欄の入力セル、明細の名前付き書式に Bennett の配色と Liberation Sans を使う。

#### 装置レビュー

1. A2 に表題、B4 に正式な文書番号、E4 に作成者、B5 に実日付（表示形式 `mmm d, yyyy`）、E5 に装置タグを入れる。管理欄の結合と書式を保つ。7行目の見出しは原稿に合わせ、使わない列の見出しは消す。明細は8行目から追加し、日付は `Bennett Date`、右揃えの数値は `Bennett Number`、率は `Bennett Percent`、記録番号や所見は左インデント付きの `Bennett Text` を使う。
2. 校正履歴の見出しは A:G の順に Visit date、Visit type、Service record、As-found、As-left、Result、Note。D・E は `Bennett Number` を適用してから表示形式を `+0.00;-0.00;0.00` にし、F の判定は原稿の基準で計算する。明細行は32 ptを目安にする。校正偏差は合算せず、最終明細の直後の A:G を結合し、`Bennett Note` で基準点・偏差の単位・許容帯を記す。対象外の旧記録は注記に記す。
3. 稼働明細は A:G の順に Period start、Period end、Source record、Active min、Active hours、Use category、Scope note。A・B は実日付、D は原記録の整数分、E は同じ行の D を60で割る式（例 `=D8/60`）。書式適用後に D を `#,##0`、E を `#,##0.0` にする。明細行は30 ptを目安にする。
4. 明細全体の合計を載せる場合は、最終明細の直後に1行作る。その行の A:B を結合し、A に「Total active use」、G に集計範囲を記し、A・C・F・G に `Bennett Total Label`、D・E に `Bennett Total` を適用する。D は実際の明細だけを合計（33行目が最終明細なら `=SUM(D8:D33)`）し、E は同行の D を60で割る。書式適用後に D を `#,##0`、E を `#,##0.0` にする。合計行は32 ptを目安にする。注記は次の行の A:G を結合して `Bennett Note` を適用する。
5. 2期間を比較する場合は、月別明細と別の `Period comparison` シートを作る。型のシートを複製し、社章も複製して両シートの管理欄を埋める（複製時に画像が引き継がれない場合は、同じ型から社章を再挿入する）。月別明細のシート名を `Monthly use` とする。集計シートの A7:G7 は Period start、Period end、Calendar days、Active min、Active hours、Change (%)、Scope。8行目を基準期間、9行目を比較期間にし、A・B に両端を含む実日付、C に日数（例 `=B8-A8+1`）、D に対象期間の明細の合計（b006 の例は `=SUM('Monthly use'!D8:D10)` と `=SUM('Monthly use'!D11:D13)`）、E に同じ行の D を60で割る式、G に基準・比較の別を入れる。A・B・G に `Bennett Total Label`、C:F に `Bennett Total` を適用し、適用後に A・B は `mmm d, yyyy`、D は `#,##0`、E は `#,##0.0` にする。F8:F9 の値は空欄にする。
6. 集計シートの10行目に差分を置く。A:B を結合して A に「Change vs. baseline」、G に分母の期間を記し、A・C・G に `Bennett Total Label`、D:F に `Bennett Total` を適用する。D に `=D9-D8`、E に `=E9-E8`、F に `=IF(E8=0,"",E10/E8)` を入れ、書式適用後に D は `+#,##0;-#,##0;0`、E は `+#,##0.0;-#,##0.0;0.0`、F は `+0.0%;-0.0%;0.0%` にする。8〜10行目は32 ptを目安にする。11行目の A:G を結合し、`Bennett Note` で集計対象・除外・開始と終了の日付の扱いを記す。b006 では各92日で 14,400 分（240.0 h）と 17,568 分（292.8 h）、差分は +52.8 h、基準期間比の変化は +22.0%。
7. 各シートの印刷範囲を A1 から最終使用列・行までに更新する。US Letter 横で横1ページ、縦は必要なページ数、1〜7行を各ページで反復する。複製したシートにも、左に `LABORATORY RECORD`、右に `Page &P / &N` の印刷フッタを引き継ぐ。結合した注記は28 ptを目安にし、管理欄・明細・集計も折返しに合わせて高さを調整する。数値列の右隣の文字列には `Bennett Text` または `Bennett Total Label` の左インデントを維持する。

#### 培養日別集計

1. `Culture date` の B4:C4 に文書番号、E4:G4 に作成者、B5:C5 に現地でのレビュー日、E5:G5 に指標と単位を入れる。B5 には実日付を入力し、型の表示形式 `mmm d, yyyy` を使う。結合範囲の左上セルを使う。番号欄は結合幅32と行高30 pt、折返し設定を持つ。A7:G7 は `Bennett Header`。条件列の表示名は原稿に合わせる。必要なら同じ管理欄・印刷設定を持つシートを複製し、別の指標を載せる。
2. 明細を8行目から必要数だけ作る。Aの通し番号と C:F の整数は `Bennett Number`（右寄せ・インデント1）、B の実日付は `Bennett Date`（左寄せ・インデント1）、G の注記は `Bennett Text`（左寄せ・インデント1）。この余白で A/B と F/G の境界を離す。率は比率値を `Bennett Percent` で表示する。行高は短い注記で26 ptを目安にし、折り返す行だけ広げる。件数を比較する場合は F を条件別件数の行合計とする。率や CV の横断集計は、分子・分母や基礎データから求める。
3. 集計が必要なら最終明細の直後に A:B を結合してラベルを入れ、A・G に `Bennett Total Label`、C:F に `Bennett Total` を使う。件数表では各列に明細範囲の `SUM` を入れる。注記がある場合は次の行の A:G を結合し、`Bennett Note` を適用する。集計行は32 pt、1行で収まる注記は28 ptを目安に、折返しに合わせて広げる。
4. 印刷範囲は A1 から実際の最終行の G 列まで。左2列と先頭7行を固定し、印刷では1〜7行を反復する。横幅を1ページに合わせ、縦方向は必要なページ数を使う。印刷プレビューで最終明細、集計、注記が同じページに載ることを確認する。分かれるときは、A:G が横1ページに収まる固定倍率に切り替え、最終明細を含む末尾のまとまりの直前に横改ページを入れる。タイトル行の反復を維持し、再度プレビューで確認する。短い表と複数ページの表で番号と日付の全文、隣接欄と続きの見出しも確認する。

#### 施設間照合

1. `Matched samples` の B4:C4 に文書番号、E4:G4 に作成者、B5:C5 にレビュー日、E5:G5 に測定単位を入れる。結合左上の B4、E4、B5、E5 を使う。日付は実日付とし、表示形式を `mmm d, yyyy` にする。A7:G7 はアイリス色の `Bennett Header`。
2. 対応 TSV を参照し、8行目から1組につき1行追加する。A は組番号に `Bennett Number`、B と D は各施設の原記録 ID に `Bennett Text`、C と E は各施設の測定値に `Bennett Measurement`、F と G は照合状態・参照注記に `Bennett Text`。ID は文字列で保持する。数値書式は右寄せ、文字書式は左寄せで各1文字分のインデントを持つ。測定値は小数2桁で表示し、元記録に応じて表示桁数を調整する。明細は36 ptを目安にし、注記の折返しに合わせて行高を調整する。
3. 集計が必要な場合は最終明細の直後に置く。A:B を結合し、A・D・G に `Bennett Total Label`、C の照合件数に `Bennett Total`、E・F の施設間差と基準に `Bennett Summary Percent` を使う。F列の照合済み状態を `Matched` とした表の集計例は `=COUNTIF(F8:F25,"Matched")`（18組の明細）。2025年1月16日の施設間差と基準は、確認済みの比率値 `0.082` と `0.10` を別々に入れる。Gに両欄の意味を記す。次の行を A:G で結合し、比較対象や適用範囲を `Bennett Note` で記す。集計行32 pt、注記行28 ptを目安に、全文が見える行高にする。
4. 印刷範囲は A1:G の実際の最終行までに更新する。型の US Letter 横・固定倍率80%・反復行1〜7を維持する。Excel の拡大縮小印刷では「拡大/縮小」を使う。openpyxl では `fitToPage=False` と `scale=80` を維持し、改ページの直前の行番号を `Break(id=…)` で追加する。36 pt の明細を使う場合、3組は改ページなし、18組は10組目（17行目）の後、30組は10・20組目（17・27行目）の後で改ページする。最後の明細ページに集計行と注記を収める。行高・明細数・集計の段数を変える場合は、印刷プレビューで末尾のまとまりが分かれない位置まで直前の改ページを早める。管理欄4項目、A/B・C/D・E/F の列境界、続頁の見出しを全ページで確認する。
5. 2026年の改訂法比較では、明細シートの集計値・単位を原稿の差の定義に合わせて表示する。濃度帯別集計は、空の型を複製して `By concentration band` を作り、社章・管理欄・印刷設定を引き継ぐ。A7:G7 に `Bennett Header` で Band、Bennett interval、Matched pairs、Bennett mean、Caldwell mean、Mean difference、Review status を置く。8行目から実際の濃度帯を1帯1行で作り、A・B・G に `Bennett Text`、C に `Bennett Number`、D:F に `Bennett Measurement` を使う。行高は36 ptを目安にする。B列の判定元・帯の境界は原稿で確定したものに合わせ、同じ範囲条件で C に件数、D・E に各施設の平均を計算する。原稿が絶対差を指定する場合はペア別絶対差から集計する。F に差を表示する。最終帯の直後に A:B を結合して全体行を置く。A・G は `Bennett Total Label`、C:F は `Bennett Total` とし、D:F は小数2桁に変更する。全体行は30組を対象として求め、次行の A:G を結合し `Bennett Note` で単位と差の向き・定義を記す。低濃度帯の確認状況は G に記載し、数値の右隣に `Bennett Text` または `Bennett Total Label` の字下げを保つ。帯別シートは A1:G の実際の最終行までを印刷する。見本の数値・帯境界は組版確認用。

#### 単一ランの再現性

1. `Run summary` と `Run readings` の B4、E4、B5 に正式な文書番号、作成者、確認日を記す。日付は実日付とする。E5 には順に、ラン識別子と元 CSV のファイル名を入れる。管理欄の結合を保つ。
2. `Run readings` は8行目から CSV の行を入力する。A は通し番号（`Bennett Number`）、B は原記録 ID（`Bennett Text`）、C は測定値（`Bennett Measurement`）、D は実日付（`Bennett Date`）、E:G はラン・確認状況・注記（`Bennett Text`）。数値と右隣の日付・文字のインデントを保ち、行高30 ptを目安に全文が読める高さにする。CSV が全測定行の原記録で、集計対象は原稿に従う。
3. `Run summary` の8行目を1ランの集計行とする。A、G は `Bennett Total Label`、B は `Bennett Date`、C は `Bennett Total`、D・E は `Bennett Total` を適用して表示形式を順に `0.0` と `0.00` にし、F は `Bennett Summary Percent`。全24行が対象であれば C:E は `'Run readings'!C8:C31` を `COUNT`、`AVERAGE`、標本標準偏差の場合は `STDEV`（Excel の `STDEV.S` と同じ標本標準偏差）で参照する。CV は丸め前の標準偏差を平均で割り、D は小数1桁、F は小数1桁の百分率で表示する。G に確認記録を記す。集計行は36 ptを目安にし、注記は次行の A:G を結合し `Bennett Note` で適用範囲を記す。
4. 両シートとも印刷範囲を A1:G の実際の最終行に合わせる。US Letter 横・固定倍率78%・反復行1〜7を維持する。24行の見本は明細12行目（シートの19行目）の後に横改ページを入れる。短い明細の場合は改ページなし。折返しに合わせた行高で、管理欄と明細末尾の注記を印刷プレビューで確認する。

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

#### 図・凡例と本文参照の照合

1. `Figure register` の B4 に照合表の正式番号、E4 に作成者、B5 に現地レビュー日、E5 に照合する報告の正式番号と版を入れる。B5 は実日付で `mmm d, yyyy`。型の結合範囲と管理欄の書式を使う。
2. 8行目から、図・パネル・系列ごとに必要な行を作る。A は図と系列、B は表示値 CSV または元表と必要な範囲、C は解析母数、D は表示単位、E は実際の凡例、F は本文の節・図番号、G は採否や母数の根拠を入れる。C は `Bennett Number`、A・B・D:G は `Bennett Text` を使う。C の右寄せと D の左インデントを保つ。行高は36 ptを目安に、折返しに合わせて調整する。
3. 図をまたいで同じ試料を使う場合、各行の母数を足すと重複する。集計が必要な場合は、重複のない対象に限定し、最終明細の直後に A:B を結合して集計範囲を記し、A・D:G に `Bennett Total Label`、C に `Bennett Total` を適用する。注記は最後の行の下で A:G を結合し、`Bennett Note` を適用する。注記行は30 ptを目安にする。
4. 印刷範囲は A1 から G 列の最終使用行まで。US Letter 横、固定倍率80%、反復行1〜7を維持する。36 pt の15明細行の見本では、9行目の明細の後で改ページする（openpyxl の `Break(id=16)`）。必要に応じて図のまとまりや末尾の注記の位置に合わせて改ページを動かす。管理欄、C/D の境界、続頁の列見出し、最終明細と注記を全ページで確認する。

### PowerPoint

`BCE Cover` は題名と番号・日付・作成者を配置する。番号・日付・作成者のプレースホルダーは濃紫 `#362742` を継承する。`BCE Heading and Body` は見出しと本文、`BCE Two Columns` は全体と培養日別などを並べる用途。 `BCE Evidence Table` は表用プレースホルダーと下部の出典欄、`BCE Figure and Caption` は左側に画像、右側に説明を入れる。表はアイリス塗り・白字の見出し、濃紫の本文、必要な集計行だけ淡藤で示す。数値列は右寄せで、続く文字列セルには左余白を約 0.2 in とる。図には測定母数、凡例・単位、出典を必要に応じて入れる。日付は表紙の管理欄に記録する。スライド番号はフッタ右端の罫線下に置き、マスターの番号欄（左11.77 in、上6.96 in、幅0.96 in、右寄せ）を5つのレイアウトで継承する。成形時はスライド番号を有効にする。プログラムで成形する場合は、レイアウトの番号プレースホルダーをスライドに追加し、番号フィールド（`slidenum`）に実際のスライド番号を入れる。

### TeX とスキャン

クラスを選んで本文の `.tex` を用意する。出力 PDF に `bin/stamp` で作成者と文書の日付を設定する。リポジトリ直下から実行する場合は `TEXINPUTS="$PWD/templates/p06-life-science-researcher:" bin/tex input.tex output.pdf` のように型の検索先を指定する。`input.tex` と生成物には作業先のパスを使う。

- Bennett は `\BennettRecord{title}{document no.}{measurement date}{recorded by}` の後に `\BennettSection{heading}` と本文を置く。受渡し票では `\BennettRecord[Handoff date]{title}{document no.}{handoff date}{recorded by}` を使う。`BennettBenchTable` は番号・参照・確認状況・所見の4列。実際の記録行だけ追加し、各行を `\\` で閉じる。番号と参照の間に 0.10 in の追加余白が入り、見出し行が改ページ時に繰り返される。Lab-book の紙記録は中身を入れてから `bin/scan` を使う。手書きとして扱う欄の描画は成形時に行う。
- Quality hold notice は `\BennettHoldNotice{title}{document no.}{effective date and time}{issued by}{distribution}` の後に `\BennettSection{heading}` で Scope、Handling、Review を組む。必要なら本文に日別の対象表を置く。
- Quality deviation report は `\BennettDeviationReport{title}{document no.}{decision date}{issued by}{status}` の後に `\BennettSection{heading}` で Occurrence and records reviewed、Disposition、Analysis and filing を組む。食い違った記載は原稿で確定した原記録の参照とともに置き、採否・原記録の保管先・解析対象への影響を記す。テキスト層付き PDF として保存する。
- Quality confirmation は `\BennettQualityConfirmation{title}{document no.}{review date}{reviewed by}{run}` の後に Records reviewed、Confirmation、Scope を `\BennettSection{heading}` で置く。参照した測定値・集計と、Anthony の確認対象、確認した値と基準への判定を記す。
- Laboratory operating decision は `\BennettOperatingDecision{title}{document no.}{decision date}{prepared by}{approved by}` の後に `\BennettSection{heading}` で Decision、Basis、Application を組む。承認された上限、群別母数と基準比、適用する業務と条件を記し、テキスト層付き PDF として保存する。
- Mason は `\MasonRecord{title}{service no.}{service date}{issued by}` と `\MasonSection{heading}` を使う。本文は左揃えで、左右 0.9 in の余白内で単語の間を使って折り返す。詳細を別ページに載せる場合は、ページ冒頭にサービス番号・装置識別・日付を再掲し、測定値を校正前・校正後に分ける。Caldwell は `\CaldwellRecord{title}{reference}{date}{from}` と `\CaldwellSection{heading}` を使う。

### 初回の施設間比較

Correspondence の送受信は EML、各施設の測定値は各自の CSV、両者の対応は TSV で保存する。Method-comparison の照合 XLSX は2つの原記録を参照できる形にし、図の PNG は原稿で確定した施設別の値と単位を使う。Angela の所見 PDF は Caldwell の型、Lab-share の討議 PPTX と決定 DOCX は Bennett の型で作る。2025年1月16日の判断は18試料の施設間差8.2%と基準10%の照合、および Robert による現条件での継続承認に限る。10月に保留した記録の処分は別の品質記録で管理する。

### 改訂法の施設間比較と A-24

4月14日の30組は Interlaboratory comparison の明細と濃度帯別集計で扱う。差の図は原稿の単位・差の定義・濃度帯ごとの母数を入れ、Angela の所見は Caldwell の型を使う。7月21日の A-24 は Bennett 単独の24試料で、Lauren の測定 CSV を Repeatability summary から参照する。平均6.8 ng/mL・CV 9.4%と Anthony の確認範囲は Quality confirmation に記す。両案件の母数・確認状況はそれぞれの記録で管理する。Lab-book の作業票は Measurement work sheet を使い、スキャン・写真・観察メモを測定日で対応させる。

### 上清保存時間試験

Storage-study の計画は Study plan を開き、設計の節に 0・24・48 時間の各12試料を記す。Lab-book の受渡し票には Measurement work sheet を使い、管理欄のほかに引渡し側・受領側、原ラベルと受渡し記録の参照、押印・注記を載せる余白を本文に設ける。押印・注記を載せた紙面を `bin/scan` に通す。測定 CSV と試料対応 TSV は原記録の値・記載を保存し、Desktop の照合メモは TXT で残す。採否は Storage study sample disposition に記録し、Quality deviation report で確定した2件と根拠を指す。予定母数と解析対象数は別に記録する。

8〜10月の解析では、確定した解析対象 12・11・11 件を Storage time group summary にまとめる。比較図 PNG は同じ集計値を使い、0 h 基準と群別の解析対象数を明記する。Findings review の `Date-level review` は `Storage-group comparison` に変更し、結果・解釈・提案を分ける。室長向け説明には Laboratory director briefing を使い、採用された24時間上限と適用範囲は10月16日付の Laboratory operating decision に記す。結論は試験で確認した条件と承認範囲に限る。

### Angela 向け共同報告

- 編集用の本文 DOCX は Findings review の表題と節見出しを差し替え、番号・日付・作成者・確認先を管理行に置く。節と図の番号は組版稿と対応させ、図注には `Caption` を使う。赤字校正紙は対象版の該当ページに削除と修正指示を載せ、`bin/office-pdf` で出した紙面を `bin/scan` に通す。
- 組版 PDF は Joint report で `\BennettJointReport{title}{document no.}{revision date}{prepared by}{review recipient}` に続けて `\section` と `\subsection` を使う。図は `figure` 内に画像、`\caption`、`\label` の順で置き、本文から `\ref` で参照する。図注には表示単位、系列の解析母数と必要な除外注記を入れる。元の DOCX と同じ版を組む場合は同じ文書番号を使う。
- 図7枚を Figure and caption register で照合する。保存試験に基づく図では、0・24・48時間の解析母数12・11・11と除外2件（24時間・48時間に各1件）を該当する図注に反映する。保存の結論は評価した範囲の24時間以内に限定する。表示値 CSV には図で用いた単位と値を保持し、PNG に表示した系列を照合表からたどれるようにする。
- 共同検討には Laboratory director briefing の2段、表、図と説明のレイアウトを使い、旧記載と改訂後の文言や図注の変更を示す。Angela 宛て EML は照合した版と修正箇所を本文に記し、Caldwell が発行した相互比較所見がある場合は Caldwell の型を使う。

### 培地交換間隔の再評価

- Medium-exchange の追跡表は Repeatability summary を別名で保存し、2シートを `Decision basis` と `Reading trace` に変更する。A2 の表題を差し替え、両シートの B4・E4・B5・E5 に正式文書番号・作成者・現地レビュー日・参照 CSV 名を入れる。D5 は `Source CSV`、B5 は実日付で表示形式 `mmm d, yyyy`。結合範囲と印刷フッタを維持する。
- `Reading trace` の A7:G7 を `Seq.`、`Interval (h)`、`Sample ID`、`Prep. date`、`Measurement round`、`Result`、`Source row / date` に替える。測定単位は原稿に従って Result 見出しに併記する。8行目から A・B に `Bennett Number`、C・E・G に `Bennett Text`、D に `Bennett Date`、F に `Bennett Measurement` を使う。C と G の左インデントで数値列との間隔を取る。調製日は実日付、測定回は初回または調製日をそろえた再試験を原記録の識別子で記し、G からその測定日と元 CSV 行をたどれるようにする。
- `Decision basis` の A7:G7 を `Interval (h)`、`Basis round`、`Sample n`、`Group CV`、`CV criterion`、`Disposition`、`Evidence reference` に替える。両シートの見出しは `Bennett Header`。各判断の根拠を8行目から置き、A に `Bennett Number`、B・F・G に `Bennett Text`、C に `Bennett Total`、D・E に `Bennett Summary Percent` を使う。G に採用した明細範囲を記し、C と D はその範囲だけを集計する。初回と再試験を併記する場合は判断に用いた行を明示する。同じ試料を複数回測定した場合は、総試料数に重複計上しない。
- 明細は30 pt、根拠行と集計行は32 ptを目安にし、折返しに合わせて広げる。集計が必要なときは最終根拠行の直後の A:B を結合し `Bennett Total Label`、件数は `Bennett Total`。注記は次の行の A:G を結合して `Bennett Note`。両シートの印刷範囲を A1:G の最終使用行までに更新し、US Letter 横・固定倍率78%・反復行1〜7を保つ。30明細を群別に10行ずつ置く場合は17・27行目の後に横改ページを入れる。少数明細では改ページを追加しない。
- 判断理由の DOCX と Desktop の面談準備稿には Findings review を使い、初回の所見、調製日をそろえた再試験、判断に採用した測定回を分けて記す。準備稿は初回と再試験の対応を紙面に注記し、`bin/office-pdf` で出した PDF を `bin/scan` に通す。群内のばらつきの PNG は追跡表と同じ採用値を使い、交換間隔、測定回、母数、単位を示す。
- Staff-training の説明には Laboratory director briefing を使い、9月23日の面談で確認した30試料、72時間群の CV 21% と基準15%、次期を48時間までとした理由を示す。合意事項のテキスト PDF は Laboratory operating decision の Decision、Basis、Application の3節で組む。ここで扱うのは培地交換の間隔。上清の保存上限24時間は別の業務記録で扱う。

## 印刷見本

見本の明細・番号・日付は組版確認用。成形ではファイル計画・台帳・原稿の確定値を使う。PPTX は表紙・本文・2段・表・図の順。培地交換間隔の追跡表は、最初のページが判断根拠、続くページが測定回の明細。

| 見本 | 元ファイル | レンダリング |
|---|---|---|
| Study plan | [DOCX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-study-plan-3.docx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-study-plan-3/bennett-study-plan-3-1.png) |
| Findings review・3行 | [DOCX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-findings-3.docx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-findings-3/bennett-findings-3-1.png) |
| Findings review・26行 | [DOCX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-findings-26.docx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-findings-26/bennett-findings-26-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-findings-26/bennett-findings-26-2.png) |
| Laboratory review minutes | [DOCX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-lab-minutes-3.docx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-lab-minutes-3/bennett-lab-minutes-3-1.png) |
| Culture-date comparison・3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-culture-day-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-culture-day-3/bennett-culture-day-3-1.png) |
| Culture-date comparison・24行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-culture-day-24.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-culture-day-24/bennett-culture-day-24-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-culture-day-24/bennett-culture-day-24-2.png) |
| Equipment review・校正履歴3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-equipment-review-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-equipment-review-3/bennett-equipment-review-3-1.png) |
| Equipment review・稼働明細26行と2期間比較 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-equipment-review-26.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-equipment-review-26/bennett-equipment-review-26-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-equipment-review-26/bennett-equipment-review-26-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-equipment-review-26/bennett-equipment-review-26-3.png), [4ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-equipment-review-26/bennett-equipment-review-26-4.png) |
| Interlaboratory comparison・3組 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-facility-comparison-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-facility-comparison-3/bennett-facility-comparison-3-1.png) |
| Interlaboratory comparison・18組 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-facility-comparison-18.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-facility-comparison-18/bennett-facility-comparison-18-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-facility-comparison-18/bennett-facility-comparison-18-2.png) |
| Repeatability summary・3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-repeatability-summary-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-repeatability-summary-3/bennett-repeatability-summary-3-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-repeatability-summary-3/bennett-repeatability-summary-3-2.png) |
| Repeatability summary・24行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-repeatability-summary-24.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-repeatability-summary-24/bennett-repeatability-summary-24-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-repeatability-summary-24/bennett-repeatability-summary-24-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-repeatability-summary-24/bennett-repeatability-summary-24-3.png) |
| Sample disposition・3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-storage-disposition-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-disposition-3/bennett-storage-disposition-3-1.png) |
| Sample disposition・36行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-storage-disposition-36.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-disposition-36/bennett-storage-disposition-36-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-disposition-36/bennett-storage-disposition-36-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-disposition-36/bennett-storage-disposition-36-3.png) |
| Storage time group summary・明細3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-storage-summary-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-3/bennett-storage-summary-3-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-3/bennett-storage-summary-3-2.png) |
| Storage time group summary・明細34行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-storage-summary-34.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-34/bennett-storage-summary-34-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-34/bennett-storage-summary-34-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-34/bennett-storage-summary-34-3.png), [4ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-storage-summary-34/bennett-storage-summary-34-4.png) |
| Figure and caption register・3明細 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-figure-register-3.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-figure-register-3/bennett-figure-register-3-1.png) |
| Figure and caption register・15明細 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-figure-register-15.xlsx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-figure-register-15/bennett-figure-register-15-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-figure-register-15/bennett-figure-register-15-2.png) |
| Laboratory director briefing・5レイアウト | [PPTX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-director-brief.pptx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-director-brief/bennett-director-brief-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-director-brief/bennett-director-brief-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-director-brief/bennett-director-brief-3.png), [4ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-director-brief/bennett-director-brief-4.png), [5ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-director-brief/bennett-director-brief-5.png) |
| Joint report | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-joint-report-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-joint-report-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-joint-report-1/bennett-joint-report-1-1.png) |
| Measurement work sheet・3行 | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-bench-record-3.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-bench-record-3.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-bench-record-3/bennett-bench-record-3-1.png) |
| Measurement work sheet・33行 | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-bench-record-33.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-bench-record-33.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-bench-record-33/bennett-bench-record-33-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-bench-record-33/bennett-bench-record-33-2.png) |
| Quality hold notice | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-hold-notice-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-hold-notice-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-hold-notice-1/bennett-hold-notice-1-1.png) |
| Quality deviation report | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-deviation-report-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-deviation-report-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-deviation-report-1/bennett-deviation-report-1-1.png) |
| Quality confirmation | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-quality-confirmation-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-quality-confirmation-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-quality-confirmation-1/bennett-quality-confirmation-1-1.png) |
| Laboratory operating decision | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-operating-decision-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-operating-decision-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-operating-decision-1/bennett-operating-decision-1-1.png) |
| Calibration service record・続紙付き | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/mason-service-record-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/mason-service-record-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/mason-service-record-1/mason-service-record-1-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/mason-service-record-1/mason-service-record-1-2.png) |
| Interlaboratory comparison note | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/caldwell-comparison-note-1.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/caldwell-comparison-note-1.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/caldwell-comparison-note-1/caldwell-comparison-note-1-1.png) |
| 培地交換間隔の追跡表・3行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-trace-3.xlsx), [参照 CSV](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-extract.csv) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-3/bennett-exchange-trace-3-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-3/bennett-exchange-trace-3-2.png) |
| 培地交換間隔の追跡表・30行 | [XLSX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-trace-30.xlsx), [参照 CSV](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-extract.csv) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-30/bennett-exchange-trace-30-1.png), [2ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-30/bennett-exchange-trace-30-2.png), [3ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-30/bennett-exchange-trace-30-3.png), [4ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-trace-30/bennett-exchange-trace-30-4.png) |
| 培地交換間隔の面談準備稿 | [DOCX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-meeting-prep.docx) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-meeting-prep/bennett-exchange-meeting-prep-1.png) |
| 培地交換間隔の面談準備稿・スキャン | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-meeting-scan.pdf) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-meeting-scan/bennett-exchange-meeting-scan-1.png) |
| 培地交換間隔の合意事項 | [PDF](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-agreement.pdf), [TeX](../../scratch/p06-life-science-researcher/b009/work/templates-0002/samples/bennett-exchange-agreement.tex) | [1ページ](../../scratch/p06-life-science-researcher/b009/work/templates-0002/render/bennett-exchange-agreement/bennett-exchange-agreement-1.png) |
