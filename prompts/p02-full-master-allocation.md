# p02 Full master allocation task

> **Historical completed-allocation prompt. Do not rerun or replace the frozen
> accepted p02 master.**

次を **新しい** Codex Local task にそのまま貼り付けます。このtaskはp02の
allocationだけを固定するものであり、最終成果物を制作するtaskではありません。

```text
このpersonaCorpus Localタスクで、p02 Full mass productionの前提となる完全master
assignmentを作成・検証してください。作業rootはCodexで開いた現在のpersonaCorpus
repository rootのままにし、別machine由来のabsolute path、別worktree／別workspace／
別runtimeを使わず、git add／commit／pushを実行しません。

これは新しいp02 parent taskです。p01 Full master allocation、p11--p13 M1、および
他personaの通常の並列作業と同時に実行して構いません。他personaのleaseや未commit
変更は正常な並列状態として扱い、他personaをshow／recover／release／検証しません。
mass p02 productionはこのtaskの範囲外であり、coordinatorがこのmasterを受入するまで
開始しません。

最初にAGENTS.md、README.md、runbooks/COMMON_RULES.md、
runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、
runbooks/GIT_MANAGEMENT.md、runbooks/personas/p02-site-reliability-engineer.md、
progress/p02/m1-assignments.md、prompts/FULL_PARENT_TEMPLATE.md、
prompts/p02-full-master-allocation.mdを完全に読んでください。

これはallocation-only taskです。workspace/people以下のfinal fileを1 byteも変更せず、
Documents/PDF/Spreadsheets/Presentations/ImageGenのartifact productionも開始しません。
scope leaseは取得せず、artifact-production scope workerは起動しません。allocation draftだけを
異なるscopeごとにSubagentへ委譲してよいですが、そのSubagentにもleaseやfinal artifactを
扱わせません。p02に一意なparent sessionを使ってparent leaseだけを取得し、allocation中の
p02 artifact productionを排除してください。
release tokenはプロセス内だけに保持し、ファイル、prompt、ログ、progress、Subagent
メッセージへ記録しません。

parent lease取得前に、p02自身のparent/scope leaseがないこと、p02 homeが正確に200 regular
filesでsymlink・非regular entryなしであることを確認してください。p11--p13等が並列で
final filesを作成中の場合があるため、ここでglobal `./bin/corpus-manifest verify`は実行しません。
代わりに後述のp02 create-only baselineの固定count／bytes／SHA-256でp02だけを検証します。
必要ならroot manifestからp02 subsetだけを抽出して現在のp02 homeと比較してよいですが、
他personaのfinal artifactsは読まない、比較しない、評価しないでください。root manifestの
更新もしません。

次の受入済みp02 preflight値を固定値として照合してください。異なればp02 final driftまたは
不正な入力状態なのでallocationを開始せず停止・報告します。

- `progress/p02/m1-assignments.md` SHA-256:
  `a3557153096edd1ef42ecae7a9a83e9832972918390b0b94b180e6e2efbd065b`
- frozen machine-readable M1 ledger SHA-256:
  `bc7c60e57d3f5a56a645a9d4feae13f0d532c5e82f12427bb750eafb9f85f41b`
- create-only canonical inventory: 15,000 source（headerを含め15,001 JSONL records）、SHA-256
  `7d10f97949085247f27217f50b4fe4a135ec6079dd0e4cd62a56423cced20223`
- immutable M1 baseline: 200 files（headerを含め201 JSONL records）、合計5,987,337 bytes、
  SHA-256 `6030fddb66249a6f480ea34ccd08ded56402f2cfa6d9e365700b237e22968fc2`
- frozen reconciliation:
  `progress/p02/full/m1-reservation-reconciliation.jsonl` SHA-256
  `cb55ac00908354632520ae4fce6dae4c7c93bddf5ad129bfafcb18b61a56dbf7`
  （4 rows、policy `same-scope-global-family-balanced-v1`、plan/render/inventory/M1 ledger/
  baseline digest bindingあり）
- reconciled M1 reservations: 200 reservations（headerを含め201 JSONL records）、SHA-256
  `824c37cb9c0087f3eb5613648fd9618209b65749cf3adbc8969a2f28fc2f2ec7`

`full-ledger`自身も、このreconciliationをp02のcanonical repository path、
reconciliation ID、SHA-256でtrust-anchorしています。別pathのcopy／symlink、同じscope・familyで
件数が整合する別source mappingも承認済みrecordの代替にはなりません。

create-only方針です。出力pathが既存なら上書き、削除、置換せず、schema・row数・digest・内容を
照合してください。異なる既存内容、p02 lease、p02 final差分、必要helperの不在・失敗、または
不完全な検証を検出したら、final artifactを変更せず、通常release可能ならparent leaseを通常
releaseして停止・報告してください。recover、_controlの手編集、追加claimは実行しません。
`progress/p02/full/`と`progress/p02/full/manifests/`がなければdirectoryだけを作成して
構いません。ただし既存outputを上書きするためにdirectoryやfileを削除してはいけません。

現在、次の1--5のportable recordはすでに正規pathへcreate-onlyで実配置済みです。新しいtaskでは
各固定SHA・record数・schema・内容を照合して**再利用し、対応コマンドを再実行しません**。将来の
clone等で当該pathが本当に欠けている場合に限って、記載コマンドで1回だけ作成します。既存pathへ
コマンドを実行して意図的なcreate-only失敗を起こさないでください。今回新規作成する主対象は
6のcontent spineと7の14,800-row master assignmentです。

次のp02 Full preparation recordsを固定してください。

1. frozen M1 machine-readable ledger
   既存`progress/p02/m1-assignment-ledger.jsonl`を固定値と照合して再利用する。欠損時だけ
   `./bin/p02-m1-assignment --out <repo-root>/progress/p02/m1-assignment-ledger.jsonl`
   を使う。header + 200 artifact rows、schema `persona-corpus.m1-assignment/v1`、正確な
   scope／scope_path／family／physical_extension／home-relative path／`state:"frozen"`を
   固定する。Markdownや拡張子だけからfamilyを独自推測しない。
2. canonical source inventory
   既存`progress/p02/full/canonical-source-inventory.jsonl`を照合して再利用する。欠損時だけ
   `./bin/full-ledger inventory --persona p02 --out <repo-root>/progress/p02/full/canonical-source-inventory.jsonl`
   を使い、canonicalの15,000 sourceをcreate-onlyで固定する。
3. immutable M1 baseline
   既存`progress/p02/full/manifests/m1-baseline.jsonl`を照合して再利用する。欠損時だけ
   `./bin/full-ledger manifest --persona p02 --out <repo-root>/progress/p02/full/manifests/m1-baseline.jsonl`
   を、p02が正確に200 filesである状態で一度だけ作成する。header + 200 file rowsを確認する。
4. frozen M1 reconciliation
   `progress/p02/full/m1-reservation-reconciliation.jsonl` は受入済みのcreate-only recordであり、
   作り直し・編集・置換しない。SHA-256、headerのplan/render/inventory/M1 ledger/baseline digest、
   reconciliation ID `p02-m1-same-scope-family-balance-v1`、`same-scope-global-family-balanced-v1`、
   4件のsame-scope例外を確認する。これはlegacy M1の
   physical familyとcanonical source familyの差異を明示するためだけのrecordである。通常の
   reservation pairingはstrictのままであり、追加例外を作成してはいけない。
5. M1 reservations
   既存`progress/p02/full/m1-reservations.jsonl`を固定SHA・200 reservation・入力digest・4件の
   mappingと照合して再利用する。欠損時だけ
   `./bin/full-ledger reserve-m1 --persona p02 --inventory <inventory> --baseline <baseline>
   --m1-assignment <m1-ledger> --reconciliation <repo-root>/progress/p02/full/m1-reservation-reconciliation.jsonl
   --out <repo-root>/progress/p02/full/m1-reservations.jsonl`
   を使い、200 M1 paths／bytes／SHA-256をcanonical sourceに一意に予約する。
6. `progress/p02/full/content-spine.md`
   p02全20 scopeで一貫して再利用できる、完全合成のorganization、service、incident、date、
   ID、SLO/error budget、measure、terminology、cross-format relationshipを固定する。既存M1
   anchors（Northstar Transit Cloud、Checkout SVC-17、2026-07-13、error budget 2.4%、
   INC-260713）を保持・包含する。秘密、資格情報、実在PII、実在非公開資料を含めない。
7. `progress/p02/full/full-assignment-ledger.jsonl`
   schema `persona-corpus.full-addition-assignment/v1`、header 1行、`persona:"p02"`、
   `plan_digest:"sha256:f4e84efd49a98760733d628aaa44342dc7039cc845aced936e8a158eada95236"`、
   `render_digest:"sha256:b99f0d937f1476868c3e3121841ec258ff332bce7c57f92fa6193bcb3b84b2a8"`、
   `assignment_id:"p02-full-master-v1"`、`batch_id:"p02-full-master-allocation-v1"`、
   `artifact_count:14800`、`pilot:false`で固定する。inventory 15,000 sourceからfrozen M1
   reservation 200 sourceを除いた14,800 sourceを、1回ずつ完全に割り当てる。各rowは安定
   artifact_id、source_id、scope_id、scope_path、family、canonical_variant、
   canonical_extension、physical_extension、scope-localかつ一意なhome-relative path、
   batch_id、dependencies、content_spine_ids、skill_required、`state:"planned"`を持つ。

assignmentはcanonicalのscope_path内だけへ配置し、M1 path、path traversal、重複path、
ad hoc extension／family推測を禁止します。batchは同一scope内で通常100--500件とし、
PDF／DOCX／XLSX／PPTX／ImageGen画像およびscan PDFは視覚QA可能なより小さいbatchに分割して
構いません。CSV／structured factsをXLSX、DOCX、searchable PDF、PPTXの前提にし、
ImageGen PNGをscan PDFの前提にするdependencyを明示してください。dependencyはmaster
内で解決し、batch順序と矛盾してはいけません。skill_requiredはdocuments、pdf、
spreadsheets、presentations、imagegenの該当形式にだけ指定します。

allocation draftをSubagentへ並列委譲する場合は、各Subagentを異なるRust scope IDのledger
draftだけに限定します。これはartifact-production scope workerではありません。親だけが完全
masterを統合します。Subagentはfinal file、lease、canonical、root manifest、他scope draftを
編集しません。

統合後、`./bin/full-ledger verify`をp02 inventory、p02 M1 ledger、reservation、
immutable M1 baseline、frozen reconciliation、complete master assignmentへ`--reconciliation
<repo-root>/progress/p02/full/m1-reservation-reconciliation.jsonl`を必ず付け、`--before`／`--after`なしで実行し、
`result: ok`と14,800/14,800の完全割当を確認してください。加えてM1 baseline、reservation、
assignmentのfamily／scope／batch／skill_required集計、ledger SHA-256、p02 final artifact差分0、
root manifest p02 subset不変を確認します。

全検証後にp02 parent leaseを正式CLIで通常releaseし、自分のp02 parent/scope leaseが残って
いないことを確認してください。Git操作、root corpus manifest更新、Kio index/history/search/
chunk/performance評価、final artifact productionは実行しません。最後に作成・再利用したrecord、
検証結果、M1 200 files不変、14,800-row allocation、通常release、mass productionが
coordinator acceptance待ちであることを報告してください。
```
