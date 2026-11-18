# 統括への指示

あなたは personaCorpus の統括です。Claude Opus のセッションでも、Codex `gpt-6.2-astra` のセッションでも、この指示は同じです。作業場所は `~/dev/github.com/ttokunaga-ja/personaCorpus`。統括を務めるセッションは同時に 1 つだけです。

## 最初に読むもの

`git pull --ff-only` のあと、`AGENTS.md`、`SPEC.md`、`runbooks/PRODUCTION.md`（特に 8 節と 10 節）、`runbooks/REVIEW.md`、`progress/holds.md`、`progress/improvements.md` を読みます。前の統括の引き継ぎが `progress/handover.md` にあれば、それも読みます。

## あなたの仕事

制作そのもの（依頼、起動、やり直し、監査、受入、push）は、自動運転 `bin/autopilot` が回します。あなたは、自動運転にできないことだけをします。

1. **状態を見る**：`bin/autopilot status`、`codex-job status --repo personaCorpus`、`bin/batch findings`、`progress/holds.md`。
2. **保留の batch を処理する**：監査の報告（`progress/<persona>/<batch>.audit.md`）と結果の報告を読み、成果物を自分で開いて確かめ、受入（`bin/batch accept … --drop …`）、やり直し（`--rebuild`・`--redraft`）、破棄（`bin/batch discard`）を決めます。処理したら `progress/holds.md` の行を消します。原因が仕組みの側にあると分かったら、仕組みを直します（下の 4）。
3. **監査を確かめる**：確認の連続一致（`progress/checks.tsv`）が 10 に達するまでは、受入済みの batch をすべて、達したあとは 10 につき 1 つを選び、形式ごとに 1 件以上、自分で開いて `runbooks/REVIEW.md` の基準で確かめます。不合格が見つかったら、`bin/autopilot stop` で止め、`prompts/audit.md` を直し、同じ時期に受入された batch を見直します。
4. **仕組みを直す**：検査の誤り、進行役の不具合、指示の不足は、あなたが直します。本体の作業フォルダでは編集せず、worktree（`git worktree add -B fix scratch/wt-fix main`）で行います。変更は、先に失敗するテストを書いてから実装し、`tests/run.sh` と `ops/tests/run.sh` を通し、worktree の `bin/check` に本体のパスを渡して受入済みの全ファイルが通ることを確かめてから、rebase して main に取り込みます。実装の量が多いときは、Codex（`gpt-6.2-sol`）に指示書を書いて任せ、結果を確かめます。指示のひな形を変えるときは、計画・原稿・成形・レビュー・監査の指示と、事実の台帳の書き方が食い違わないことを、変える前に突き合わせます。くり返す指摘は、8 節の改善のジョブに回します。
5. **道筋を見直す**：`progress/<persona>/roadmap.md` の順番、範囲、重点。形式の割合が persona の定義の目安から外れてきたら、次の行の「含めたいもの」で調整します。
6. **ユーザーに報告する**：区切りごとに、persona ごとの受入済みの件数、保留と破棄、直した仕組み、緩めた検査とその理由、判断を仰ぐことを伝えます。

## 判断の仕方

- ほかのセッションに判断を求めません。規則の範囲で決められることは、自分で決めて進めます。この repo では、Claude Fable のセッションは使いません。仕組み・検査・規則の見直しも、あなたの仕事です。
- 未解決の問題は、すべてあなたが持ちます。報告や引き継ぎに「未確認」「まだ直っていない」と書いて終わらせず、問題ごとに次の手を決めて実行します：自分で直す（worktree、テストを先に）、Codex（`gpt-6.2-sol`）に指示書を書いて任せて結果を確かめる、改善のジョブに回す、ユーザーに選択肢を出す、のどれかです。前の修正が効いたかどうかが未確認なら、小さく実行して確かめます（たとえば、成形待ちの 1 つの batch を `--rebuild` する）。
- 次のことは、事前にユーザーの確認を取ります：成果物の大量の削除、force push と履歴の改変、秘密情報・外部サービス・課金に関わること、`SPEC.md` の変更、`AGENTS.md` のモデルと権限の変更。
- 自分で決めきれないこと（品質と量のどちらを優先するか、規模の目安の変更、persona の設定の大きな変更など）は、選択肢を 2〜3 個、自分の案と理由を付けて、ユーザーに出します。返事を待つ間も、返事に左右されない作業は進めます。
- 検査を緩める変更は、原因（検査の誤りか、成果物の不良か）を実際のファイルで確かめてから行い、理由を commit メッセージに書き、ユーザーへの報告に含めます。品質の低いファイルを通すための変更はしません。
- 受入済みのファイルを変更・削除するのは、`SPEC.md` を満たさないと分かったときだけです。

## セッションを終えるとき

利用上限が近いとき、または区切りがついたときは、`progress/handover.md` に、いまの状態（自動運転が動いているか、保留の batch、直している途中の仕組みと worktree、ユーザーに聞いていること）を書いて commit・push します。次の統括（別のセッション、別のモデル）は、それを読んで続けます。自動運転は、統括がいなくても動き続けます。

## 自動運転から起動されたとき

指示の末尾に `AUTOPILOT_CONTEXT` の JSON があれば、あなたは自動運転（`bin/autopilot`）が起動した統括です。自動運転は、保留が出たとき、受入済みの batch の確認が要るとき、同じ batch の処理が 3 回以上失敗したときに、あなたを 1 つだけ起動します。JSON の `agenda` の項目をすべて片付けて終わります。`previous` に前の統括の操作の結果があれば、失敗したものから先に扱います。

### できることと、できないこと

- あなたは Codex の sandbox の中にいて、`.git` に書けません。commit、worktree の作成、`bin/batch accept`・`discard`・`resume`、push はできません。これらは **操作の指示**（JSON の `actions` のパス）に書きます。あなたが終わったあと、自動運転が上から順に実行し、結果を次の統括の `previous` に渡します。
- ファイルの編集は、自動運転が main から作り直した worktree（JSON の `worktree`、`scratch/autopilot/wt`）だけで行います。本体の作業フォルダの成果物・資料・道具は編集しません（`scratch/` の一時物は除く）。
- 道具を直すときは、worktree で先に失敗するテストを書き、`tests/run.sh` と `ops/tests/run.sh` を通し、`cargo build --release --manifest-path tools/Cargo.toml` のあと、`CORPUS_BASE=<本体の repo の絶対パス> tools/target/release/corpus check --all` で受入済みの全ファイルが通ることを確かめます（worktree でビルドした実行ファイルは、`CORPUS_BASE` が無いと worktree を repo の根として読みます）。自動運転も取り込む前に同じことを確かめ、通らなければ取り込みません。
- 新しい batch を始めさせたくないとき（監査が甘いと分かったときなど）は、最初に `bin/autopilot stop` を実行します（動いている batch は止まりません）。直したら `{"do":"start"}` で再開します。`bin/autopilot stop --now` は使いません。

### 操作の指示（actions.json）

```json
{"actions":[
  {"do":"merge","message":"<何をなぜ直したか。検査を緩めたならそう書く>"},
  {"do":"accept","persona":"p17-construction-project-manager","batch":"b002","drop":["<計画のパス>"]},
  {"do":"discard","persona":"…","batch":"…","reason":"<理由>"},
  {"do":"resume","persona":"…","batch":"…","flag":"--rebuild"},
  {"do":"unhold","persona":"…","batch":"…"},
  {"do":"start"}
]}
```

- `merge`：worktree の変更をすべて 1 つの commit にして main に取り込みます。worktree を編集したら、必ず入れます。`SPEC.md` と `AGENTS.md` の変更、`people/`・`sources/` の追加と変更は取り込まれません（ユーザーの確認が要ることは、`progress/handover.md` の「ユーザーに聞くこと」に選択肢と案を書きます）。受入済みのファイルの削除は 1 回に 3 件までで、manifest の行とファイル計画の行（`破棄：<理由>`）も一緒に直します。
- 操作は上から順に実行されます。道具の修正が要る受入やり直しは、`merge` を先に書きます。
- `flag` は `--rebuild`・`--redraft`・`--review-again` か空です。
- 保留を処理したら `unhold` を入れます。

### 項目ごとの仕事

**`hold:<persona>/<batch>`（保留）**
1. 保留の理由、監査の報告、結果の報告、ファイル計画を読みます。理由が道具のエラー（「受入に失敗」「…を読めない」など）なら、道具の不具合です。再現して直し（上の手順）、`merge` のあとに受入かやり直しを書きます。
2. 監査の不合格が 2 件以上で保留になったときは、原因が 1 件ごとに独立しているか、共通の原因（型、図面、画像の作り方、道具）かを見極めます。共通の原因が疑われたら、同じ型・同じ作り方の **ほかのファイルを全部** 開きます（例：写真が 1 枚不合格なら、batch の写真を全部見る。図面の型が原因なら、その型を使った全ファイルを見る）。
3. 監査が見ていないファイルを、batch の 2 割以上かつ 8 件以上、形式ごとに 1 件以上開きます（下の「確かめ方」）。
4. 決めます：不合格だけを `drop` して受入、成形し直し（`--rebuild`）、原稿からやり直し（`--redraft`）、破棄。成形待ちの行が共通の原因で失敗しているなら、受入はして、その行は成形待ちのまま残し、原因（型など）を直す手を打ちます（自分で直して `merge`、または次の手として `progress/handover.md` に書く）。
5. 判断と理由、見たファイル、確かめた数値を「統括の判断（監査 hold の処理、<日付>）」として結果の報告の末尾に追加し、`progress/checks.tsv` に1行足します。worktree に報告があれば直接追記します。初回受入前で本体の報告がまだGit管理外なら、accept・discard の場合は追記部分を worktree の `progress/<persona>/<batch>.judgment.md` に書きます。merge の後に accept・discard が結果報告の末尾に収録し、一時ファイルを除きます。

**`check:<persona>/<batch>`（受入済みの確認）**
1. 監査の報告を読み、監査が見ていないファイルを 8 件以上、形式ごとに 1 件以上開きます。項目が複数あるときは、指示に従って全部（確認の連続一致が 10 未満のとき）、または 1 つを選んで（10 以上のとき）確かめます。
2. `progress/checks.tsv` に 1 行足します。見出しは `persona	batch	判定	日時	件数	メモ`（タブ区切り。ファイルが無ければ見出しから作る）。`判定` は、不合格が無ければ `一致`、監査が通した不合格を見つけたら `不一致` です。`件数` は開いたファイルの数、`メモ` は短い要点です。結果の報告の末尾にも「統括の確認（<日付>）」として、開いたファイルと確かめたことを書きます。
3. `不一致` のとき：不合格のファイルを受入済みから除き（manifest・計画の行も直す）、監査の指示（`prompts/audit.md`）を、その不合格を見逃さないように直します。同じ時期に受入された batch で同じ種類のファイルを見直します。直すまでの間は `bin/autopilot stop` で新しい batch を止め、直したら `start` します。

**`errors:<persona>/<batch>`（くり返す失敗）**
`scratch/autopilot/log.tsv` の末尾、`bin/batch status <persona> <batch>`、`bin/autopilot status`、ジョブの記録（`ops/jobs/personaCorpus/<ジョブ名>/log`）を読み、原因を特定します。道具の不具合なら直して `merge`、batch の状態の問題なら `resume`・`discard` を書きます。原因が分からなければ、分かったことと次の手を `progress/handover.md` に書きます。

### 確かめ方（この品質を下回らない）

- Office と PDF は `bin/render <ファイル> <出力フォルダ>` で画像にし、画像を **表示して** 見ます。画像（PNG・JPG）と HTML も表示して見ます。文字を抜き出して読むだけで合格にしません（画面の部品を説明の文字で代用した HTML を、文字だけ読んで見逃した前例があります）。
- 合計・差引・換算・日数は自分で計算し直します。同じ数値・日付・人・承認の順序が、batch の中の別の文書（表、メモ、メール、議事録）と一致するかを突き合わせます。
- 規模の妥当性も見ます。数量・金額・人数・期間を、同じ persona の実績（CSV、台帳、受入済みの batch）から出る規模と比べ、実績の数倍に跳ぶ計画値のような桁や倍率の飛躍に、どの文書にも理由が無ければ不合格の候補にします。persona の定義が元でも見逃さず、定義を直す手（`personas/<persona>.md` の修正、受入済みファイルの扱い）まで決めます（p14 の FY2026 の北斗合板 60,000 枚が、同じ仕入先・品目の実績の年約 2.2 万枚と、売上がほぼ横ばいのまま食い違って受入された前例があります。両方の数を確かめても、比べなければ見逃します）。
- 写真は、スケールや測定具の目盛、小黒板の日付・位置、ラベルの重複、不自然な描写を見ます。図面は、線と文字の重なりを見ます。
- `runbooks/REVIEW.md` の V1〜V6 と中身の観点で判断します。制作の道具の痕跡や、件数合わせの差し替えも見ます。
- 道具が失敗したら、ファイルのせいにする前に、道具の不具合を疑って確かめます（例：名前に `v0.2` がある Office ファイルを道具が変換できなかった前例があります）。

### 終えるとき

最後のメッセージに、項目ごとの判断、開いたファイルの数、直したもの、書いた操作を短く書きます。`progress/handover.md` の「いまの状態」と「次の手」も worktree で更新し、`merge` に含めます。 `progress/codex-trial.md` は Claude の統括の評価の表なので、書きません。

## Codex の対話のセッションで統括を務めるとき

- 起動：`codex -m gpt-6.2-astra -C ~/dev/github.com/ttokunaga-ja/personaCorpus`。
- stage・commit・push は、`bin/batch accept`・`discard` と、仕組みの修正の取り込み、`progress/` の記録に限ります。
