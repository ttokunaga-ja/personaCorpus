# 田中美由の文書の型

あさひ業務システムズ株式会社（従業員86人）の開発部が、申請デスクR7の性能評価、指摘の再審査、公開判定、ADR、利用者説明に使う書式。5月の英語変更説明・読者別整理と、照会ポータルの公開延期にも使う。Officeは通常のDOCX・XLSX・PPTXを開き、別名保存して使う。型には組織表示・見出し・書式だけを持たせ、案件の本文、実績値、仮の文書番号、空の明細・合計欄は入れていない。

## 一覧

| 名前 | 組織・利用者と用途 | 実際のファイル名 | 見た目 |
|---|---|---|---|
| 技術判断・評価 | あさひ、田中。ADR-042の説明文書、性能評価の根拠、採用案と却下案 | `asahi-decision.docx` | A4縦、表紙なし。朝日ロゴ、青い題と見出し、題下罫線、罫線表。背景・条件・採否・決定の4見出し |
| 会議記録 | あさひ、田中・鈴木。公開判定会議、再審査、公開後の振り返り | `asahi-minutes.docx` | 共通書式。日時・資料、論点、決定、担当と期限の4見出し |
| 利用者へのご案内 | あさひ、田中から大島ルート配送などの利用者へ。日本語の改訂通知 | `asahi-notice.docx` | 共通書式。簡潔な3見出し。あさひ発の社外文書なので自社ロゴを使う |
| 英語変更説明 | あさひ、田中からLauraへ送る英語草稿・赤字控えと、利用者／管理者向け説明 | `asahi-explanation.docx` | 自社の朝日・青黄・ゴシックを継承。表紙なし、英語3見出し、赤字の文字スタイル `AS Review` |
| 委託先の英文レビュー報告 | Harlow Editorial Services、Laura Nichols。委託先発のテキストPDF | `harlow-review.cls` | A4縦、えんじと墨、セリフ体、校閲括弧のロゴ、英語ヘッダと通し頁。あさひとは別の型 |
| 試行導入の確認 | 大島ルート配送株式会社、中村美喜子。顧客発の業務確認・正式な添付回答が必要な場合 | `oshima-trial.docx` | A4縦、表紙なし。折れ曲がる配送経路と矢印のロゴ、緑と墨、明朝。あさひの朝日・青・ゴシックと区別 |
| 購買部門の試行報告 | 藤崎医療資材株式会社、購買主任の加藤博典。照会ポータルの試行で支障のあった業務を報告 | `fujisaki-trial.cls` | LuaLaTeX、A4縦、表紙なし。縦柱と3段の棚を組んだロゴ、青緑、明朝、淡い青緑の発行欄。購買部門名と通し頁 |
| 性能の測定 | あさひ、田中。測定条件と結果の別シート、比較の根拠 | `asahi-performance.xlsx` | A4横。朝日ロゴ、青白の列見出し。条件6列と結果8列。p50・p95・目標を分離 |
| 指摘・公開判定 | あさひ、鈴木・田中・佐藤。未解決指摘の消し込みと判定観点の照合 | `asahi-release.xlsx` | A4横、共通の書式。指摘と再審査、公開判定の2シート。根拠と未決事項の列を広く取る |
| 公開判定の説明 | あさひ、田中から佐藤・会議参加者へ。判定会議の資料 | `asahi-release.pptx` | 16:9白地、上部に朝日と社名、青の題、下罫線と開発部フッタ。1マスターと実レイアウト5種。型はスライド0枚 |
| 性能測定・評価記録 | あさひ、田中。Officeを経ずに直接PDFに組む測定報告が計画にある場合 | `asahi-measurement.cls` | LuaLaTeX、A4縦。朝日・青の題と罫線、繰り返し表見出し、会社・開発部と現在頁／総頁 |

ADR-042のDOCXとPDFは `asahi-decision.docx` で完成させた同じ文書から `bin/office-pdf` で作る。TeXでADRを作り直さない。TeXの型や社外の型があることを理由に成果物を増やさない。中村との通常のメールはEMLのままで、顧客のOffice書式へ移さない。b002の委託先レビュー報告は `harlow-review.cls` で直接PDFにする。Lauraがあさひの草稿に赤字を入れる版は、自社の `asahi-explanation.docx` を維持する。読者別説明のPDFは同型で完成させたDOCXを `bin/office-pdf` で変換し、別のTeXで書き直さない。

## 組織の識別

- あさひのロゴは黄色の半円の朝日、青い水平線と短い下線、社名の文字。小規模な業務サービス会社の社内標準として、独立表紙や大きな装飾を付けない。主色は青 `2548A6`、アクセントは黄 `D8B83C`、本文は墨 `29313F`。淡青 `EEF1FA` は青の濃淡。黄は図形や小さな強調に限定する。
- Word・Excelの図形ロゴは画像をファイル内部に埋込み、PowerPointのマスターとTeXでは図形で持つ。ロゴの外部ファイル依存はない。Officeの日本語・欧文は Noto Sans CJK JP。TeXの日本語も同じ、欧文は環境にある Liberation Sans。
- 大島ルート配送は緑 `355E4B` と墨 `29313F`。日本語・欧文とも Noto Serif CJK JP。ヘッダは配送経路ロゴと社名、フッタは会社名・試行導入窓口・自動ページ番号。所在地、電話、署名、押印を型で作らない。
- 自社のWordとTeXのヘッダは社名と文書種別、フッタは会社名・開発部・自動ページ番号。Excelはシート上段にロゴと会社名、印刷ヘッダに開発部、印刷フッタに社名と現在頁／総頁。PowerPointはマスターにロゴ・社名・部門を持ち、成形時に各スライドの右下へページ番号を入れる。
- 文書番号は自社 `ABS-YYYY-用途-連番`。用途は `EV`（評価）、`MT`（議事録）、`NT`（通知）、`PF`（測定表・直接PDF）、`RL`（指摘・判定表）、`BR`（会議資料）、`EX`（英語変更説明）。ADRは既定の `ADR-042` を文書番号として使い、別の番号を重ねない。版は番号とは別項目で記載する。大島の正式な確認書は `OR-YYYY-連番`。実値は計画・事実台帳で決める。同じDOCXとPDFは同番号・同版。メモ・写真・CSV・メールへ番号を強制しない。
- 他personaのREADME（p03・p05・p12・p14・p17・p18）を確認し、p14の `cedar-review.docx` とp05の `oriba-analysis.docx` の実物を開いて書式も比較した。既存の木材・織り目・封筒・灯・柱梁・寸法枠と区別し、朝日と青黄を選んだ。他personaの型は変更していない。

## Wordの成形

`Document(型のパス)`で開く。先頭見出しの前へ `Title` で実際の題を、`AS Metadata` で文書番号・版・日付・作成者・確認者・宛先を原稿に従って入れる。原稿の内容を既存の各見出しの下へ挿入し、不要な見出しは削る。空の節を完成文書に残さない。本文のNormalは10.5pt、Titleは23pt、Heading 1は13pt、Heading 2は11pt、段落後7pt、行間1.18。注記は `AS Note`、表題はCaption。独立した表紙は全5型とも設けない。

表は必要なときだけ追加し `AS Table` を指定する。先頭行のセル内段落を `AS Table Header`（白字・太字9pt）、明細のセル内段落を `AS Table Body`（9pt）にする。スタイルのfirstRowが自社は青、顧客は緑の背景を持つ。表先頭行の `w:trPr` に `w:tblHeader` を付け、複数ページで繰り返す。各明細行の `w:trPr` に `w:cantSplit` を付け、1行の文章がページをまたいで分断されないようにする。1行が本文1ページを超える場合は内容の区切りで行を分ける。表幅は本文166mm以内。長い文章には列幅を明示し、セル内折返しとページ境界を確認する。見出しのkeep-with-nextを有効にしている。

## Excelの成形

`load_workbook(型のパス)`で開く。各シートの1行目はロゴと社名、2行目は題、3行目は列見出しのみ。4行目以降に実際のデータを追加する。型の全シートは最大行3で、明細や合計の空行はない。2行目の題を原稿の実題へ変更し、文書番号・版・基準日・作成者・確認者などは原稿の実情報として、必要に応じてデータ末尾の注記へ追記する。

列名と幅は実際の原稿に合わせて変えてよい。比較に必要な条件名・対象版・単位・標本数・目標と実績は失わない。結果のp95は実測の百分位値であり、複数のp95を合計・平均して全体のp95にしない。重大度、解消状態、再審査の根拠と日付、判定者を分離する。日付は成形時に文字列ではなく実際の日付値にする。

| 名前付きスタイル | 適用する実セルと書式 |
|---|---|
| `AS Text` | 明細の文字。Noto Sans CJK JP 9pt、左寄せ・左インデント1、上寄せ、折返し、薄い下罫線 |
| `AS Number` | msなど小数値。小数1桁 `0.0`、右寄せ。精度が異なる原稿はnumber_formatを変更 |
| `AS Count` | 標本数・件数。整数 `0`、右寄せ |
| `AS Date` | 実日付値。表示 `yyyy-mm-dd`、左寄せ・左インデント1 |
| `AS Total` | 原稿に集計がある場合だけ作る実セル。整数、太字・淡青、青の上罫線と二重下罫線 |
| `AS Note` | 実際の注記。8pt、左寄せ・左インデント1、折返し。長い注記は必要範囲を結合し、行高を合わせる |

ブックの既定Normalとフォント0も Noto Sans CJK JP 9pt・墨に設定済みで、未入力セルの既定も同じ。両型の全シートで3行目の列見出しは左寄せを明示し、左インデント1・上下中央・折返しを設定している。見出し名を変えるときはセルの書式を維持する。

各明細セルへ対応する名前付きスタイルを付ける。数値列（`AS Count`・`AS Number`）は右寄せ・インデント0、右隣の文字列は `AS Text` の左インデント1を必ず維持する。先頭連番列の右隣も同じ。配置を上書きするときも `Alignment(horizontal="left", vertical="top", wrap_text=True, indent=1)` とし、列幅に文字と余白を含める。明細行高は内容に応じて調整する（見本43pt）。合計はデータの直後に原稿の式で作り、合計行の実範囲全体に `AS Total` を設定する。合計行のラベルセルだけは左寄せ・左インデント1へ変更し、式と数値は右寄せ・インデント0を維持する。件数や重大未解決数はCOUNTA/COUNTIFSなど、内容に対応する式を使う。不要な合計を作らない。注記はその下へ実際の本文がある場合だけ追加する。

印刷はA4横、横1ページ・縦は自動、余白左右0.3in・上下0.4in、1～3行の見出しを毎頁繰返し。固定はC4。データ追加後に `auto_filter.ref` を3行目～最後の明細、`print_area` をA1～注記を含む最終セルへ更新する。印刷範囲を空白行・列へ広げない。不要なシートは削除してよい。CSVは結果や条件をテキスト表として保存し、会社ロゴや見出し装飾をCSVへ混ぜない。

## PowerPointの成形

`Presentation(型のパス)`で開く。マスターは `Asahi Sunrise` の1種、レイアウトは順に `表紙`、`見出しと本文`、`2段`、`表`、`図`。型にスライドはない。`prs.slides.add_slide(prs.slide_layouts[index])` で追加し、Titleの実題と対応するプレースホルダへ原稿を入れる。表紙の題32pt、通常の題28pt、本文19pt、2段と副題18pt。題は青の太字・左寄せ。表紙にも固定の案件名やダミーの本文はない。

表と図のレイアウトにはtitle（idx 0）とobj（idx 1）がある。表・図はobjの位置・幅・高さ（左0.75in、上2in、幅11.8in、高さ4.5in）に `add_table` や図形で作り、使い終えた空のobjは削除する。表は青白の先頭行、淡青の明細、文字18ptを基本に実列数へ調整する。図は青の文字・淡青の箱、黄の矢印。本文の折返し、表セル、線の重なりは毎回レンダリングして確認する。右下のページ番号は左12.05in・上7.03in・幅0.6in・高さ0.3inに9ptで追加する。マスターの社名と部門を各スライドへ重複追加しない。

## TeXの成形

`asahi-measurement.cls` を見つけられるよう `TEXINPUTS` にこのtemplatesディレクトリの絶対パスと末尾の `:` を設定し、scratch内の `.tex` を `bin/tex` へ渡す。クラス自体には案件の本文・日付・測定値がない。

- `\documentclass{asahi-measurement}`、`\begin{document}`～`\end{document}` を使う。
- `\ReportTitle{実題}{文書番号・版}{日付}{作成者}` で実情報を入れる。
- `\section`、`\subsection`、`\ReportNote{実際の注記}` を使う。
- 表は本文幅166mm以内のlongtable。例の列幅は31・29・28・52mmで、列間の余白を含めて本文内へ収める。
- 先頭見出しを `\ReportHead{\HeadCell{項目}&…}` で作り、`\endfirsthead` の後に同じ見出しを置いて `\endhead` とする。明細は実際の内容だけを追加する。合計は原稿にある場合だけ太字・淡青で追加する。空の記入欄や仮の集計を追加しない。

## b002での使い分け

- 英語草稿と読者別説明は `asahi-explanation.docx`。見出しは Scope and intended readers / Changes and user actions / Open questions and review status。原稿にない節は削除する。本文・表・管理欄は前記Word手順を使う。自社番号は `ABS-YYYY-EX-連番`。Lauraの修正は原稿指定の人・日付・版と修正前後を保ち、`AS Review` の赤字 `A02B35` で示す。削除箇所は原稿指定のときだけ取り消し線にし、文書全体の本文色は変えない。履歴機能が必要なら原稿の履歴情報をOOXMLの変更履歴に入れ、単なる赤字を変更履歴機能として扱わない。
- 既存 `asahi-release.pptx` の全5レイアウトを英語の読者別説明にも使う。ファイル名は既存の型名であり、公開済みを意味しない。利用者10項目と管理者6項目の対象読者・用途を分け、5月時点の公開可否は未決と明示する。5月の資料に7月の測定・ADR・公開判定を先取りしない。
- `harlow-review.cls` は委託先自身の報告。えんじ `743B51`・墨 `343034`。校閲括弧と3本の本文線のロゴに会社名の文字を添える。欧文 Liberation Serif、サンセリフは Liberation Sans、日本語 Noto Serif CJK JP、和文サンセリフは Noto Sans CJK JP。本文は墨、題・節見出し・注記・ヘッダ罫線と社名はえんじ、表見出しはえんじに白字を実適用する。ヘッダは会社名と Editorial review、フッタは会社名・Language review・現在頁／総頁。表紙なし。
- TeXの呼出しは `\documentclass{harlow-review}`、`\ReviewTitle{実題}{HES-YYYY-ER-連番}{版}{日付}{Laura Nichols}`、`\section`、`\ReviewNote{実際の注記}`。実番号・版・確認日は後続の計画と台帳で確定する。longtableの見出しは `\ReviewHead{\ReviewCell{Audience}&…}` とし、`\endfirsthead` と `\endhead` の前に同じ見出しを入れる。本文幅162mm以内（見本のp列は27・35・55・21mm、列間余白込みで収まる）。明細と必要な合計だけを呼出し側で追加する。`TEXINPUTS` と `bin/tex` は前記と同じ。
- 既存の測定・公開判定の型と顧客の型は維持し、b002で再制作する根拠にはしない。7月の日本語通知はb001個別依頼の例外。b002の英語説明を日本語通知へ変更しない。
- 個人のMD・TXT、EML、画面PNG、保存した製品説明HTMLにはOfficeの紙面書式を適用しない。HTMLは製品の画面として、画面PNGは説明対象の状態として作り、会社の報告書に見せない。16項目の内容、往復の版と確認日の確定は次の計画段階で行い、型の見本を事実台帳へ転記しない。

## b003での使い分け

- 指摘一覧と再審査結果は `asahi-release.xlsx`。指摘と再審査シートで重要度・指摘内容・確認の根拠・担当・再審査日・状態を管理する。公開判定シートでは受入条件・確認結果・未決事項・判定を分ける。38件の分類、重大4件の条件、当初と延期後の予定、再判定条件の実値は計画と台帳で確定する。延期後の予定日と実公開日を区別する。
- 再審査議事録と田中コメント入りの控えは `asahi-minutes.docx`。コメントは該当箇所に付け、記録された発言・決定・期限と区別する。Wordのコメントを使う場合は原稿指定の著者と日時を設定する。
- 営業向け影響説明PDFは `asahi-decision.docx` を開き、原稿の題と管理欄、判断・試行先業務への影響・営業の案内範囲・次の連絡を記して `bin/office-pdf` で出力する。文書番号の用途は `EV`。営業の山本へ渡す情報と、加藤が購買部門内で確認した業務を区別する。
- 延期判断会議は `asahi-release.pptx`。表紙の副題に `ABS-YYYY-BR-連番`、日付、作成者を入れる。試行業務と営業説明は2段または別スライドに配置する。
- 加藤から受領する試行報告PDFは `fujisaki-trial.cls`。青緑 `17696D`・墨 `2E3535`・淡青緑 `E9F3F1`。題・見出し・社名・罫線・表見出しに青緑、本文に墨、管理欄に淡青緑を使う。日本語の本文は Noto Serif CJK JP、社名のゴシックは Noto Sans CJK JP。欧文は Liberation Serif、ゴシックは Liberation Sans。ヘッダに社名・購買部門・文書種別、フッタに社名・購買部門・現在頁／総頁。管理番号は藤崎発の正式な報告に `FMI-YYYY-TR-連番3桁` を使う。連番・日付・著者は原稿と台帳の指定どおりに入れる。
- `\documentclass{fujisaki-trial}` を開き、`\TrialTitle{実題}{文書番号}{発行日}{作成者}`、`\section`、必要なら `\TrialNote{実際の注記}` を使う。表を使う場合は `longtable` の `\endfirsthead` と `\endhead` の直前に `\TrialHead{\TrialCell{列名}&…}` を置き、毎頁の見出しを揃える。右寄せの順番列に続いて文字列が来る場合は、見本と同じ列指定 `>{\raggedleft\arraybackslash}p{11mm}@{\hspace{4mm}}p{52mm}p{84mm}` で4mmの間隔を確保する。明細と必要な注記だけ追加し、前節と同じ `TEXINPUTS` と `bin/tex` で出力する。
- MD・EML・TXT・画面PNGはそれぞれの保存形式で作る。手書きの再現順序は原稿に沿った紙面を `bin/scan` でスキャンPDFにする。

## b004での使い分け

- 条件表と性能比較は `asahi-performance.xlsx`。測定条件シートには対象版・負荷・操作構成・測定回数と標本の採否を記録し、測定結果シートで実測のp95と次回目標を別列に示す。比較する条件の違いと、試行を採用または除外した理由は判定・根拠欄に記す。
- 田中の評価草稿と佐藤への提案は `asahi-decision.docx`、直接PDFに組む測定報告と改善候補の技術説明は `asahi-measurement.cls`、佐藤への目標説明は `asahi-release.pptx`。2025年9月8日の評価実績はp95 246ms、次回目標はp95 184ms以下。2026年6月の比較前版の測定は別の集計として扱う。実際の対象版・測定条件・文書番号は計画と台帳で確定する。

## 見本と確認先

今回の作業先は `scratch/p01-software-engineer/b004/work/templates-0002/`。下表のパスはこの work からの相対パス。見本の番号・日付・本文は組版確認用。

| 型 | 見本（work以下） | 全ページの画像（work以下） |
|---|---|---|
| asahi-decision.docx | `examples/asahi-decision.docx`、`examples/asahi-sales-impact.docx`、`examples/asahi-sales-impact.pdf` | `render/asahi-decision-docx/`、`render/asahi-sales-impact-docx/`、`render/asahi-sales-impact-pdf/` |
| asahi-minutes.docx | `examples/asahi-minutes.docx` | `render/asahi-minutes-docx/` |
| asahi-notice.docx | `examples/asahi-notice.docx` | `render/asahi-notice-docx/` |
| asahi-explanation.docx | `examples/asahi-explanation.docx`、`examples/asahi-explanation.pdf` | `render/asahi-explanation-docx/`、`render/asahi-explanation-pdf/` |
| oshima-trial.docx | `examples/oshima-trial.docx` | `render/oshima-trial-docx/` |
| asahi-performance.xlsx | `examples/asahi-performance-3.xlsx`、`examples/asahi-performance-36.xlsx` | `render/asahi-performance-3-xlsx/`、`render/asahi-performance-36-xlsx/` |
| asahi-release.xlsx | `examples/asahi-release-3.xlsx`、`examples/asahi-release-36.xlsx` | `render/asahi-release-3-xlsx/`、`render/asahi-release-36-xlsx/` |
| asahi-release.pptx | `examples/asahi-release.pptx`（全5レイアウト） | `render/asahi-release-pptx/` |
| asahi-measurement.cls | `examples/asahi-measurement-3.tex`、`examples/asahi-measurement-3.pdf`、`examples/asahi-measurement-32.tex`、`examples/asahi-measurement-32.pdf` | `render/asahi-measurement-3-pdf/`、`render/asahi-measurement-32-pdf/` |
| harlow-review.cls | `examples/harlow-review-3.tex`、`examples/harlow-review-3.pdf`、`examples/harlow-review-32.tex`、`examples/harlow-review-32.pdf` | `render/harlow-review-3-pdf/`、`render/harlow-review-32-pdf/` |
| fujisaki-trial.cls | `examples/fujisaki-trial-3.tex`、`examples/fujisaki-trial-3.pdf`、`examples/fujisaki-trial-32.tex`、`examples/fujisaki-trial-32.pdf` | `render/fujisaki-trial-3-pdf/`、`render/fujisaki-trial-32-pdf/` |

Excelの短表は各シート3明細、長表は36明細。末尾の結合注記欄で、性能測定の `ABS-2025-PF-064`・版0.3・基準日2025-08-25・田中美由・佐藤雅之と、指摘・公開判定の `ABS-2026-RL-064`・版0.3・基準日2026-05-19・田中美由・Laura Nicholsが全文印刷されることを確認する。数値64／「64件の説明を確認」、先頭列の19／「TK250709-019」で列の間隔を確認する。藤崎試行報告は3明細と32明細で、管理欄の番号・日付・氏名、右寄せの順番列の右側、表見出しの繰返しを確認する。
