# 指摘を検査と制作の指示に取り込む

AGENTS.md、SPEC.md、runbooks/PRODUCTION.md の「8. 指摘から仕組みを直す」、runbooks/REVIEW.md、`progress/findings.tsv`、`progress/batches.tsv`、既存の `progress/improvements.md` を読んでください。`bin/batch findings` の集計と現行の検査・プロンプト・テストを照合し、くり返す問題の原因を確かめます。原稿は D1〜D10、見た目は V1〜V6、自動検査は検査名、テンプレートのレビューは templates-review の記録として扱います。

作業場所は git の worktree `scratch/wt-improve`（branch `improve`）です。**本体の作業フォルダ（repo の直下の `bin/`・`prompts/`・`tests/` など）は変更しません。** 本体では batch が動いていて、指示のひな形と検査を段階ごとに読むためです。コマンドは `scratch/wt-improve` に移動して実行し、変更・テスト・`uv sync` もその中で行います。本体の `people/`・`sources/`・`progress/` は読むだけにします（全ファイルの検査は、worktree の `bin/check` に `--root`・`--sources`・`--progress`・`--personas`・`--manifests` で本体のパスを渡して実行します）。一時物・集計・報告の出力先は `scratch/wt-improve/scratch/` です。計画・原稿・台帳・状態の書式を変える変更と、段階の順序を変える変更はしません。必要と判断したら、変更せずに理由を報告します。

してよいことは次の範囲です。

1. 機械的に判定できる指摘を `bin/check` か `bin/batch` の原稿の検査に、期待する振る舞いを表すテストを先に追加して取り込む。
2. 作る前に防げる指摘を `prompts/` の指示に足す。
3. 型の作り方に原因がある指摘を `prompts/templates.md` に足す。
4. 追加した検査・指示、その対象の観点または検査名、日付（UTC の YYYY-MM-DD）と変更の理由を `progress/improvements.md` に記録する。同じ仕様を重複して書かない。

既存の検査の条件を緩めること、レビューの観点を削る・弱めること、SPEC.md・AGENTS.md・runbooks/ を変えること、成果物と原稿を変えることは禁止です。必要と判断しても変更せず、具体的な根拠と理由を報告してください。新しい停止規則や承認手続きも追加しません。

`tests/run.sh` を実行し、変更した検査の回帰を確認します。新しい検査を `bin/check --all --accepted` で受入済みの全ファイルに当て、落ちたものは不良の見逃しか検査の誤りかを、実際のファイルで切り分けます。検査の誤りならその検査を入れません。未実行や失敗を成功と扱いません。

最終出力には、対象の指摘と観点、変更箇所、追加したテストと実行結果、全ファイルの検査結果、保留した提案と理由を書き、`scratch/wt-improve/scratch/result.md` にも残してください。統括が差分を読み、既存条件を緩めていないことと全ファイルの不合格の原因を確認します。stage・commit・push はしません。
