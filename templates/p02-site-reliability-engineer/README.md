# BRT Booking Systems Ltd の文書の型

## BRT の型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Incident review | BRT Reliability Engineering。Daniel と Matthew が、事後報告で発生・検知・復旧の時刻、影響の根拠、対策を記録 | `brt-incident-review.docx` | A4 縦、独立表紙なし。社名と段形のマーク、青い題と節見出し、橙の罫線 |
| Reliability meeting record | BRT Reliability Engineering。Daniel が打合せで照合した根拠、決定と担当者を記録 | `brt-reliability-meeting.docx` | A4 縦、独立表紙なし。事後報告と共通の書式で、議題・照合・決定の三節 |
| Night duty handover | BRT Reliability Engineering。引継ぎ担当者が当番枠、未解決の通知、送付先と受領確認を記録。旧票 BRT-NH-20240408-001 は Andrew Cartwright が Daniel の当番について作成 | `brt-night-handover.docx` | A4 縦、表紙なし。共通の青と橙、当番枠・未解決事項・連絡の三節 |
| Change decision | BRT Operations。Anjali が通常変更の凍結・延期・限定再開の判断、根拠、次回確認の条件を記録。承認記録はこの型で編集して PDF にできる | `brt-change-decision.docx` | A4 縦、独立表紙なし。冒頭が判断欄。フッタは Operations • Internal |
| Monthly error budget | BRT Reliability Engineering。Daniel が UTC 窓ごとの対象件数・消費量を照合し、月初来の残率を計算 | `brt-error-budget.xlsx` | A4 横。青地白抜きの見出し、橙のブランド罫線、青い上罫線の集計行 |
| Peak capacity comparison | BRT Reliability Engineering。Daniel が混雑時間帯の同一基準需要に対し、旧上限と承認後の上限を比較 | `brt-capacity-comparison.xlsx` | A4 横。エラーバジェットと共通の段形マークと青・橙。需要・上限・符号付き差分を小数 2 桁で横に並べる |
| On-call duty exchanges | BRT Reliability Engineering。Daniel と Andrew が翌月の交代枠、交代前と確定後の担当者を照合 | `brt-duty-exchange.xlsx` | A4 横。青地白字の見出し、橙のブランド罫線。英国現地の開始・終了、時間帯、交代前後の担当を分ける |
| Ticket archive recovery guidance | BRT Customer Support。Claire が点検で確認した復旧点と、運行側がどの時点までの乗車券記録を確認できるかを説明する修正案。確定稿は同じ Word 文書から PDF に書き出せる | `brt-ticket-recovery-guidance.docx` | A4 縦、表紙なし。青い見出しと橙の罫線。各ページのフッタに Customer Support とページ番号 |
| Recovery point comparison | BRT Reliability Engineering。Daniel と Sanjay が基準時刻、復旧点、時点の差と目標を照合 | `brt-recovery-point-comparison.xlsx` | A4 横。青地白字の繰返し見出し、橙の罫線、二つの UTC 日時列と右寄せの分・目標列 |
| Reliability briefing | BRT Reliability Engineering。Daniel が運用上の判断と根拠を共有し、Emma と Passenger Identity の初回・再演習の結果を比較する。引継ぎ研修の説明にも使う | `brt-reliability-briefing.pptx` | 16:9 白地。全レイアウト共通で上部に社名と段形のマーク、上下に橙の細い罫線 |
| Customer service update | BRT Customer Support。Claire が Westbury などへ復旧と引き続き適用する制約を説明する直接組版 PDF | `brt-customer-update.cls` | A4 縦、独立表紙なし。青の題と小見出し。各ページ下部に Customer Support とページ番号 |
| Handover retraining record | BRT Reliability Engineering。Daniel が模擬引継ぎで確認した誤り、再受講の確認、当番入りの判断を記録する直接組版 PDF | `brt-handover-retraining.cls` | A4 縦、表紙なし。青の題と段階別の確認表、橙の罫線、各ページ下部に Reliability Engineering とページ番号 |
| Service recovery exercise report | BRT Reliability Engineering。Daniel が Emma と確認した Passenger Identity の復旧演習について、計測範囲・承認待ち・判断を記録する直接組版 PDF | `brt-recovery-exercise.cls` | A4 縦、表紙なし。青い題と区間・所要分・観察事項の表、橙の罫線。段形の社名マークと Reliability Engineering • Internal、ページ番号 |

## 社外の型

| 型の名前 | 組織・作成者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Service observation | Westbury Coach Cooperative。Stephen が発券・配車窓口の観察事項を BRT に正式な文書で返す場合 | `westbury-service-observation.docx` | A4 縦、独立表紙なし。中央に円と橋のマーク、緑の明朝体の社名・見出し、鈍い金色の罫線 |
| Demand forecast and capacity assessment | Palmer Capacity Services。Louise が予測値、共通基準、繁忙帯、見積もりの前提を報告する直接組版 PDF | `palmer-capacity-forecast.cls` | A4 縦、独立表紙なし。右上に重ねた菱形と社名。紫のセリフ体の表題、薄紫の管理欄、ページ下部に部門名と頁 |
| Storage recovery witness observations | RJS Data Storage Ltd、Patrick。BRT の乗車券保管復旧点検に立ち会い、時計の読み合わせ、確認した復旧点と目標の比較、持ち帰り事項を記す直接組版 PDF | `rjs-storage-witness.cls` | A4 縦、表紙なし。左上に積層マークと社名、右上に Storage Assurance、青緑の短い太罫と薄緑灰の管理欄。頁番号は下中央 |

Westbury と RJS の型は各社が発行する文書に用いる。Claire が Westbury に送る PDF には BRT の型を用いる。配布物の形式と必要性はファイル計画に従う。Desktop の調査図やメモ、監視集計 CSV、メールには、それぞれの自然な形式を使う。当番の聞取り MD、判断に迷った事例の TXT、交代確認 EML、連絡順の板書写真 JPG も、各形式のまま残す。

## 共通の書式

- BRT は、高さの異なる青い矩形三つと最後の橙の短線に、社名 `BRT Booking Systems Ltd` を添える。交通事業者向けの予約基盤を運用する 135 人の会社の社内書式として、白地に青 `#233E8B`、橙 `#DB703A`、本文の濃い青灰 `#242D3B` を使う。Office の和文・欧文は Noto Sans CJK JP。TeX は和文 Noto Sans CJK JP、欧文 Liberation Sans。PDF の英数字は ASCII のまま検索・コピーできる。織庭の藍と織り目や、オルタの青緑の巡回矢印とはマークと罫線で区別する。
- Palmer は塗りと輪郭の菱形を重ねたマークを右上に置く。紫 `#532C55`、薄紫 `#E9DFEB`、本文の墨紫 `#302A34`。本文・見出しは Liberation Serif と Noto Serif CJK JP。社名・管理欄・表の見出し・フッタは Liberation Sans と Noto Sans CJK JP。
- Westbury は円枠の中に橋のアーチと橋桁を描き、社名を中央に置く。緑 `#3B5239`、鈍い金色 `#987440`、本文の濃い灰緑 `#34382F`。和文・欧文は Noto Serif CJK JP。
- RJS は保管層を表す三段の青緑の図形マークを社名の左に置く。青緑 `#12665E`、管理欄と表見出しの薄緑灰 `#E5EFED`、本文の濃い灰緑 `#253331`。欧文は Liberation Sans、和文は Noto Sans CJK JP。
- BRT の番号は `BRT-種別-YYYYMMDD-連番3桁`。種別は `IR`（事後報告）、`MT`（打合せ）、`CD`（変更判断）、`EB`（エラーバジェット）、`CP`（容量比較）、`RB`（説明資料）、`CU`（顧客説明）、`NH`（夜間引継ぎ）、`OX`（当番交代）、`TR`（再受講記録）、`EX`（復旧演習報告）、`RP`（復旧点比較）。Westbury は `WCC-SO-YYYYMMDD-連番3桁`、Palmer の予測・再確認は `PCS-DF-YYYYMMDD-連番3桁`、RJS の立会い所見は `RJS-WS-YYYYMMDD-連番3桁`。いずれも日付部分は文書の発行日、連番は組織・種別・発行日ごとに `001` から取る。業務上の障害番号 `INC-260713` を本文で引用する場合は、文書の管理番号と別に記す。
- 管理欄は完全な番号、発行日（例 `14 Jul 2026`）、作成者のフルネームを入れる。障害の時刻は UTC で表記する。勤務表の日時は Europe/London の現地時刻で、BST または GMT を明記する。初報と確定した内容を両方記す場合は、対象範囲と確認時点を添える。
- Office のファイル内部の作成者と日時は、配布物の成形後に進行役が `bin/stamp` で原稿の作成者と計画の日時にそろえる。Excel は保存時に更新日時が書き換わるため、この処理は保存と再計算の後に行う。

## Word

1. 型を開き、表題の直後に `Record metadata` で二つの段落を足す。1 段目に `Reference  BRT-IR-20260714-001     |     Issued  14 Jul 2026`、2 段目に `Prepared by  Daniel Jennings` の形式で実値を入れる。各組織の番号規則を使う。
2. 既存の見出しの下に原稿を入れ、使わない節は削る。本文は Normal 10.5 pt、段落後 6 pt、行間 1.20。Title 25 pt、Heading 1 は 13 pt、Heading 2 は 11 pt。ヘッダと部門名・ページ番号のフッタは各ページに出る。
   夜間引継ぎは、当番枠とカバー、未解決の通知と最後の確認、連絡先と受領確認を順に入れる。旧版では当時の記載を残す。
3. 表を使うときだけ、実データの行数で `Record table` を挿入する。幅は本文領域の 164 mm 以内。事後報告の照合表は 12 / 31 / 114 mm を目安とする。先頭行を複数ページで繰り返す設定にし、セルを組織の主色で塗って各段落に `Record table heading`（白字・9 pt）を適用する。明細段落は `Record table body`（9 pt）。各行をページ内で分割しない。文字と数値が隣接する場合は数値を右寄せ、数値の直後の文字段落を左寄せ・左インデント 3 mm にする。番号列の次も同様。
4. 行を追加したあとに折返しと改ページを確認する。表の末尾に集計が必要なときは実際の集計行を作り、`Record table body` に太字と主色の上罫線を加える。限定再開の判断は `brt-change-decision.docx` の各節に、承認する変更の範囲、実測余裕とその根拠、再確認の条件を入れる。PDF の場合は `bin/office-pdf 入力.docx 出力.pdf` で書き出す。

5. 乗車券保管復旧の顧客説明は `brt-ticket-recovery-guidance.docx` を開き、管理欄に `CU` の番号と発行日、作成者を入れる。3 節に確認した復旧点、説明できる記録の範囲、照会の扱いを入れる。確定稿から配布用 PDF を出力する場合は `bin/office-pdf` を使う。検証で参照した時刻と点の差は復旧点の古さとして扱い、復旧に要した作業時間と区別する。

## Excel

### Monthly error budget

1. `Budget` シートを開く。B3:C3 に完全な番号、E3:F3 に作成者、B4:C4 に Excel の日付、E4:F4 にサービス名と集計範囲を入れる。openpyxl では結合セルの左上（B3、E3、B4、E4）に値を入れる。管理欄は折返し付きで、行高は 26 pt。範囲の文が長ければ 4 行目の行高を増やす。
2. 6 行目が列見出し。7 行目以降に原稿の明細だけを追加する。A、C、D は `BRT Count`（桁区切り整数、右寄せ）、B、F は `BRT Text`（折返し、左寄せ）、E は `BRT Rate`（小数 1 桁の百分率、右寄せ）。数値と文字の間隔は、右寄せと左寄せの双方に設定したインデント 1 で確保する。行高は 25 pt を目安に、長い根拠は行高を増やす。
3. 原稿の対象・分母・集計の締切時刻を守って式を入れる。対象件数ベースで 99.9% 目標の見本では E7 を `=IF(SUM(C$7:C7)=0,"",1-SUM(D$7:D7)/(SUM(C$7:C7)*0.001))` とする。行ごとの率を平均せず、重複しない UTC 窓の対象件数と消費量を積み上げる。対象が 0 の場合は空欄、超過は負の残率として表示する。
4. 最後の明細の直後に集計を追加する。A:B を結合して A に集計名、C と D に各列の SUM、E に最終明細行の残率（対象が 0 の場合は空欄）、F に必要な判断や確認先を入れる。文字、整数、率にはそれぞれ `BRT Total Text`、`BRT Total Count`、`BRT Total Rate` を使う。見本と同じ全幅の上罫線にする場合は、結合後に B の上罫線も A とそろえる。注記があれば次の行で A:F を結合し、左上に `BRT Note` を適用。折返しに合わせて行高を決める。
5. 印刷範囲は `A1:F<最後に使った行>`。A4 横、横 1 ページ・縦自動、1〜6 行目を各ページに繰り返す。集計が次ページに単独で出る場合は行高か改ページ位置を調整する。数式の結果を保存する工程では `bin/recalc` を使い、`bin/render` で全ページを確認する。

### Peak capacity comparison

1. `Capacity` シートを開き、管理欄は Budget と同じセルに入力する。E4 にはサービス名と比較に使う基準を入れる。横 1 ページ・縦自動の A4 横、1〜6 行の繰返しと、最終使用行までの印刷範囲も Budget と同じ。
2. 7 行目から、A に行番号 (`BRT Count`)、B に比較案または時間帯 (`BRT Text`)、C に予測需要、D に設備の上限、E に `=D7-C7` の形式で差分を入れる。C:E は `BRT Multiple`。基準に対する倍率を小数 2 桁で右寄せし、負の差分を不足として表示する。F は根拠または判断 (`BRT Text`)。数値側と文字側のインデント 1 を保つ。明細の行高は 25 pt を目安に折返しに合わせて調整する。
3. 同じ基準の差分を要約する場合は最後の明細の直後に集計行を作る。A:B を結合して A に `BRT Total Text` を指定し、B:D に A と同じ上罫線を設定する。E は `BRT Total Multiple` で `=MIN(E7:E<最終明細行>)`、F は `BRT Total Text` で集計の意味を書く。倍率の単純合計は行わない。次行の A:F を結合し、左上に `BRT Note` で差分の定義や基準の補足を入れる。集計と注記の行高は 25 pt を目安にする。集計がページから離れたら、行高または改ページ位置を調整する。
4. 翌年の実測余裕は観測条件を添えて割合で記録する。比較表の差分（基準に対する倍率）とは単位が異なるため、この差分列には入れない。

### On-call duty exchanges

1. `Duty exchanges` シートの B3 に完全な番号、E3 に作成者、B4 に発行日、E4 に対象月と `Europe/London` を入れる。B3:C3、E3:F3、B4:C4、E4:F4 は結合済み。見本では B3 に `BRT-OX-20250509-001`、B4 に `09 May 2025`、E3 に `Daniel Jennings`、E4 に `June 2025 • Europe/London` を入れている。
2. 7 行目から実際の交代枠だけを追加する。A は `BRT Count`、B・C は英国現地での開始と終了の実日時を `BRT Rota Time` で表示し、日付をまたぐ場合も両方の日付を残す。D は `BRT Text` で `BST` または `GMT` を入れる。切替をまたぐ枠は D に開始と終了の時間帯を順に示す（例 `BST → GMT`）。E・F は `BRT Text` で旧担当と確定担当。日時・文字は左インデント 1、行番号は右インデント 1。行高は 25 pt を目安にし、担当名が折り返す場合は広げる。
3. 交代枠の集計が必要なら最後の明細の次に行を足し、A に `=COUNT(A7:A<最終明細行>)` を `BRT Total Count` で設定する。B:F を結合し、B に `BRT Total Text` で何を数えたかを書き、結合後の C:F に B と同じ上罫線を設定する。注記を追加するときは次行の A:F を結合して A に `BRT Note` を使い、折返しを見て行高を調整する。
4. A4 横、横 1 ページ・縦自動、1〜6 行を繰り返す。印刷範囲を `A1:F<最後に使った行>` に広げる。式を入れた場合は `bin/recalc` を使い、`bin/render` で全ページを確認する。

### Recovery point comparison

1. `Recovery points` シートを開く。B3 に完全な番号（例 `BRT-RP-20260511-001`）、E3 に作成者、B4 に Excel の日付、E4 に対象と点検日を入れる。B3:C3、E3:F3、B4:C4、E4:F4 は結合済み。管理欄は折返し付きで、行高は 26 pt。対象が長い場合は 4 行目を広げる。
2. 7 行目から実際の確認ごとに行を追加する。A は `BRT Count` の行番号。B・C は原稿で確認した基準時刻と復旧点を UTC にそろえ、日付付きの Excel 日時を `BRT UTC Time` で表示する。D は復旧点の古さ（分）、E は目標（分）で、どちらも `BRT Count`。分までの証拠を照合する場合、D7 は `=ROUND((B7-C7)*1440,0)`。復旧点が基準時刻以前であることを照合してから、F に `BRT Text` で判定を書く。D が E 以下なら目標内。明細は高さ 25 pt を目安にし、右寄せの数値と次の左寄せ列にインデント 1 を保つ。
3. 一覧で最大の古さを示す場合は最後の明細の次に A:C を結合して A に `BRT Total Text` の集計名、D に `BRT Total Count` で D 列の MAX、F に `BRT Total Text` で対象範囲を入れる。結合後の B・C と空の E の上罫線を A とそろえる。古さの単純合計は行わない。注記を付ける場合は次の行で A:F を結合して A に `BRT Note` を適用する。集計・注記は高さ 25 pt を目安に折返しに合わせて調整する。
4. A4 横、横 1 ページ・縦自動、1〜6 行を繰り返す。印刷範囲は最終使用行まで。集計が単独のページに出る場合は行高か改ページ位置を調整する。数式を使ったブックは `bin/recalc` で結果を保存し、`bin/render` で全ページを確認する。

## PowerPoint

`brt-reliability-briefing.pptx` を開き、共通マスターから目的のレイアウトでスライドを追加する。既定の題は青、本文は青灰。見本の図のように説明文は図と別に置ける。全 5 レイアウトの右下にある `Document reference`（idx 9）に、完全な番号（例 `BRT-RB-20260714-001`）を入力する。見本は表紙の管理欄と全スライドの右下に同じ番号を使う。python-pptx では `slide.placeholders[9].text = reference` で入力し、サイズ・書式は型から引き継ぐ。

番号欄の**左上**は、スライド左上から右へ 10.38、下へ 7.00 インチ。幅 2.30、高さ 0.32 インチで、ボックスの右端は x=12.68（スライド右端から約 0.65 インチ内側）。左右の内側余白は各 0.08 インチで、有効幅は 2.14 インチ。Noto Sans CJK JP 10 pt、青灰 `#242D3B`、右寄せ・上下中央、1 行固定で、番号末尾はスライド右端から約 0.73 インチ内側にそろう。

| レイアウト | 入力先 |
|---|---|
| Cover | idx 0 に題（37 pt）、idx 1 に管理欄（18 pt）。1 行目に完全な文書番号、2 行目に日付と作成者 |
| Heading and body | idx 0 に題（29 pt）、idx 1 に本文（22 pt） |
| Two columns | idx 0 に題、idx 1 と 2 に左・右の本文（20 pt） |
| Table | idx 0 に題、idx 1 に表。openpyxl で作った表を貼る場合も、実データ行だけを入れる。python-pptx では `placeholders[1].insert_table(行数, 列数)` |
| Figure | idx 0 に題、idx 1 に図（枠 11.90 × 3.825 インチ）、idx 2 に図注（14 pt）。挿入時に図の文字や凡例が裁ち落とされないよう元画像の縦横比を枠に合わせる |

表の先頭行は青地 `#233E8B` に白の太字、明細は白地に濃い青灰。見本の表は 17 pt、左右のセル余白は約 0.19 インチ。右寄せの数値の次に左寄せの文字を置くときも、この余白を保つ。載せきれない表は後続のスライドに分け、同じ列名を表示する。

## TeX

- 原稿用の `.tex` で `\documentclass{brt-customer-update}` を指定する。本文の先頭で `\BRTUpdate{題}{完全な文書番号}{発行日}{作成者}` を呼ぶ。小見出しは `\BRTSection{見出し}`、注記は `\BRTNote{注記}`。
- 時間帯や確認項目に表が必要なら `BRTEvents` 環境に実際の `番号 & 項目 & 説明 \\` を入れる。先頭列は右寄せで、文字列との間には 5 mm の間隔がある。青地白字の見出しは改ページ後も繰り返す。顧客向けの日付や件数はコピーして照合する。
- 表全体にかかる注記は、最終明細を `\\*` で終え、表の中の次行に `\BRTEventNote{注記}` を入れる。最終明細と注記を同じページに載せる。本文中の注記には `\BRTNote` を使う。
- リポジトリ直下で `TEXINPUTS="$PWD/templates/p02-site-reliability-engineer:"` を環境変数に設定し、`bin/tex 入力.tex 出力.pdf`、続けて `bin/render 出力.pdf 画像ディレクトリ` を実行する。末尾のコロンで TeX の標準検索先を維持する。

### Palmer の予測・容量確認

- `\documentclass{palmer-capacity-forecast}` を指定し、`\PalmerReport{題}{完全な文書番号}{発行日}{作成者}{提出先}` を本文の先頭で呼ぶ。表題は 26 pt の左寄せで、管理欄と同じ幅で折り返す。発行者と提出先を別に示す。見出しは `\PalmerSection{見出し}`、注記は `\PalmerNote{注記}`。
- 根拠の一覧は `PalmerChecks` 環境に `番号 & 前提・確認項目 & 扱いまたは制約 \\` を 1 行ずつ入れる。番号は右寄せ、次列との間は 6 mm。濃い紫の表見出しを改ページ後も繰り返す。予測倍率の基準または実測余裕の条件を、数値と一緒に記す。ビルドと確認には上記と同じ `TEXINPUTS`・`bin/tex`・`bin/render` を使う。
- 比較の基準値など表全体にかかる注記は、最終明細を `\\*` で終え、表の中の次行に `\PalmerCheckNote{注記}` を入れる。最終明細と注記を同じページに載せる。

### 引継ぎの再受講記録

- `\documentclass{brt-handover-retraining}` を指定し、`\BRTUpdate{題}{完全な文書番号}{発行日}{作成者}` で管理欄を入れる。見出しと注記は `\BRTSection` と `\BRTNote`。顧客説明の型と共通の BRT 書式を読み込み、社内部門名をフッタに表示する。
- `BRTEvents` には `段階 & 確認項目 & 証拠または結果 \\` を実際の確認項目ごとに入れる。初回演習で分かったこと、再受講で確認した結果、当番入りの条件と判断を時点ごとに記す。青地白字の列見出しは改ページ後も繰り返す。組版と確認は上記の `TEXINPUTS`・`bin/tex`・`bin/render` を使う。
- 表全体にかかる短い注記は、最終明細を `\\*` で終え、表の中の次行に `\BRTEventNote{注記}` を入れる。最終明細と注記が同じページに載る。本文にかかる注記は `\BRTNote` を使う。

### Passenger Identity の復旧演習

- 改善前の報告 PDF は `\documentclass{brt-recovery-exercise}` を指定する。冒頭の `\BRTUpdate{題}{完全な文書番号}{発行日}{作成者}`、節の `\BRTSection{見出し}`、本文注記の `\BRTNote{注記}` は BRT 共通。管理欄の発行日と、本文の実施日・UTC 時刻はそれぞれ原稿に従う。
- `BRTExerciseIntervals` に `区間 & 経過分 & 観察事項 \\` を入れる。分は右寄せで、観察事項との間に 6 mm の間隔がある。青地の列見出しを各ページに載せる。本文に計測の起点・終点、20 分の目標、判断を引き取る条件と権限点検の結果を記す。初回報告は発行時点の内容を保ち、同じ計測範囲での再演習との比較は `brt-reliability-briefing.pptx` にまとめる。どちらも演習として扱う。
- 目標時間と計測範囲など表全体にかかる注記は、最終明細を `\\*` で終え、表の中の次行に `\BRTExerciseNote{注記}` を入れる。最終明細と注記を同じページに載せる。

### RJS の立会い所見

- `\documentclass{rjs-storage-witness}` を指定し、`\RJSWitnessReport{題}{完全な文書番号}{発行日}{立会い担当者}{提出先}` で管理欄を入れる。見出しは `\RJSSection{見出し}`、本文注記は `\RJSNote{注記}`。
- `RJSObservations` には `番号 & 照合した根拠 & 立会い所見 \\` を入れる。番号は右寄せで、隣の左寄せ欄との間に 10 pt の余白がある。所見の列見出しは改ページ後も出る。表の注記は最終明細を `\\*` で閉じ、表の中の次の行に `\RJSObservationNote{注記}` を入れる。`TEXINPUTS` を上記と同じように指定し、`bin/tex` と `bin/render` を使う。

## 成形見本

今回の見本と全ページ画像は b005 の `work/templates-0004` にある。見本内の時点、集計値、追記事項は書式確認用であり、計画で使う事実は依頼・設定・台帳で照合する。

| 型 | 入力済み見本 | ページ画像 |
|---|---|---|
| 事後報告・短い表 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-incident-review.docx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-incident-review/) |
| 事後報告・長い表 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-incident-review-extended.docx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-incident-review-extended/) |
| 打合せ記録 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-reliability-meeting.docx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-reliability-meeting/) |
| 変更判断・限定再開 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-change-decision.docx)・[PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-change-decision.pdf) | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-change-decision/)・[PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-change-decision-pdf/) |
| Westbury の観察記録 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/westbury-service-observation.docx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/westbury-service-observation/) |
| エラーバジェット・短い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-error-budget.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-error-budget/) |
| エラーバジェット・長い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-error-budget-extended.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-error-budget-extended/) |
| 旧引継ぎ票 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-night-handover.docx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-night-handover/) |
| 当番交代・短い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-duty-exchange.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-duty-exchange/) |
| 当番交代・長い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-duty-exchange-extended.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-duty-exchange-extended/) |
| 説明資料・5 レイアウト | [PPTX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-reliability-briefing.pptx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-reliability-briefing/) |
| 顧客説明・短い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-customer-update/) |
| 顧客説明・長い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update-extended.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update-extended.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-customer-update-extended/) |
| 顧客説明・17 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update-page-boundary.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-customer-update-page-boundary.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-customer-update-page-boundary/) |
| 容量比較・短い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-capacity-comparison.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-capacity-comparison/) |
| 容量比較・長い表 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-capacity-comparison-extended.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-capacity-comparison-extended/) |
| Palmer 予測・3 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/palmer-capacity-forecast/) |
| Palmer 予測・30 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-extended.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-extended.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/palmer-capacity-forecast-extended/) |
| Palmer 予測・16 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-page-boundary.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-page-boundary.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/palmer-capacity-forecast-page-boundary/) |
| Palmer 予測・長い表題 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-long-title.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/palmer-capacity-forecast-long-title.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/palmer-capacity-forecast-long-title/) |
| 再受講記録・短い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-handover-retraining/) |
| 再受講記録・長い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining-extended.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining-extended.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-handover-retraining-extended/) |
| 再受講記録・19 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining-page-boundary.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-handover-retraining-page-boundary.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-handover-retraining-page-boundary/) |
| 復旧演習報告・短い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-recovery-exercise/) |
| 復旧演習報告・長い表 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise-extended.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise-extended.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-recovery-exercise-extended/) |
| 復旧演習報告・17 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise-page-boundary.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-exercise-page-boundary.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-recovery-exercise-page-boundary/) |
| 乗車券保管復旧の顧客説明 | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-ticket-recovery-guidance.docx)・[配布用 PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-ticket-recovery-guidance.pdf) | [DOCX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-ticket-recovery-guidance/)・[PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-ticket-recovery-guidance-pdf/) |
| 復旧点比較・2 確認 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-point-comparison.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-recovery-point-comparison/) |
| 復旧点比較・30 確認 | [XLSX](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/brt-recovery-point-comparison-extended.xlsx) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/brt-recovery-point-comparison-extended/) |
| RJS 立会い所見・4 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/rjs-storage-witness.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/rjs-storage-witness.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/rjs-storage-witness/) |
| RJS 立会い所見・30 明細 | [PDF](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/rjs-storage-witness-extended.pdf)・[TeX 原稿](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/samples/rjs-storage-witness-extended.tex) | [全ページ](../../scratch/p02-site-reliability-engineer/b005/work/templates-0004/render/rjs-storage-witness-extended/) |
