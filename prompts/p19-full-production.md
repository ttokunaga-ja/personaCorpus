# p19 Full mass production task

Paste the text below into **one new Codex Local task** opened directly at the
`personaCorpus` repository root.  Do not continue the allocation task that
created this ledger.  This prompt is safe both for an initial production run
and for resuming from accepted p19 Full checkpoints.

```text
このLocalタスクでp19（Educator and Instructional Designer）のFull mass physical-artifact
productionを担当してください。既存M1 200件を一切変えず、Full追加8,800件を制作・受入し、
最終9,000 regular filesまで、通常の制作、QA、差戻し、再開判定をユーザー確認なしで自律的に
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
- `runbooks/personas/p19-educator-instructional-designer.md`
- `progress/p19/full/content-spine.md`
- `progress/p19/full/full-assignment-ledger.jsonl`
- 存在する場合は`progress/p19/full/checkpoints/*.json`

accepted Rust plan、materialization record、workspace ownerだけをcanonical authorityとし、
canonicalを再生成・再解釈しません。次のportable inputは開始時にSHA-256を照合し、
再生成、編集、置換、移動、削除しません。

- human M1 assignment display: `e35eedc4590fe31acfc7f0f7036a46f86b7c4c0fbe627d4ef0431c7c2337744d`
- frozen M1 assignment ledger: `8711998fd9a07dd0ff687dbf1fb5ba89515cab80220e76693b615d9e242340dd`
- canonical source inventory: `49cac673140f43c5a21b216799698ac9f1eaaa836b6ce91351fa9e08caa9b3d0`
- immutable M1 baseline: `171bec6dde0901f35f2874678c519b5f110867fa9ebdec8c828717824c1ef8dd`
  (`72,848,754` bytes)
- frozen M1 reconciliation: `31ca6e559b0022109014cce7142a599429add5f997e00d2fe702cc6746c56e4a`
- reconciled M1 reservations: `01a06ec564be697ff389a0c053ac0be43b4a20802f08482725626bc6fc0b93bf`
- content spine: `7aa5dfdfa5076ffa73d52f66dd07ad1cf2c56a03ecfac110f7a72acaaa33fa43`
- complete Full assignment: `d7c9211cc41d582906aed9837b7098a08701773f4f40d22daf3c85767bbd0b50`
- isolated p19 ledger CLI (`bin/p19-full-ledger`):
  `7f9e587f8da9f53d456777396d161b3bb40a4c19af098d2fc4e59bf05385ec62`

固定状態はinventory 9,000 sources、M1 reservation/baseline 200 files、Full assignment
8,800 rows、107 batches、6,864 dependency edges are scope-local and strictly earlier batchesです。
初回のFull追加残量は8,800 files／107 batches、最終目標は9,000 filesです。

**常に `./bin/p19-full-ledger` を使います。共有 `./bin/full-ledger` は使わず、両CLIを
変更しません。** p19の隔離CLIは、他personaの稼働中Full waveと共有trust anchorを干渉させ
ないための明示的なproduction boundaryです。

reconciliationはID `p19-m1-same-scope-family-balance-v1`だけです。追加例外、別mapping、
copy、symlink、別pathを作りません。すべてのverifyで`--reconciliation`に必ず次のexact
absolute pathを渡してください。相対pathは使いません。

`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/m1-reservation-reconciliation.jsonl`

read-only preflightでは`result: ok`、inventory 9,000、reservation 200、assignment 8,800、
baseline 200を確認してください。

```sh
./bin/p19-full-ledger verify \
  --persona p19 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/manifests/m1-baseline.jsonl \
  --reconciliation /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/m1-reservation-reconciliation.jsonl
```

`./bin/check-ready`のglobal `active_leases`は情報に留めます。他personaのleaseと並列working-tree
変更は正常なので、他personaをshow/claim/recover/release/検証しません。Git cleanやglobal
`active_leases=0`は開始・完了条件にしません。p19自身のparent/scope leaseがすでに存在する
場合だけ、何も変更せずowner sessionを報告して停止します。preflight合格後、一意なsession名で
p19 parent leaseを正式CLIからclaimし、release tokenは親プロセス内だけに保持します。tokenを
prompt、Subagentメッセージ、progress、scratch、log、成果物、Gitへ記録しません。

## 初回状態とresume判定

この判定は末尾の**Mandatory v2 acceptance gate**だけを唯一の実行可能なauthorityとします。初回と
resumeのどちらでも、そのgateが成功するまでlease claim、worker起動、final treeへの書込みをしません。
checkpointが存在しないdirectoryは初回の正当な状態です。存在、`state:"planned"`、worker自己申告だけで
受入済みと判断せず、create-only evidenceを削除・上書きして再開を装いません。

## 自律スケジューラと原子的受入

残りが0になるまで次のacceptance-group loopを継続します。進捗報告や通常のQA修正のためには
停止しません。

1. checkpointからaccepted batch集合を再計算します。
2. 各scopeで、未受入batchのうちbatch外dependencyが全てaccepted済みである最初のbatchを最大1件
   選びます。masterの6,864 edgeはすべて同一scopeかつ**strictly earlier batch**です。同一batch
   dependencyも作らず、workerはそのbatch内の生成順もdependencyを満たします。
3. 異なるscopeのready batchを、利用可能で品質を維持できるSubagent枠まで同じgroupへ入れます。
   固定少数上限を置かず通常はreadyな全scopeを並列化します。同一scopeは常に1 writer/1 batchです。
4. `p19-full-wave-NNN`の一意なgroup IDを採番し、開始前cumulative before manifestを
   `progress/p19/full/manifests/<group-id>.before.jsonl`にcreate-onlyで生成します。
5. 親だけが選択scopeのscope leaseをworker起動直前にclaimします。tokenを親内だけに保持し、
   claim済みscopeだけ委譲します。1 batchは1 Subagentです。
6. workerはmasterの自batch rowsだけを読み、固定scope_path配下にだけ作成します。workerへtokenを
   渡さず、lease CLI、Git、canonical、owner、runtime、root corpus manifest、他persona/他scope、
   assignment、manifest、checkpointを操作させません。
7. 全workerの生成・QA後、親が各memberを独立QAします。不具合は同じworkerへ差し戻し、group leaseを
   保持して修正・再QAします。
8. 合格後だけ親scratch `scratch/p19/full/<group-id>/`に一意なcandidate-after manifestを
   create-onlyで生成し、before/master/baseline/reconciliation/全member batch IDでatomic verifyします。
9. candidateが`result: ok`の場合だけ公式afterを
   `progress/p19/full/manifests/<group-id>.after.jsonl`へcreate-onlyで生成し、candidateとbyte一致、
   公式afterでも同じverifyを行います。不合格なら公式after/checkpointを作りません。
10. 公式verify合格後だけ、memberごとの
    `progress/p19/full/checkpoints/<batch-id>.json`をcreate-onlyで記録します。schemaは
    `persona-corpus.full-batch-checkpoint/v2`とし、exact scope/batch、plan digest、count/family count、
    M1不変、acceptance_group、共有before/after相対pathとbyte SHA、6つのQA、UTC accepted_at、
    scan provenanceを含めます。
11. checkpointを再読込して一致確認後、親がscope leaseを正式CLIで通常releaseし、不在確認後に次groupへ
    進みます。

group verifyでは全memberについて`--batch-id`を繰り返し、必ず`./bin/p19-full-ledger verify`と
上記のabsolute reconciliation pathを使います。candidateまたはofficial after以外の入力を省略せず、
partial accept/releaseをしません。

## Worker制作・品質規則

workerはAGENTS.md、Full protocol/ledger spec、p19 persona brief、content spine、該当skillを読みます。
DOCXはDocuments、searchable PDFとscan PDFはPDF、XLSXはSpreadsheets、PPTXはPresentations、PNGは
ImageGenで実ファイルとして作り、各skillのrender-and-verifyを省略しません。temporary source、render、
montage、inspection outputは`scratch/p19/<scope-id>/<batch-id>/`だけに置き、final treeに混入させません。
各workerは書込み前にrepository rootと自分のfinal leafのabsolute pathを解決し、final leafが正確に
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p19-educator-instructional-designer/home/<scope_path>/`
配下であることを確認します。兄弟の`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/workspace/`、Kio、
別clone、別persona、またはrepo外pathへ成果物を生成しません。

content spineのMaple Learning、Maple Learning Design Standard ML-19、2026-09-14、Course Alpha 2026 Term 1、
Course Beta 2026 Term 2、ML26-S001〜ML26-S036、A-101/A-102/A-103/B-201/B-202/B-203、Evidence 6 + Reasoning 6 +
Organization 4 + Format 4 = 20、216 synthetic LMS score rowsを一貫して使います。CSV/structured data、Office、PDF、
画像で数値・日付・IDを整合させます。空テンプレート、lorem ipsum、TODO/TBD、壊れた拡張子、秘密、
資格情報、実在PII、実在未公開資料、routable IPを作りません。ネットワーク表現はdocumentation用予約
addressだけです。成果物は教育設計、授業計画、学習評価、教材運用、受講者支援の合成記録に限定し、
セキュリティ監査・統制保証・侵入試験・インシデント対応など別personaの業務内容を混入させません。

704件の`pdf_scan`はすべて同一scopeのfinal ImageGen image artifactを明示dependencyとして使います。scratch-only
scan sourceは作成せず、scanのtext layerは0にします。

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

Full追加8,800件のfamily集計は次のとおりです。

- `md 704`; `txt_log 440`; `code 0`; `structured_text 176`
- `csv_tsv 440`; `html_eml 440`; `pdf_text 1,760`; `pdf_scan 704`
- `docx 1,320`; `xlsx 616`; `pptx 1,056`; `image 704`
- `domain_binary 176`; `ipynb 0`; `media 264`

`skill_required`はnone 2,640、Documents 1,320、PDF 2,464、Spreadsheets 616、Presentations 1,056、
ImageGen 704です。各groupと最終ゲートでpromptの数字ではなくmasterから再計算します。

## 自律修正と安全停止

通常の生成失敗、構文エラー、render崩れ、数式エラー、overflow、内容重複、軽微なworker停止は原因を
診断し、同じscope leaseと同じworkerで修正・再検証して継続します。利用可能な他scopeは安全に継続します。

ただし次の条件では追加変更、recover、手動`_control`編集をせず停止し、persona、parent session、scope、
worker session、batch/group、最後のaccepted checkpoint、current file count、manifest diff、残る自分の
leaseを具体的に報告します。

- parent/scope release token喪失、通常release失敗、named recoverが必要なexpired/unknown p19 lease
- 固定input SHA、isolated CLI SHA、reconciliation trust anchor、schema/完全性の不一致
- M1またはaccepted Full変更、assignment外file、symlink/nonregular、未受入delta、出所を上記条件で
  一意に証明できないpyc/__pycache__
- strict earlier dependencyを固定ledgerどおり満たせずledger revisionが必要
- secret、credential、実在PII、実在資料を安全な合成内容へ修正できない
- 必須skill/runtime/serviceが反復試行後も利用不能
- 同じworkerで安全に再開できず、同一scope重複writerとなる危険

fixed ledger/reconciliation/CLIを変更せず、未受入finalを削除せず、duplicate workerまたはpartial
accept/releaseで停止条件を迂回しません。

## 完了ゲート

107 batchを全受入後、次を検証してください。

- p19 home exactly 9,000 regular files、assignment外0、symlink/nonregular 0、`__pycache__`/`*.pyc` 0
- immutable M1 200 filesのpath/bytes/SHA-256が全件不変
- Full追加8,800/8,800 rowsとcheckpoint 107/107がmasterに完全一致
- scope/family/skill_required集計がmaster/canonicalと一致し、actual SHA-256は9,000件すべて一意
- scan PDF 704件の全QA、704 same-scope final-image dependenciesが全て合格
- 全構造検証・skill QA合格、placeholder、secret、credential、実在PII、routable IP 0
- 最終cumulative official after manifestが現在homeとbyte-for-byte一致
- p19の全scope leaseを通常releaseし、最後にp19 parent leaseを通常releaseして両方不在

他persona leaseには触らず、global `active_leases=0`を要求しません。Git add/commit/push、root corpus
manifest更新、branch/worktree、Kioへの書込み、Kio index/history/search/chunk/performance評価は実行しません。
実際に全完了した場合だけ「p19 Full production完了」と報告し、9,000件、107/107 batches/checkpoints、
形式集計、最終manifest SHA-256、M1不変、QA集計、p19 lease不在を示してください。未完了なら完了を主張せず、
停止証跡と安全に再開できる次batchだけを報告してください。

## Mandatory v2 acceptance and resume gate

This executable gate replaces the removed bespoke resume shell logic and is the sole initial/resume authority.
Run it before lease/write, before every group, and after checkpoint writes before release; no checkpoint directory
is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p19 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p19-educator-instructional-designer/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p19/full/manifests --require-scan-provenance
```

Before each initial/candidate/official verify, pin isolated `bin/p19-full-ledger`
`7f9e587f8da9f53d456777396d161b3bb40a4c19af098d2fc4e59bf05385ec62` and invoke its exact absolute-path command
above with reconciliation and relevant before/all group batch-id/after. Exactly one create-only
`persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` exists for every exact master batch and contains
scope/batch, shared group before/after, plan_digest, exact counts/family_counts, all-true
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after manifest
byte SHA fields, and scan_provenance. Every pdf_scan requires exactly one master-declared final image dependency
accepted in an earlier group (not scratch_input), bound to artifact/path/file SHA and
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` plus source/rendered normalized-RGB digests.
Stop on mismatch. Gate after checkpoint before scope release; completion needs checkpoint IDs equal master IDs.
```
