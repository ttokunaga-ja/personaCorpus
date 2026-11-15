# agent-ops

Codex ジョブを tmux で実行し、状態変化だけをファイルへ通知する。bash、Python 3 標準ライブラリ、tmux、Codex CLI が必要。

```sh
codex-job start --repo personaCorpus --name example --brief /absolute/path/brief.md --model gpt-6.1-terra --effort medium
codex-job status --repo kio
codex-job tail kio example
codex-job stop kio example
agent-alert list --repo kio        # 未読を表示（既読にしない）
agent-alert list --repo kio --all  # 全件を表示
agent-alert wait --repo kio       # 未読が来るまで待ち、表示して既読にする
```

既定値は model `gpt-6.1-terra`、effort `medium`、sandbox `workspace-write`（ネットワーク有効）。各リポジトリの AGENTS.md に応じて呼び出し元がモデルを指定する。`--add-dir` は繰り返し指定できる。stdin は `/dev/null` に接続する。ジョブ名は英数字で始まる英数字・ハイフン・アンダースコアのみ。既存ジョブの記録は上書きせず、再実行には別名を使う。

Claude 側ではジョブ起動後、`agent-alert wait --repo kio` を Bash のバックグラウンドで実行する。終了通知を受けたら出力を確認して対応し、再び wait する。repo ごとに独立した既読位置を使う。同一 repo の複数 wait は直列化される。`list` は既読位置を動かさない。ログとアラートにはジョブ出力が含まれるため、取り扱いに注意する。

`jobs/<repo>/<job>/` にメタデータ、log、最終出力、終了情報を保存する。`alerts/<repo>.jsonl` に状態変化を追記し、60 秒ごとに検査する。watchd が停止していれば次の `codex-job` または `agent-alert` で起動する。手動起動は `agent-watchd`。監視はプロセスを終了しない。

状態は `running`（tmux セッションが存在）、`done`（終了コード 0）、`failed`（非 0 または終了情報なしでセッション消失）、`stalled`（ログが既定20分更新されない）、`limit`（末尾に利用上限の兆候）、`stdin-wait`（末尾に stdin 待ちの文言があり、既定20分ログ更新なし）。終了情報を最優先し、その次にセッション消失、limit、stdin-wait、stalled の順に判定する。状態が継続する間は重複通知せず、回復後に再発すれば再通知する。セッションの存在だけでは内部プロセスの進行を保証しない。

上限検出は直近16 KiB に対する大文字小文字を無視した `usage limit|rate limit|\b429\b|\bquota\b`。`codex exec --help` はエラー文言を列挙しないため、手順書に挙がった語を兆候として採用した。実際のエラーログで網羅性は未検証で、誤検出もあり得る。stdin 文言は手順書に示された `Reading additional input from stdin` を使う。

`tests/run.sh` は独立した一時ディレクトリと tmux サーバーでダミージョブを検証する。監視間隔などの `AGENT_*` 環境変数と `--test-command` はテスト用。
