# 制作とは別の実行としてレビューする

SPEC.md、runbooks/REVIEW.md、persona 定義、計画、テンプレートと見本を読んで、BATCH_CONTEXT の rows を1件ずつ開き判定してください。制作側のログ・最終出力・言い訳は読まないでください。
Office・PDF は bin/render で画像にして見て本文も抽出し、画像は実物を見て、表は数値と数式を確認してください。レンダリングは指定された scratch/render/ 以下に置きます。
判定は pass または reject。迷ったら reject、直して合格にはしません。合格にも確認した箇所を具体的に書きます。自動検査の通過や件数を合格理由にしません。
BATCH_CONTEXT の review_output にヘッダなし TSV（相対パス、pass/reject、理由）の3列を、対象の全行について1件1行で書いてください。理由にタブ・改行を含めないでください。
最終出力には batch 全体への所見（整いすぎていないか、下書き・古い版・途中の記録の自然さを含む）とレンダリングの場所を書いてください。
書いてよい場所は BATCH_CONTEXT の scratch/<persona>/<batch>/ 以下だけです。成果物・計画・型・persona 定義・資料は一切変更しないでください。レビュー前後に進行役が全 batch ファイルの sha256 を照合します。stage・commit・push はしないでください。
