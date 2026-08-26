# p01 Full mass production task

> **Historical completed-production prompt. Do not run against the current
> 12,000-file accepted p01 home without a separately approved revision.**

Paste the text below into **one new Codex Local task** opened at the
`personaCorpus` repository root. It starts from the accepted 12-file pilot and
is also safe to reuse after an accepted checkpoint if the task must be resumed.

```text
このLocalタスクでp01（software engineer）のFull mass physical-artifact productionを
担当し、受入済みpilotを含むFull追加11,800件、最終12,000 filesまで、通常の確認を
ユーザーへ求めず自律的に継続してください。これはallocation taskの続きではなく、新しい
production親タスクです。作業rootはCodexで開いた現在のpersonaCorpus repository rootを
そのまま使い、別worktree、別workspace、別runtimeを作りません。この同じ指示は、受入済み
checkpointからの再開にも使用できます。

## Authorityと固定入力

最初に次を完全に読んでください。

- `AGENTS.md`
- `README.md`
- `runbooks/COMMON_RULES.md`
- `runbooks/FULL_PRODUCTION_PROTOCOL.md`
- `runbooks/FULL_LEDGER_SPEC.md`
- `runbooks/SESSION_HANDOFF.md`
- `runbooks/personas/p01-software-engineer.md`
- `progress/p01/full/content-spine.md`
- `progress/p01/full/full-assignment-ledger.jsonl`
- 既存の`progress/p01/full/checkpoints/*.json`

accepted Rust plan、materialization record、workspace ownerだけをauthorityとし、canonicalを
再生成・再解釈しません。次のportable inputのSHA-256を開始時に照合してください。

- canonical source inventory:
  `c0013f9925d581f536dad7eabf3b451d4fb8dfed5af80416b4f5b0e8514dea92`
- M1 assignment v2:
  `3c4a88a489e7fc311dd9636148c4faf3e9ed644ed400dd7bce08367fb80914ff`
- M1 reservations v3:
  `556f81bc3540dea44d0b96a9ca708c07ec4299a61704ecd77b7b97fab504ba6f`
- immutable M1 baseline:
  `6b59e44cd520134928b710303f0ddaa2890f006d58d7c582dc07bc280f429bec`
- accepted pilot after manifest:
  `4dae33fac0543a2f692d55c5159e70d40f5d2708ae1429936ba16272db44affb`
- content spine:
  `9ea5b4ca505aa653e861cbdfc01a48a81ab65b4443d2c271b4d5b166ec6900f5`
- complete Full assignment:
  `1759244777dfec07553fb991a775aa33ab7ffcea1edc60593ae0a6496be4ad0d`

固定入力はinventory 12,000 sources、M1 reservation/baseline 200 files、Full assignment
11,800 rows、91 batchesです。`p01-primary-01-full-pilot-b001`の12 rowsは受入済みで、
再制作・再QA・再checkpointしません。未制作の初期残量は11,788 files、90 batchesです。
固定ledger、content spine、M1、accepted Full artifactを編集・再割当・改名・移動・削除・
上書きしません。

次をread-only preflightとして実行し、`result: ok`、inventory 12,000、reservation 200、
assignment 11,800、baseline 200を確認してください。

./bin/full-ledger verify \
  --persona p01 \
  --inventory progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations progress/p01/full/m1-reservations-v3.jsonl \
  --assignment progress/p01/full/full-assignment-ledger.jsonl \
  --baseline progress/p01/full/manifests/m1-baseline.jsonl

`./bin/check-ready`のglobal `active_leases`は情報に留めます。他personaのleaseと並列の
working-tree変更は正常であり、他personaをshow／claim／recover／release／検証しません。
Git cleanを開始・完了条件にしません。p01自身のparentまたはscope leaseが既に存在する場合
だけ、何も変更せず所有sessionを報告して停止してください。

preflight合格後、一意なsession名でp01 parent leaseを正式CLIからclaimし、release tokenを
親プロセス内だけに保持してください。tokenはprompt、Subagentメッセージ、progress、
scratch、ログ、成果物、Gitへ絶対に記録しません。claim後に固定入力と現在状態をもう一度
照合してから制作を開始します。

## 初回状態と安全なresume判定

初回はp01 homeがexactly 212 regular filesで、現在manifestがaccepted pilot afterと
byte-for-byte一致することを確認します。再開時は212を固定条件にせず、次の規則だけで
受入済み集合を再構成してください。

1. accepted batchは、schemaが正しいtoken-free checkpointと、そこから参照される
   create-only before/after manifestがすべて存在するものに限定する。
2. pilot checkpointは常に受入済み集合へ含める。その他のbatch ID、scope、件数はcomplete
   assignmentと一致しなければならない。
3. 現在のhome manifestが、immutable M1 baseline 200 pathsとaccepted batch rowsの和に
   path単位で完全一致し、M1のbytes/SHA-256が不変であることを確認する。
4. checkpointのないassignment path、assignment外path、削除、symlink、非regular entry、
   accepted fileの変更が1件でもあれば、未受入deltaとして証跡を報告して停止する。
5. 最大file_countを持つaccepted after manifestのいずれかが現在manifestと完全一致することを
   確認する。同一acceptance groupのmemberは同じbefore/afterを参照してよい。

ファイルの存在だけ、ledgerの`state:"planned"`だけ、workerの自己申告だけでbatchを受入済みと
判断しません。create-only evidenceを削除・上書きして再開を装いません。

## 自律スケジューラ

残りが0になるまで次のacceptance-group loopを継続してください。定期的な進捗確認や通常の
QA修正についてユーザーへ判断を求めません。

1. accepted batch集合をcheckpointから再計算する。
2. 各scopeで、未受入batchのうち、batch外dependenciesがすべてaccepted済みである最初の
   batchを最大1件選ぶ。同一batch内dependencyはそのworkerがtopological orderで制作する。
3. 異なるscopeから選べるready batchを、実際に利用可能で品質を維持できるSubagent枠まで
   同じacceptance groupへ入れる。任意の固定少数上限は置かず、通常はreadyな全scopeを
   並列化する。ただし同一scopeは常に1 writer・1 batchで、同一scopeの2 batchを同時に
   起動しない。
4. `p01-full-wave-NNN`形式の一意なacceptance group IDを採番し、worker開始前に現在homeの
   cumulative before manifestを
   `progress/p01/full/manifests/<group-id>.before.jsonl`へcreate-onlyで生成する。既存pathを
   上書きせず、衝突時は内容を照合して未使用の次IDを選ぶ。
5. 親だけが、選択した各Rust scope IDのscope leaseをworker起動直前にclaimし、各tokenを
   親プロセス内だけに保持する。claim済みscopeだけを委譲する。
6. 1 batchを1 Subagentへ割り当てる。各workerはcomplete assignmentから自分のbatch IDの
   artifact rowsだけを抽出し、指定scope_path配下の指定pathだけを作成する。
7. 全workerの生成とQAが完了した後、親が全memberをQAする。不具合は可能な限り同じworkerへ
   具体的に差し戻し、groupのleaseを保持したまま修正・再QAを継続する。
8. 全member合格後だけ、1つのcumulative after manifestを
   `progress/p01/full/manifests/<group-id>.after.jsonl`へcreate-onlyで生成する。
9. complete master、immutable baseline、shared before/afterを指定し、memberごとに
   `--batch-id`を繰り返す1回の`./bin/full-ledger verify`でunionをatomicに検証する。
   memberを個別に先行acceptしない。
10. union verify合格後だけ、各memberの
    `progress/p01/full/checkpoints/<batch-id>.json`をcreate-onlyで記録する。各checkpointは
    schema `persona-corpus.full-batch-checkpoint/v1`、正確なscope/batch/count、M1不変、
    `acceptance_group`、共有before/after相対path、family別QA結果、UTC accepted_at、
    token-free noteを含める。
11. checkpointを再読込・検証してから、親が各scope leaseを正式CLIで通常releaseする。
    release成功と対象scope lease不在を確認して次groupへ進む。

異なるscope間にdependencyはありません。ledger内dependencyは同一scope内で解決され、循環は
ありません。したがって、上のready判定に従えばscope間並列とscope内逐次を両立できます。
初回の通常mass batchはpilotではなく、各scopeの最初のready batchです。
`p01-primary-01`では`p01-primary-01-full-b002`（317 files）が最初です。

groupのverify commandは概ね次の形です。`--batch-id`を選択member全件について渡します。

./bin/full-ledger verify \
  --persona p01 \
  --inventory progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations progress/p01/full/m1-reservations-v3.jsonl \
  --assignment progress/p01/full/full-assignment-ledger.jsonl \
  --baseline progress/p01/full/manifests/m1-baseline.jsonl \
  --before progress/p01/full/manifests/<group-id>.before.jsonl \
  --after progress/p01/full/manifests/<group-id>.after.jsonl \
  --batch-id <member-1> --batch-id <member-2> ...

## Workerへ必ず渡す規則

workerは他のworkerと同じ共有repositoryにいることを理解し、自分のbatch/scope以外を変更せず、
他者の変更をrevertしません。workerにはrelease tokenを渡さず、lease CLI、Git、canonical、
owner、runtime、root corpus manifest、他persona、他scope、assignment、checkpointを操作させません。

各workerはAGENTS.md、Full protocol/ledger spec、p01 persona brief、content spine、該当する
artifact skillを自身で完全に読みます。DOCXはDocuments、PDFはPDF、XLSXはSpreadsheets、
PPTXはPresentations、画像とscan-source artworkはImageGenで実ファイルとして制作します。
workerのtemporary source、render、montage、inspection outputはworker固有の
`scratch/p01/<scope-id>/<batch-id>/`だけに置き、final scopeへ混入させません。
`scratch/p01/full/<group-id>/`は親だけがaggregate QAへ使用し、worker間で共有しません。

content spineの組織、人物alias、project、date、ID、measure、terminologyを使い、同じ依存系の
CSV/structured data、Office、PDF、画像で数値・日付・IDを整合させます。同一内容の水増し、
空テンプレート、lorem ipsum、TODO/TBD、壊れた拡張子、秘密、資格情報、実在PII/PHI、実在の
非公開資料を作りません。batch内でも各artifactを用途・内容・構造のある独立した現実的な
ファイルにします。accepted artifactやM1は修正せず、未受入の今回selected pathだけは親QAに
合格するまで同じworkerが修正できます。

worker自身が、担当全件の存在、形式、構文、開封、dependency/content-spine整合を確認します。
skill対象はskill所定のrender-and-verifyを省略せず、PDF全ページ、DOCX全ページ、XLSX全sheetと
formula/type/error、PPTX全slideとoverflow、画像のpixel/dimension/原寸表示、scan PDFの画像面と
OCR対象性を確認します。worker報告はbatch ID、作成件数、family件数、QA結果、証跡path、失敗path
だけを含むtoken-free報告にします。

Python sourceの構文QAは、source textを読み込んで`ast.parse`または`compile(..., mode="exec")`を
メモリ内だけで実行します。final treeのPythonをimport／実行せず、`py_compile`、`compileall`、
`python -m py_compile`を使いません。`PYTHONDONTWRITEBYTECODE`だけへ依存しません。これらは
`__pycache__/*.pyc`をassignment外へ生成し得るためです。QA後、final treeに`__pycache__`または
`*.pyc`が0件であることを親とworkerの双方が確認します。

親はworker報告だけで受入せず、assignment path/family/scope、magic/parse/open、全skill render、
内容品質、依存整合、placeholder/secret/PII、unplanned path、exact SHA-256 duplicateを独立確認します。
QA用の一時成果物はGit管理対象にしません。

## 自律修正と停止条件

通常の生成失敗、構文エラー、render崩れ、数式エラー、overflow、内容重複、軽微なworker停止は
原因を診断し、同じscope leaseと同じworkerを優先して修正・再検証し、可能な限り次groupまで
自律継続してください。利用可能な他scopeの作業も安全な範囲で進めます。進捗報告のためだけに
停止しません。

次の安全条件に限り、追加変更やrecoverを行わず停止して、persona、parent session、scope、
worker session、batch/group、最後のaccepted checkpoint、現在file count、manifest差分、
残っている自分のleaseを具体的に報告してください。

- parentまたはscope release tokenを保持できない、token喪失、通常release失敗
- stale/unknown leaseがあり、named `lease recover`または`lease scope recover`が必要
- 固定inputのSHA、authority binding、ledger schema/完全性が不一致
- M1またはaccepted Fullの変更、assignment外file、symlink/nonregular、未受入deltaを検出
- dependencyを固定ledgerどおりに満たせず、ledger revisionが必要
- secret、credential、実在PII/PHIを検出し、安全な合成内容への修正では解消できない
- 必須skill/runtime/serviceが反復試行後も利用不能で、そのbatchを正しく完成できない
- 同じworkerで安全に再開できず、別workerを同一scopeへ重複起動する危険がある

recover、手動`_control`編集、固定ledgerの変更、未受入finalの削除、別workerの重複起動で
停止条件を迂回しません。group memberの1件でも未合格ならafter/checkpointを作らず、groupを
部分acceptしません。

## 完了ゲート

全90 remaining batchesを受入後、次をすべて検証してください。

- p01 homeはexactly 12,000 regular files、assignment外file 0、symlink/nonregular 0
- immutable M1 200 filesはpath、bytes、SHA-256が全件不変
- Full追加はpilotを含め11,800/11,800 rowsと完全一致
- pilot 1件とmass 90件、合計91 batch checkpointsが存在してmasterと一致
- scope/family/skill_required集計がmasterおよびcanonicalと一致
- actual SHA-256は12,000件すべて一意、exact duplicate 0
- 全構造検証とskill QAが合格し、placeholder、secret、credential、実在PII/PHIは0
- 最後のcumulative after manifestが現在homeとbyte-for-byte一致
- p01の全scope leaseを通常release済み

最後にp01 parent leaseを正式CLIで通常releaseし、p01 parentと全20 scope leaseが不在である
ことを確認します。他personaのleaseは触らず、global `active_leases=0`を要求しません。

このproductionタスクはgit add／commit／push、root corpus manifest更新、別branch/worktree、
Kioへの書込み、Kio index/history/search/chunk/performance評価を実行しません。実際に全完了した
場合だけ「p01 Full production完了」と報告し、最終件数、90/90 mass batches、91 checkpoints、
形式集計、manifest SHA-256、M1不変、QA集計、p01 lease不在を示してください。完了していない
場合は完了を主張せず、上の停止証跡と、安全に再開できる次batchだけを報告してください。
```
