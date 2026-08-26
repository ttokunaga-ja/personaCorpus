# p01 Full wave-001 remediation and autonomous resume

> **Historical resolved remediation. Do not execute against the accepted p01 home.**

Send the following only to the **existing p01 Full production Local task** that
still owns the named parent and twenty scope leases. Do not open a replacement
task: the existing parent must retain its in-memory release tokens and reuse its
existing Subagents.

```text
p01 Full productionの停止証跡を受入れました。同じp01親タスク、parent session、20 scope
lease、既存workerを維持したまま`p01-full-wave-001`を是正し、原子的受入後は元の
`prompts/p01-full-production.md`のschedulerへ戻ってFull最終完了まで自律継続してください。

この指示により、現在確認済みのassignment外Python bytecode 81件だけを、再生成可能なQA副作用
として限定除去することを明示承認します。対応する`.py`、M1、pilot、assignment内artifact、
manifest、checkpoint、他personaのファイルは削除・変更しません。recover、release、追加claim、
新worker、手動`_control`編集をcleanup前またはwave受入前に実行しません。

## 固定状態の再確認

最初にAGENTS.md、runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、
prompts/p01-full-production.mdを再読し、次が現在もすべて一致する場合だけ進めてください。

- parent session: `p01-full-mass-production-20260823T181827Z-98146`
- parent lease 1件、p01全20 scope lease、他のp01 lease 0
- scope worker session: `p01-full-wave-001-<scope-id>-worker`
- 親がparentと20 scopeのrelease tokenを現在もプロセス内に保持している
- `p01-full-wave-001.before.jsonl`は212 filesで、SHA-256
  `4dae33fac0543a2f692d55c5159e70d40f5d2708ae1429936ba16272db44affb`
- current home: 5,467 regular files、symlink 0、nonregular 0
- accepted 212 files: missing 0、bytes/SHA-256変更0
- selected wave: 20 batches、5,696 rows
- selected path present 5,174、missing 522、off-wave master path 0
- assignment外は下記の81 regular filesだけ
- wave-001 after manifestなし、wave checkpoint 0、checkpoint総数1

tokenが失われている、既存Subagentへfollow-upできない、lease sessionが異なる、ファイル集合が
上記から変化した、対象外のextraがある場合は、何も削除せずrecoverも行わず停止してください。

## 限定cleanupの承認範囲

対象は次の3ディレクトリ直下の`*.cpython-314.pyc`だけです。

- `workspace/people/p01-software-engineer/home/documents/work/product-alpha/api-contracts/__pycache__/`
  20 files
- `workspace/people/p01-software-engineer/home/documents/work/product-beta/architecture/__pycache__/`
  17 files
- `workspace/people/p01-software-engineer/home/meetings/engineering/__pycache__/`
  44 files

削除前に、対象が合計81、すべてregular file、suffixが正確に`.cpython-314.pyc`、3ディレクトリに
他file／子directoryがなく、各basenameに対応する親directoryの`.py`が1件存在することを再確認
してください。対応する81 `.py`はcomplete masterと現在waveの次batchだけに属し、不変であることを
確認します。対象一覧・bytes・SHA-256をtoken-free evidenceとして
`scratch/p01/full/p01-full-wave-001-remediation/`へ保存してください。

cleanup直前のcurrent homeを同scratch directoryのcreate-only manifestへ記録します。これは
remediation証跡であり、公式の
`progress/p01/full/manifests/p01-full-wave-001.before.jsonl`を削除・上書き・置換しません。
公式beforeはworker開始前のaccepted 212-file境界のまま保持します。

対象を事前に完全解決したリストから1 fileずつunlinkし、81件以外を削除しないでください。
再帰削除、`rm -rf`、親directoryへのglob削除、未検証の環境変数やpathを使いません。81件のunlink
成功後、3つの`__pycache__`が空であることを確認し、その空directoryだけを正確なabsolute pathで
`rmdir`します。非emptyならrmdirせず停止します。

cleanup後にcreate-only remediation-after manifestをscratchへ作り、precleanupとの差分が上記81
pathの削除だけであることを確認してください。次の状態を要求します。

- regular files: 5,386
- assignment外0、off-wave master path 0、symlink 0、nonregular 0
- accepted 212 filesとselected existing 5,174 filesはbytes/SHA-256不変
- current-wave missingは正確に522 filesのまま
- `__pycache__`と`*.pyc`は0

## Python QAの副作用防止

81個の対応Python sourceは、import、実行、`py_compile`、`compileall`、
`python -m py_compile`を使わず、textを読んで`ast.parse`または
`compile(source_text, filename, "exec")`をメモリ内で行うだけの構文QAへ切り替えてください。
`PYTHONDONTWRITEBYTECODE`だけへ依存しません。以後の全p01 workerにも同じ規則を渡します。

構文QA後に2回再計数し、`__pycache__`／`*.pyc`が0のままであることを確認します。同じ3箇所へ
同種のpycが再生成された場合は受入せず、1対1のselected source対応を再確認したうえで同じ限定
cleanupを行い、QA commandを修正します。別種類または別directoryのextraが1件でも生じた場合は
削除せず停止してください。

## 未生成2 batchの補完

新しいSubagentをspawnせず、既存の同一Subagent／worker sessionへfollow-upして、固定masterの
未生成pathだけを作成させてください。

- `p01-primary-01-full-b002`: 317 files
  worker session `p01-full-wave-001-p01-primary-01-worker`
- `p01-secondary-07-full-b001`: 205 files
  worker session `p01-full-wave-001-p01-secondary-07-worker`

各workerは自scopeの固定522 rows以外を作成せず、既存の5,174 selected filesとaccepted 212を
変更しません。該当skill、構造、syntax、content-spine、placeholder／secret／credential／PII、
duplicateをQAします。残り18 memberも既存成果物を再生成せず、親がassignment・内容・構造・
既存worker QA evidenceを再確認します。

全20 memberが合格するまで部分accept、公式after、checkpoint、scope releaseを行いません。
全件完成時の期待状態は次のとおりです。

- regular files: 5,908 = accepted 212 + selected wave 5,696
- current-wave missing 0、assignment外0、off-wave master path 0
- symlink 0、nonregular 0、`__pycache__` 0、`*.pyc` 0
- accepted 212不変、selected rows/path/family/scope完全一致
- exact SHA-256 duplicate 0、全QA合格

## wave-001の原子的受入

公式afterを作る前に、scratchへcreate-only candidate-after manifestを生成し、既存の公式before、
immutable baseline、complete master、下記20 batch IDを指定して`full-ledger verify`してください。
candidateが不合格なら公式after/checkpointを作らず、同じworkerで修正します。

選択20 batchは次のとおりです。

- `p01-primary-01-full-b002` 317
- `p01-primary-02-full-b001` 287
- `p01-primary-03-full-b001` 389
- `p01-primary-04-full-b001` 267
- `p01-primary-05-full-b001` 338
- `p01-primary-06-full-b001` 290
- `p01-primary-07-full-b001` 286
- `p01-primary-08-full-b001` 256
- `p01-primary-09-full-b001` 241
- `p01-primary-10-full-b001` 397
- `p01-primary-11-full-b001` 201
- `p01-primary-12-full-b001` 264
- `p01-secondary-01-full-b001` 240
- `p01-secondary-02-full-b001` 395
- `p01-secondary-03-full-b001` 372
- `p01-secondary-04-full-b001` 263
- `p01-secondary-05-full-b001` 283
- `p01-secondary-06-full-b001` 210
- `p01-secondary-07-full-b001` 205
- `p01-secondary-08-full-b001` 195

verifyは次の完全な形で行います。candidate pathはscratchの実pathへ置換します。

./bin/full-ledger verify \
  --persona p01 \
  --inventory progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations progress/p01/full/m1-reservations-v3.jsonl \
  --assignment progress/p01/full/full-assignment-ledger.jsonl \
  --baseline progress/p01/full/manifests/m1-baseline.jsonl \
  --before progress/p01/full/manifests/p01-full-wave-001.before.jsonl \
  --after <scratch-candidate-after-manifest> \
  --batch-id p01-primary-01-full-b002 \
  --batch-id p01-primary-02-full-b001 \
  --batch-id p01-primary-03-full-b001 \
  --batch-id p01-primary-04-full-b001 \
  --batch-id p01-primary-05-full-b001 \
  --batch-id p01-primary-06-full-b001 \
  --batch-id p01-primary-07-full-b001 \
  --batch-id p01-primary-08-full-b001 \
  --batch-id p01-primary-09-full-b001 \
  --batch-id p01-primary-10-full-b001 \
  --batch-id p01-primary-11-full-b001 \
  --batch-id p01-primary-12-full-b001 \
  --batch-id p01-secondary-01-full-b001 \
  --batch-id p01-secondary-02-full-b001 \
  --batch-id p01-secondary-03-full-b001 \
  --batch-id p01-secondary-04-full-b001 \
  --batch-id p01-secondary-05-full-b001 \
  --batch-id p01-secondary-06-full-b001 \
  --batch-id p01-secondary-07-full-b001 \
  --batch-id p01-secondary-08-full-b001

candidate verifyが`result: ok`、before 212、current batch 5,696、after 5,908となった場合だけ、
`progress/p01/full/manifests/p01-full-wave-001.after.jsonl`をcreate-onlyで生成し、candidateと
byte-for-byte一致させ、同じ完全commandを公式afterで再実行します。

公式verify合格後だけ、20個の
`progress/p01/full/checkpoints/<batch-id>.json`をcreate-onlyで記録します。各checkpointは正確な
scope／batch／assignment_count、`accepted_additions`、M1不変、
`acceptance_group:"p01-full-wave-001"`、共有before/after相対path、family別QA、UTC accepted_at、
token-free noteを含めます。20 checkpointを再読込し、総数がpilot 1 + wave 20 = 21/91、全件が
masterおよび共有manifestと一致することを確認してから、20 scope leaseを保持tokenで通常release
し、p01 scope lease 0を確認します。

parent leaseはここでreleaseしません。同じparent sessionのまま元の自律schedulerへ戻り、
`p01-full-wave-002`のshared beforeをwave-001 official afterから開始して、残る70 mass batchesを
依存順・scope間並列で処理します。通常エラーは同じworkerへ修正依頼し、Full 12,000 files、
91 checkpoints、p01全lease通常releaseまで進捗報告だけで停止しません。

release token喪失、既存workerへ安全にfollow-up不能、unexpected file、固定ledger／accepted file
変更、candidate/official verify不一致、通常scope release失敗が起きた場合だけ停止し、recover、
ledger変更、手動control編集、Git add／commit／push、Kio操作を行わず証跡を報告してください。
```
