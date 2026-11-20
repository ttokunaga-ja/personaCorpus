# 配布物に残った旧ファイル名を直す

AGENTS.md、SPEC.md、runbooks/PRODUCTION.md の「受入済みファイルの名前の付け直し」を読みます。対象は RENAME_CONTEXT の persona です。`progress/<persona>/rename.tsv` と `rename-refs.tsv` を開き、一覧の配布物に残っている旧名を置換済みの原稿に合わせて直します。本文や版、数値、添付の中身など、名前に関係しない内容を保ちます。前回の不合格の理由が示されていれば、該当箇所も確認します。

- EML は添付の filename と本文・引用中の言及を直し、MIME の構成と添付本体を保ちます。付け直しで添付元の参照表記も変わった場合は、新しい保存版を添付します。
- Office は文中の言及を直し、型、レイアウト、数式を保ちます。
- PDF は元の計画と原稿で作り方を確かめ、`templates/<persona>/` と指定の `bin/tex`、`bin/office-pdf`、`bin/scan` を使って再成形します。

成形した文書は `bin/stamp` で元の作成者・日時に戻し、`bin/render` で見た目を確かめます。一時物とレンダリングは RENAME_CONTEXT の scratch 以下です。最後に `bin/rename verify <persona>` を通してください。対応表・原稿・規則はそのまま使い、stage・commit・push はしません。
