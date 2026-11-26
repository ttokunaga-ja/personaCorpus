# Leslie Crawford の文書の型

Allegheny Civic News Cooperative の調査報道デスク、Mapleton Falls Borough、Keller Municipal Paving Inc、Parkview Court Tenants Association の文書に使う。本文は英語（en-US）、時刻は America/New_York、用紙は US Letter。見本の参照番号・明細・金額などは書式の確認用。制作時の値は計画と事実の台帳から取る。

| 名前 | 組織・利用者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Story pitch | Allegheny Civic News Cooperative、Leslie から Richard へ。取材の問い、公開資料、未入手記録、企画の判断 | `acn-pitch.docx` | Letter 縦、表紙なし。えんじのマストヘッドと大きなセリフ体の題、問いを先頭に置く |
| Story outline | 同編集部、Leslie。検討した導入、構成、採否と理由 | `acn-outline.docx` | Letter 縦、表紙なし。マストヘッドの下に三つの編集用見出し |
| Story draft | 同編集部、Leslie の本文稿と Veronica の赤字返却、編集長への回覧稿 | `acn-road-draft.docx` | Letter 縦。マストヘッドと管理欄、版を持つ記事本文。変更履歴とコメントは Word の機能を使う |
| Records scope memo | 同編集部、Leslie・Keith。公開請求の対象と次の対応を共有。住宅記事では Leslie が写真説明と掲載保留を回覧する | `acn-scope-memo.docx` | Letter 縦、表紙なし。編集メモの見出しと任意の対照表 |
| Records scope workbook | 同編集部、Leslie。契約別の範囲と、年度・区画ごとの請求資料種別を対照 | `acn-records-scope.xlsx` | Letter 横。題と管理欄を上段に置き、淡い赤みの地にえんじの見出し。本文は白地と細い横罫 |
| Story source check workbook | 同編集部、Veronica が Sources と Holds で根拠の範囲と保留を管理。Leslie と Miguel は Tax basis で税区分を照合し、Leslie は同シートを設備の見積比較にも使う | `acn-source-check.xlsx` | Letter 横。えんじの見出し。Sources と Holds は資料と頁・時刻を追える6列。Tax basis は根拠・条件、右寄せの金額3列、判定を並べる8列 |
| Editorial brief | 同編集部、Leslie から Richard らへ。編集会議で問い、根拠、区画を説明 | `acn-editorial-brief.pptx` | 16:9。白地、えんじの細い上下罫、左上にマストヘッド。表紙・本文・2 段・表・図の実レイアウト |
| Public records correspondence | 同編集部、Leslie から Dorothy へ。最初の請求と、対象を絞った再請求の添付 PDF | `acn-records-letter.cls` | Letter 縦。編集部のマストヘッド、管理欄、宛先、本文と署名。必要な場合は明細を添える |
| Public Records Office reply | Mapleton Falls Borough、Dorothy から Leslie へ。窓口が独立した PDF として出す回答、通知、開示送付状 | `mapleton-falls-records.cls` | Letter 縦。丸い庁舎章と名称の中央組み、青い帯の件名とラベル付きの管理欄。サンセリフ体 |
| Library procurement records | Mapleton Falls Borough。図書室の見積集計、保守条件の記録、選定資料。開示資料中の区役所作成分に使う | `mapleton-falls-records.cls` | 庁舎章、青い件名帯、原記録の参照・日付・担当を置いた管理欄。金額明細は青の横罫 |
| Public Works contract and change schedule | Mapleton Falls Borough が契約当事者と作成・保管する契約書、変更指示、価格明細 | `mapleton-falls-records.cls` | 同じ庁舎章と青帯。契約参照番号と契約先の欄、金額を右揃えにした明細と繰り返し見出し |
| Public Works field records | Mapleton Falls Borough、Eleanor Briggs が管理する受領簿・入構票・交通整理準備票の紙控え | `mapleton-falls-records.cls` | Public Works の庁舎章と青帯。受領簿は受付明細の横罫、入構票と準備票は欄名と記入値の組 |
| Supplemental work schedule | Keller Municipal Paving Inc、Raymond Salazar が提供する自社の工事・数量明細 | `keller-work-schedule.cls` | Letter 縦。路面と破線のマークを左上に置き、黄土色の上罫と砂色の明細見出し。番号と数量は右寄せ |
| Repair record | Parkview Court Tenants Association、Theresa Coleman。住所を含まない修繕記録の紙控え | `parkview-repair-record.cls` | Letter 縦。家形と名称、深緑の題、薄緑の管理欄。控えの参照番号は等幅、明細は日付・記録種別・修繕内容を横罫で整理 |

保存した公示 HTML は Mapleton Falls Borough の掲載物として、庁舎章の円と青・濃灰のサンセリフ体を使う。公示の表題、掲載日、発注参照番号、掲示された区画を保存時点の文面で残す。メールは EML のヘッダと本文を使い、差出人・宛先・日付・件名と返信関係を保つ。請求 PDF は Leslie が書く一通の請求につき一つ。手書きの区画メモは原稿に沿って手帳の画像を作り `bin/scan` で組む。掲示写真は現場の掲示物と読める区画表記を画像にする。これらの形式はそれぞれ元資料の外観を使う。


## 道路の追加工事の取材（2025年6〜7月）

7月9日に Raymond へ送る12問と返信は EML で保存する。草稿は `acn-road-draft.docx` を使い、会社が述べた理由、契約記録・公道から確認できた事項、なお確かめる事項を原稿の構成に沿って示す。記事とは別に照合経過を回覧する場合は `acn-scope-memo.docx` の見出しを取材内容に合わせる。継ぎ目や補修箇所の写真は JPG、手書きの現場手帳は手帳面を `bin/scan` に通して画像だけの PDF にする。

受領した工事明細は発行元の書式を使う。区役所の変更記録は `mapleton-falls-records.cls`、Keller が自社資料として出す数量明細は `keller-work-schedule.cls` を使い、契約参照・変更番号で照合する。契約書や改訂明細の予定数量と実測値、会社の説明と独立に確認できた結果を分けて記載する。

## 図書室設備の取材（2025年10月〜2026年1月）

10月20日の企画書は `acn-pitch.docx`、見積9件の比較は下記の Equipment quote comparison を使う。Gerald の5月の説明は、USD 18,600 が保守費を含む差だったという既知の経緯として企画に反映する。1月12日受領の76ページでは、選定理由が確認できる箇所と資料が欠けている箇所を出典別に整理する。

区役所作成の見積集計・保守条件・選定資料は `mapleton-falls-records.cls` を使う。元の部署名を `\BoroughUnit` に指定し、`\BoroughHeading{題}{原記録の参照}{原記録の日付}{第3欄の値}{第4欄の値}{第3欄名}{第4欄名}` に発行元の管理欄を入れる。価格明細には下記の `BoroughPriceSchedule`、保守範囲・除外・期間や選定資料の節には `\BoroughSection` を使う。見積の引用・集計では見積元と原表の参照を各行に残す。添付された見積原本の外観は発行者の書式に従う。電子文書は組版 PDF、紙で交付された面は組版後に `bin/scan` に通す。

会議日を確かめる公表文は、区役所の保存 HTML として掲載日・会議日・会議体を残す。Dorothy との受領・不足議事録2回分の再請求は EML、Gerald への確認事項は TXT。再請求では公表文で確認した会議を指定し、受領資料の範囲と不足している記録を明示する。

## 住宅修繕の取材（2026年2〜3月）

写真説明と掲載判断の回覧には `acn-scope-memo.docx` を使う。原稿に合わせて三節の見出しを変更し、写真で見える修繕部位、説明に使える範囲、判断を保留する事項を分ける。写真説明に対照表を使う場合は採用候補だけを載せる。選外理由と撮り直し指示は TXT、照会は EML、根拠の日付は CSV。3月24日付の回覧では、22件のうち前回完了日が不明だった8件について、完了日を確認できた5件と裏付け不足で掲載保留の3件を分ける。

Theresa が提供する協会控えは `parkview-repair-record.cls` で組んでからスキャンする。住所・住戸・住民の識別情報を作らず、記録自体の参照と日付の意味を残す。

## 住宅修繕の初回取材（2024年7〜8月）

受付簿と住人用作業票の紙控えも `parkview-repair-record.cls` で組んでからスキャンする。協会配布案内の電子 PDF は同じ型の協会名・配色と管理欄を使い、右上の用途を “Resident notice”、左下を “Parkview Court Tenants Association” に替える。本文は案内の見出しと段落で組む。

## 書体・組織・管理番号

- 編集部は、階段状に置いた三つの四角と社名でマストヘッドを作る。欧文は **Liberation Serif**。墨 `#232826` は本文、えんじ `#792C3D` は組織名・見出し・罫、淡い赤みの紙色 `#F6EFEB` は表の帯と強調欄。通常紙面は白。ヘッダにマストヘッド、フッタに Investigations desk。縦文書とワークブックの印刷にはページ番号を付ける。
- 編集部の番号は `ACN-INV-YYYYMMDD-用途-連番3桁`。用途は企画 `PIT`、構成 `OUT`、範囲メモ `MEM`、照合表 `LOG`、会議資料 `BRF`、請求 `REQ`、記事本文稿 `DRF`。例は `ACN-INV-20240725-REQ-001`。日付は文書の日付、連番は同日・同用途内。独立した再請求は自身の文書番号を持ち、本文で初回請求を参照する。単なる書き出しは元文書の番号を引き継ぐ。
- 区役所は、丸い枠に庁舎の輪郭を描いた章と名称を並べる。欧文は **Liberation Sans**。青 `#234C69` は章、タイトル帯、ラベル、罫。濃灰 `#293942` は本文。フッタは発行部署（Public Records Office、Public Works、Library など）とページ番号。公開窓口の発行番号は `MFB-PRO-YYYYMMDD-連番3桁`（例 `MFB-PRO-20240726-001`）。図書室の発行番号を割り当てる場合は `MFB-LIB-YYYYMMDD-連番3桁`。受領資料に記載済みの番号は原表どおり転記する。照会相手の番号を引用する場合は本文に記す。契約書と変更明細の管理欄には、記録自体の契約参照番号（例 `PW-23-03`）と、原表にある変更番号を使う。開示の番号や日時は送付状で管理する。
- Keller は、車道をかたどった平行四辺形と二本の区画線を社名の左に置く。欧文は **Liberation Sans**。アスファルト色 `#252C30` をロゴと本文、黄土色 `#AE741C` を区画線と罫、砂色 `#F3EAD8` を表の見出し地に使う。下端に Project records とページ番号。Reference には契約参照・変更番号と、別の発行番号がある場合はその番号を併記する。Keller が新たに発行する工事明細の番号は `KMP-YYYYMMDD-WS-連番3桁`（例 `KMP-20250715-WS-001`）。受領済み資料の管理番号・日付は原記録から転記する。
- Parkview は、家形と協会名を用いる。欧文は **Liberation Sans**、参照番号のみ **Liberation Mono**。深緑 `#315642` は組織名・題・罫、墨緑 `#28332E` は本文、薄緑 `#E7EDE4` は管理欄と表の見出し。下端に Association file copy とページ番号。協会が発行する控えの参照番号は `PCTA-RPR-YYYYMMDD-連番3桁`（同日内の通番）。原記録の参照がある場合は全文を転記する。表示日付は英語表記の月・日・年。
- 作成者・日付は管理欄とファイルの内部プロパティを揃える。既定の文書言語は英語。地名・日付・文書番号は行の隣に実際の値を入れて読み合わせる。

## Word

型を開き、文書種別のラベルの直後に `Title` の段落を挿入する。Document no.、Date、Prepared by の各行のラベルの後に管理値を追記する。固定見出しごとの本文には `Normal`（12 pt、段落後 8 pt、行間 1.18）、大見出しは `Heading 1`（16 pt）、小見出しは `Heading 2`（13 pt）。使用する見出しを原稿の構造に合わせて残す。

対照表が必要な文書には `Desk Evidence` の表を作る。先頭行のスタイルを有効にする（Word の「見出し行」、OOXML では `w:tblLook` の `w:firstRow="1"`）。見出しセル全体に紙色 `#F6EFEB` が付き、表の上下と行間にえんじ `#792C3D` の細い横罫が付く。見出し段落を `Desk Table Head`、明細を `Desk Table Text` にし、文字列の左に 0.12 in のインデントを保つ。番号列は右寄せにして右側に 0.08 in の余白を取る。見本の表は No. / Public source / Follow-up の 3 列（0.58 / 2.86 / 3.40 in）。固定幅は列とセルの両方に指定し、見出し行の繰り返しも設定する（`w:tblHeader`）。各行をページ途中で分断しない。明細行は原稿の件数分を作り、合計は根拠があるときだけ末尾に付ける。

### 記事本文稿（Story draft）

`acn-road-draft.docx` の Story draft ラベルの次に `Title` を置き、既存の管理欄に Document no.、Date、Prepared by、Version の値を入れる。改訂・赤字・抜粋は元の本文稿の番号を引き継ぎ、台帳で指定した版を記す。記事本文に固定見出しは置かず、見出しと段落は原稿に従う。書き手の注や保留の札も原稿にある場合だけ載せ、Word のコメントと変更履歴には指摘した人と日時を入れる。

## Excel

すべてのシートで、A1 の三つの四角と B1 からの社名がマストヘッド。四角はえんじの文字を階段状に配置したセル内容で、印刷タイトルの 1〜6 行に含まれる。シートを転用するときはこの範囲を維持する。

1. Contract scope は契約参照番号・年度・区画・公示の出所・対象の判断を一行で読む。Record types は資料種別・年度・区画・請求状態・確認事項を対応させる。いずれも 1 行目が組織、2 行目が題、3〜4 行目が管理、6 行目が列見出し。
2. 使用する両シートの `C3` に文書番号、`F3` に作成者、`C4` に Excel の日付値を入れる。番号欄は C:D の結合セル、28 pt の行高と折返しを使う。Normal の設定は Liberation Serif 11 pt。列 A・C の入力用書式は `Desk Number`、B・D〜F は `Desk Text`。
3. 7 行目から実際の明細のみ追加する。各セルへ列ごとの名前付きスタイルを明示的に設定する。Excel と openpyxl のどちらでも列の既定への依存を避けられる。番号と年度は整数値・右寄せ、文字は左寄せ・折返し・1 文字分のインデント。印刷見本では 40 pt の明細行を用い、長い区画や備考は行高を調整する。
4. 件数が必要なときは直後の行で B:F を結合し、A に明細の A 列のみを数える `COUNT` 式、B に件数の意味を記す。A は `Desk Total Number`、B は `Desk Total`、行高 25 pt。脚注を付ける場合は次の行で A:F を結合し、`Desk Note` と折返しに収まる行高を指定する。集計対象が区画×資料種別なら件数は資料行の数として表示する。
5. フィルターを A6:F の最終明細まで、印刷範囲を A1:F の最終使用行まで設定する。Letter 横、横 1 ページ・縦自動、印刷時に 1〜6 行を各ページで繰り返す。数字と文字、管理値、改ページ後の列見出しを確認する。
6. 最終保存後に `bin/stamp` で作成者と文書日時を設定する。openpyxl は保存時に内部の更新日時を現在時刻で書き直すため、再保存した場合はスタンプもやり直す。

### 税区分の照合（Tax basis）

1. `acn-source-check.xlsx` を別名保存し、Tax basis を残して Sources と Holds を削除する。`C3` に LOG 番号、`F3` に作成者、`C4` に日付を入れる。F:H の結合セルに作成者を入れる。番号欄は C:D の結合と折返しで全文を表示する。
2. 7 行目から、原表の各金額につき 1 行を作る。A が照合行番号（`Desk Number`）、B が契約参照番号、C が資料名・開示頁・欄、D が原表で確認した区分（`Included`、`Excluded`、`Unconfirmed`）、H が確認結果。B〜D と H は `Desk Text`（折返しと左インデント 1）。E は原表の金額、F は E に含まれることを確認した税額、G は税別換算額。E〜G に `Desk USD`（USD、小数 2 桁、右寄せ）を指定する。明細行高は 46 pt を目安に本文に合わせる。
3. 税別額は D が `Excluded` なら E、D が `Included` かつ F が数値なら E−F。原表で確認できない場合は空欄にする。7 行目の式は `=IF(AND(ISNUMBER(E7),D7="Excluded"),E7,IF(AND(ISNUMBER(E7),D7="Included",ISNUMBER(F7)),E7-F7,""))`。換算額が両方確認できた組だけを比較する。原表の税込・税別の混在額を合算しない。異なる資料で同じ額を再掲している場合は重複計上しない。
4. 注記は A:H を結合し `Desk Note` を適用する。件数を表示する場合は A に `COUNT`、B:H を結合して件数ラベルを置き、`Desk Total Number`／`Desk Total` を指定する。明細末尾までをフィルター、注記・集計を含む最終行までを印刷範囲にする。Letter 横に幅 1 ページ、縦は自動、1〜6 行を繰り返す。文字と金額の間隔、数字・番号の全桁、改ページ後の見出しを確認する。数式は `bin/recalc` で確認し、最終保存後に `bin/stamp` を適用する。

### 図書室設備の見積比較（Equipment quote comparison）

1. `acn-source-check.xlsx` を別名保存し、Tax basis を残して Sources と Holds を削除する。残したシート名を `Quote comparison`、A2 を `Library equipment | Quote comparison` にする。C3、F3、C4 に LOG 番号、作成者、日付を入れる。6 行目の書式を保って A:H を `No.`、`Quote / issuer`、`Source / page`、`Service terms`、`Equipment (USD)`、`Service (USD)`、`Comparable (USD)`、`Match / follow-up` に替える。列幅は A:H の順に 6、19、25、23、17、16、17、29 を目安とする。
2. 7 行目から見積の実数分を入れる。A は `Desk Number`、B〜D と H は `Desk Text`、E〜G は `Desk USD`。D に保守の期間・適用範囲・除外条件、H に数量・仕様・税の前提を含む比較可否と確認先を記す。E と F は分けて確認した設備額と保守額。F に 0 を入れるのは原記録で無償が確認できた場合。込みで区分不能の保守額や記載自体がない額は空欄で、扱いを H に記す。D と H が読めるよう明細行を 46 pt を目安に調整する。
3. G は同一条件の比較に使える見積だけに入れる。同じ税・数量・保守条件の内訳が確認でき、E と F に重複がない場合は `=IF(COUNT(E7:F7)=2,SUM(E7:F7),"")`（行番号は明細に合わせる）。同じ条件の保守込み総額だけが原表で確認できる場合はその総額を G に記録し、H で根拠を示す。条件が揃わない行の G は空欄にする。
4. 金額は見積ごとに比較し、ブック末尾の集計には見積件数を使う。税区分表と同じ要領で A に件数式、B:H に件数ラベル、必要なら A:H の注記を置く。明細だけをフィルター対象にし、集計・注記までを印刷範囲にする。9 件の本表では A7:A15 が件数式の範囲。1〜6 行の繰り返し設定と横幅 1 ページを維持し、見出し・金額と次の文字の間隔・管理番号を全ページで確認する。最後に `bin/recalc`、`bin/stamp` を適用する。

### 記事の出典点検（Story source check）

1. `acn-source-check.xlsx` を別名保存し、Sources と Holds を残して Tax basis を削除する。管理欄・書体・色・ページ番号は範囲表と共通。両シートの `C3` に LOG 番号、`F3` に作成者、`C4` に Excel の日付値を入れる。1〜6 行を印刷時に繰り返し、Letter 横、幅 1 ページ、縦は自動とする。
2. Sources は A〜F に出典行番号、文書または証言の名、頁・時刻、確認した事実、支える範囲、限界や追跡先を置く。頁・時刻は表記を保つため文字列にする。Holds は A〜F に保留番号、草稿内の主張、Sources の出典行番号、不足する根拠、担当、状態を置く。Holds の A は整数を `"H"0` で表示し、C は照合先の整数とする。
3. 7 行目から実際の明細ごとに入れ、番号セルには `Desk Number`、本文セルには `Desk Text` を適用する。折り返した文が読める行高を取り、明細終了までをフィルター対象にする。各シートで末尾に B:F を結合して件数の意味を添え、A に A 列の明細だけを数える `COUNT` 式を置く。印刷範囲に集計と注記まで含め、組み直したファイルを `bin/recalc` で検算し `bin/stamp` で整える。

## PowerPoint

スライド 0 枚、共通のマスター 1 個。新規スライドは必要なレイアウトから追加する。すべて題 idx=0、表紙の管理欄・本文・左段・表・図は idx=1。右段と図のキャプションは idx=2。表紙の管理欄に文書番号を 1 行、日付と作成者を次の行に入れる。表は表プレースホルダーに挿入し、Liberation Serif 20 pt、見出しは紙色地にえんじの太字、本文は白地に墨。表のデータに数字と文字が隣接するときは、番号を右寄せ、右隣のセルの左余白を 0.15 in 以上にする。図はプレースホルダーに合わせてトリミングされるため、必要なラベルが全て枠内に見える比率で挿入する。

| レイアウト | 使い方 |
|---|---|
| Cover | 題と管理欄 |
| Heading and body | 問い、確認手順、報告方針など一続きの本文 |
| Two columns | 公開済みの根拠と請求して確認する事項などの対照 |
| Evidence table | 出典と照合事項の表 |
| Map or figure | 区画図や掲示の引用画像と説明 |

## TeX

本文の `.tex` を作業場所に置き、リポジトリ直下から `TEXINPUTS="$PWD/templates/p20-investigative-journalist:" bin/tex "$source" "$pdf"` で組む。`source` に本文のパス、`pdf` に出力先のパスを指定する。各クラスは LuaLaTeX を使う。組版時に PDF のメタデータは除去される。通常の制作では成形後に進行役が `bin/stamp` を実行する。単独で作る見本にも、管理欄と同じ作成者、計画の日付と時刻を指定して適用する。たとえば July 25, 2024 の請求書を現地時刻 12:00 の見本にする場合は、次のように組版後の PDF を更新する。

```sh
bin/stamp "$pdf" --author "Leslie Crawford" --date "2024-07-25T12:00:00" --timezone America/New_York
TZ=America/New_York pdfinfo "$pdf"
```

`pdfinfo` の Author、CreationDate、ModDate を確認する。区役所の回答では、同じ手順で差出人の氏名と回答自身の日時を指定する。

- `acn-records-letter` は `\DeskRequest{題}{番号}{日付}{差出人}{宛先}{書き出し}`、必要に応じ `\DeskHeading{見出し}` を使う。請求種別や区画の明細は `DeskSchedule` 環境内で `番号 & 資料または参照 & 対象範囲 \\` を実際の件数分だけ書く。番号列と文字列の間は 0.17 in。紙色 `#F6EFEB` の見出しとえんじ `#792C3D` の罫を使い、継続ページにも見出しが入る。署名が明細の直後に来るときは、最終明細を `\\*` で閉じ、環境の中で直後に `\DeskScheduleSign{氏名\\所属}` を置く。最終明細と署名を同じページに収める。本文に続けて署名する場合は `\DeskSign{氏名\\所属}` を使う。
- `mapleton-falls-records` は `\BoroughReply{題}{番号}{日付}{宛先}{差出人}`、本文と任意の `\BoroughSection{小見出し}`、担当者の署名。PDF として発行された行政文書に使い、スレッド内のメールは EML として保存する。

- 区役所が保管する契約・変更記録は `\BoroughContract{題}{契約参照・変更番号}{原記録の日付}{契約先}{発行部署}` を使う。条項・根拠・署名欄は原稿に沿って組む。価格明細は `BoroughPriceSchedule` 内に `番号 & 品目または金額の欄名 & USD額 & 税区分または原表の参照 \\` の順に書く。番号・金額の右に 0.17 in の間隔を取り、列見出しを各ページに繰り返す。未記載の金額は空欄のままにする。紙原本として交付された文書は、組版した PDF を `bin/scan` に通して画像だけの PDF にする。契約書・変更明細の番号と日付は原記録のものを使い、送付状の番号と受領日は開示の記録に残す。

### Keller の工事明細

本文で `\KellerSchedule{題}{契約参照・変更番号と発行番号}{原記録の日付}{道路名・区間}{発行者}` を呼ぶ。発行番号も記すときは参照欄の契約・変更番号に続けて `\KellerIssue{発行番号}` を置き、別行に番号全文を表示する。区間は長さに応じて折り返す。続けて、原稿にある条件と数量の根拠を置く。`KellerItems` 環境内で `番号 & 工事項目 & 数量 & 単位 & 原表の参照 \\ ` の順に実際の明細を組む。番号・数量の右側には 0.19 in の間隔があり、表の見出しは各ページに繰り返される。行数は実明細に合わせ、数量欄の値と単位・項目の対応を確認する。必要な注記や、同じ単位かつ原記録で裏付けられる集計は表の後に置く。

### Public Works の紙帳票

`mapleton-falls-records` の本文冒頭で `\renewcommand{\BoroughUnit}{Public Works}` を設定し、`\BoroughHeading{帳票名}{原記録の参照}{原記録の日付または期間}{第3欄の値}{管理者氏名}{第3欄名}{Custodian}` を使う。表の見出しは `\BoroughAmountHead{欄名}` とし、`\arrayrulecolor{BoroughBlue}` で横罫をそろえる。日付や参照番号は文字列として置き、折り返す欄は `p{幅}` で組む。

| 帳票 | 管理欄 | 明細の組み方 |
|---|---|---|
| Receiving register | Reference は該当面の契約番号、Date は綴りの年度、Record series は原簿の種別、Custodian は Eleanor Briggs | 原簿の各面を改ページで分け、同じ管理欄と明細の見出しを入れる。契約参照・受付日・受付の種類・受領した書類を横罫の表に置く。Alder FY22 は変更ファイルの窓口受領、Mill FY22 は技術審査束の回付受付として記録する。綴じ穴は左右の余白に収める。 |
| Depot Gate truck entry | Reference は `PW-23-01`、Date は原票の受付日、Location は Depot Gate、Custodian は Eleanor Briggs | 一面に運送者・車両区分・受付の記録を欄名と記入値の組で置く。受付には原稿にある印を添える。 |
| Traffic control staging sheet | Reference は `PW-23-03`、Date は原票の記入日、Location は Weighhouse Gate、Custodian は Eleanor Briggs | 一面に作業区分とバリケード配置を欄名と記入値の組で置き、原票の施工実績欄は空欄で組む。 |

原記録にある欄と値を組んでから `bin/scan "$pdf" "$scan"` に通す。開示番号、2025年3月3日の交付日時、交付単位の紙面数は Dorothy の送付状に載せる。スキャンのメタデータは `bin/stamp` で原記録の管理者と計画にある保存日時を設定する。`pdfinfo` で面数、`pdftotext` でテキスト層が空であること、`bin/render` で綴じ穴・原票の欄・受付印の視認性を確認する。


### Parkview の修繕控え

`\ParkviewRecord{帳票名}{記録参照の全文}{控えの日付}{作成者}` の後に、原記録の明細を `ParkviewEntries` 環境で組む。各行は `番号 & 記録された日付 & 記録種別 & 修繕の記載 \\` の順。通番と日付列の間隔は 0.18 in、見出しは各ページに繰り返す。受付・訪問・完了・台帳記入の日付は、原記録にある種別のまま示す。

`bin/tex` で組んだ PDF を `bin/scan "$pdf" "$scan"` に通し、協会控えの作成者と計画上の保存日時を `bin/stamp` で設定する。`pdftotext` で抽出できる文字がないこと、`bin/render` の画像で管理欄・記録種別・日付が読めることを確かめる。

## 記入見本の確認

各型を開いて管理欄に文書番号の全文・日付・作成者を入れ、本文と短い明細を組む。Word の対照表、Excel の各照合表、TeX の各明細表は複数ページの見本も作る。番号の全桁、数値と次の文字列の間隔、明細行の折返し、繰返し見出し、最終明細と署名の配置を `bin/render` の全ページで確認する。Editorial brief は5つのレイアウトを1枚ずつ確認する。見本と画像は制作時の作業フォルダに置く。受入・破棄時に消える作業フォルダの参照は、当該実行の報告に記録する。
