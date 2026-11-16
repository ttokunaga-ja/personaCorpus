# 指示書：repo のツールを Rust に書き換える

## 目的
`bin/` と `ops/bin/` のツール（今は Python のスクリプト）を、Rust の 1 つの実行ファイルに書き換えて統一する。動作は変えない。仕様の正本は `runbooks/PRODUCTION.md`、`runbooks/REVIEW.md`、`AGENTS.md`、`SPEC.md`、`ops/README.md` と、今のテスト（`tests/`、`ops/tests/`）である。

## 前提
- どの batch も動いていないときに行う（`codex-job status --repo personaCorpus` と `bin/batch status` で確かめてから始める）。動いている batch があれば、何も変更せずに終わる。
- personaCorpus は他の repo に依存しない。他の repo のコードを使わない。crate は crates.io の一般的なものだけを使う。

## 作るもの
- `tools/`：Cargo の workspace。実行ファイルは 1 つ（`corpus`）で、サブコマンドは `check`・`batch`・`manifest`・`pack`・`render`・`office-pdf`・`tex`・`scan`・`stamp`・`recalc`・`job`（今の `codex-job`）・`alert`（今の `agent-alert`）・`watchd`。
- `bin/` と `ops/bin/` の各コマンドは、同じ名前のまま、`tools/target/release/corpus <サブコマンド>` を呼ぶ薄い shell スクリプトにする（無ければ `cargo build --release` を促して失敗する）。資料に書いてあるコマンドの名前、引数、出力の形式、終了コードを変えない。
- 外部コマンド（`soffice`、`pdftotext`、`pdfinfo`、`pdftoppm`、`tesseract`、`ffprobe`、`lualatex`、`tmux`、`git`、`codex`）の呼び方は今と同じにする。
- Python は、制作のときに Codex が文書を組み立てるためのライブラリの環境（`pyproject.toml`）としてだけ残す。repo のツールとテストからは Python を無くす。

## 進め方
1. 先に、今のテストを「コマンドを実行して結果を見る」形にそろえる。Python のモジュールとして `bin/check` や `bin/batch` を読み込んでいるテストは、コマンドの実行に置き換える。この時点で、今の Python のツールに対してテストがすべて通ることを確かめる。
2. テストを Rust の統合テスト（`tools/tests/`）に移す。fixture の文書（DOCX・XLSX・PPTX・PDF）を作る部分は、テスト用の小さな Python スクリプト（`uv run`）を呼んでよい。
3. サブコマンドを 1 つずつ Rust に書き換え、そのたびにテストを通す。順序は `manifest`・`pack` → `render`・`office-pdf`・`tex`・`recalc` → `stamp`・`scan` → `check` → `job`・`alert`・`watchd` → `batch`。
4. すべて置き換えたら、Python のツールと、不要になったテストを消す。新旧を併存させない。
5. `tests/run.sh` と `ops/tests/run.sh` は、`cargo test --release` を呼ぶ形にして残す。

## 確認
- すべてのテストが通ること。
- 受入済みの全ファイルで、書き換え前後の `bin/check --all --report` の結果が同じであること（書き換え前の結果を先に `scratch/rewrite/before.tsv` に保存して比べる）。`bin/manifest <persona> --verify` が全 persona で通ること。`bin/pack` の出力が書き換え前とバイト単位で同じであること。
- `bin/check --all` の所要時間を、書き換え前後で報告する。
- 一時物は `scratch/rewrite/` か `$TMPDIR`。`people/`、`sources/`、`personas/`、`templates/`、`progress/`、`manifests/`、資料は変更しない。資料のうちコマンドの実装に触れている箇所（`README.md` の `bin/` の説明、`runbooks/PRODUCTION.md` 0 節の準備）で書き換えが要る点は、変更せずに最終メッセージに挙げる。stage・commit・push はしない。

## 止まってよい条件
取り返しのつかない操作・外部公開・秘密情報・課金に関わる判断だけ。それ以外は自分で決めて、最終メッセージに書く。

## 返すもの（最終メッセージ、日本語）
作ったファイルと行数、消したファイル、テストの結果、前後の比較の結果と所要時間、決めたこと、資料の書き換えが要る点。
