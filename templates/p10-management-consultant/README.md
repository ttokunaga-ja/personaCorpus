# 経営コンサルタントの文書の型

Whitfield Components Ltd 日本事業の支払診断・実行案・試行、Marston Carton Packaging Ltd の2工場の原価診断、Pioneer Trade Distribution Inc の仕入先選定と例外申請の診断に使う。本文は英語、Holt の日付は `07 Oct 2024` の形式。Nicholas の PC の時刻は Europe/London。受領した記録の時刻とタイムゾーンは原資料に従う。Office の型は開いて別名保存する。

| 型 | 組織と用途 | ファイル | 見た目 |
|---|---|---|---|
| Evidence note | Holt & Reeves。Neha Patel が Whitfield の引渡条件と試行担当者の指摘、Marston での現場説明と観察の食い違いを確認する | `holt-handoffs.docx` | A4縦。表紙なし。濃い紫灰の表見出し、茶赤の節見出し、淡い管理欄 |
| Meeting record | Holt & Reeves。Nicholas Hargreaves が聞き取り後の論点、決定、担当を記録する | `holt-meeting-record.docx` | A4縦。表紙なし。25 pt の題と `Recorded by` 欄。項目、議論、次の対応を横に並べる |
| Review memorandum | Holt & Reeves。Nicholas が説明案への指摘、根拠の差し替え、配布前の修正、Elizabeth の実行案審査への回答を残す | `holt-review-memo.docx` | A4縦。表紙なし。32 pt の題と `Prepared by` 欄。修正箇所と修正理由を並べる |
| Working sheet | 型なし。Connor が同僚との照合用に使う作業表。案件符号ごとのイベント対の採否や時間の内訳を確認する | — | 列に見出しとフィルタを付けた通常の Excel ブック。時刻と時間の単位を区別する |
| Process timings | Holt & Reeves。Connor Atkinson が工程別に実作業、滞留、承認待ちの時間を比較する | `holt-process-timings.xlsx` | A4横。淡い管理欄、濃い紫灰の8列見出し、時間は小数1桁、割合は小数1桁の百分率 |
| Cost diagnosis | Holt & Reeves。Connor Atkinson と Deepa Desai が対象工場の歩留まりを比較し、設備更新の回収年数の根拠を確認する | `holt-cost-diagnosis.xlsx` | A4横。共通ロゴ、濃い紫灰の列見出し、淡い管理欄。歩留まり比較と設備回収で列構成を分ける |
| Whitfield finance schedules | Holt & Reeves。Connor Atkinson が旧試算、二重計上の訂正内訳、初期費用の発生時期を管理する | `holt-whitfield-finance.xlsx` | A4横。共通ロゴ、淡色の管理欄、濃い紫灰の8列見出し。円額を右寄せし、根拠と参照キーを並べる |
| Whitfield payment pilot | Holt & Reeves。Connor Atkinson が2部門の平均承認日数を再計算し、保留案件の扱いを原記録に照合する | `holt-whitfield-pilot.xlsx` | A4横。共通ロゴ、紫灰の8列見出し。比較、案件別時刻、保留照合の3シート |
| Pioneer buying schedules | Holt & Reeves。Nicholas が Meera Kulkarni と仕入先の絞込み条件を確認し、Neha Patel が例外申請の理由と需要が判明した時点を照合する | `holt-pioneer-buying.xlsx` | A4横。共通ロゴ、淡色の管理欄、濃い紫灰の8列見出し。仕入先選定と例外理由の再分類の2シート |
| Diagnosis briefing | Holt & Reeves。Nicholas が Kenneth Townsend に支払診断と試行拡大の判断材料を説明し、Elizabeth に Whitfield の実行案を提示する。Marston では設備案と作業標準統一、Pioneer では全社再交渉の旧案と Meera 向けの判断資料に使う | `holt-diagnosis.pptx` | 16:9。上に社名と茶赤の罫線、下に社名と管理情報。表紙、見出しと本文、2段、表、図の5レイアウト |
| Client note | Holt & Reeves。Nicholas が Kenneth Townsend に見込の訂正を文書で伝える | `holt-client-note.cls` | A4縦、表紙なし。共通ロゴ、茶赤の上罫と節見出し、淡い管理欄、3列の対照表 |
| Supplier payment procedure | Whitfield Components Ltd。日本事業の Finance が発行し、Kenneth Townsend が承認する支払手順と試行の現場手順 | `whitfield-payment-procedure.cls` | A4縦。表紙なし。深緑の三つのブロックと社名、右に `JAPAN FINANCE`。セリフ体、薄緑の発行欄、深緑の表見出し |
| Payment pilot scope change | Whitfield Components Ltd。Japan Finance が管理する試行範囲の変更合意書。Kenneth と Holt & Reeves 側の承認者が署名する | `whitfield-scope-change.cls` | A4縦。Whitfield 共通の深緑とセリフ体、右上に `JAPAN FINANCE / CHANGE CONTROL`。薄緑の4項目の管理欄、両社の署名欄 |
| Production work standard | Marston Carton Packaging Ltd。生産部門が発行し、Caroline Wood が診断時に現行版を提供する | `marston-work-standard.cls` | A4縦。表紙なし。折り箱のロゴ、青い文字と琥珀色の帯、淡青灰の版管理欄。4列の作業手順表 |
| Buying exception guidance | Pioneer Trade Distribution Inc。調達部長の Meera Kulkarni が、申請に必要な証拠、回付先、判断の記録方法を示す現行要領 | `pioneer-exception-guidance.cls` | Letter縦。表紙なし。合流する経路のロゴ、青緑の縦帯と節見出し、白地の管理欄、淡い青緑の手順表 |

## Holt & Reeves 共通

ロゴは上下辺が水平な平行四辺形を2枚使い、左下に紫灰、右上に茶赤を置いて社名を添える。濃い紫灰 `302435` を本文・表見出し背景、茶赤 `964B37` を罫線・注記・修正指示、薄い茶灰 `F4ECE8` を管理欄・合計欄に使う。表見出しの文字は白。英字は Liberation Sans、日本語が入る箇所は Noto Sans CJK JP。Excel はセルごとにフォントが決まるため、日本語を含むセルには書式適用後に書体名だけを Noto Sans CJK JP に替える。

文書番号は `HRA-<案件>-<種別>-<西暦4桁>-<連番3桁>`。案件符号は Whitfield が `WHI-DIAG`、Marston が `MAR-COST`、Pioneer が `PIO-BUY`。種別は Evidence note が `EV`、Meeting record が `MIN`、Review memorandum が `REV`、Process timings が `TIM`、Diagnosis briefing が `BRF`。Cost diagnosis は歩留まり比較が `YLD`、設備回収が `CAP`。Whitfield finance schedules は旧試算が `EST`、訂正内訳が `REC`、初期費用の発生時期が `CST`。Pioneer buying schedules は仕入先選定が `SEL`、例外理由の再分類が `EXC`。Client note は `NTE`。例は `HRA-PIO-BUY-SEL-2025-001`、`HRA-PIO-BUY-EXC-2025-001`、`HRA-PIO-BUY-BRF-2025-001`。改訂で番号を引き継ぐ場合は版を別記する。制作時の番号・作成者・日付は計画と事実の台帳に合わせる。

### Whitfield の支払試行と拡大判断

Holt の案件符号は既存の `WHI-DIAG` を使う。試行ワークブックの種別は `PLT`（例 `HRA-WHI-DIAG-PLT-2025-001`）。2部門で観測した6.2日と3.8日の比較は、観測期間、部門、起点と終点、保留を含める条件まで記す。判断資料は「試行で測った結果」「5部門へ渡す手順」「Kenneth が承認した範囲・条件」を分ける。担当者の指摘は Evidence note に、指摘の内容、発生した引渡条件、証拠、対応担当を記す。許可済み抜粋 CSV は受領時の列・対象範囲・時刻のオフセットを保つ。

### Marston の診断資料

対象は4工場から絞った2工場。Word の照合記録では面談での説明、観察記録、未確認事項を分ける。Excel では観測した歩留まりと、材料価格の影響を除いて推定した改善余地2.4ポイントを区別する。設備試算には初期投資、年間便益と追加費用、年間純便益を示し、回収が6年を超える根拠を残す。PowerPoint では、面談調整で8週から9週になった経緯と、設備更新に先立って作業標準を統一する Siobhan の提言を扱える。顧客の材料費内訳 CSV は受領時の列と単位を保ち、日程変更のメールは EML で保存する。顧客から受領する現行作業標準 PDF には Marston の型を使う。

### Word

各型を開き、最初の `Title` 段落と管理表の2行目を埋める。管理欄は文書番号・日付・作成者の順に幅 66／38／58 mm、値は `Holt Value`。本文は `Normal`（11 pt）、節は `Heading 1`（茶赤16 pt）と `Heading 2`（紫灰12 pt）、出典は `Caption`。全ページのヘッダにロゴと社名、フッタに社名とページ番号が入る。

明細表が必要なら、表本体を追加して `Holt Evidence Table` を選ぶ。番号・項目・説明の3列なら、固定幅 18／46／98 mm を表グリッドとセルに設定する。先頭行に `Holt Column`、明細に `Holt Cell` を適用し、見出し行の背景を `302435` にする。番号列は右寄せ、文字列は左寄せ。型の表スタイルの左右セル余白で隣接列の間隔を取る。先頭行を各ページで繰り返す（`w:tblHeader`）。実データの行だけを追加し、各明細行にページをまたがない設定（`w:cantSplit`）を使う。

### Excel — Whitfield の工程別集計

`Process timings` シートを開く。`A2` に表題、`A4` に文書番号、`D4` に日付型の値、`F4` に作成者を入れる。各結合範囲の左上セルへ入力すれば、型のフォント・折返し・行高を引き継げる。印刷ヘッダは社名、フッタ左には文書番号・日付・作成者を ` | ` でつなぎ、Liberation Sans 9 pt・紫灰 `302435` を設定する。右のページ番号欄は型を使う。

6行目の列見出しを残し、7行目から実データの行だけを作る。書式はブックに登録済みの名前付きスタイルを、値を書いた後で `cell.style` に指定する。

| 列 | 値 | 明細のスタイル |
|---|---|---|
| A | 工程番号 | `Holt Index`。右寄せ、2桁 |
| B | 工程名 | `Holt Text`。左寄せ、左インデント1 |
| C／D／E | 実作業／滞留全体／滞留のうち承認待ち（時間） | `Holt Hours`。右寄せ、小数1桁 |
| F | 承認待ち時間 ÷ 滞留全体 | `Holt Share`。右寄せ、百分率 |
| G／H | イベント区分／根拠または条件 | `Holt Text`。左寄せ、左インデント1 |

明細の行高は24 ptを目安に、折り返した行は行数に応じて広げる。合計が必要なときは最後の明細の次行を25 ptにし、A:B を結合して `Holt Total Label`、C〜E の列合計に `Holt Total Hours`、F に E列合計 ÷ D列合計を計算して `Holt Total Share` を使う。分母が0の行は意味に応じて処理する。G:H に合計の分母を記す場合は結合して `Holt Total Label` を使う。注記があれば次行の A:H を結合し、`Holt Note`、行高27 ptを目安に設定する。列見出しの繰り返しは6行目、印刷はA4横・横1ページ・縦自動。最終行を決めて `A1:H<最終行>` を印刷範囲に設定する。

### Excel — Marston の歩留まり比較と設備回収

`holt-cost-diagnosis.xlsx` を開き、`Yield comparison` または `Equipment payback` を使う。1文書に片方だけ使う場合は他方のシートを削除する。各シートの `A2` に表題、`A4` に文書番号、`D4` に実日付、`F4` に作成者を入れる。結合セルの左上に入力し、管理値行は34 pt。印刷フッタ左の設定は Whitfield と共通。6行目は列見出し、実データを7行目から追加し、次の名前付きスタイルを値の入力後に適用する。A列の番号と数値は右寄せ、文字列は左インデント1。行高は24 ptを目安とし、長い根拠で折り返す行を広げる。

| 列 | Yield comparison | Equipment payback |
|---|---|---|
| A | 行番号、`Holt Index` | 行番号、`Holt Index` |
| B | 工場・ライン、`Holt Text` | 費用・便益の項目、`Holt Text` |
| C | 材料区分、`Holt Text` | 初期費用（GBP）、`Holt GBP` |
| D | 投入重量（kg）、`Holt Quantity` | 年間総便益（GBP）、`Holt GBP` |
| E | 良品重量（kg）、`Holt Quantity` | 年間追加費用（GBP）、`Holt GBP` |
| F | 良品重量 ÷ 投入重量、`Holt Share` | 年間純便益（D − E、GBP）、`Holt GBP` |
| G | 比較基準との差（百分率の差 × 100）、`Holt Points` | 初期費用 ÷ 年間純便益（年）、`Holt Years` |
| H | 比較条件・根拠、`Holt Text` | 前提・出典、`Holt Text` |

歩留まり表は F 列に `=IF(D7>0,E7/D7,"")` のように計算式を入れ、G 列には比較可能な対象だけにポイント差を計算する（例 `=(F8-F7)*100`）。合計を出せる集計単位では、明細の次行で A:C を結合して `Holt Total Label`、D/E の合計に `Holt Total Quantity`、F の良品重量合計 ÷ 投入重量合計に `Holt Total Share` を適用する。G のポイント差は加算せず、H に集計範囲を記す。

設備表は F 列に `=D7-E7` を使う。G は回収を計算できるまとまりだけに `=IF(AND(C7>0,F7>0),C7/F7,"")` を入れる。同じ設備案の加算可能な費用・便益をまとめる場合は、次行で A:B を結合して `Holt Total Label`、C:E の合計と F の差額に `Holt Total GBP`、G の C ÷ F に `Holt Total Years`、H の前提に `Holt Total Label` を適用する。年間純便益が非正なら H に回収年数が算定できない旨を書く。代替案の数値は案ごとに示す。

合計行は25 ptが目安。注記が必要なら、次行の A:H を結合して `Holt Note` を適用し、行高を27 ptから折返しに応じて調整する。印刷はA4横・横1ページ・縦自動。続頁の列見出しは6行目を繰り返し、印刷範囲を `A1:H<最終行>` に更新する。

### Excel — Whitfield の旧試算・訂正内訳・費用発生時期

`holt-whitfield-finance.xlsx` を開き、下表から用途に合うシートを選ぶ。別々のブックとして保存するときは、原稿にあるシートだけを残す。管理欄は `A2` に表題、`A4` に文書番号、`D4` に実日付、`F4` に作成者を入れる。結合セルの左上に入力し、管理値行は34 pt。フッタ左に番号・日付・作成者を ` | ` でつなぎ、Liberation Sans 9 pt・紫灰 `302435`。6行目の列見出しを残し、7行目から実データの行を作る。下表のスタイル名は `Holt ` を先頭に付けて、入力後に `cell.style` に指定する。

| 列 | Original estimate | Savings bridge | Cost timing |
|---|---|---|---|
| A | 行番号、`Index` | 行番号、`Index` | 行番号、`Index` |
| B | 旧試算の項目、`Text` | 訂正対象の項目、`Text` | 初期費用の項目、`Text` |
| C | 旧シート・行の参照、`Text` | 旧シート・行の参照、`Text` | 費用見積りの参照、`Text` |
| D | 旧年換算額（円）、`JPY` | 旧年換算額（円）、`JPY` | 初期費用（円）、`JPY` |
| E | 算定区分、`Text` | 控除する重複額（円、正数）、`JPY` | 時期を置く基準、`Text` |
| F | 重複照合キー、`Text` | 訂正後年換算額（D − E、円）、`JPY` | 基準から数えた着手月、`Index` |
| G | 残す・外す等の区分、`Text` | 重複照合キー、`Text` | 費用発生条件、`Text` |
| H | 根拠・仮定、`Text` | 根拠・判断理由、`Text` | 根拠・依存条件、`Text` |

`Holt JPY` は右寄せで1円単位、負数は括弧、ゼロは `0`。文字列は `Holt Text` の左インデント1を使う。行高は30 ptを目安に、折り返しに応じて広げる。訂正内訳の F7 は `=D7-E7` とし、明細の全行に適用する。10月15日の照合は旧試算の参照先と重複照合キーをたどれるようにし、年 JPY 26,000,000 から年 JPY 4,000,000 を控除して年 JPY 22,000,000 に至る。同じ作業を指す行に共通のキーを記す。

合計行は最後の明細の次に作り、行高25 pt、A:C を結合して `Holt Total Label`。年換算額または初期費用の列だけに `SUM` と `Holt Total JPY` を使う。旧試算と費用時期は D 列、訂正内訳は D〜F 列が対象。着手月は加算しない。初期費用 JPY 18,000,000 と実現まで9か月は別に示し、初年度の実現額は裏付けがある期間と条件に基づいて記述する。注記は次行の A:H を結合し、`Holt Note` と27 pt以上の行高を設定する。印刷はA4横・横1ページ・縦自動、続頁に6行目を繰り返す。各シートで `A1:H<最終行>` に印刷範囲を設定する。

### Excel — Pioneer の仕入先選定と例外理由の再分類

`holt-pioneer-buying.xlsx` を開き、原稿にあるシートを残す。各シートで表題は `A2`、文書番号は `A4`、日付型の値は `D4`、作成者は `F4` に入れる。各結合範囲の左上セルを使い、管理値行は34 pt。印刷フッタ左に番号・日付・作成者を ` | ` でつなぎ、Liberation Sans 9 pt・紫灰 `302435` とする。右には型のページ番号が入る。

6行目は列見出し。7行目から実データの行を追加し、表のスタイル名の先頭に `Holt ` を付けて `cell.style` に設定する。`Index` は右寄せ2桁、`Amount` は右寄せで桁区切り・小数2桁、負数は括弧。`Text` は左インデント1と折返しを持つ。明細の行高は32 ptを目安とし、折返しに応じて調整する。フィルタは `A6:H<明細最終行>` に設定する。

| 列 | `Supplier selection` | `Exception recoding` |
|---|---|---|
| A | 取引順位、`Index` | 行番号、`Index` |
| B | 仕入先、`Text` | 申請 ID、`Text` |
| C | 取引明細への参照、`Text` | 申請時の優先度、`Text` |
| D | 取引金額、`Amount` | 申請時の理由、`Text` |
| E | 通貨コード、`Text` | 需要が最初に判明した日時、`Text` |
| F | 対象・対象外の判断、`Text` | 申請日時、`Text` |
| G | 取引を集計した期間、`Text` | 照合後の理由、`Text` |
| H | 順位・採否の根拠、`Text` | 証拠の参照先・未確認事項、`Text` |

仕入先選定では42社の母集団、対象12社の線引き、取引の集計期間と比較条件が追えるようにする。同じ通貨・期間・集計基準の行を合計する場合、明細の次行で A:C と F:H をそれぞれ結合し、A に集計対象、F に金額の基準、E に通貨を入れて `Holt Total Label` を適用する。D に対象範囲の `SUM` と `Holt Total Amount` を使う。

例外理由の再分類では申請理由、需要が判明した時点、申請した時点、再分類の根拠を並べる。日時は原資料のタイムゾーンを添え、確認中ならその状態を記す。全93件と、その内数の緊急37件を区別する。緊急件数の集計が必要な場合は、明細の次行で A:G を結合して集計範囲を記し、`Holt Total Label` を使う。H に対象範囲の優先度を数える式（例 `=COUNTIF(C7:C99,"Urgent")`）と `Holt Total Count` を使う。分類理由が不明な行は照会先と確認事項を H に残す。

合計行は25 ptを目安とする。注記が必要なときは次行の A:H を結合して `Holt Note` を適用し、27 ptを目安に行高を調整する。印刷はA4横、横1ページ、縦自動。続頁には6行目の列見出しを繰り返す。印刷範囲は `A1:H<注記または表の最終行>` に更新する。

### Excel — Whitfield の試行前後の承認日数と保留理由

`holt-whitfield-pilot.xlsx` を開く。管理欄は各シートの `A2` に表題、`A4` に文書番号、`D4` に日付型の値、`F4` に作成者を入れる。いずれも結合セルの左上で、管理値行は34 pt。印刷フッタ左に番号・日付・作成者を ` | ` でつなぎ、Liberation Sans 9 pt・紫灰 `302435` を指定する。フッタ右のページ番号は型の設定を使う。

6行目は列見出し。7行目から実データ行を作り、下表のスタイル名の先頭に `Holt ` を付けて `cell.style` に指定する。同じ案件 ID と調査時点の区分で原記録、案件明細、保留照合を突き合わせる。A列の連番は `Index`（右寄せ2桁）。`Text` と `Timestamp` は左インデント1と折返しを持つ。日数と件数は右寄せ、日数は小数1桁。明細行は32 ptを目安に、長文が収まるよう広げる。

| 列 | Days comparison | Approval cases | Hold reconciliation |
|---|---|---|---|
| A | 比較区分、`Text` | 行番号、`Index` | 行番号、`Index` |
| B | 対象部門、`Text` | 案件 ID、`Text` | 案件 ID、`Text` |
| C | 観測期間、`Text` | 比較区分、`Text` | 比較区分、`Text` |
| D | 起点のイベント、`Text` | 部門、`Text` | 部門、`Text` |
| E | 終点のイベント、`Text` | 起点時刻、`Timestamp` | ログの保留理由、`Text` |
| F | 平均に算入した件数、`Count` | 終点時刻、`Timestamp` | 確認後の保留理由、`Text` |
| G | 平均日数、`Days` | 算入・除外・判定待ち、`Text` | 比較上の扱い、`Text` |
| H | 保留の取扱い・根拠、`Text` | 経過日数、`Days` | 根拠の参照先・次の対応、`Text` |

案件明細は JST の日時を Excel 日時値として入れる。日数の単位と丸めは原稿に従う。24時間を1日とする経過日数なら H7 に `=IF(AND(G7="Included",ISNUMBER(E7),ISNUMBER(F7),F7>=E7),F7-E7,"")` を設定する。両時刻と採否を確認した行だけが数値になる。保留案件はログの理由、確認後の理由、採否、その根拠を残し、除外・判定待ちは分母からも外す。両時点で比較条件が異なる場合は違いを明記する。

比較表は区分・部門ごとに、案件明細の H が数値である件数と、その H の合計を件数で割った平均を出す。例えば案件明細が7〜12行なら F7 に `=SUMPRODUCT(--('Approval cases'!$C$7:$C$12=A7),--('Approval cases'!$D$7:$D$12=B7),--ISNUMBER('Approval cases'!$H$7:$H$12))`、G7 に `=IF(F7=0,"",SUMIFS('Approval cases'!$H$7:$H$12,'Approval cases'!$C$7:$C$12,A7,'Approval cases'!$D$7:$D$12,B7)/F7)` を使う。各式の末尾行は案件明細の実データ範囲に合わせる。

試行前と試行中の2部門全体を比較するときは、明細の次に区分ごとの集計行を作る。A:E を結合して対象区分と部門の範囲を書き `Holt Total Label`、F に該当明細の件数合計と `Holt Total Count`、G に両部門の該当区分の日数合計 ÷ F と `Holt Mean Days` を使う。H に集計の根拠を `Holt Total Label` で入れ、折返しに応じて25 pt以上にする。案件明細と保留照合の件数を添える場合は、A:G を結合し集計対象を明記して `Holt Total Label`、H に件数式と `Holt Total Count` を適用する。

注記は次行の A:H を結合して `Holt Note` を適用し、27 pt以上で調整する。フィルタを使うときは `A6:H<明細最終行>`。印刷は A4 横、横1ページ・縦自動、続頁に6行目を繰り返す。印刷範囲は各シートで `A1:H<表または注記の最終行>` に更新する。

### PowerPoint

型にはスライドマスター1つと以下のレイアウトがある。PowerPoint の「新しいスライド」から、または `slide_layouts.get_by_name()` でレイアウトを選ぶ。

| レイアウト | 入力箇所 |
|---|---|
| `Cover` | 題（0）、説明先・日付・作成者・文書番号（1） |
| `Headline and Body` | 題（0）、本文（1）。改稿で修正箇所に茶赤を使える |
| `Two Columns` | 題（0）、左（1）、右（2）。罫線を挟んで時間の種類や証拠を比較する |
| `Evidence Table` | 題（0）、表（1）。最上行は濃い紫灰に白文字、明細は薄い茶灰と白 |
| `Process Figure` | 題（0）、横長の図（1、画像プレースホルダー）、図の下の説明（2） |

各スライドの下端右に文書番号、日付、作成者、スライド番号の順で9 ptの管理情報を置く。テキストボックスは左から4.12インチ、上から7.10インチ、幅8.62インチ、内部余白0、右寄せ。図形内での折り返しを有効にし、自動調整をオフにする（python-pptx では `text_frame.word_wrap = True`、`text_frame.auto_size = MSO_AUTO_SIZE.NONE`）。

Whitfield の実行案では、審査前の表現と Elizabeth の指摘後の版を版で区別する。審査用の版は年換算 JPY 22,000,000、初期費用 JPY 18,000,000、効果実現まで9か月を別の項目として示す。Word の Review memorandum には、審査での指摘、初年度の全額計上を削った箇所と理由、修正先のスライドを記録する。

Pioneer の全社再交渉の旧案と Meera 向け比較資料は、それぞれの作成日と判断時点が分かるようにする。比較資料では、`Two Columns` で再交渉と需要共有の着手順を比較し、`Evidence Table` で仕入先の選定範囲と例外申請の証拠を整理できる。需要共有の図は `Process Figure` の配色に揃え、需要発生の記録、Buying への共有、次の判断の担当を示す。CSV は受領時の列と単位を保ち、判別に迷った申請の照会は EML で保存する。

#### Evidence Table の成形

PowerPoint では表のプレースホルダーから必要な行数の表を挿入し、「テーブル デザイン」で `Holt Evidence Table` を選ぶ。「見出し行」と「縞模様（行）」を有効にする。配色は見出しが紫灰 `302435` に白太字、最初の明細から薄い茶灰 `F4ECE8` と白 `FFFFFF` の交互に紫灰の文字。表スタイル一覧に表示されない場合は、同じ色を行ごとの塗りつぶしと文字に指定する。左右余白を各0.16インチとり、数値列は右寄せ、文字列は左寄せにする。

python-pptx の表挿入は組み込みのスタイルを指定するため、成形時に各セルの塗りと文字を設定する。以下は `slide` に `Evidence Table` を選び、`rows` に見出しとそのスライド分の実データを用意した後の手順。行高は折り返しに合わせ、表がプレースホルダーの高さ（4.55インチ）を超える場合は、続きのスライドを同じレイアウトで追加して見出し行を繰り返す。各スライドの最初の明細を薄い茶灰にする。

```python
from pptx.dml.color import RGBColor
from pptx.enum.text import MSO_ANCHOR, PP_ALIGN
from pptx.oxml.ns import qn
from pptx.util import Inches, Pt

placeholder = slide.placeholders[1]
table = slide.shapes.add_table(
    len(rows), len(rows[0]), placeholder.left, placeholder.top,
    placeholder.width, Inches(0.7 * len(rows)),
).table
placeholder._element.getparent().remove(placeholder._element)
table._tbl.tblPr.find(qn("a:tableStyleId")).text = "{ED0EEDBF-3F01-5D6B-A43A-BCDD1DF306E9}"
table.first_row = table.horz_banding = True
for row_index, values in enumerate(rows):
    header = row_index == 0
    background = "302435" if header else ("F4ECE8" if row_index % 2 else "FFFFFF")
    for column_index, value in enumerate(values):
        cell = table.cell(row_index, column_index)
        cell.margin_left = cell.margin_right = Inches(0.16)
        cell.vertical_anchor = MSO_ANCHOR.MIDDLE
        cell.fill.solid()
        cell.fill.fore_color.rgb = RGBColor.from_string(background)
        paragraph = cell.text_frame.paragraphs[0]
        paragraph.text = str(value)
        numeric = column_index in numeric_columns  # 0 始まりの数値列番号の集合
        paragraph.alignment = PP_ALIGN.RIGHT if numeric else PP_ALIGN.LEFT
        paragraph.font.name = "Liberation Sans"
        paragraph.font.size = Pt(17)
        paragraph.font.bold = header
        paragraph.font.color.rgb = RGBColor.from_string("FFFFFF" if header else "302435")
```

`numeric_columns` と列幅は表の内容に合わせて指定する。日本語の文字はテーマの東アジア用書体 Noto Sans CJK JP を使う。

#### Process Figure の成形

図の枠は左0.63、上2.01、幅約11.94、高さ約3.49インチで、縦横比は幅:高さ = 65:19（約3.42:1）。説明（2）は図の下にある。1300 × 380 px など同じ比率の画像は、枠1の画像アイコンから直接挿入できる（python-pptx では `slide.placeholders[1].insert_picture(str(image_path))`）。

縦横比が異なる場合は、PowerPoint の「図の形式 → トリミング → 枠に合わせる（Fit）」で図全体を枠内に収める。python-pptx では次の手順で縦横比を保ったまま中央に置く。枠内に収まる倍率を使うため、端の工程名や矢印も表示される。描画後に両端と、図と説明の間隔を確認する。

```python
from PIL import Image

frame = slide.placeholders[1]
with Image.open(image_path) as image:
    pixel_width, pixel_height = image.size
scale = min(frame.width / pixel_width, frame.height / pixel_height)
width, height = round(pixel_width * scale), round(pixel_height * scale)
left = frame.left + (frame.width - width) // 2
top = frame.top + (frame.height - height) // 2
slide.shapes.add_picture(str(image_path), left, top, width, height)
frame._element.getparent().remove(frame._element)
```

## Holt & Reeves 発行の顧客向け訂正説明 PDF

LuaLaTeX で `\documentclass{holt-client-note}` を指定し、プリアンブルで `\holtcontrol{文書番号}{日付}{作成者}{宛先}` を設定する。見出しは `\holttitle{表題}` と `\holtsection{節見出し}`。管理欄は番号、日付、作成者と、その下に宛先が入る。各ページのヘッダは Holt & Reeves の共通ロゴと社名、フッタは番号・日付・ページ番号。英字は Liberation Sans、日本語は Noto Sans CJK JP。共通の紫灰を本文と表見出し、茶赤を上罫と節見出し、淡い茶灰を管理欄と表の横罫に使う。

対照表を使う場合は `\begin{holttable}{項目列の見出し}{数値列の見出し}{根拠列の見出し}` と `\end{holttable}` の間に、実データの分だけ `\holtitem{項目}{数値}{根拠}` を追加する。第2列は右寄せ。左右の内部余白を各5 pt取り、続頁には見出し行を繰り返す。クラスを入力と同じ作業フォルダに置くか、そのディレクトリを `TEXINPUTS` に追加して `bin/tex <入力.tex> <出力.pdf>` で組む。

10月の文書では、Kenneth が差し替えるべき年換算額と訂正理由を明記する。送付理由と添付関係は EML に残し、照合手順の個人メモは Markdown で保存する。

## Whitfield 発行の支払手順 PDF

英字は Liberation Serif、日本語は Noto Serif CJK JP。深緑 `214738` をロゴ・社名・罫線・表見出し背景、墨緑 `27322E` を本文、薄緑 `E9EFEA` を発行欄に使う。全ページのヘッダに社名と `JAPAN FINANCE`、フッタに手順番号・日付・ページ番号を載せる。

手順番号は `WCL-JP-FIN-PAY-<西暦4桁>-<連番3桁>`。見本は `WCL-JP-FIN-PAY-2024-001`。LuaLaTeX の入力で `\documentclass{whitfield-payment-procedure}` を指定する。プリアンブルの `\wccontrol{手順番号}{発行日}{承認者}` で管理欄を埋め、本文では `\wctitle{表題}` と `\wcsection{節見出し}` を使う。工程表は `wcsteps` 環境の中に実データの分だけ `\wcstep{番号}{工程}{統制と引渡条件}` を追加する。番号を右寄せにし、次の文字列との間に両セルの余白を取る。続頁は列見出しを繰り返す。TeX ソースとクラスを同じ作業フォルダに置くか、型のディレクトリを `TEXINPUTS` に追加し、`bin/tex <入力.tex> <出力.pdf>` で組む。

## Whitfield 管理の支払試行範囲変更合意書

発行元は Whitfield の Japan Finance。手順の型と同じ社名・ロゴ・書体・色を使い、右上の部署表示には `CHANGE CONTROL` が加わる。変更番号は `WCL-JP-FIN-SCA-<西暦4桁>-<連番3桁>`。管理欄の基準文書には、変更対象の合意または手順の参照番号と版を記す。フッタは変更番号、発効日、ページ番号。

LuaLaTeX で `\documentclass{whitfield-scope-change}` を指定し、プリアンブルに `\wcscontrol{変更番号}{発効日}{作成者}{基準文書}`、本文先頭に `\wcstitle{表題}` を入れる。`\wcsection{節見出し}` を使い、既存の試行範囲、承認された変更と発効条件、移行する手順と担当を記す。必要なら `wcsteps` に実データを入れる。末尾の `\wcsapprovals{Whitfield の署名者・役職}{署名日}{Holt の署名者・役職}{署名日}` が2社の署名欄を作る。署名者、発効日、各署名日は承認メールと計画・原稿に合わせる。

この型と `whitfield-payment-procedure.cls` があるディレクトリを `TEXINPUTS` に追加して `bin/tex <入力.tex> <署名欄つきの原本.pdf>` で組む。署名済みの紙面を作り、`bin/scan <署名済みの原本.pdf> <配布するスキャン.pdf>` を実行して全ページを確認する。

## Marston 発行の作業標準 PDF

折り箱の輪郭と社名をヘッダ左、`PRODUCTION / WORK STANDARD` を右に置く。英字は Liberation Sans、日本語は Noto Sans CJK JP。青 `15374C` を本文と表見出し背景、琥珀色 `D89C36` を箱の輪郭と上罫、淡青灰 `EDF2F4` を版管理欄に使う。フッタに標準番号・版・発効日・ページ番号。

標準番号は `MCP-OPS-STD-<西暦4桁>-<連番3桁>`。見本は `MCP-OPS-STD-2024-001`。LuaLaTeX の入力で `\documentclass{marston-work-standard}` を指定する。プリアンブルの `\mccontrol{標準番号}{版}{発効日}{発行者}` で管理欄を埋め、本文では `\mctitle{表題}`、`\mcsection{節見出し}` を使う。手順は `mcsteps` 環境に実データ分の `\mcstep{番号}{作業}{管理・判定基準}{記録・対応}` を書く。番号は右寄せ。隣の文字列との間に両セルの余白を設け、続頁には列見出しを繰り返す。クラスを TeX ソースと同じ作業フォルダに置くか、型のディレクトリを `TEXINPUTS` に追加し、`bin/tex <入力.tex> <出力.pdf>` で組む。

## Pioneer 発行の例外申請要領 PDF

Letter縦、表紙なし。ヘッダ左に合流経路の図形と社名、右に `BUYING OPERATIONS`。墨色 `20262A` を本文と社名、青緑 `11677C` をロゴ・表題脇の縦帯・節見出し・管理欄のラベル、淡青緑 `E5F3F5` を表見出しの背景と罫線に使う。欧文は Liberation Sans、手順番号は Liberation Mono、日本語は Noto Sans CJK JP。

手順番号は `PTD-BUY-EXC-<西暦4桁>-<連番3桁>`。見本は `PTD-BUY-EXC-2025-001`。LuaLaTeX の入力で `\documentclass{pioneer-exception-guidance}` を指定し、プリアンブルで `\ptdcontrol{手順番号}{版}{発効日}{発行者}` を設定する。初頁の管理欄は手順番号・版・発効日・発行者の順。各ページのフッタには手順番号・版・発効日とページ番号が入る。

本文では `\ptdtitle{表題}` と `\ptdsection{節見出し}` を使う。手順表は `\begin{ptdroute}` から `\end{ptdroute}` までに実データ分の `\ptdrow{番号}{申請事項・証拠}{回付先と対応}` を書く。番号は右寄せで、次の文字列との間には両セル各6 ptの余白を取る。続頁には列見出しを繰り返す。クラスを入力と同じ作業フォルダに置くか、型のディレクトリを `TEXINPUTS` に追加し、`bin/tex <入力.tex> <出力.pdf>` で組む。発効日と発行者は顧客文書の計画と原稿に従う。
