# 受入済みファイルの名前を決め直す

AGENTS.md、SPEC.md、runbooks/PRODUCTION.md の「受入済みファイルの名前の付け直し」と runbooks/REVIEW.md の名前の観点を読みます。対象は RENAME_CONTEXT の persona です。

1. `personas/<persona>.md` の先頭設定に `date_prefix:` を足します。日付を名前の先頭に置くときの書式と区切りを書きます（例 `YYYY-MM-DD_`、`MMDD_`、`DD Mon - `）。トークンは YYYY、MM、DD、M、D、Mon、Month。書式を複数使い分けるときは `|` で区切ります。「フォルダ構成」の後に「## 名前の付け方」を書き、日付と区切り、言語、手で付ける名前と写真・スクリーンショット・ダウンロード・保存メールの自動名の使い分けを、その人の整理の癖に合わせて例 5〜8 個で示します。
2. `manifests/<persona>.tsv` の全ファイルを見ます。本文の要約・結論・会話文になっている名前、日付の書式が混ざった名前、日付フォルダと同じ日付を重ねた名前、長すぎる名前、`bin/rename validate` が指摘する名前をすべて付け直します。文書の種類・相手・案件・日付など、その人が探す手がかりで名前を付けます。拡張子を除いて 40 文字まで、保存メールは 60 文字までです。件名を使った保存メールは文の形でも構いません。
3. `progress/<persona>/b*.md` の「内容」と `sources/<persona>/` の原稿で中身を確かめます。本人が付け直さない写真・スクリーンショット・ダウンロードの一部には `IMG_0412.jpg`、`Screenshot 2025-03-12 at 14.05.12.png`、`export_2025-03.csv`、`Invoice-0412.pdf` のような、機器やダウンロード元の名前を使えます。数字を 5 桁以上続けません。
4. `progress/<persona>/rename.tsv` に旧パス、新パス、短い理由を 1 件 1 行で書きます。見出しは `旧<TAB>新<TAB>理由` です。両パスは `people/<persona>/` からの相対パスで、拡張子を保ち、原則として同じフォルダに置きます。

`bin/rename validate <persona>` の指摘を 0 件にして終わってください。上記の persona 定義と対応表だけを編集します。一時物は RENAME_CONTEXT の scratch 以下に置きます。stage・commit・push はしません。
