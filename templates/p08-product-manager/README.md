# 石田由美子の文書の型

オルタ現場システムズ株式会社（104人）のプロダクト企画と、取引先から受領する文書の書式。Officeの型を開き、別名保存して使う。型には組織表示・節見出し・書式だけを持ち、案件本文・仮番号・記入例・明細・合計は入れていない。

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 公開後報告・判断記録 | オルタ、石田。申請再利用の要件、店舗権限の例外条件、判断理由、指標の定義と公開後報告 | `olta-report.docx` | A4縦、表紙なし。巡回経路ロゴ、青緑の題、4節、細い罫線表 |
| 打合せ記録 | オルタ、石田。小野との受付観察、社内の見積り確認と決定 | `olta-minutes.docx` | 共通便箋。日時と参加者・確認事実・決定・担当と期限の4節 |
| 管理者へのご案内 | オルタ、石田・中川。若松の管理者説明会への案内・準備連絡 | `olta-guidance.docx` | 共通便箋、3節。発信元がオルタのため自社ロゴを使う |
| 指標の定義・集計 | オルタ、三浦・石田。期間・分母・分子と採用率の照合 | `olta-metrics.xlsx` | A4横、青緑白抜き見出し。7列、分母・分子・比率・目標を分離 |
| 要望と工数の照合 | オルタ、石田・太田。要望の出所、見積人週、依存工程、公開時の扱いを要望番号単位で照合 | `olta-requests.xlsx` | A4横、青緑白抜き見出し。先頭に順位と要望番号、工数の右に依存工程 |
| 拠点別の利用確認 | オルタ、石田・中川。12拠点の利用と教育を区別して整理 | `olta-sites.xlsx` | A4横、共通書式。7列、教育状況と利用状況を別列にする |
| 投資回収の前提比較 | オルタ、中野・石田。有償社数、月次寄与、初期費用を条件別に置き、回収月数と未確認事項を比較 | `olta-payback.xlsx` | A4横、共通書式。7列、金額と回収月数を右揃え、右隣の未確認事項は字下げ |
| 説明図の読み取り・設問別集計 | オルタ、岡本・石田。匿名回答を設問ごとに集計し、人数と判断根拠を照合 | `olta-reading-results.xlsx` | A4横、青緑白抜き見出し。7列、判定人数と割合のあとに読み取り課題・判定基準 |
| 企画・公開範囲の説明 | オルタ、石田・原田。開発案の比較、営業への説明、公開後報告、投資案、承認担当者向けの演習、利用者への読み取り課題と社内の結果説明 | `olta-review.pptx` | 16:9、白地、上左の巡回経路ロゴと社名、青緑の題、下罫線。1マスター・5レイアウト、スライド0枚 |
| 利用状況の出力・経営会議資料・承認担当者向け手引き | オルタ、石田。利用記録、経営会議への提出資料、承認担当者の手引き、巡回チェック利用者への配布資料を直接PDFにする | `olta-extract.cls` | LuaLaTeX、A4縦、共通ロゴ、用途別の右ヘッダ、青緑見出しと表ヘッダ、繰返しヘッダ・自動頁 |
| 拠点確認書 | 若松設備管理、小野。顧客発の拠点確認と正式な添付回答 | `wakamatsu-confirmation.docx` | A4縦、表紙なし。屋根と幹の図形、灰緑・明朝、控えめな事務用紙 |
| 依頼受付の手順 | 若松設備管理、管理部の小野。依頼受付の流れを案内する直接出力のテキストPDF | `wakamatsu-intake.cls` | LuaLaTeX、A4縦。屋根と幹の図形、灰緑の細い上罫、明朝体、管理部と自動頁 |
| 店舗別の役割・閲覧範囲 | ひかり店舗管理、管理担当の竹内。18店舗の役割・閲覧範囲・承認条件をオルタへ送る | `hikari-store-roles.xlsx` | A4横。えんじ色のひさし形と社名、生成りの管理ラベル、6列のえんじ色見出し |

受付観察は `olta-minutes.docx`、再利用要件は `olta-report.docx` を使う。開発向け比較と営業向け説明は共通の `olta-review.pptx` から別々に作る。DOCXと同内容のPDFは完成したDOCXから `bin/office-pdf` で出し、小野から届く受付手順は若松の `wakamatsu-intake.cls` で組む。

2024年12月〜2025年1月の年間計画では、全体開発の旧稿と試行範囲の文書を `olta-report.docx`、福田向け投資案と試行担当者向けの説明を `olta-review.pptx` からそれぞれ作る。経営会議に直接渡すPDFには `olta-extract.cls` の経営会議用管理欄を使う。商談条件のCSVには確認済み・未確認の区別を持たせる。見込4社と有償移行の想定は別に扱い、1月24日の提案は試行に限定する。試行で確かめる項目と再判断の条件は原稿に従う。決定メール・説明資料には対象の暦年と会計年度の四半期を併記する。

2025年2〜6月の店舗権限分離では、竹内から届いた役割表に `hikari-store-roles.xlsx`、オルタ側で詰めた例外条件に `olta-report.docx`、担当者向け演習に `olta-review.pptx`、配布する手引きに `olta-extract.cls` の手引き用管理欄を使う。対面協議の合意票は、オルタで用意する用紙なら `olta-minutes.docx` の節と表を使って協議内容の欄を組み、承認した条件と手書きの確認事項を反映した原稿から `bin/scan` で画像だけのPDFにする。

2025年7〜8月の巡回チェックでは、小野との試行振り返りに `olta-minutes.docx`、観察項目・つまずきの整理に `olta-report.docx` を使う。7月の再利用率・想定と集計条件は `olta-metrics.xlsx` に記録できる。8月の読み取り課題と結果説明は `olta-review.pptx` でそれぞれ組み、回答の設問別集計は `olta-reading-results.xlsx`、利用者に渡すテキストPDFは `olta-extract.cls` を使う。小野の所感は若松からのEMLとして保存する。岡本の書き込みがある旧説明図と修正箇所の図は、それぞれ画像として保存する。

## 組織の識別と管理項目

- 自社ロゴは、下を回って右へ上がる青緑の巡回経路と、右上の黄土の矢印、その右の社名。青緑 `006D77`、黄土 `D69E36`、本文の墨 `283C40` の3色。Officeのテーマにも同じ配色を登録している。淡青緑 `EAF3F3` は主色の背景用濃淡。Word・Excelは図形部分を内部に埋め込み、社名は編集可能な文字。PowerPointのマスターとTeXは図形で持つ。外部画像依存はない。
- オルタのOfficeは日本語・欧文とも Noto Sans CJK JP。既定のNormal、見出し、表、テーマのmajor/minor、Excel既定フォントと管理入力欄にも設定済み。オルタのTeXの日本語は Noto Sans CJK JP、欧文は Liberation Sans。どちらも環境に存在する。
- 自社のWordは上部にロゴと正式社名、下部にプロダクト企画・社名・自動頁。PowerPointは同じ識別をマスターに持つ。Excelは上部にロゴと正式社名、印刷ヘッダにプロダクト企画、フッタに社名と現在頁／総頁。TeXは上部にロゴと社名・用途、下部に部門・社名・現在頁／総頁。
- 文書番号は `OFS-YYYYMM-用途-連番3桁`。用途は `RP`（報告・定義・判断）、`MT`（議事録）、`NT`（案内）、`KM`（指標集計・試算）、`ST`（拠点表）、`RQ`（要望と工数の照合）、`BR`（スライド）、`EX`（直接出力PDF）。実際の番号・版は計画と事実台帳で決める。同じDOCXとPDFは同じ番号・同じ版。全ての正式文書に日付・作成者・版を添える。期間や基準日と発行日・出力日時を混同しない。
- 若松のロゴは屋根形と垂直な幹、下の水平線。灰緑 `52644C` と墨 `283C40`、白地。Wordは日本語・欧文とも Noto Serif CJK JP。ヘッダに正式社名、フッタに管理部・社名・自動頁。若松発の正式な確認書は `WSK-YYYYMM-CF-連番3桁`、受付手順は `WSK-YYYYMM-PR-連番3桁`。受領側は発行元の番号を保持する。
- ひかりのロゴは店頭のひさしとカウンター、右に正式社名。えんじ `87394C`、生成り `F6ECE5`、墨 `302B2D`。日本語・欧文とも Noto Sans CJK JP を使い、ExcelのNormal・管理欄・表にも設定する。印刷ヘッダに店舗運営、フッタに社名・管理担当・現在頁／総頁。ひかりの役割表の管理番号は `HKM-YYYYMM-RL-連番3桁`。作成日・版・担当を併記し、受領側でも発行元の番号を保持する。
- オルタの識別には青緑の巡回経路を使う。織庭の藍と織り目、湊梁建設の橙と柱梁とは、図形と配色で判別できる。

## Wordで中身を入れる

`Document(型のパス)` で開く。最初の節より前へ `Title` の題と `OF Metadata` の文書番号・版・日付・作成者を挿入する。社外向けには原稿の宛先・発信者も記す。各節の直下へ本文を入れ、節名は用途に合わせて変更する。再利用要件なら報告型の「対象と集計条件」「確認した結果」を「対象と背景」「再利用要件」に変更できる。空の節や管理項目を完成文書に残さない。全4型とも独立表紙なし。

A4縦、左右23mm、上29mm、下23mm。Normalは10.5pt、行間1.16、段落後6pt。Titleは24pt、Heading 1は13pt、Heading 2は11pt、見出しは主色・太字。`OF Metadata`、`OF Note`、Captionは9pt。社外型でもスタイル名は同じで、実フォントと主色が若松のものになる。

必要な表だけ追加し `OF Table` を指定する。先頭行の段落は `OF Table Header`（9pt・太字・白）、本文は `OF Table Body`（9pt・墨）を指定する。表の先頭行の塗りは型のfirstRow条件が持つ。列幅の合計は本文幅164mm以内にし、固定レイアウトでは列と各セルに同じ幅を設定する。右寄せの数値の後に文字が来る列では、セルの左右余白を確保し、必要なら文字列側に1字分の左インデントを足す。長文のセルは折返し、行高は自動。1行がページより高くなる内容は本文に移す。明細と本文は原稿にある分だけ追加する。

見出し行を各ページに再掲し、同じ行の店舗番号と条件を同一ページに収める。Wordでは「表のプロパティ」→「行」で全行の「行の途中で改ページする」を外し、1行目に「各ページにタイトル行を表示する」を指定する。Word 4型の `OF Table` には行分割を防ぐ既定値（`w:trPr/w:cantSplit`）がある。`bin/render` の LibreOffice 24.2.7.2 では、この既定値だけだと行がページをまたぐ。Pythonで成形するときは次のとおり作成する全行に直接指定する。

```python
from docx.oxml import OxmlElement

for index, row in enumerate(table.rows):
    properties = row._tr.get_or_add_trPr()
    if properties.find(OxmlElement("w:cantSplit").tag) is None:
        properties.append(OxmlElement("w:cantSplit"))
    if index == 0 and properties.find(OxmlElement("w:tblHeader").tag) is None:
        properties.append(OxmlElement("w:tblHeader"))
```

## Excelで明細を作る

`load_workbook(型のパス)` で開く。1行目はロゴ・社名、2行目は題、3〜4行目は管理欄、5行目は未書式の余白、6行目は列見出し。7行目以降にはセル・空の書式行・数式・仮の合計を置いていない。

1. B3:D3の結合セルへ正式番号（例 `OFS-202407-RQ-001`）、F3:G3へ版、B4:D4へ基準日、F4:G4へ作成者を入れる。入力先にもフォント・色・折返し・縦中央を設定済み。3行目30pt、4行目25pt。基準日は実日付と `yyyy-mm-dd` を使う。番号の全文と隣接する版・日付・作成者を埋めた見本を印刷確認している。列幅を変えるときも、この結合と折返し・高さを維持して番号を再確認する。
2. 7行目から原稿の明細だけを追加する。文字は `OF Text`、整数は `OF Number`、比率は `OF Percent`（値は0.32のような実数）、要望表の人週は `OF Effort`、実日付は `OF Date`、注記は `OF Note` の名前付きスタイルを各実セルへ割り当てる。文字左・数値右、上揃え・折返し、10pt（注記9pt）。`OF Text`・`OF Date`・`OF Note` は左インデント1、数値スタイルは右インデント1を持つ。明細の整数・比率・人週には薄い右罫線も付く。要望表のA列の順位→B列の要望番号、E列の工数→F列の依存工程、指標表のF→G列、拠点表のC→D列でも右隣の文字に同じスタイルを割り当て、数値との間隔を保つ。整数は `#,##0;[Red](#,##0);0`、比率は `0.0%`、人週は `0.0`、日付は `yyyy-mm-dd`。明細の見本は36pt高。長文は必要な高さへ増やす。
3. 合計に意味がある場合だけ最終明細の次に実セルを作り、`OF Total`（太字・淡青緑・二重下罫線）を適用する。ラベルは左寄せにする。原稿で確認した対象・実範囲のSUM等を入れ、採用率は集計後の分子÷分母とする。率の単純平均、未確認のゼロ扱い、異なる期間や重複する対象の合算をしない。合計率のセルには `0.0%` を明示する。要望表の人週を合計するときは同じ案・重複のない範囲を明示し、E列の式セルに `0.0` を指定する。見積り未確定の要望は扱い欄にその旨を記す。拠点表は確認明細と必要な注記を入れる。
4. 注記は存在するときだけ実内容の末尾へ追加し、必要ならA:Gを結合する。長さに応じて行高を設定する。明細が少なくても空行で紙面を埋めない。
5. `auto_filter.ref` はA6から最終明細のG列まで。合計・注記を含めない。`print_area` はA1から実内容の最終行G列まで。A4横、幅1ページ・高さ自動、1〜6行目の印刷繰返し、C7固定、左右0.3inch・上下0.4inch。全ページをレンダリングし、読める文字サイズ・番号・列見出し・行の折返しを確認する。数式は `bin/recalc` で計算して確認する。

回収比較の列はAから順に試算条件、有償想定社数、月次寄与（円／社）、初期費用（円）、回収月数、未確認の条件、出典・扱い。A・F・Gは `OF Text`、Bは `OF Number`、C・Dは `OF Yen`（円単位・桁区切り）、Eは `OF Months`（小数1桁）。月次寄与と初期費用の範囲は原稿に従う。単純回収月数を使う場合、E7に `=IF(OR(COUNT(B7:D7)<>3,B7<=0,C7<=0),"",D7/(B7*C7))` を入れ、実際の明細行に合わせて参照を変える。未確認の入力セルは値を空にし、回収月数が空になる理由と確認先はF・Gへ記す。仮定の異なるケースは集計せず、必要な注記を末尾へ入れる。

### 説明図の読み取り・設問別集計

共通手順で管理欄を埋める。B4:D4の実施日は読み取りを行った日とし、B3:D3には計画で決めた `OFS-YYYYMM-KM-連番3桁`、F3:G3とF4:G4には版・作成者を入れる。行6の列順は設問、回答数（人）、基準を満たした人数、割合、読み取り課題、判定基準、迷った箇所・注記。行7以降のA・E〜Gは `OF Text`、B・Cは `OF Number`、Dは `OF Percent` を使う。D7に `=IF(B7>0,C7/B7,"")` を入れ、参照を明細行に合わせる。無回答の扱いと判定条件は原稿に従う。率の右隣の課題欄には `OF Text` の左インデントが入る。

設問間で回答者が重なるため、設問別の人数・割合は足し上げない。全体の理解人数を記載する場合は匿名回答を人単位で照合し、判定に使った設問と基準とともに注記する。A:Gを結合した実内容の注記は `OF Note` を使う。明細は36ptを目安に、判定基準などが折り返す行は高さを増やす。A6:Gの最終明細までにフィルタ、A1:Gの最終記載行までに印刷範囲を設定する。

### ひかり店舗管理の役割表

1. B3:C3に発行元の管理番号（例 `HKM-202502-RL-001`）、E3:F3に版、B4:C4に作成日（実日付、`yyyy-mm-dd`）、E4:F4に担当を入れる。いずれも結合範囲の左端へ書き込む。入力欄は折返し・縦中央、管理番号行は27pt、日付行は24pt。
2. 7行目から店番、店舗、対象の役割、閲覧範囲、承認・代理の条件、確認事項を原稿から入れる。店番の整数は右寄せ・右インデント1・右罫線付きの `HK Number`、B〜F列は左寄せ・左インデント1の `HK Text`。実セルにだけスタイルを割り当て、明細は上揃え・折返しにする。見本は38pt高。原稿が長いときは必要な行だけ広げる。店番に先頭ゼロや文字が含まれる場合は、文字列として `HK Text` で保存する。
3. 役割明細は通常、合計を置かない。店舗数を明示する必要があれば末尾に確認済みの対象店舗数を置く。同じ店舗に複数行ある場合は重複を除く。数値には `HK Total`（生成り・上二重罫・整数）、ラベルには同スタイルを指定して左寄せにする。注記があるときだけ、末尾のA:Fを結合して `HK Note` と必要な行高を設定する。見本の注記は26pt。
4. `auto_filter.ref` はA6から最終明細のF列まで、`print_area` はA1から実内容の最終行F列まで。A4横、幅1ページ・高さ自動、1〜6行目を各ページに再掲、C7固定。左右0.32inch、上0.44inch、下0.47inch。全ページを見て、番号・日付・担当、店番と店舗名の間隔、最終行の折返しを確認する。

## PowerPointで説明資料を作る

`Presentation(型のパス)` で開き、実レイアウトを選ぶ。index 0は表紙、1は見出しと本文、2は2段、3は表、4は図。型はスライド0枚で、例文はない。題のplaceholderはidx=0、本文・図表領域はidx=1、2段右はidx=2。表紙題34pt、その他題28pt、本文20pt、表・図18pt以上。表紙副題へ文書番号・版・日付・作成者・対象四半期を入れる。

表・図はplaceholderの座標（左0.85、上2.12、幅11.6、高さ4.4inch）に実要素を作り、空のplaceholderを削除する。表の見出しは青緑白字、本文は淡青緑・墨、数値は右寄せ。数値の右隣の文字セルには左マージンを付け、少なくとも文字1字分の間隔を取る。図の経路・矢印は青緑、黄土は着眼点に限る。注記12pt以上。成形時に右下（左12.15、上7.08、幅0.5、高さ0.22inch）へ実ページ番号を10pt・青緑で追加する。全てのレイアウトが共通マスターを継承する。開発向けは工数と依存工程を比較表で示し、営業向けは公開対象と案内時の条件を先に示す。発表者ノートは原稿に従う。

## TeXで直接PDFを作る

オルタの利用状況出力・経営会議資料と若松の受付手順に使う。`olta-extract.cls` を指定scratchのTeX原稿と同じディレクトリへコピーし、`\documentclass{olta-extract}` で読み込む。`bin/tex 原稿.tex 出力.pdf` で組む。クラス単体に本文・例文はない。

`\OFRecord[版]{題}{文書番号}{基準日}{出力日時}{作成者}` で実値を指定する。版を管理しない出力では角括弧を省ける。本文は10pt・墨、題は青緑、`\OFSection{見出し}` も青緑。余白23mm、上29mm、下24mm。`longtable` の見出し行は `\rowcolor{OFteal}` と各セルの `\OFHead{列名}` を使い、`\endfirsthead` と `\endhead` で繰返す。列幅の合計と列間余白を本文幅164mm以内に収める。見本では15＋45＋90mmと列間余白を使う。先頭を右寄せにする場合は `>{\raggedleft\arraybackslash}p{15mm}@{\hspace{5mm}}` で次の左寄せ列との間を空ける。型の色は定義するだけでなく、本文・管理題・見出し・ヘッダ・フッタ・ロゴへ適用済み。長い出力でも管理欄と表を重ねない。

経営会議へ直接提出する場合は、本文の最初に `\OFMeetingRecord{題}{文書番号}{版}{発行日}{作成者}{対象期間}` を使う。右ヘッダが「経営会議資料」になり、番号・版・発行日・作成者・計画の対象期間を表示する。見出し・表・次頁は利用状況出力と同じ仕様。

承認担当者へ配る手引きでは、本文の最初に `\OFGuideRecord{題}{文書番号}{版}{発行日}{作成者}{配布先}` を使う。右ヘッダに「承認担当者向け手引き」を表示する。代理者を決める手順と担当外の閲覧可否は別の節で示し、手順の見出しに `\OFSection` を使う。

巡回チェックの説明図を利用者に配る場合は `\OFGuideRecord[利用者向け配布資料]{題}{文書番号}{版}{発行日}{作成者}{配布先}` を使う。右ヘッダに「利用者向け配布資料」、題の下に文書番号・版・作成者・発行日・配布先が出る。文書番号は案内の `NT` を使う。読み取り方や操作条件を `\OFSection` で区切り、説明の本文は選択できるテキストとして組む。

`wakamatsu-intake.cls` も原稿と同じディレクトリへコピーし、`\documentclass{wakamatsu-intake}` で読み込む。`\WSKRecord{題}{文書番号}{版}{発行日}{作成者}` に送付元の値を入れ、手順の区切りを `\WSKStep{見出し}` で示す。日本語は Noto Serif CJK JP、欧文は Liberation Serif、本文11pt・墨、題と見出しは灰緑。必要に応じて上と同じ方法で表を置き、見出し行は `\rowcolor{WSKgreen}` と `\WSKHead{列名}` を使う。出力PDFはテキストを選択できることを確認する。

## 型を使わない記録

CSVは原稿の素の列と値だけを出し、ロゴ・管理行を混入させない。EMLは送受信ヘッダと本文・引用・実添付。公開範囲の決定メールには対象四半期を記す。石田の翌朝の書き直しメモはMarkdownやTXTで、会社の便箋にしない。画面スクリーンショットはアプリの表示として作り、Officeの紙面へ貼り替えない。製品名、期間、画面に表示する集計値は計画と台帳に従う。写真・手書き画像を制作する段階ではimagegen skillを読む。

## 組版見本と検証先

見本は `scratch/p08-product-manager/b005/work/templates-0002/examples/`、レンダリング画像は同じ作業フォルダの `render/`。見本の文書番号・明細・設問・試算値は組版確認用。成形時には計画・台帳・原稿の値を使う。

| 型 | examples以下の見本 | render以下の全ページ一覧 |
|---|---|---|
| `olta-guidance.docx` | `olta-guidance-short.docx`・`olta-guidance-long.docx` | `olta-guidance.jpg` |
| `olta-minutes.docx` | `olta-minutes-short.docx`・`olta-minutes-long.docx` | `olta-minutes.jpg` |
| `olta-report.docx` | `olta-report-short.docx`・`olta-report-long.docx` | `olta-report.jpg` |
| `wakamatsu-confirmation.docx` | `wakamatsu-confirmation-short.docx`・`wakamatsu-confirmation-long.docx` | `wakamatsu-confirmation.jpg` |
| `olta-metrics.xlsx` | `olta-metrics-3.xlsx`・`olta-metrics-38.xlsx` | `olta-metrics.jpg` |
| `olta-sites.xlsx` | `olta-sites-3.xlsx`・`olta-sites-38.xlsx` | `olta-sites.jpg` |
| `olta-requests.xlsx` | `olta-requests-3.xlsx`・`olta-requests-38.xlsx` | `olta-requests.jpg` |
| `olta-payback.xlsx` | `olta-payback-3.xlsx`・`olta-payback-38.xlsx` | `olta-payback.jpg` |
| `olta-reading-results.xlsx` | `olta-reading-results-3.xlsx`・`olta-reading-results-38.xlsx` | `olta-reading-results.jpg` |
| `hikari-store-roles.xlsx` | `hikari-store-roles-3.xlsx`・`hikari-store-roles-38.xlsx` | `hikari-store-roles.jpg` |
| `olta-review.pptx` | `olta-review.pptx`（全5レイアウト） | `olta-review.jpg` |
| `olta-extract.cls`（利用状況） | `olta-extract-short.tex`・`olta-extract-long.tex` と `olta-extract-short.pdf`・`olta-extract-long.pdf` | `olta-extract.jpg` |
| `olta-extract.cls`（経営会議） | `olta-meeting-short.tex`・`olta-meeting-long.tex` と `olta-meeting-short.pdf`・`olta-meeting-long.pdf` | `olta-meeting.jpg` |
| `olta-extract.cls`（承認担当者向け手引き） | `olta-guide-short.tex`・`olta-guide-long.tex` と `olta-guide-short.pdf`・`olta-guide-long.pdf` | `olta-guide.jpg` |
| `olta-extract.cls`（利用者向け配布資料） | `olta-handout-short.tex`・`olta-handout-long.tex` と `olta-handout-short.pdf`・`olta-handout-long.pdf` | `olta-handout.jpg` |
| `wakamatsu-intake.cls` | `wakamatsu-intake-short.tex`・`wakamatsu-intake-long.tex` と `wakamatsu-intake-short.pdf`・`wakamatsu-intake-long.pdf` | `wakamatsu-intake.jpg` |

各ページのPNGは `render/` にある見本と同じ拡張子なしの名前のフォルダに格納。作業フォルダ内の `commands.json` にレンダリングの実行結果、`structure.json` に型と見本の検査結果を記録した。
