# p20 Full mass production task

Open one new Codex **Local** task at the `personaCorpus` repository root and
paste the following prompt. Do not reuse the allocation task that prepared the
ledger. This prompt is safe for an initial run or for a resume from accepted
checkpoints.

```text
このLocalタスクでp20（Investigative Journalist）のFull physical-artifact productionを担当してください。
既存M1 200件を一切変えず、Full追加9,800件を制作・受入し、最終10,000 regular filesまで、
通常の制作、QA、差戻し、再開判定をユーザー確認なしで自律的に継続します。全内容は完全に合成された
調査報道ワークスペースです。実在の情報源、人物、位置情報、未公開資料、資格情報、秘密、PII、実環境、
攻撃手順を作成・参照・主張しません。

これはallocation taskとは別の新しいproduction親タスクです。rootは
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`を直接使い、別worktree、別workspace、
別runtime、別clone、別PC bundleを作りません。同じpromptは受入済みcheckpointからの再開にも使えます。

## Authorityと固定入力

開始時に次を完全に読んでください。

- `AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`
- `runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、`runbooks/SESSION_HANDOFF.md`
- `runbooks/personas/p20-investigative-journalist.md`
- `progress/p20/full/content-spine.md`、`progress/p20/full/full-assignment-ledger.jsonl`
- 存在する場合は`progress/p20/full/checkpoints/*.json`

accepted Rust plan、materialization record、workspace ownerだけをcanonical authorityとし、canonicalを
再生成・再解釈しません。以下のportable inputは開始時にSHA-256照合し、再生成、編集、置換、移動、
削除しません。

Full authority revision is `p20-full-master-v2` / `p20-content-spine-v2`; the
discarded v1 allocation accepted no Full artifact and must not be revived.

- human M1 display: `3059db87d85d4c5896ccfa7cc43fab8781e8313cf550869ad357ae07e0293693`
- frozen M1 ledger: `7708aa6e346fc0b37059b85d778f8810f5a1aba288659f1bd85e489fcb6dcde9`
- canonical source inventory: `07ddf5c46ed022d56617c93998f950bba3a2480e55b88c10da83d5df44f68cd3`
- immutable M1 baseline: `62292ec602e0f0b92df82b9d25aa5c7a2d3efaee544a9dabada1411746029cb8`
- strict M1 reservations: `c56af6500fa8235ed9886ac7042e4f5d16a99283b158b7713532f3b050737263`
- content spine: `45c070e2ff7d76f187bcc178b2efd28adbe978f258733ea84c13ac0f6967cc10`
- complete Full assignment: `555ef0fb49b2d8c317bacbf69017a5c79150d8b307d47aed8c30b2f740ff8bcd`
- shared `bin/full-ledger`: `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`

固定状態はinventory 10,000 sources、M1 baseline/reservation 200 files、Full assignment 9,800 rows、
106 batchesです。reconciliationはありません。追加例外、別mapping、symlink、別pathを作りません。
開始時のFull追加残量は9,800 files/106 batches、最終目標は10,000 filesです。

最初に`shasum -a 256 bin/full-ledger`が上記shared helper SHAと完全一致することを確認します。不一致なら
何も変更せず停止します。**helper hash gateと下記live manifest gateに合格するまで`verify`、lease claim、
worker起動を行いません。**

初回（checkpoint 0件）は、scratch内の一意なtemp directoryだけに`full-ledger manifest --persona p20 --out`
でlive manifestをcreate-only生成し、immutable M1 baselineと`cmp`でbyte-for-byte一致させます。不一致なら
JSONLをpath/SHA/bytesで比較してmissing/extra/changedを報告し即停止します。

resume（checkpointが1件以上）はbaseline全体へcmpしてはいけません。token-free checkpointを全件読込み、
masterのbatch/scope/count、create-only before/after path、shared acceptance_group、UTC時刻、前group afterへの
連続性を検証します。重複batch、欠落/参照不能manifest、不連続chain、複数のlatest candidate、afterと
checkpoint内容の矛盾は曖昧として即停止します。唯一のlatest accepted cumulative `after` manifestを選び、
同じscratch temp outputに生成したlive manifestと`cmp`でbyte一致させます。さらにbaselineの全200 M1 pathが
current/after内でbytesとSHA-256まで不変であることを確認します。不一致時はmissing/extra/changedを報告して
即停止します。このresume gateが成功した後だけ、accepted集合を次のscheduler入力にします。

上のfail-closed gate後、read-only preflightで次を実行し、`result: ok`、inventory 10,000、reservation 200、
assignment 9,800、baseline 200を確認してください。

```sh
./bin/full-ledger verify \
  --persona p20 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/manifests/m1-baseline.jsonl
```

global `active_leases`とGit cleanは開始/完了条件ではありません。他personaのleaseと並列working-tree
変更は正常です。他personaをshow、claim、recover、release、検証しません。p20自身のparentまたはscope
leaseが既にある場合だけ、何も変更せずowner sessionを報告して停止してください。preflight合格後に
一意なsession名でp20 parent leaseを正式CLIでclaimし、release tokenを親プロセス内だけに保持します。
tokenをprompt、Subagentメッセージ、progress、scratch、log、成果物、Gitへ記録しません。

## 初回状態とresume判定

初回/resumeとも上記manifest gateを唯一の開始判定とします。checkpoint外path、assignment外path、symlink、
nonregular、M1/accepted Full変更が1件でもあれば、未受入deltaの証跡を報告して停止します。存在、
`state:"planned"`、worker自己申告だけではacceptedとみなしません。

## 自律スケジューラと原子的受入

残りが0になるまで次を繰り返します。

1. checkpointからaccepted batch集合を再計算する。
2. 各scopeから未受入でbatch外dependencyが全てaccepted済みの最初のbatchを最大1件選ぶ。同一scopeは
   常に1 writer/1 batch。masterのdependencyは全て同一scopeかつstrictly earlier batchであり、同一batch
   dependencyとcycleはない。
3. 異なるscopeのready batchは品質を維持できる全てのSubagent枠まで同一acceptance groupで並列化する。
   `p20-full-wave-NNN`の一意group IDを使い、開始前にcumulative before manifestを
   `progress/p20/full/manifests/<group>.before.jsonl`へcreate-onlyで作る。
4. 親だけが選択scopeをclaimし、1 batchを1 Subagentへ割当てる。workerにtokenを渡さず、workerは自batchの
   fixed rowsとscope leaf以外を変更しない。Git、lease CLI、canonical、root manifest、checkpoint、別personaを
   workerへ操作させない。
5. 全workerと親QAが合格後だけ、親scratchにcandidate after manifestをcreate-onlyで作り、before/master/
   baseline/全member batch IDでatomic verifyする。合格時だけ公式afterをcreate-onlyで作りcandidateとbyte一致を
   確認し、公式afterでもverifyする。partial accept/releaseをしない。
6. 合格後だけ各batchのtoken-free checkpointをcreate-onlyで記録する。schema
   `persona-corpus.full-batch-checkpoint/v2`、exact scope/batch、plan digest、count/family count、M1不変、
   acceptance_group、共有before/after relative pathとbyte SHA、6つのQA、UTC accepted_at、scan provenanceを
   含める。再読込後にscope leaseを通常releaseし、不在確認後に次groupへ進む。

各verifyは上記absolute inputを使い、全memberについて`--batch-id`を繰り返します。candidate不合格なら
公式after/checkpointは作らず、同じworkerと同じscope leaseで修正・再QAします。

## 制作とQA

workerはAGENTS.md、Full protocol/ledger spec、p20 brief、content spine、該当skillを読みます。DOCXはDocuments、
searchable/scan PDFはPDF、XLSXはSpreadsheets、PPTXはPresentations、PNGはImageGenで実ファイルを作り、
skill所定のrender-and-verifyを行います。temporary source、render、inspection outputは
`scratch/p20/<scope-id>/<batch-id>/`だけに置き、final treeに混入させません。書込み前にfinal leafが正確に
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p20-investigative-journalist/home/<scope_path>/`
配下か確認し、Kio、兄弟workspace、repo外pathへ生成しません。

Civic Ledger、Story Alpha 2026、Story Beta 2026、FOIA batch 17、verification date 2026-08-19、
publication date 2026-08-22、synthetic source cards 36、fact-check claims 24をcontent spineどおりに一貫して
使います。全人物/記録は合成し、`.invalid`以外のメール、実在PII、実在資料、credentials、秘密、routable IP、
lorem ipsum、TODO/TBD、空テンプレート、壊れた拡張子を含めません。

`p20-primary-01`から`p20-primary-05`の専用Story Alpha scopeは、ファイル名・題材・参照をStory Alphaに限定し、
`p20-primary-06`から`p20-primary-09`の専用Story Beta scopeはStory Betaに限定します。共有scopeだけが、
content spineに明示された中立または比較題材として両方を参照できます。

全980件のscan PDFは各rowの**同一scope・先行batch**のfinal ImageGen PNG dependencyから作ります。PNGを
原寸表示して可読性とpixel/source-to-scan一致を確認し、scan PDFは全ページに画像面がありaccidental text
layer 0とします。scratch-only画像やcross-scope画像は作りません。searchable PDFは全ページとtext layer、
DOCXは全ページ、XLSXは全sheet/formula/type/error、PPTXは全slide/overflow、PNGはpixel/dimension/原寸を
workerと親の双方で確認します。

Python構文QAはsource textの`ast.parse`またはメモリ内`compile(source_text, filename, "exec")`のみを
使い、final treeをimport/実行せず、`py_compile`、`compileall`、`python -m py_compile`を使いません。
worker/親とも`__pycache__`と`*.pyc`が0であることを確認します。今回selected scopeの構文QAが作ったことを
path/mtime/source/before manifestから一意に証明できる未受入cacheだけは、対象一覧とSHA-256をscratchに記録後、
親が自律削除してAST-only QAとassignment外file検査を再実行できます。証明できないcache、他batch/他persona/
repo外fileは削除せず安全停止します。

## 固定集計と停止条件

Full追加9,800件は `md 784`, `txt_log 1,764`, `code 98`, `structured_text 294`, `csv_tsv 784`,
`html_eml 980`, `pdf_text 1,568`, `pdf_scan 980`, `docx 784`, `xlsx 196`, `pptx 196`, `image 784`,
`media 392`, `domain_binary 196`、`ipynb 0`です。skill_requiredはnone 5,292、Documents 784、PDF 2,548、
Spreadsheets 196、Presentations 196、ImageGen 784です。promptの数字でなくmasterで毎回再計算します。

通常の生成失敗、構文エラー、render崩れ、数式エラー、overflow、内容重複、軽微なworker停止は同じworker/
scope leaseで自律修正します。ただしtoken喪失、通常release失敗、recoverが必要なlease、固定input/schema不一致、
M1/accepted Full変更、assignment外file、symlink/nonregular、証明できないcache、固定dependencyを満たせない
ledger revision、secret/実在PII、必須skillの反復不能、repo外誤配置では追加変更/recover/manual control編集をせず
停止し、persona/session/scope/worker/batch、最後のcheckpoint、file count、manifest diff、残存leaseを具体的に報告します。

106 batchを全受入後、p20 home exactly 10,000 regular files、M1 200 filesのbyte/SHA不変、Full 9,800/9,800、
checkpoint 106/106、全SHA一意、scan PDF 980件を含む全skill QA、assignment外0、symlink/nonregular/cache 0、
最終after manifestとcurrent home一致を確認します。全scopeを通常release後、p20 parentを通常releaseして両方不在を
確認します。他persona leaseに触れず、global active_leases=0を要求しません。Git add/commit/push、root corpus
manifest、branch/worktree、Kio書込み・評価は行いません。全ゲートを満たした場合だけ完了を主張し、それ以外は停止証跡と
安全に再開できる次batchを報告します。

## Mandatory v2 acceptance and resume gate

This executable generic gate supersedes the earlier toy prose while retaining the v2 authority and semantic
constraints. Run it before lease/write, before every group, and after checkpoint writes before release; an absent
checkpoint directory is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p20 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p20-investigative-journalist/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p20/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, pin shared `bin/full-ledger`
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`, then invoke its exact absolute-path
command above with before/every group batch-id/after. Each master batch gets exactly one create-only
`persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` with exact scope/batch, shared before/after,
plan_digest, counts/family_counts, all-true `{structure,family,content_spine,dependencies,skill_required,secrets_pii}`,
UTC accepted_at, before/after byte SHA fields, and scan_provenance. Each pdf_scan needs exactly one earlier
accepted master-declared final image, never scratch_input, bound with artifact/path/file SHA and
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` plus source/rendered normalized-RGB digests.
Stop on mismatch; post-checkpoint gate precedes release; exact checkpoint/master batch-ID equality is required.
```
