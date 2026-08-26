# p02 Full mass production task

> **Historical completed-production prompt. Do not run against the current
> 15,000-file accepted p02 home without a separately approved revision.**

Paste the text below into **one new Codex Local task** opened directly at the
`personaCorpus` repository root. Do not continue the completed allocation-only
task. This prompt is also safe to reuse after an accepted p02 Full checkpoint.

```text
このLocalタスクでp02（site reliability engineer）のFull mass physical-artifact
productionを担当し、既存M1 200件を保持したままFull追加14,800件、最終15,000 filesまで、
通常の制作・QA・差戻しについてユーザー確認を求めず自律的に継続してください。

これは完了済みのallocation taskとは別の、新しいproduction親タスクです。作業rootは
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`を直接使用するLocal環境とし、
別worktree、別workspace、別runtime、別PC bundleを作りません。この同じpromptは、受入済み
checkpointからの再開にも使用できます。

## Authorityと固定入力

最初に次を完全に読んでください。

- `AGENTS.md`
- `README.md`
- `runbooks/COMMON_RULES.md`
- `runbooks/FULL_PRODUCTION_PROTOCOL.md`
- `runbooks/FULL_LEDGER_SPEC.md`
- `runbooks/SESSION_HANDOFF.md`
- `runbooks/personas/p02-site-reliability-engineer.md`
- `progress/p02/full/content-spine.md`
- `progress/p02/full/full-assignment-ledger.jsonl`
- 既存の`progress/p02/full/checkpoints/*.json`（存在する場合）

accepted Rust plan、materialization record、workspace ownerだけをcanonical authorityとし、
canonicalを再生成・再解釈しません。次のportable inputを開始時にSHA-256照合し、再生成・編集・
置換・移動・削除しないでください。

- human M1 assignment display:
  `a3557153096edd1ef42ecae7a9a83e9832972918390b0b94b180e6e2efbd065b`
- frozen M1 assignment ledger:
  `bc7c60e57d3f5a56a645a9d4feae13f0d532c5e82f12427bb750eafb9f85f41b`
- canonical source inventory:
  `7d10f97949085247f27217f50b4fe4a135ec6079dd0e4cd62a56423cced20223`
- immutable M1 baseline:
  `6030fddb66249a6f480ea34ccd08ded56402f2cfa6d9e365700b237e22968fc2`
- frozen M1 reconciliation:
  `cb55ac00908354632520ae4fce6dae4c7c93bddf5ad129bfafcb18b61a56dbf7`
- reconciled M1 reservations:
  `824c37cb9c0087f3eb5613648fd9618209b65749cf3adbc8969a2f28fc2f2ec7`
- content spine:
  `c2ad3d9e5eb95f9657b0c4ace882504e3ea0d0580790792197e8b5b0a12a775f`
- complete Full assignment:
  `deb618bdd9d7452ea93936494a5b0caf2c1adb39e454aeb12cf1eb79ab806e91`

固定状態はinventory 15,000 sources、M1 reservation/baseline 200 files、Full assignment
14,800 rows、85 batchesです。p02にはpilotまたは事前受入済みFull batchはありません。
初回開始時のFull追加残量は14,800 files／85 batches、最終目標は15,000 filesです。

reconciliationはID `p02-m1-same-scope-family-balance-v1`、policy
`same-scope-global-family-balanced-v1`の受入済み4-row例外だけです。追加例外、別mapping、
copy、symlink、別pathを作りません。`full-ledger`のtrust anchorは正準absolute pathを要求する
ため、すべてのverifyで`--reconciliation`へ必ず次のexact pathを渡してください。相対pathは
同じファイルを指していても不合格なので使用しません。

`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/m1-reservation-reconciliation.jsonl`

次をread-only preflightとして実行し、`result: ok`、inventory 15,000、reservation 200、
assignment 14,800、baseline 200を確認してください。

./bin/full-ledger verify \
  --persona p02 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/manifests/m1-baseline.jsonl \
  --reconciliation /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/m1-reservation-reconciliation.jsonl

`./bin/check-ready`のglobal `active_leases`は情報に留めます。他personaのleaseと並列の
working-tree変更は正常であり、他personaをshow／claim／recover／release／検証しません。
Git cleanやglobal `active_leases=0`を開始・完了条件にしません。p02自身のparentまたはscope
leaseがすでに存在する場合だけ、何も変更せず所有sessionを報告して停止してください。

preflight合格後、一意なsession名でp02 parent leaseを正式CLIからclaimし、release tokenを
親プロセス内だけに保持してください。tokenはprompt、Subagentメッセージ、progress、scratch、
ログ、成果物、Gitへ絶対に記録しません。claim後に固定入力とp02現在状態を再照合して制作を
開始します。

## 初回状態と安全なresume判定

初回はp02 homeがexactly 200 regular files、symlink／nonregular 0で、現在manifestがimmutable
M1 baselineとbyte-for-byte一致することを確認します。既存checkpointがある再開時は200を固定
条件にせず、次だけで受入済み集合を再構成してください。

1. accepted batchは、schemaが正しいtoken-free checkpointと、そこから参照されるcreate-only
   before/after manifestがすべて存在するものに限定する。
2. checkpointのbatch ID、scope、件数はcomplete assignmentの85 batchのいずれかと正確に
   一致し、同一batchのcheckpointは1件だけでなければならない。
3. 現在home manifestが、immutable M1 baseline 200 pathsとaccepted batch rowsの和にpath単位で
   完全一致し、M1とaccepted Fullのbytes/SHA-256が不変であることを確認する。
4. checkpointのないassignment path、assignment外path、削除、symlink、非regular entry、
   accepted fileの変更が1件でもあれば、未受入deltaとして証跡を報告して停止する。
5. accepted checkpointが0件なら現在manifestはM1 baselineと一致する。1件以上なら最大
   file_countを持つaccepted after manifestのいずれかが現在manifestと完全一致する。同じ
   acceptance groupのmemberは同じbefore/afterを参照してよい。

ファイルの存在、ledgerの`state:"planned"`、worker自己申告だけで受入済みと判断しません。
create-only evidenceを削除・上書きして再開を装いません。

## 自律スケジューラ

残りが0になるまで次のacceptance-group loopを継続し、進捗報告や通常のQA修正だけを理由に
停止しないでください。

1. accepted batch集合をcheckpointから再計算する。
2. 各scopeで未受入batchのうち、batch外dependenciesがすべてaccepted済みである最初のbatchを
   最大1件選ぶ。同一batch内dependencyはworkerがtopological orderで制作する。
3. 異なるscopeから選べるready batchを、実際に利用可能で品質を維持できるSubagent枠まで同じ
   acceptance groupへ入れる。任意の固定少数上限は置かず、通常はreadyな全scopeを並列化する。
   同一scopeは常に1 writer・1 batchとし、同一scopeの2 batchを同時起動しない。
4. `p02-full-wave-NNN`形式の一意なgroup IDを採番し、worker開始前に現在homeのcumulative
   before manifestを`progress/p02/full/manifests/<group-id>.before.jsonl`へcreate-onlyで
   生成する。既存pathを上書きせず、衝突時は既存内容を照合して未使用の次IDを選ぶ。
5. 親だけが、選択した各Rust scope IDのscope leaseをworker起動直前にclaimし、tokenを親
   プロセス内だけに保持する。claim済みscopeだけを委譲する。
6. 1 batchを1 Subagentへ割り当てる。workerはcomplete assignmentから自分のbatch IDのrowsだけを
   抽出し、指定scope_path配下の固定pathだけを作成する。
7. 全workerの生成・QA後に親が全memberを独立QAする。不具合は同じworkerへ具体的に差し戻し、
   group leaseを保持したまま修正・再QAする。
8. 全member合格後、まず親専用`scratch/p02/full/<group-id>/`へ一意なcreate-only
   candidate-after manifestを生成する。candidateを既存の公式before、complete master、
   immutable baseline、reconciliation、全member batch IDでatomic verifyする。
9. candidate verifyが`result: ok`の場合だけ、公式
   `progress/p02/full/manifests/<group-id>.after.jsonl`をcreate-onlyで生成し、candidateと
   byte-for-byte一致させ、公式afterでも同じatomic verifyを再実行する。candidate不合格時は
   公式after/checkpointを作らず、同じworkerで修正し、別の一意なcandidate revisionを使う。
10. 公式verify合格後だけ、各memberの
    `progress/p02/full/checkpoints/<batch-id>.json`をcreate-onlyで記録する。各checkpointは
    schema `persona-corpus.full-batch-checkpoint/v1`、正確なscope/batch/count、M1不変、
    `acceptance_group`、共有before/after相対path、family別QA、UTC accepted_at、token-free
    noteを含める。
11. checkpointを再読込してmaster／manifestとの一致を確認後、親が各scope leaseを保持tokenで
    正式CLIから通常releaseする。release成功と対象scope lease不在を確認して次groupへ進む。

異なるscope間のdependencyは0で、1,480 dependency edgeはすべて同一scope内かつforward
dependency 0です。各scopeの初回ready batchは正確に
`<scope-id>-b001-foundation`（例: `p02-primary-01-b001-foundation`）です。その後はledger内
dependencyとbatch順に従います。

group verifyは、`--batch-id`を全memberについて繰り返し、次の完全な形で実行してください。
`--reconciliation`は必ず下記absolute canonical pathのままにします。

./bin/full-ledger verify \
  --persona p02 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/manifests/m1-baseline.jsonl \
  --reconciliation /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/m1-reservation-reconciliation.jsonl \
  --before /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p02/full/manifests/<group-id>.before.jsonl \
  --after <candidate-or-official-after-absolute-path> \
  --batch-id <member-1> --batch-id <member-2> ...

## Workerへ必ず渡す規則

workerは共有repository上で、自分のbatch/scope以外を変更せず他者の変更をrevertしません。
workerにはrelease tokenを渡さず、lease CLI、Git、canonical、owner、runtime、root corpus manifest、
他persona、他scope、assignment、manifest、checkpointを操作させません。

各workerはAGENTS.md、Full protocol/ledger spec、p02 persona brief、content spine、該当artifact
skillを完全に読みます。DOCXはDocuments、searchable PDFはPDF、XLSXはSpreadsheets、PPTXは
Presentations、PNG画像はImageGenで実ファイルとして制作します。p02のcanonical `pdf_scan`は
0件なのでscan PDFを作成せず、画像をscan PDFへ派生させません。temporary source、render、
montage、inspection outputはworker固有の`scratch/p02/<scope-id>/<batch-id>/`だけに置き、final
scopeへ混入させません。親のaggregate QAは`scratch/p02/full/<group-id>/`だけを使います。

content spineのNorthstar Transit Cloud、Checkout SVC-17、2026-07-13、error budget 2.4%、
INC-260713と、service／incident／SLO／measure／terminologyを一貫して使います。同じ依存系の
CSV/structured data、Office、PDF、画像で数値・日付・IDを整合させます。同一内容の水増し、
空テンプレート、lorem ipsum、TODO/TBD、壊れた拡張子、秘密、資格情報、実在PII、顧客データ、
実在の非公開資料、routable IPを作りません。network表現にはdocumentation用の予約済みaddress
だけを使います。各artifactを用途・内容・構造のある独立した現実的ファイルにします。

accepted artifactとM1は修正せず、未受入の今回selected pathだけは親QAに合格するまで同じ
workerが修正できます。worker自身が担当全件の存在、形式、構文、開封、dependency／content
spine整合を確認します。skill対象はskill所定のrender-and-verifyを省略せず、searchable PDF
全ページとtext layer、DOCX全ページ、XLSX全sheetとformula/type/error、PPTX全slideとoverflow、
PNGのpixel/dimension/原寸表示を確認します。worker報告はbatch ID、作成件数、family件数、QA
結果、token-free証跡path、失敗pathだけを含めます。

Python sourceの構文QAはsource textを読んで`ast.parse`または
`compile(source_text, filename, "exec")`をメモリ内だけで実行します。final treeのPythonを
import／実行せず、`py_compile`、`compileall`、`python -m py_compile`を使いません。
`PYTHONDONTWRITEBYTECODE`だけへ依存しません。QA後、final treeに`__pycache__`／`*.pyc`が0件で
あることをworkerと親の双方が確認します。

親はworker報告だけで受入せず、assignment path/family/scope、magic/parse/open、全skill render、
内容品質、dependency、M1／accepted Full不変、placeholder／secret／PII、unplanned path、
exact SHA-256 duplicateを独立確認します。QA用一時成果物はGit管理対象にしません。

## 固定集計

85 batch、最大488 files、Full追加14,800件のfamily集計は次のとおりです。

- `md 2,960`; `txt_log 3,256`; `code 2,220`; `structured_text 2,960`
- `csv_tsv 740`; `html_eml 444`; `pdf_text 592`; `pdf_scan 0`
- `docx 296`; `xlsx 148`; `pptx 148`; `image 296`
- `ipynb 0`; `media 0`; `domain_binary 740`

`skill_required`はDocuments 296、PDF 592、Spreadsheets 148、Presentations 148、ImageGen
296、skill不要13,320です。M1とFull追加の最終合計はcanonicalどおり15,000 filesです。
各groupと最終ゲートでこの集計をmasterから再計算し、promptの数字だけをauthorityにしません。

## 自律修正と停止条件

通常の生成失敗、構文エラー、render崩れ、数式エラー、overflow、内容重複、軽微なworker停止は、
原因を診断して同じscope leaseと同じworkerで修正・再検証し、次groupへ自律継続してください。
利用可能な他scopeも安全な範囲で進め、進捗報告のためだけに停止しません。

次の安全条件に限り、追加変更やrecoverをせず停止し、persona、parent session、scope、worker
session、batch/group、最後のaccepted checkpoint、現在file count、manifest差分、残存する自分の
leaseを具体的に報告してください。

- parent／scope release token喪失、通常release失敗
- stale／unknown p02 leaseがあり、named recoverが必要
- 固定input SHA、reconciliation trust anchor、ledger schema／完全性の不一致
- M1またはaccepted Full変更、assignment外file、symlink／nonregular、未受入delta
- dependencyを固定ledgerどおり満たせず、ledger revisionが必要
- secret、credential、実在PII、顧客データを安全な合成内容へ修正できない
- 必須skill/runtime/serviceが反復試行後も利用不能
- 同じworkerで安全に再開できず、同一scopeへ重複workerを起動する危険がある

recover、手動`_control`編集、固定ledger／reconciliation変更、未受入final削除、別workerの重複
起動で停止条件を迂回しません。group memberの1件でも未合格なら公式after/checkpointを作らず、
groupを部分accept/releaseしません。

## 完了ゲート

85 batchをすべて受入後、次を検証してください。

- p02 home exactly 15,000 regular files、assignment外0、symlink／nonregular 0
- immutable M1 200 filesのpath、bytes、SHA-256が全件不変
- Full追加14,800/14,800 rowsが完全一致
- Full checkpointが85/85存在し、scope/batch/countとshared manifestsがmasterに一致
- scope/family/skill_required集計がmasterおよびcanonicalと一致
- actual SHA-256は15,000件すべて一意、exact duplicate 0
- 全構造検証とskill QA合格、placeholder、secret、credential、実在PII、routable IP 0
- 最後のcumulative official after manifestが現在homeとbyte-for-byte一致
- p02の全scope leaseを通常release済み

最後にp02 parent leaseを正式CLIで通常releaseし、p02 parentと全20 scope leaseが不在であることを
確認します。他personaのleaseは触らず、global `active_leases=0`を要求しません。

このproduction taskはgit add／commit／push、root corpus manifest更新、別branch/worktree、Kioへの
書込み、Kio index/history/search/chunk/performance評価を実行しません。実際に全完了した場合だけ
「p02 Full production完了」と報告し、最終15,000件、85/85 batches/checkpoints、形式集計、最終
manifest SHA-256、M1不変、QA集計、p02 lease不在を示してください。未完了なら完了を主張せず、
停止証跡と安全に再開できる次batchだけを報告してください。
```
