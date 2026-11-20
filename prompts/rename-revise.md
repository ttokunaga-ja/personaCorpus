# 監査で保留になった名前を見直す

AGENTS.md、SPEC.md、runbooks/PRODUCTION.md の「受入済みファイルの名前の付け直し」と runbooks/REVIEW.md の名前の観点を読みます。対象は RENAME_CONTEXT の persona です。

1. `progress/<persona>/rename.audit.md` の hold の行と理由を読み、`rename.tsv` で該当行を特定します。persona の「名前の付け方」と `date_prefix` を確認し、現在の新パスにある原稿・配布物と計画で中身や置き場所を確かめます。
2. 指摘された行の「新」と、必要に応じて「理由」だけを書き直します。「旧」、ほかの行、ほかのファイルは変えません。配布物と原稿の移動や参照の置換は道具が行います。
3. `bin/rename validate <persona>` の指摘を 0 件にして終わってください。前回の不合格の理由が示されていれば、該当箇所も確認します。

一時物は RENAME_CONTEXT の scratch 以下に置きます。stage・commit・push はしません。
