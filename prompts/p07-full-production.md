# p07 Full mass production task

> **Accepted READY package; physical Full production has not started.** Run
> this prompt only in one new p07 Local task.

Open one new Codex Local task directly at the `personaCorpus` repository root,
then paste the following instruction. This is a new production parent, not the
allocation task; it is safe to reuse from accepted checkpoints.

```text
このLocal taskでp07（humanities researcher）のFull physical-artifact productionを担当してください。既存M1 200件をbyte-for-byte保持し、Full追加6,800件、最終7,000 filesまで自律的に制作・QA・受入を継続してください。rootは`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`のみです。別worktree、別workspace、別clone、Git操作、Kio操作は行いません。

最初にAGENTS.md、README.md、runbooks/COMMON_RULES.md、runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、runbooks/SESSION_HANDOFF.md、runbooks/personas/p07-humanities-researcher.md、progress/p07/full/content-spine.md、progress/p07/full/full-assignment-ledger.jsonl、既存checkpoints（あれば）を完全に読んでください。canonical plan/render/ownerだけがauthorityであり、解釈・変更しません。

開始時に次のSHA-256を照合し、これらのportable inputsを編集、再生成、置換、削除しません。
- human M1 display: `86167aaaad7764f33e9fa7a7913ccd8f8c20ad384f02b34e798370b68c56f669`
- machine M1 ledger: `c30e39a4ddf8bbd6b7f27daf3e1bdcfb7b5d1171d700fd7af8d1a71152074c01`
- inventory: `dda7dbd52126991e69396361122f36869d9701c7bc7b4aeac3be9c4f11c89c10`
- immutable M1 baseline: `dccc310598d32eb217094be2aff848a5716da10a393bfa74fb977d5332a96531`
- reservations: `7ad9dd2c513626c52e7ff28aebba54b726f4afc6985aa44e31fb49d92f04d3cd`
- content spine: `c332ed21dff56d41e8ddbda11fd8add879c01b3200b47b90060dd508d636613b`
- full master: `38fe0e6212ed41e8ed1ff4f5ead590179d9a34f4fd27a2cbf56b322534efd1a2`
- shared `bin/full-ledger`: `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`

`./bin/p07-m1-assignment`はbackfill helperであり、production中に実行しません。p07はstrict same-scope/family reservationであり、reconciliationもisolated CLIもありません。次をread-only preflightとして実行し、`result: ok`、inventory 7,000、reservation/baseline 200、assignment 6,800を確認してください。

./bin/full-ledger verify --persona p07 --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p07/full/canonical-source-inventory.jsonl --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p07/m1-assignment-ledger.jsonl --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p07/full/m1-reservations.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p07/full/full-assignment-ledger.jsonl --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p07/full/manifests/m1-baseline.jsonl

他personaのleases、uncommitted changes、global active_leasesは通常状態です。見ない・操作しないでください。p07自身のparent/scope leaseが存在する場合だけ、何も変更せずsessionを報告して停止してください。preflight後に一意sessionでp07 parent leaseをclaimし、release tokenは親process内だけに保持します。tokenをprompt、Subagent message、progress、scratch、log、artifact、Gitに書きません。

初回はhomeが200 regular filesかつbaselineとbyte一致することを確認します。再開ではtoken-free checkpoint、参照するcreate-only before/after manifest、masterのbatch/scope/count、現在manifestを照合してaccepted集合だけを再構成します。checkpointなしのpath、assignment外path、M1/accepted変更、symlink/nonregular、未受入deltaがあればrecoverや削除をせず停止します。

残りが0まで次を繰り返します。各scopeでexternal dependenciesがaccepted済みの最初の未受入batchを最大1つ選び、readyな異scope batchesを品質を維持できる全Subagent枠で並列groupにします。同scopeは常に1 writerです。group前にcreate-only cumulative before manifestを作り、親だけがscope leaseをclaimし、1 worker=1 scope/batchでmaster該当rowsだけを正規scope leafに生成させます。全workerの親QA後、scratch candidate-afterを作成して全member `--batch-id`を一つのatomic verifyに渡します。成功時だけ公式after、token-free checkpoint、通常scope releaseを行います。partial accept/releaseはしません。parent leaseは全scope不在後に通常releaseします。

各workerはDocuments=DOCX、PDF=searchable/scan PDF、Spreadsheets=XLSX、Presentations=PPTX、ImageGen=PNGとscan source artworkを使用し、render-and-verifyを全件で行います。temporary source/render/QAは`scratch/p07/<scope>/<batch>/`だけです。Linden Archive Project、Haldenmere Civic Repository、Box 14、folio 184、citation LA-042、2026-07-13とcontent spineを一貫させ、実在archive、PII、credential、著作権資料、routable IP、placeholder/TODOを作りません。

scan PDFがfinal同scope image dependencyを持つ時は、そのaccepted imageを用います。masterに`scratch_input`があるscan rowだけは指定された`scratch/p07/<scope>/<batch>/scan-sources/`の一意ImageGen PNGを使います。これはfinal artifactでもworkspace fileでもcount対象でもなく、原寸可読性、PNG-to-scan pixel/source一致、scan PDF zero text layerをQAします。

Python構文QAは`ast.parse`またはmemory-only compileだけです。final treeをimport/実行せず、py_compile/compileallを使用しません。`__pycache__`/`*.pyc`はfinal treeで0件です。今回selected workerが生成したことをbefore/mtime/sourceから一意に証明できるcacheだけは、SHA evidenceをscratchへ残した後に削除できます。それ以外のcache、repo外path、原因不明の外部誤配置は変更せず停止します。

通常の制作/QA失敗は同worker/scope leaseで修正して継続します。ただしtoken喪失、通常release失敗、recover、fixed input mismatch、M1/accepted変更、assignment外/symlink/nonregular、unproven cache、scope/finality boundary違反、secret/real PII、skill不能、duplicate writer危険では何も追加変更せず、停止証跡と残存leaseを報告してください。

完了時だけ、home 7,000 regular files、M1 200不変、Full 6,800/6,800と全checkpoint、scope/family/skill集計、actual SHA unique、全skill QA、scan PDF QA、manifest一致、p07 parent/scope lease不在を示して「p07 Full production完了」と報告してください。Git add/commit/push、root manifest、branch/worktree、Kio index/history/search/performanceの主張は禁止です。
```
