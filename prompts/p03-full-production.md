# p03 Full mass production task

> **Historical completed-production prompt. Do not run against the current
> 10,000-file accepted and remote-pushed p03 home without a separately approved revision.**

Paste the text below into **one new Codex Local task** opened directly at the
`personaCorpus` repository root.  Do not continue the allocation task that
created this ledger.  This prompt is safe both for an initial production run
and for resuming from accepted p03 Full checkpoints.

```text
このLocalタスクでp03（Security GRC Analyst）のFull mass physical-artifact
productionを担当してください。既存M1 200件を一切変えず、Full追加9,800件を制作・受入し、
最終10,000 regular filesまで、通常の制作、QA、差戻し、再開判定をユーザー確認なしで自律的に
継続してください。これは防御的で完全に合成されたコーパス制作です。実在環境へのアクセス、
攻撃・侵入・exploit手順、資格情報、秘密、実在PII/PHI、実在顧客・未公開資料を作成しません。

これはallocation taskとは別の新しいproduction親タスクです。rootは
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`を直接使い、別worktree、
別workspace、別runtime、別PC bundleを作りません。同じpromptは受入済みcheckpointからの
再開にも使えます。

## 最初に読むものと固定authority

最初に次を完全に読んでください。

- `AGENTS.md`
- `README.md`
- `runbooks/COMMON_RULES.md`
- `runbooks/FULL_PRODUCTION_PROTOCOL.md`
- `runbooks/FULL_LEDGER_SPEC.md`
- `runbooks/SESSION_HANDOFF.md`
- `runbooks/personas/p03-security-grc-analyst.md`
- `progress/p03/full/content-spine.md`
- `progress/p03/full/full-assignment-ledger.jsonl`
- 存在する場合は`progress/p03/full/checkpoints/*.json`

accepted Rust plan、materialization record、workspace ownerだけをcanonical authorityとし、
canonicalを再生成・再解釈しません。次のportable inputは開始時にSHA-256を照合し、
再生成、編集、置換、移動、削除しません。

- human M1 assignment display: `30dd24c19194ff422501bd1fb31f46cefa10a455c295028bfe904d9936233bbc`
- frozen M1 assignment ledger: `65915c2f5ffffe99e2a38b3ca4850b28f26cad1130640478acec2abf657f1802`
- canonical source inventory: `e9cc92078770d5f031d80ddd456a0a59c6c1150f9d82cf180a90f604ebd90d68`
- immutable M1 baseline: `4ddc3e4e6edc755b1d031c8095377402717c10863d8fb4de6ff20fcc36121350`
  (`20,472,981` bytes)
- frozen M1 reconciliation: `5e723463a6322aad8f0c2c8248c851e33797504603dfbd7b0524e69168100d5d`
- reconciled M1 reservations: `5dc9c7c18d615f978085b484b166a3857365a4acb5067c08b48794e9a6943f1f`
- content spine: `9347a607d2a27b1677409ec4646a90d790ca722e42a32a3be8c24e191fcbeb52`
- complete Full assignment: `62440cc218f205ec8114fc90a666295a43062d371c48fc27fd98ed93a6a20501`
- isolated p03 ledger CLI (`bin/p03-full-ledger`):
  `0023ba39fb47f91ee8fd3dff31ac8a9c15542e4f868714d99d595690eb099759`

固定状態はinventory 10,000 sources、M1 reservation/baseline 200 files、Full assignment
9,800 rows、85 batches、dependency edge 3,817（すべてstrictly earlier batch）です。
初回のFull追加残量は9,800 files／85 batches、最終目標は10,000 filesです。

**常に `./bin/p03-full-ledger` を使います。共有 `./bin/full-ledger` は使わず、両CLIを
変更しません。** p03の隔離CLIは、他personaの稼働中Full waveと共有trust anchorを干渉させ
ないための明示的なproduction boundaryです。

reconciliationはID `p03-m1-same-scope-family-balance-v1`だけです。追加例外、別mapping、
copy、symlink、別pathを作りません。すべてのverifyで`--reconciliation`に必ず次のexact
absolute pathを渡してください。相対pathは使いません。

`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/m1-reservation-reconciliation.jsonl`

read-only preflightでは`result: ok`、inventory 10,000、reservation 200、assignment 9,800、
baseline 200を確認してください。

```sh
./bin/p03-full-ledger verify \
  --persona p03 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/manifests/m1-baseline.jsonl \
  --reconciliation /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p03/full/m1-reservation-reconciliation.jsonl
```

`./bin/check-ready`のglobal `active_leases`は情報に留めます。他personaのleaseと並列working-tree
変更は正常なので、他personaをshow/claim/recover/release/検証しません。Git cleanやglobal
`active_leases=0`は開始・完了条件にしません。p03自身のparent/scope leaseがすでに存在する
場合だけ、何も変更せずowner sessionを報告して停止します。preflight合格後、一意なsession名で
p03 parent leaseを正式CLIからclaimし、release tokenは親プロセス内だけに保持します。tokenを
prompt、Subagentメッセージ、progress、scratch、log、成果物、Gitへ記録しません。

## 初回状態とresume判定

初回はp03 homeがexactly 200 regular files、symlink/nonregular 0で、現在manifestがimmutable
M1 baselineとbyte-for-byte一致することを確認します。checkpointがある再開では、次だけで
accepted集合を再構成します。

1. accepted batchは、正しいtoken-free checkpointと参照先のcreate-only before/after manifestが
   全てあるものだけです。
2. checkpointのbatch ID/scope/countはmasterの85 batchのいずれかと完全一致し、同一batchの
   checkpointは1件だけです。
3. 現在home manifestは、M1 baseline 200 pathsとaccepted batch rowsの和にpath単位で完全一致し、
   M1とaccepted Fullのbytes/SHA-256が不変です。
4. checkpointのないassignment path、assignment外path、削除、symlink、nonregular、accepted
   file変更が1件でもあれば、未受入deltaの証跡を報告して停止します。
5. checkpointが0件なら現在manifestはM1 baselineと一致します。1件以上なら最大file_countを
   持つaccepted after manifestのいずれかが現在manifestと完全一致します。同一acceptance group
   のmemberは同じbefore/afterを参照できます。

存在、`state:"planned"`、worker自己申告だけで受入済みと判断せず、create-only evidenceを削除・
上書きして再開を装いません。

## 自律スケジューラと原子的受入

残りが0になるまで次のacceptance-group loopを継続します。進捗報告や通常のQA修正のためには
停止しません。

1. checkpointからaccepted batch集合を再計算します。
2. 各scopeで、未受入batchのうちbatch外dependencyが全てaccepted済みである最初のbatchを最大1件
   選びます。masterの3,817 edgeはすべて同一scopeかつ**strictly earlier batch**です。同一batch
   dependencyも作らず、workerはそのbatch内の生成順もdependencyを満たします。
3. 異なるscopeのready batchを、利用可能で品質を維持できるSubagent枠まで同じgroupへ入れます。
   固定少数上限を置かず通常はreadyな全scopeを並列化します。同一scopeは常に1 writer/1 batchです。
4. `p03-full-wave-NNN`の一意なgroup IDを採番し、開始前cumulative before manifestを
   `progress/p03/full/manifests/<group-id>.before.jsonl`にcreate-onlyで生成します。
5. 親だけが選択scopeのscope leaseをworker起動直前にclaimします。tokenを親内だけに保持し、
   claim済みscopeだけ委譲します。1 batchは1 Subagentです。
6. workerはmasterの自batch rowsだけを読み、固定scope_path配下にだけ作成します。workerへtokenを
   渡さず、lease CLI、Git、canonical、owner、runtime、root corpus manifest、他persona/他scope、
   assignment、manifest、checkpointを操作させません。
7. 全workerの生成・QA後、親が各memberを独立QAします。不具合は同じworkerへ差し戻し、group leaseを
   保持して修正・再QAします。
8. 合格後だけ親scratch `scratch/p03/full/<group-id>/`に一意なcandidate-after manifestを
   create-onlyで生成し、before/master/baseline/reconciliation/全member batch IDでatomic verifyします。
9. candidateが`result: ok`の場合だけ公式afterを
   `progress/p03/full/manifests/<group-id>.after.jsonl`へcreate-onlyで生成し、candidateとbyte一致、
   公式afterでも同じverifyを行います。不合格なら公式after/checkpointを作りません。
10. 公式verify合格後だけ、memberごとの
    `progress/p03/full/checkpoints/<batch-id>.json`をcreate-onlyで記録します。schemaは
    `persona-corpus.full-batch-checkpoint/v1`、scope/batch/count、M1不変、acceptance_group、共有
    before/after相対path、family QA、UTC accepted_at、token-free noteを含めます。
11. checkpointを再読込して一致確認後、親がscope leaseを正式CLIで通常releaseし、不在確認後に次groupへ
    進みます。

group verifyでは全memberについて`--batch-id`を繰り返し、必ず`./bin/p03-full-ledger verify`と
上記のabsolute reconciliation pathを使います。candidateまたはofficial after以外の入力を省略せず、
partial accept/releaseをしません。

## Worker制作・品質規則

workerはAGENTS.md、Full protocol/ledger spec、p03 persona brief、content spine、該当skillを読みます。
DOCXはDocuments、searchable PDFとscan PDFはPDF、XLSXはSpreadsheets、PPTXはPresentations、PNGは
ImageGenで実ファイルとして作り、各skillのrender-and-verifyを省略しません。temporary source、render、
montage、inspection outputは`scratch/p03/<scope-id>/<batch-id>/`だけに置き、final treeに混入させません。
各workerは書込み前にrepository rootと自分のfinal leafのabsolute pathを解決し、final leafが正確に
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p03-security-grc-analyst/home/<scope_path>/`
配下であることを確認します。兄弟の`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/workspace/`、Kio、
別clone、別persona、またはrepo外pathへ成果物を生成しません。

content spineのCobalt Harbor Systems、Aegis Control AC-27、2026-07-13、evidence set 184、RISK-031、
CASE-260713-184、synthetic defensive data onlyを一貫して使います。CSV/structured data、Office、PDF、
画像で数値・日付・IDを整合させます。空テンプレート、lorem ipsum、TODO/TBD、壊れた拡張子、秘密、
資格情報、実在PII、実在未公開資料、routable IPを作りません。ネットワーク表現はdocumentation用予約
addressだけです。攻撃・侵入・回避・破壊の実行可能な詳細を生成せず、GRCの防御的な記録と統制保証だけを
表現します。

490件の`pdf_scan`には厳格な例外規則があります。

- 485件は同一scopeのfinal `image` artifactを明示dependencyとして使います。別scope画像、同一batch
  画像、未受入画像を使いません。
- `p03-secondary-07`（mail/recent）の5 `pdf_scan` rowsだけは、そのrowの`scratch_input` objectどおりの
  一意なImageGen PNGを
  `scratch/p03/p03-secondary-07/<batch-id>/scan-sources/`に作ります。これは`required_skill:imagegen`、
  `final_artifact:false`、`state:planned-scratch-only`のscratch sourceで、final artifact、workspace file、
  master artifact row、cross-scope dependencyには絶対にしません。
- この5 PNGはfinal file countに含めません。各sourceを原寸表示し、可読性、PNG→scan PDFのpixel/source
  一致、scan PDFのzero text layerを確認します。scan PDFは画像面のみで、偶発的text layerを許しません。

`p03-secondary-08`の最初の`foundation-small` batchは94件です。この唯一の小さいfoundation batchを
他batchと結合せず、その後のbatchへ依存を逆転・同一batch化しません。

workerは担当全件について存在、形式、構文、開封、dependency/content-spine整合を確認します。searchable
PDFは全ページとtext layer、scan PDFは全ページとzero text layer、DOCXは全ページ、XLSXは全sheetと
formula/type/error、PPTXは全slideとoverflow、PNGはpixel/dimension/原寸表示を確認します。親はworker
報告だけで受入せず、path/family/scope、magic/parse/open、全skill render、内容品質、dependency、M1/
accepted Full不変、placeholder/secret/PII、unplanned path、exact SHA-256 duplicateを独立確認します。
QA不合格の今回selected artifactは、同じworkerが同じscope leaseを保持している間だけ修正または置換でき
ます。M1、checkpoint受入済みartifact、別batch、別scopeは変更しません。

Python sourceの構文QAはsource textに`ast.parse`またはメモリ内の`compile(source_text, filename,
"exec")`だけを使います。final treeのPythonをimport/実行せず、`py_compile`、`compileall`、
`python -m py_compile`を使いません。`PYTHONDONTWRITEBYTECODE`だけに依存せず、worker/親ともfinal treeの
`__pycache__`と`*.pyc`が0件であることを確認します。今回selected scopeの構文QAが作ったことを親が
path、mtime、対応source、before manifestから一意に証明できる未受入`__pycache__`/`*.pyc`だけは、
final成果物ではない再生成可能cacheとして、対象一覧とSHA-256をscratchへ記録後に親が自律削除し、
AST-only QAとassignment外file検査を再実行して構いません。証明できないcache、他batch・他persona・
repo外のcacheは削除せず安全停止します。

## 固定集計

Full追加9,800件のfamily集計は次のとおりです。

- `md 980`; `txt_log 1,176`; `code 784`; `structured_text 1,470`
- `csv_tsv 980`; `html_eml 784`; `pdf_text 1,470`; `pdf_scan 490`
- `docx 490`; `xlsx 392`; `pptx 196`; `image 294`
- `domain_binary 294`; `ipynb 0`; `media 0`

`skill_required`はnone 6,468、Documents 490、PDF 1,960、Spreadsheets 392、Presentations 196、
ImageGen 294です。scratch-only ImageGen PNG 5件はこのfinal集計・9,800件・10,000件のいずれにも
含めません。各groupと最終ゲートでpromptの数字ではなくmasterから再計算します。

## 自律修正と安全停止

通常の生成失敗、構文エラー、render崩れ、数式エラー、overflow、内容重複、軽微なworker停止は原因を
診断し、同じscope leaseと同じworkerで修正・再検証して継続します。利用可能な他scopeは安全に継続します。

ただし次の条件では追加変更、recover、手動`_control`編集をせず停止し、persona、parent session、scope、
worker session、batch/group、最後のaccepted checkpoint、current file count、manifest diff、残る自分の
leaseを具体的に報告します。

- parent/scope release token喪失、通常release失敗、named recoverが必要なstale/unknown p03 lease
- 固定input SHA、isolated CLI SHA、reconciliation trust anchor、schema/完全性の不一致
- M1またはaccepted Full変更、assignment外file、symlink/nonregular、未受入delta、出所を上記条件で
  一意に証明できないpyc/__pycache__
- strict earlier dependencyを固定ledgerどおり満たせずledger revisionが必要
- scratch-only sourceまたはfinal scan/imageのscope/finality/text-layer boundaryを保てない
- secret、credential、実在PII、実在資料を安全な合成内容へ修正できない
- 必須skill/runtime/serviceが反復試行後も利用不能
- 同じworkerで安全に再開できず、同一scope重複writerとなる危険

fixed ledger/reconciliation/CLIを変更せず、未受入finalを削除せず、duplicate workerまたはpartial
accept/releaseで停止条件を迂回しません。

## 完了ゲート

85 batchを全受入後、次を検証してください。

- p03 home exactly 10,000 regular files、assignment外0、symlink/nonregular 0、`__pycache__`/`*.pyc` 0
- immutable M1 200 filesのpath/bytes/SHA-256が全件不変
- Full追加9,800/9,800 rowsとcheckpoint 85/85がmasterに完全一致
- scope/family/skill_required集計がmaster/canonicalと一致し、actual SHA-256は10,000件すべて一意
- scan PDF 490件の全QA、485 final-image dependency、5 scratch-only source boundaryが全て合格
- 全構造検証・skill QA合格、placeholder、secret、credential、実在PII、routable IP 0
- 最終cumulative official after manifestが現在homeとbyte-for-byte一致
- p03の全scope leaseを通常releaseし、最後にp03 parent leaseを通常releaseして両方不在

他persona leaseには触らず、global `active_leases=0`を要求しません。Git add/commit/push、root corpus
manifest更新、branch/worktree、Kioへの書込み、Kio index/history/search/chunk/performance評価は実行しません。
実際に全完了した場合だけ「p03 Full production完了」と報告し、10,000件、85/85 batches/checkpoints、
形式集計、最終manifest SHA-256、M1不変、QA集計、p03 lease不在を示してください。未完了なら完了を主張せず、
停止証跡と安全に再開できる次batchだけを報告してください。
```
