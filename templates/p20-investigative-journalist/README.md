# Leslie Crawford の文書の型

Allegheny Civic News Cooperative の調査報道デスクと Mapleton Falls Borough の公文書窓口の文書に使う。本文は英語（en-US）、時刻は America/New_York、用紙は US Letter。記入見本の道路名、対象年度、資料名の内訳、文書番号、追加の日付はレイアウト確認用。制作では計画と事実の台帳で確定した値を使う。

| 名前 | 組織・利用者と用途 | ファイル名 | 見た目 |
|---|---|---|---|
| Story pitch | Allegheny Civic News Cooperative、Leslie から Richard へ。取材の問い、公開資料、未入手記録、企画の判断 | `acn-pitch.docx` | Letter 縦、表紙なし。えんじのマストヘッドと大きなセリフ体の題、問いを先頭に置く |
| Story outline | 同編集部、Leslie。検討した導入、構成、採否と理由 | `acn-outline.docx` | Letter 縦、表紙なし。マストヘッドの下に三つの編集用見出し |
| Records scope memo | 同編集部、Leslie・Keith。公開請求の対象と次の対応を共有 | `acn-scope-memo.docx` | Letter 縦、表紙なし。対象範囲、必要な資料、対応を三節に分ける |
| Records scope workbook | 同編集部、Leslie。契約別の範囲と、年度・区画ごとの請求資料種別を対照 | `acn-records-scope.xlsx` | Letter 横。題と管理欄を上段に置き、淡い赤みの地にえんじの見出し。本文は白地と細い横罫 |
| Editorial brief | 同編集部、Leslie から Richard らへ。編集会議で問い、根拠、区画を説明 | `acn-editorial-brief.pptx` | 16:9。白地、えんじの細い上下罫、左上にマストヘッド。表紙・本文・2 段・表・図の実レイアウト |
| Public records correspondence | 同編集部、Leslie から Dorothy へ。最初の請求と、対象を絞った再請求の添付 PDF | `acn-records-letter.cls` | Letter 縦。編集部のマストヘッド、管理欄、宛先、本文と署名。必要な場合は明細を添える |
| Public Records Office reply | Mapleton Falls Borough、Dorothy から Leslie へ。窓口が独立した PDF として出す回答や通知 | `mapleton-falls-records.cls` | Letter 縦。丸い庁舎章と名称の中央組み、青い帯の件名とラベル付きの管理欄。サンセリフ体 |

保存した公示 HTML は Mapleton Falls Borough の掲載物として、庁舎章の円と青・濃灰のサンセリフ体を使う。公示の表題、掲載日、発注参照番号、掲示された区画を保存時点の文面で残す。メールは EML のヘッダと本文を使い、差出人・宛先・日付・件名と返信関係を保つ。請求 PDF は Leslie が書く一通の請求につき一つ。手書きの区画メモは原稿に沿って手帳の画像を作り `bin/scan` で組む。掲示写真は現場の掲示物と読める区画表記を画像にする。これらの形式はそれぞれ元資料の外観を使う。

## 書体・組織・管理番号

- 編集部は、階段状に置いた三つの四角と社名でマストヘッドを作る。欧文は **Liberation Serif**。墨 `#232826` は本文、えんじ `#792C3D` は組織名・見出し・罫、淡い赤みの紙色 `#F6EFEB` は表の帯と強調欄。通常紙面は白。ヘッダにマストヘッド、フッタに Investigations desk。縦文書とワークブックの印刷にはページ番号を付ける。
- 編集部の番号は `ACN-INV-YYYYMMDD-用途-連番3桁`。用途は企画 `PIT`、構成 `OUT`、範囲メモ `MEM`、照合表 `LOG`、会議資料 `BRF`、請求 `REQ`。例は `ACN-INV-20240725-REQ-001`。日付は文書の日付、連番は同日・同用途内。独立した再請求は自身の文書番号を持ち、本文で初回請求を参照する。単なる書き出しは元文書の番号を引き継ぐ。
- 区役所は、丸い枠に庁舎の輪郭を描いた章と名称を並べる。欧文は **Liberation Sans**。青 `#234C69` は章、タイトル帯、ラベル、罫。濃灰 `#293942` は本文。フッタは Public Records Office とページ番号。区役所自身の番号は `MFB-PRO-YYYYMMDD-連番3桁`（例 `MFB-PRO-20240726-001`）。照会相手の番号を引用する場合は本文に記す。
- 作成者・日付は管理欄とファイルの内部プロパティを揃える。既定の文書言語は英語。地名・日付・文書番号は行の隣に実際の値を入れて読み合わせる。

## Word

型を開き、文書種別のラベルの直後に `Title` の段落を挿入する。Document no.、Date、Prepared by の各行のラベルの後に管理値を追記する。固定見出しごとの本文には `Normal`（12 pt、段落後 8 pt、行間 1.18）、大見出しは `Heading 1`（16 pt）、小見出しは `Heading 2`（13 pt）。使用する見出しを原稿の構造に合わせて残す。

対照表が必要な文書には `Desk Evidence` の表を作る。先頭行のスタイルを有効にする（Word の「見出し行」、OOXML では `w:tblLook` の `w:firstRow="1"`）。見出しセル全体に紙色 `#F6EFEB` が付き、表の上下と行間にえんじ `#792C3D` の細い横罫が付く。見出し段落を `Desk Table Head`、明細を `Desk Table Text` にし、文字列の左に 0.12 in のインデントを保つ。番号列は右寄せにして右側に 0.08 in の余白を取る。見本の表は No. / Public source / Follow-up の 3 列（0.58 / 2.86 / 3.40 in）。固定幅は列とセルの両方に指定し、見出し行の繰り返しも設定する（`w:tblHeader`）。各行をページ途中で分断しない。明細行は原稿の件数分を作り、合計は根拠があるときだけ末尾に付ける。

## Excel

1. Contract scope は契約参照番号・年度・区画・公示の出所・対象の判断を一行で読む。Record types は資料種別・年度・区画・請求状態・確認事項を対応させる。いずれも 1 行目が組織、2 行目が題、3〜4 行目が管理、6 行目が列見出し。
2. 使用する両シートの `C3` に文書番号、`F3` に作成者、`C4` に Excel の日付値を入れる。番号欄は C:D の結合セル、28 pt の行高と折返しを使う。Normal の設定は Liberation Serif 11 pt。列 A・C の入力用書式は `Desk Number`、B・D〜F は `Desk Text`。
3. 7 行目から実際の明細のみ追加する。各セルへ列ごとの名前付きスタイルを明示的に設定する。Excel と openpyxl のどちらでも列の既定への依存を避けられる。番号と年度は整数値・右寄せ、文字は左寄せ・折返し・1 文字分のインデント。印刷見本では 40 pt の明細行を用い、長い区画や備考は行高を調整する。
4. 件数が必要なときは直後の行で B:F を結合し、A に明細の A 列のみを数える `COUNT` 式、B に件数の意味を記す。A は `Desk Total Number`、B は `Desk Total`、行高 25 pt。脚注を付ける場合は次の行で A:F を結合し、`Desk Note` と折返しに収まる行高を指定する。集計対象が区画×資料種別なら件数は資料行の数として表示する。
5. フィルターを A6:F の最終明細まで、印刷範囲を A1:F の最終使用行まで設定する。Letter 横、横 1 ページ・縦自動、印刷時に 1〜6 行を各ページで繰り返す。数字と文字、管理値、改ページ後の列見出しを確認する。
6. 最終保存後に `bin/stamp` で作成者と文書日時を設定する。openpyxl は保存時に内部の更新日時を現在時刻で書き直すため、再保存した場合はスタンプもやり直す。

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

本文の `.tex` を作業場所に置き、リポジトリ直下から `TEXINPUTS="$PWD/templates/p20-investigative-journalist:" bin/tex "$source" "$pdf"` で組む。`source` に本文のパス、`pdf` に出力先のパスを指定する。両クラスとも LuaLaTeX を使う。組版時に PDF のメタデータは除去される。通常の制作では成形後に進行役が `bin/stamp` を実行する。単独で作る見本にも、管理欄と同じ作成者、計画の日付と時刻を指定して適用する。たとえば July 25, 2024 の請求書を現地時刻 12:00 の見本にする場合は、次のように組版後の PDF を更新する。

```sh
bin/stamp "$pdf" --author "Leslie Crawford" --date "2024-07-25T12:00:00" --timezone America/New_York
TZ=America/New_York pdfinfo "$pdf"
```

`pdfinfo` の Author、CreationDate、ModDate を確認する。区役所の回答では、同じ手順で差出人の氏名と回答自身の日時を指定する。

- `acn-records-letter` は `\DeskRequest{題}{番号}{日付}{差出人}{宛先}{書き出し}`、必要に応じ `\DeskHeading{見出し}`、末尾に `\DeskSign{署名}`。請求種別や区画の明細を載せるときは `DeskSchedule` 環境内で `番号 & 資料または参照 & 対象範囲 \\` を実際の件数分だけ書く。番号列と文字列の間に 0.17 in の間隔がある。見出しは紙色 `#F6EFEB` の帯に置き、横罫はえんじ `#792C3D`。改ページ後の見出しにも同じ色が適用される。短い請求では一つの一覧、範囲の照合も添える場合は独立した見出しを付けて一覧を分けられる。署名の氏名と所属は同じ `\DeskSign` に入れ、行は `\\` で区切る。署名全体が一つのブロックとして収まるページに置かれる。
- `mapleton-falls-records` は `\BoroughReply{題}{番号}{日付}{宛先}{差出人}`、本文と任意の `\BoroughSection{小見出し}`、担当者の署名。PDF として発行された行政文書に使い、スレッド内のメールは EML として保存する。

## 記入見本

下のパスは作業フォルダ `scratch/p20-investigative-journalist/b001/work/templates-0005/` からの相対パス。

| 決めた書式を確認する箇所 | 記入したファイル | 全ページの画像 |
|---|---|---|
| 企画書、明細 1 行 | `pitch-short.docx` | `render/pitch-short/` |
| 企画書、明細 18 行、見出しを繰り返す | `pitch-long.docx` | `render/pitch-long/` |
| 採らなかった構成案 | `outline.docx` | `render/outline/` |
| 構成案、対照表付き | `outline-table.docx` | `render/outline-table/` |
| 請求範囲のメモ | `scope-memo.docx` | `render/scope-memo/` |
| 請求範囲のメモ、対照表付き | `scope-memo-table.docx` | `render/scope-memo-table/` |
| 照合表、契約 3 行と資料種別 6 行 | `scope-short.xlsx` | `render/scope-short/` |
| 照合表、各シートが 2 ページ | `scope-long.xlsx` | `render/scope-long/` |
| 表紙、本文、2 段、表、図 | `editorial-brief.pptx` | `render/editorial-brief/` |
| 対象記録 6 種を列挙した請求 | `request-short.tex`、`request-short.pdf` | `render/request-short/` |
| 区画明細 36 行、継続ページの見出しと署名 | `request-long.tex`、`request-long.pdf` | `render/request-long/` |
| 区役所からの範囲確認書 | `borough-reply.tex`、`borough-reply.pdf` | `render/borough-reply/` |
