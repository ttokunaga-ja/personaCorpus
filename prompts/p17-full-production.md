# p17 Full mass production task

Open one **new Codex Local task** at the `personaCorpus` repository root and
paste the following text. Do not reuse the allocation task. This prompt is safe
for an initial run and for resuming accepted p17 Full checkpoints.

```text
このLocal taskでp17（construction project manager）のFull physical-artifact
productionを担当してください。既存M1 200件を一切変更せず、Full追加7,800件を制作・受入し、
最終8,000 regular filesまで通常の制作、QA、差戻し、resumeをユーザー確認なしで自律継続してください。
すべては完全に合成された防御的評価用資料です。実在住所、site access情報、図面、顧客/作業員情報、
資格情報、秘密、実在PII、未公開資料を作成しません。

作業rootは`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`を直接使うLocal環境です。
別worktree、別workspace、別runtime、別PC bundleを作りません。Git add/commit/push、Kio操作、
canonical/owner/runtime/root corpus manifestの変更はしません。

## Authority と固定入力

最初に`AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`、
`runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、
`runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p17-construction-project-manager.md`、
`progress/p17/full/content-spine.md`、`progress/p17/full/full-assignment-ledger.jsonl`、
存在する場合は`progress/p17/full/checkpoints/*.json`を完全に読んでください。

accepted Rust plan、materialization record、workspace ownerのみをcanonical authorityとし、
canonicalを再生成・再解釈しません。開始時に以下のSHA-256を照合し、これらを再生成、編集、置換、
移動、削除しません。

- human M1 assignment display: `440b26763fc5a2252fa93c7a93f0f131149586fa1bf4910414644c6a1ec09608`
- frozen M1 assignment ledger: `8f341a3205c9b76f647e82f251b546e97782e5edcc6c667a7b3a61362bd98c91`
- canonical source inventory: `a37a1d0c6530bf46c129bc911f158d7eda29af8b88bdac4edb7010f360933b18`
- immutable M1 baseline: `350ef46f62459a8395444edce81f8ebc8f96e6250ae65b10f630c0564120d269`
- strict M1 reservations: `2c2e01b5c34036b1cc1c891e6f155d333e68d56ad4c3f5de06258a53d5beb884`
- content spine: `6ea3a3174bdab55c283a1a75f61b32d91b6bd760a62f616499df898571a59bef`
- complete Full assignment: `866126c44b84809e4f202813f08b57743008f7ceb0b59914e688dff35c108b79`

固定状態は inventory 8,000 sources、M1 baseline/reservation 200 files、Full assignment 7,800 rows、
112 batches、dependency edge 4,212（全て同一scopeかつstrictly earlier batch）です。p17はstrict reservationで
ありreconciliation fileを作成・使用しません。初回のFull追加残量は7,800、最終目標は8,000 filesです。

read-only preflightでは次を実行し`result: ok`を確認してください。

```sh
./bin/full-ledger verify \
  --persona p17 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/manifests/m1-baseline.jsonl
```

`check-ready`のglobal active_leasesは情報に留めます。他personaのleaseとworking-tree変更は正常な
並列状態です。他personaをshow/claim/recover/release/検証しません。Git cleanやglobal active_leases=0を
開始・完了条件にしません。p17自身のparentまたはscope leaseが存在する場合だけ、何も変更せずowner
sessionを報告して停止します。preflight後に一意なp17 parent leaseをclaimし、release tokenは親プロセス
だけに保持します。tokenをprompt、subagentメッセージ、progress、scratch、log、成果物、Gitへ書きません。

## Resume と原子的受入

初回はp17 homeがexactly 200 regular files、symlink/nonregular 0、現在manifestがM1 baselineと
byte-for-byte一致することを確認します。checkpointがあるresumeでは、正しいtoken-free checkpointと
参照するcreate-only before/after manifestが揃い、batch/scope/countがmasterと一致するものだけをaccepted
とします。現在homeはM1 baseline + accepted batch rowsと完全一致し、M1/accepted Fullのbytes/SHA-256が
不変でなければなりません。checkpointなしassignment path、assignment外path、削除、symlink、nonregular、
accepted file変更が1件でもあれば証跡を報告して停止します。存在やworker自己申告では受入しません。

残りが0になるまで、scopeごとに未受入で外部dependencyが既にacceptedの最初のbatchを最大1件選びます。
readyな異なるscopeは利用可能な品質維持可能なSubagent枠まで同一acceptance groupで並列化し、同一scopeは
常に1 writer/1 batchです。親だけがscope leaseをclaimしtokenを保持し、1 batchを1 Subagentへ委譲します。
workerは自batchの固定rowsと正確なscope_path配下だけを作成し、lease CLI、Git、canonical、manifest、
checkpoint、他scope/他personaを操作しません。

group開始前に`progress/p17/full/manifests/<group-id>.before.jsonl`をcreate-onlyで作成します。全workerの
独立QA合格後、まず`scratch/p17/full/<group-id>/`にcandidate after manifestをcreate-onlyで作成し、
baseline/master/全member batch IDとともにverifyします。candidateがokの時だけ公式after manifestを
create-onlyで作成してcandidateとbyte一致を確認し、同じatomic verifyを再実行します。公式verify合格後だけ
memberごとのtoken-free checkpointをcreate-onlyで記録し、再読込確認後にscope leaseを通常releaseします。
partial acceptance/releaseはしません。最後にp17 parent leaseを通常releaseします。

## 制作・品質規則

workerは該当skillのSKILL.mdを読んでから使います。DOCXはDocuments、PDF text/scanはPDF、XLSXは
Spreadsheets、PPTXはPresentations、PNGはImageGenで実ファイルを作り、skill指定のrender-and-verifyを
省略しません。temporary source/render/inspection outputは`scratch/p17/<scope>/<batch>/`だけに置き、
final treeへ混入させません。書込み前にfinal leafが必ず
`.../personaCorpus/workspace/people/p17-construction-project-manager/home/<scope_path>/`配下であることを
確認し、Kio、兄弟workspace、repo外pathには絶対に生成しません。

Harbor Build、2026-07-29、RFI-042、SUB-118をcontent spineどおりに一貫利用します。primary-01から
primary-06はProject Alpha専用で、Project Betaを引用せず、CO-014とJPY 18,600,000はこのAlpha専用scope
だけで使用します。primary-07からprimary-10はProject Beta専用で、Project Alpha、CO-014、CO-014金額を
引用しません。primary-11、primary-12、全secondary scopeは共有portfolio scopeであり、master rowが明示的な
比較を割り当てない限り両projectを同時に引用しません（現在のmasterに比較rowはありません）。Project Alphaと
Betaが物理siteを共有するとは主張しません。空テンプレート、
lorem ipsum、TODO/TBD、壊れた拡張子、秘密、資格情報、実在PII、実在資料、routable IPは作りません。
画像とscan-source artworkは架空のdiagram/field-note styleで、実在図面の再現ではありません。

全936 `pdf_scan`は同一scopeのfinal image artifactを明示dependencyとして使います。画像は必ず既に
accepted済みのearlier batchです。scan PDFは画像面のみでtext layer 0、全page renderとsource-to-scan
pixel一致を確認します。scratch-only inputはこのp17 masterには存在しないため、ad hoc PNGや新しい
master rowを作りません。

workerと親はPDF page/text-layer、DOCX page、XLSX全sheet/formula/type/error、PPTX全slide/overflow、
PNG pixel/dimension/原寸、その他のmagic/parse/open/syntaxを検証します。親はworker報告だけで受入せず、
path/family/scope、dependency/content-spine、M1/accepted Full不変、placeholder/secret/PII、unplanned path、
actual SHA-256 duplicateを独立検証します。Python sourceはast.parseまたはメモリ内compileのみを使い、
final treeをimport/実行せず、py_compile/compileallを使いません。worker/親とも`__pycache__`と`*.pyc`が
final treeで0件を確認します。

## 固定集計と停止条件

Full追加7,800のfamily集計は md 234、txt_log 312、structured_text 156、csv_tsv 390、html_eml 312、
pdf_text 1,560、pdf_scan 936、docx 624、xlsx 780、pptx 312、image 936、media 78、domain_binary 1,170です。
skill_requiredはnone 2,652、Documents 624、PDF 2,496、Spreadsheets 780、Presentations 312、ImageGen 936です。
group/最終gateではprompt数字でなくmasterから再計算します。

通常の生成/render/formula/overflow/構文/内容重複の失敗は同じworker・同じscope leaseで修正・再QAして
継続します。ただしrelease token喪失、通常release失敗、recover必要、固定input不一致、M1/accepted Full
変更、assignment外file、symlink/nonregular、未受入delta、dependency/skill boundary違反、秘密/実在PII、
必須skill反復不能、重複writer危険では、追加変更/recover/manual control editをせず停止し、session、scope、
batch、checkpoint、file count、manifest diff、残leaseを具体的に報告します。

完了時はp17 home 8,000 regular files、M1 200不変、Full 7,800/7,800、checkpoint 112/112、
assignment外/symlink/nonregular/pyc/cache 0、actual SHA-256 8,000 unique、全skill/構造QA合格、
最終official after manifest一致、p17 scope/parent leases不在を確認します。Kio indexing/history/search/
chunk/performanceの達成は主張しません。

### Mandatory v2 acceptance and resume gate

The executable gate supersedes all weaker initial/resume prose. Before lease/write, before every group, and after
checkpoint writes before release (missing checkpoint directory is valid initial state), fail closed:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p17 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p17-construction-project-manager/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p17/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, pin shared `bin/full-ledger`
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` and invoke the exact absolute-path
verifier above with before/every group batch-id/after. Exactly one create-only `persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` per master
batch includes scope/batch, shared before/after, plan_digest, counts/family_counts, all-true
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after byte
SHA fields, and scan_provenance. Each pdf_scan needs exactly one earlier accepted master-declared final image,
never scratch_input, bound with artifact/path/file SHA and `pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}`
plus source/rendered normalized-RGB digests. Stop on mismatch. Post-checkpoint gate precedes release; completion
requires checkpoint IDs equal master IDs exactly.
```
