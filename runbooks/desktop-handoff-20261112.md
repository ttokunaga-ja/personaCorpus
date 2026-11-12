# personaCorpus：個人デスクトップでの再開と見直し（2026-11-12）

この文書は personaCorpus のセッション専用です。kio の作業は kio のセッションで行い、ここでは扱いません。役割・モデル・commit の粒度は `AGENTS.md` の「エージェント運用」に従います。

- **Fable 5.2 のセッション**：「3. 見直し項目」を調べ、提案し、資料・規則・スクリプトの修正まで行う。
- **Opus 5.5 のセッション（統括）**：見直しで決まった方式に沿って、Codex への制作指示、受入検証、commit・push を行う。

## 1. 現状（2026-11-12 時点の実測）

- **移行**
  - Mac から移行済み。HEAD `764048a15` に、このあと AGENTS.md の commit 2 件が加わっている（未 push）。
  - 未 commit は `git status` で 2,998 行、ファイル数で 67,385（うち `workspace/people` が 66,303）。Mac と sha256 で一致を確認済み。
  - Git 管理外の 150GB（`scratch/` 147GB を含む、381 万ファイル）も照合済み。
  - 証跡は `../migration-from-mac/`。
- **Linux で制作できない原因（最初に直す）**
  - `.runtime/kio-eval` が macOS arm64 用の実行ファイルで、`bin/check-ready` が `Exec format error` で失敗する。kio の該当 commit から Linux 用に作り直し、固定の手順も見直す必要がある。
  - `workspace/persona-workspace-owner.json` の `workspace_root` が Mac の絶対パスのままで、`bin/p14-full-ledger verify` が `workspace owner root mismatch` で失敗する。
  - `README.md`、`runbooks/BATCH_PROTOCOL.md` などに Mac の絶対パスが残っている。
- **残っている lease（2026-10 に中断したセッションの残骸。今は動いていない）**
  - p09：`p09-full-structure-v2-20261022-root-01`。group 039 の after manifest の 7,254 件と、home の実ファイル数が一致する。
  - p17：`p17-g005-parent-20261029-h21-v1` と、scope primary-02・04・05。group 005 は before manifest しかない。
  - 回復の手順は `runbooks/` に従う。回復するかどうかは統括が判断する。
- **未 commit の成果物**
  - checkpoint には受入記録があるが、現物・ledger・manifest との照合が済んでいないため、commit してよいと確定した単位はまだない（棚卸し：`../migration-from-mac/desktop-jobs/pc.out.md`）。
  - p16 と p18 の assignment ledger に変更がある。
  - 未追跡の helper と tests、`.gitignore` の変更（23 行）がある。
- **GitHub の secret scanning 誤検知（alert #1）**
  - p16 の `pdf_scan/*.pdf` 45 件で、ASCII85 で文字列にした画像データの中に、偶然 Vault トークンと同じ形の文字列ができていた。
  - 2026-11-12 に false positive として閉じた。
  - 配布物が同じように検出されないよう、PDF の作り方（ASCII85 を使わない等）を見直す必要がある。
- **repo 直下の不明なファイル**
  - `--psm.txt`、`-.jpg`、`-.png`、`-.ppm`、`scratch_rows.*`、空の `p09-full-004364/`、未追跡の `corpus/`・`data/`・`finance/`（33 ファイル）。
  - OCR ツールの引数が誤って解釈され、作業フォルダに出力したものとみられる。出力したスクリプトを特定して直す。
- **旧監視**
  - Mac の Codex automation 4 件は、5〜10 分ごとに LLM を起動しており、1 日あたり約 720 回だった。プロンプトは最大 7,000 文字、memory は 244 行まで膨らみ、利用率の閾値などの規則も複雑だった。
  - これは廃止し、LLM を使わない `../agent-ops` に置き換えた。

## 2. 統括（Opus）の手順

- **Codex の起動**：`codex-job start --repo personaCorpus --name <job> --brief <path> --model <AGENTS.md の指定> --effort medium`
- **待ち方**：`agent-alert wait --repo personaCorpus` を Bash の background で実行し、通知が来たときだけ処理する。待っている間はトークンを使わない。
- **状態の確認**：`codex-job status --repo personaCorpus`。詳しい使い方は `../agent-ops/README.md`。
- **commit**：AGENTS.md の「commit の粒度」に従い、persona × batch ごとに 1 commit とし、受入のたびに push する。

## 3. 見直し項目（Fable が調べ、提案し、実行する）

Fable は自由に調べ、提案し、実行してよい。ただし次は事前にユーザーの確認を取る。
- 成果物を大量に削除すること
- force push や履歴の改変
- 秘密情報・外部サービス・課金に関わること
- AGENTS.md のうち、モデル・権限に関わる変更

1. **目的に合っているかの監査**
   - 目的は、Kio の検証のために、さまざまな職種の人のデスクトップ環境を、一般的な PC と同じフォルダ構造で再現すること。
   - `workspace/people` の全体を、次の観点で調べる。
     - 一般的な PC にない制作用・メタ情報のファイルが混じっていないか
     - フォルダ構造が不自然でないか。予備調査では `foundation/` が 25,083 件、`visual/` が 20,848 件（うち `visual/source-art/` が 5,631 件）あった
     - ファイル名が不自然でないか。予備調査では、末尾が `-s` と 6 桁の連番になっている名前が 48,182 件あった（例：`p14-finance-controller/home/desktop/working/foundation/logs/…-s006601.txt`）
     - ファイルの種類の割合、日付や更新時刻、職種ごとの違い
   - 調べた結果をもとに、常設の構造検査（受入条件）を作る。
2. **制作環境と配布物の分離、配布できる状態かの検証**
   - 制作のための資料（canonical、progress、ledger、scratch、_control など）と、配布する「実際のデスクトップ環境」が混ざっていないか。
   - 配布の単位と形式（どのフォルダをどう梱包するか）。
   - 秘密情報・個人情報らしい文字列の検査。secret scanner の誤検知への対策も含む。
   - ライセンス・著作権。容量。配布物単体で再現できるか。
   - 配布物を組み立てて検証する手順を作る。
3. **制作方式の見直し（効率と品質）**
   - 「作る → 自動検査 → 受入 → commit・push」を、persona × batch の小さい単位で回す方式にする。
   - 受入をまとめて後回しにしたせいで、未 commit が 67,385 ファイル溜まり、1 commit が 2.2GB になって push の上限を超えた。
   - 品質の確保は、自動検査を基本にし、Fable や Sol による抜き取りレビューを組み合わせる。
   - 制作は Terra で行い、品質が安定しない場合は Sol に切り替える。どの段階でどのモデルを使うか（Luna をどこまで使えるか）も決める。
   - scratch の保持方針（何を残し、何を捨て、どれくらい保持するか）。
4. **止まりやすさの解消**
   - AGENTS.md と runbooks には、過去に Codex が自分で足した厳しすぎる規則や複雑な規則がある。例：容量の承認（admission）についての複雑な規定、lease、利用率の閾値。
   - これらを洗い出し、削除や簡素化を行う。
   - Codex が止まってよい条件を、AGENTS.md の「停止の基準」に揃える。
   - lease には有効期限と生存確認（heartbeat）を入れ、統括が回復できる手順にする。
5. **移行の残り**
   - 1 の「Linux で制作できない原因」を直す。
   - 未 commit 分を、照合したうえで persona × batch ごとに commit・push する（3 の方式が決まった後）。
   - lease を回復する。
