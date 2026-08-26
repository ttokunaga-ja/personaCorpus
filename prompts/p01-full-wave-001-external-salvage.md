# p01 Full wave-001 external batch salvage and autonomous resume

> **Historical resolved salvage instruction. Do not execute against the accepted p01 home.**

Send the following only to the **existing p01 Full production Local task** that
still owns the named parent and twenty scope leases. Do not open a replacement
task. This is a one-time authorization for the exact misplaced 317-file leaf;
it is not permission to clean the surrounding external workspace.

```text
p01 Full productionの新しい停止証跡を受入れました。同じp01親タスク、parent session、20 scope
lease、release token、既存workerを維持したまま、repo外へ誤配置された
`p01-primary-01-full-b002`の317件を検証付きで正規repositoryへ救済してください。その後、
`p01-full-wave-001`を原子的に受入れ、元のschedulerへ戻ってFull最終完了まで自律継続します。

この指示により、下記のexact source leafに現在存在し、immutable master ledgerの
`p01-primary-01-full-b002`と一致する317 regular filesだけについて、次を明示承認します。

1. 正規repositoryの対応する317 destination pathへの非上書きcopy
2. path／bytes／SHA-256一致と全QA合格後、同じ外部source 317 filesだけの個別unlink
3. 317件削除後に空である場合だけ、exact source leaf自身を`rmdir`

copy検証が終わるまでsourceを削除しません。削除後も同一bytesは正規repository側に残るため、
成果物は失われません。外部workspaceの他file、他directory、source leafの親directory、
他persona、正規M1／pilot／accepted artifactを変更・削除する承認ではありません。

## 固定pathと状態

最初にAGENTS.md、runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、
prompts/p01-full-production.md、prompts/p01-full-wave-001-remediation.mdを再読してください。

source leafは、`personaCorpus`を含まない次のexact absolute pathです。

`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/workspace/people/p01-software-engineer/home/documents/work/product-alpha/architecture`

destination leafは、`personaCorpus`を含む次のexact absolute pathです。

`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p01-software-engineer/home/documents/work/product-alpha/architecture`

両方を`realpath`／`lstat`で解決し、上記とbyte-for-byte同じabsolute pathで、互いに異なり、
どのancestorにもsymlinkがないことを確認します。短縮path、相対path、未検証の環境変数、globで
代用しません。

次が現在もすべて一致する場合だけ進めてください。

- parent session: `p01-full-mass-production-20260823T181827Z-98146`
- parent lease 1件、p01全20 scope lease、他のp01 lease 0
- 親がparentと20 scopeの全release tokenを現在もプロセス内に保持している
- primary worker: `p01-full-wave-001-p01-primary-01-worker`
- secondary worker: `p01-full-wave-001-p01-secondary-07-worker`
- official `p01-full-wave-001.before.jsonl`: 212 files、SHA-256
  `4dae33fac0543a2f692d55c5159e70d40f5d2708ae1429936ba16272db44affb`
- 正規home: 5,386 regular files、symlink／nonregular／assignment外／off-wave／pyc／
  `__pycache__` 0
- accepted 212 filesとselected existing 5,174 filesはbytes／SHA-256不変
- selected wave 5,696 rows中、present 5,174、missing 522
- missingはprimary-01 batch 317とsecondary-07 batch 205だけ
- official wave afterなし、wave checkpoint 0、checkpoint総数1/91
- cleanup target evidence SHA-256:
  `777bb7646cda68401b3dbe90ef39990cafe8e151c01ac8ad24899bca333ebe02`
- postcleanup manifest SHA-256:
  `3fcf4fdeaa4fd0590dba403be5c4709075dd0c1a4713fbe55811d221ab34ea93`

上記はこの指示の初回状態です。token喪失、既存workerへfollow-up不能、lease session相違があれば、
copy・削除・recover・releaseを行わず停止してください。正規／外部状態が初回値と異なる場合は、
後述の「限定的な冪等resume」に完全一致する、この指示自身の部分実行状態だけを許容します。
それ以外のdriftでは何も変更せず停止します。

## 外部317件の再検査と証跡

immutable masterから`batch_id == "p01-primary-01-full-b002"`の317 rowsを抽出し、これだけを
expected authorityにします。この指示の初回状態では、外部source leafを再検査して次をすべて
要求します。すでにこの指示のoriginal-source manifestが存在する中断再開時は、ここで初回の
317件状態を要求せず、直後の「限定的な冪等resume」へ進んで許可された状態だけを検証します。

- exact 317 regular files、子directory 0、symlink 0、nonregular 0
- home-relative path集合が317 ledger rowsと完全一致し、missing／extra 0
- 全pathが`documents/work/product-alpha/architecture/`直下のledger path
- 合計bytes 151,677
- SHA-256は317件すべて一意
- 正規homeの既存5,386件とのexact SHA-256 duplicate 0
- ledgerのfamily／physical_extensionとfile magic／parse結果が一致
- secret、credential、実在PII、placeholder、routable IPの検出0

source setの各path、bytes、SHA-256、`st_dev`、`st_ino`、`nlink`、family、extensionをtoken-free
create-only JSONLとして
`scratch/p01/full/p01-full-wave-001-external-salvage/`へ保存し、そのmanifest SHA-256を記録
してください。外部sourceをこの段階で変更しません。

### 限定的な冪等resume

この指示のcopy／unlinkが中断した後に同じ既存親タスクで再開する場合だけ、上記scratchの
create-only original-source manifestを基準として次の3状態を許容します。manifest自体がない、
schema／317 ledger rows／bytes／SHA-256を検証できない、またはこの指示より前の状態から由来した
可能性がある場合はresumeしません。

1. **copy途中**: 外部source 317件がすべてoriginal manifestどおり残り、正規destinationには
   0--317件のbatch subsetだけが存在し、存在する各fileがsource manifestとbytes／SHA-256一致する。
   正規home countは`5,386 + copied_count`、selected missingは`522 - copied_count`である。
2. **unlink途中**: 正規destination 317件がすべてoriginal manifestどおり存在し、外部sourceには
   original manifestの0--317件のsubsetだけが同じbytes／SHA-256で残る。source leafのextra、
   symlink、nonregular、子directoryは0で、正規homeは5,703件である。
3. **外部cleanup完了**: 正規destination 317件がoriginal manifestどおり存在し、exact source
   leafは不存在、正規homeは5,703件である。

どの状態でも、accepted 212と元のselected 5,174はpostcleanup evidenceどおり不変、正規側の
batch以外の新規path0、assignment外／off-wave／pyc／symlink／nonregular 0、release token／lease
session不変を要求します。1件でもhash不一致、expected union外path、count矛盾があれば、既存copyを
削除・上書きせず、外部sourceも追加削除せず停止します。

copy途中ならhash一致済みdestinationを保持し、未copyのexplicit mappingだけをno-clobber copy
します。unlink途中なら正規317件を保持し、original manifestにある残存sourceだけを再検証して
unlinkします。cleanup完了なら外部操作を繰り返さずsecondary-07へ進みます。これはこの限定
salvage以外の未受入deltaを受け入れる一般的resume規則ではありません。

外部workspace上位には、このbatchと無関係なp01の別leafおよび他personaのfile／directoryが
存在します。source leaf外を削除、移動、改名、内容検査、manifest対象にしません。特に
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/workspace`、その`people`、persona home、
`documents`、`work`、`product-alpha`のいずれも再帰操作・削除しません。

## 正規leafへの非上書きcopy

正規destination leaf自体には既存M1／pilot等の非batch fileがあります。leaf directoryをrename、
replace、merge-deleteしません。初回copy前は317個のexact destination pathがすべて未存在で、
destinationの各ancestorがreal directoryであることを`lstat`で確認してください。冪等resume時に
存在を許すのは、original-source manifestとbytes／SHA-256一致する既copyのbatch subsetだけです。
それ以外の既存path、symlink、nonregular、path escapeがあれば何もcopyせず停止します。

事前解決した317 source→destination mappingを使い、親が1 fileずつcreate-exclusive／no-clobberで
bytesをcopyします。検証済みsource／destination leafのdirectory FDを保持し、必ずdescriptor-
relative `openat`、`O_NOFOLLOW`、destinationの`O_CREAT|O_EXCL`相当を使います。copy前後に
sourceを`fstat`し、regular、同一device/inode、`nlink == 1`、size／SHA-256がoriginal manifestと
一致することを要求します。`cp -R`、leaf全体の`mv`／rename、`rsync --delete`、recursive copy、
broad glob、既存destination上書きは使いません。各copy直後にsource／destinationのbytesと
SHA-256を照合します。

途中でcopyまたは照合に失敗した場合、sourceを1件も削除せず、正規側へcopy済みの未受入pathも
削除・上書きしません。原因が一時的で、限定的な冪等resumeのcopy途中状態を完全に再検証できる
場合だけ、未copyのexplicit mappingを再試行します。hash／path／inode不一致、原因不明、反復失敗
ならexact copied subsetとエラーを報告して停止します。recover、追加claim、新worker、rollback
目的の広域削除は行いません。同じpromptの再開時も上記冪等resumeだけを使います。

317件すべてのcopy成功後、正規homeについて次を要求します。

- regular files: 5,703 = 5,386 + 317
- batch 317 pathがすべて存在し、sourceとbytes／SHA-256一致
- selected present 5,491、missing 205（secondary-07だけ）
- assignment外／off-wave／symlink／nonregular／pyc／`__pycache__` 0
- accepted 212と既存selected 5,174は不変
- 正規home 5,703件のexact SHA-256 duplicate 0

親と既存primary workerが正規destination上の317件を全数QAしてください。このbatchは
`skill_required`なしですが、Markdown／text／JSONL、Python／Rust／TypeScript、JSON／YAML／
XML／SQL、CSV／TSV、HTML／EML、IPYNB、PCAPを各形式の正規parser／magicで検証し、content
spine、dependency、用途・内容品質、secret／PII／placeholder、予約済みnetwork addressだけの
使用を確認します。Pythonはimport、実行、`py_compile`、`compileall`を使わず、textへの
`ast.parse`またはメモリ内`compile(..., "exec")`だけを使用し、pycを生成しません。

QAで修正が必要なら、外部sourceを削除・変更せず停止してfindingを報告してください。317件が
sourceとbyte一致したまま全QA合格した場合だけ、次の限定source cleanupへ進みます。

## 外部source 317件だけの限定削除

通常のunlink開始直前には外部source manifestを再生成し、最初のcreate-only manifestと
path／bytes／SHA-256が完全一致すること、正規destination 317件も同じbytes／SHA-256で存在する
ことを再確認します。unlink途中からの冪等resumeでは、外部sourceの残存setがoriginal manifestの
正確なsubsetであり、正規317件がすべてoriginal manifestと一致することを要求します。sourceに
extra、変更、symlink、nonregularが1件でもあれば外部fileを削除せず停止します。

検証済みmanifestのexplicit absolute path listから、外部source filesだけを1件ずつunlinkして
ください。通常開始時は317件、冪等resume時はmanifestどおり残存するsubsetだけです。保持した
source leaf directory FDに対する`fstatat(..., AT_SYMLINK_NOFOLLOW)`相当で、unlink直前にも
regular、同一device/inode、`nlink == 1`、size／SHA-256を再確認します。`rm -rf`、recursive
deletion、directory glob、親directory指定、未検証pathを使いません。部分失敗時は状態を再検証し、
限定的なunlink途中resumeに完全一致する場合だけ残存explicit pathsを再試行します。不一致、原因
不明、反復失敗なら削除済み／残存pathを報告して停止します。正規destination fileは削除しません。

original manifestの317 sourceがすべてunlink済みになった後、exact source leafが空であることを
確認します。検証時から保持したsource leaf親directory FDと親のdevice/inodeを再照合し、
`unlinkat(parent_fd, "architecture", AT_REMOVEDIR)`相当で、その空leaf自身だけをrmdirします。
空でない、親identityが変わった、またはFD相対削除を使用できない場合はleafをrmdirせず停止します。
source leafより
上の`product-alpha`、`work`、`documents`、home、persona directory、`people`、external
`workspace`は、空に見えてもrmdirしません。

削除receiptは元の317 path／bytes／SHA-256、正規copy確認、unlink結果、source leaf rmdir結果を
token-freeでscratchへcreate-only保存します。終了時にsource leaf不存在、正規317件存在、正規
home 5,703件、他の正規file不変を再確認します。

## secondary-07の205件とwave-001受入

外部cleanup合格後、同じ既存primary workerへ正規317件の最終確認結果を返させます。次に、新しい
Subagentをspawnせず、既存`p01-full-wave-001-p01-secondary-07-worker`へfollow-upして
`p01-secondary-07-full-b001`の未生成205 rowsだけを作成させてください。

workerへ、作業rootがexact
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`、final rootがその配下の
`workspace/people/p01-software-engineer/home`であることを、write前に`pwd`／`realpath`で確認
させます。repo root外へのfallback、短縮absolute path、別workspaceを禁止します。

205件の生成・worker QA・親QA後、次を要求します。

- regular files: 5,908 = accepted 212 + selected wave 5,696
- selected present 5,696、missing 0
- assignment外／off-wave／symlink／nonregular／pyc／`__pycache__` 0
- accepted 212と全selected artifactの配置／family／scope／内容が固定masterと一致
- exact SHA-256 duplicate 0、全20 memberのQA合格

ここまでofficial after、checkpoint、scope releaseを行いません。全件合格後は
`prompts/p01-full-wave-001-remediation.md`の「wave-001の原子的受入」に記載されたexact 20
batch IDと完全verify commandをそのまま使用します。まずscratch candidate-afterを作り、
before 212、current batch 5,696、after 5,908、`result: ok`を要求します。合格時だけofficial
`p01-full-wave-001.after.jsonl`をcreate-only生成し、candidateとbyte-for-byte一致させて再verify
します。

公式verify合格後だけ20 checkpointをcreate-only生成・再読込し、checkpoint総数21/91を確認して
20 scope leaseを保持tokenで通常releaseします。parent leaseはreleaseせず、同じparent sessionで
元のschedulerへ戻り、wave-002以降の残る70 batchesをFull 12,000 files／91 checkpoints／p01全
lease通常releaseまで自律継続してください。

source/destination drift、冪等resumeでも解消不能なcopy／unlink部分失敗、QA不合格、unexpected
path、release token喪失、既存worker follow-up不能、candidate／official verify不一致、通常scope
release失敗時だけ停止し、
recover、追加claim、手動`_control`編集、ledger変更、Git add／commit／push、root manifest更新、
Kio操作を行わず証跡を報告してください。
```
