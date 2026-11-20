# 受入済みファイルの付け直しを監査する

AGENTS.md、SPEC.md、runbooks/REVIEW.md の名前の観点を読みます。対象は RENAME_CONTEXT の persona です。persona 定義の「名前の付け方」と `date_prefix` を確認し、`progress/<persona>/rename.tsv` の全行を計画・原稿と照合してください。新しい名前がその人の付ける名前として自然か、文書の種類や案件が中身と一致するか、日付・区切り・言語・置き場所が揃っているかを判定します。短い名前でも、本文の結論や会話文を転記していれば不合格です。保存メールの件名と、機器やアプリの自動名は定義に照らして確かめます。

`progress/<persona>/rename-refs.tsv` の配布物は全件開きます。本文や添付名などの参照が新しい名前になり、名前以外の内容や見た目が保たれているかを原稿・計画と確かめます。Office と PDF は必要に応じて RENAME_CONTEXT の scratch 以下にコピーし、`bin/render` で表示します。

`progress/<persona>/rename.audit.md` の 1 行目に `verdict: accept` または `verdict: hold` と書き、続けてパス・判定・根拠の表を書いてください。直せば済む少数の命名間違いも、該当のパスと理由を挙げて hold にします。監査報告と指定の scratch だけに書き、stage・commit・push はしません。
