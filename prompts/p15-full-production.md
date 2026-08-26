# p15 Full production task

> **Accepted READY package; physical Full production has not started.** Run
> this prompt only in one new p15 Local task.

Open a **new Codex Local task** at the `personaCorpus` repository root and paste
the following prompt. This is a physical-artifact production task, not the
completed p15 allocation task. It may be reused only to resume from accepted
checkpoints.

```text
このLocal taskは p15（recruiter / People Ops）のFull physical-artifact productionを
担当します。既存M1 200件をbyte-for-byte保持し、Full追加7,800件を受入して最終8,000 filesまで
自律的に継続してください。作業rootは
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`そのものです。別worktree、
別workspace、別runtime、外部bundleを作りません。

最初に `AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`、
`runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、
`runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p15-recruiter-people-ops.md`、
`progress/p15/full/content-spine.md`、`progress/p15/full/full-assignment-ledger.jsonl`、
既存checkpoint（あれば）を完全に読んでください。canonical plan/render/ownerだけを
authorityとし、再生成・再解釈・編集しません。

以下の固定inputを開始時にSHA-256照合し、再生成・編集・置換・移動・削除しません。

- M1 assignment display: `3c11435dd6131c2c37fea85b7545bbb9676e3dd73dd8cd918f0cbf0b8cd36a27`
- shared `bin/full-ledger`: `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`
- frozen M1 assignment: `5f3685406085f356598a8bfda7bb6477cf0e1d18d5171898baa54a63926e90ea`
- inventory: `a3030bd0298d7ea1e0cdb4e2fc7ddf06dc0341b1dce07f91edc24087077c9528`
- immutable M1 baseline: `c0ddbd1edb0cc7fc719710f20dfaa953e585084618b925e7f67d38ec9402b751`
- strict M1 reservations: `b49098678213640feb496c28f0d136f598f1d03c6f8e5777fa23c6c749bd2e47`
- content spine: `753ce25aa606d0fb74bc65977332cd4e92fe2fd5a174474c9e073ab5d66cac54`
- complete Full assignment: `a940a25098243413fba0021b42975d188168ac71ee94fb9ed63af98cb1272baf`
- p15 content-spine gate: `bbb786443fbeb6a567d9432ff0646efd47b9d125f64098fcc81767d40ab20094`

固定状態はinventory 8,000 sources、M1 baseline/reservations 200 files、Full assignment
7,800 rows、116 batchesです。p15にはpilot、reconciliation、事前受入済みFull batchはありません。
strict reservationを維持し、例外mappingやreconciliationを作成しません。

開始時と各acceptance groupで、必ず以下をread-only preflightとして実行し、`result: ok`、
inventory 8,000、reservation 200、assignment 7,800、baseline 200を確認してください。
最初に`bin/full-ledger`のSHA-256が上記固定値と一致しない場合は、shared helperを編集・置換・
回避せず停止してください。

~~~sh
expected_full_ledger_sha=f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2
actual_full_ledger_sha=$(shasum -a 256 ./bin/full-ledger | awk '{print $1}')
if [ "$actual_full_ledger_sha" != "$expected_full_ledger_sha" ]; then
  printf '%s\n' "p15 preflight: bin/full-ledger SHA-256 mismatch: expected $expected_full_ledger_sha, got $actual_full_ledger_sha" >&2
  exit 1
fi
PYTHONDONTWRITEBYTECODE=1 ./bin/p15-full-content-spine verify
./bin/full-ledger verify \
  --persona p15 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p15/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p15/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p15/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p15/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p15/full/manifests/m1-baseline.jsonl
~~~

`check-ready`のglobal lease数と他personaのworking-tree変更は情報であり、停止条件ではありません。
他personaをinspect、claim、recover、release、検証しません。p15自身のparent/scope leaseが既に
ある場合だけ、何も変更せずsessionを報告して停止してください。preflight後、parentだけが一意な
session名でp15 leaseをclaimし、release tokenを親プロセス内だけに保持します。tokenをprompt、
Subagent message、progress、scratch、ログ、成果物、Gitへ記録しません。

初回はp15 homeがexactly 200 regular files、symlink/nonregular 0で、現在manifestがM1 baselineと
byte-for-byte一致することを確認します。再開時は、正しいtoken-free checkpointとcreate-only
before/after manifestが全てあるbatchだけをacceptedとして再構成してください。checkpointのない
assignment path、assignment外path、symlink/nonregular、M1またはaccepted Fullの変更が1件でもあれば、
受入を進めず証跡を報告して停止します。

残りが0になるまでacceptance-group loopを継続します。各scopeで未受入batchのうち依存先が既に
acceptedされた最初のbatchを最大1件選び、異なるscopeのready batchは実際に利用可能なSubagent枠を
全て使って並列化します。同一scopeは常に1 writer・1 batchです。親だけがscope leaseをclaim/release
します。workerは自分の固定batch/pathだけを作り、lease CLI、Git、canonical、owner、runtime、
manifest、checkpoint、他scopeを操作しません。

groupごとにcreate-only shared before manifestを作り、全memberの生成・親QA後、scratch下の
candidate-after manifestを作成します。完全master、M1 baseline、selected全batchを同時指定して
atomic verifyし、`result: ok`の場合だけ公式after manifestをcreate-onlyで作成し、candidateと
byte-for-byte比較後に再verifyします。その後だけ各batchのtoken-free checkpointをcreate-onlyで記録し、
再読込確認後にscope leaseを通常releaseします。group memberを部分accept/releaseしません。

artifactはscope-localな固定pathにだけ作ります。Documents→DOCX、PDF→searchable PDFとscan PDF、
Spreadsheets→XLSX、Presentations→PPTX、ImageGen→PNGとscan source artworkを必ず使い、各skillの
render-and-verifyを完了します。scan PDFはtext layer 0で、同scopeの先行ImageGen final imageとの
pixel/source整合を確認します。temporary input/renderは
`scratch/p15/<scope-id>/<batch-id>/`だけに置き、final treeへ混入させません。

content spineに従い、Orchid People、Requisition Alpha/Beta、2026-08-18、Q3 2026、採用枠3、
synthetic JPY 7.2–8.4M bandを一貫させます。候補者・従業員・連絡先・履歴書・給与・IDは完全に
合成し、実在PII、PHI、credential、secret、顧客データ、実在非公開資料、routable IPを含めません。
空テンプレート、lorem ipsum、TODO/TBD、壊れた拡張子、同一内容の水増しを作りません。

Python source QAは`ast.parse`またはメモリ内`compile(source_text, filename, "exec")`だけを使い、
final treeのPythonをimport/実行せず、py_compile/compileallを使いません。workerと親の双方が最終treeの
`__pycache__`/`*.pyc` 0を確認します。親はworker報告だけで受入せず、path/family/scope、open/parse、
skill render、formula/overflow、content-spine/dependency整合、M1/accepted Full不変、placeholder/
secret/PII/unplanned path/exact SHA重複を独立QAします。

cache、repo外leaf、外部誤配置、scope escape、別persona/別batch由来fileを検出した場合は、出所を
一意に証明できない限りcopy、delete、cleanup、移動をせず、そのまま停止してpath、manifest差分、
scope/batch/sessionとともに報告します。未受入`*.pyc`または`__pycache__`のcleanupを許容できるのは、
同一scope・同一worker・同一未受入batchが生成元であることをpathと生成時点の証跡でexactに証明でき、
かつ対象cacheだけを限定できる場合だけです。accepted artifact、M1、repo外file、出所不明fileは
cleanup対象にしません。

Full追加の固定family集計は md312、txt_log390、structured_text156、csv_tsv546、html_eml1170、
pdf_text1560、pdf_scan624、docx1560、xlsx624、pptx234、image390、media78、domain_binary156です。
skill_requiredはDocuments1560、PDF2184、Spreadsheets624、Presentations234、ImageGen390、none2808です。

通常の生成・render・formula・overflow・内容重複・worker停止は同じworker/scope leaseで自律修正して
継続します。ただしtoken喪失/通常release失敗、stale leaseでrecoverが必要、固定input不一致、M1または
accepted Full変更、assignment外file、symlink/nonregular、dependency不能、secret/PIIを安全修正不能、
必須skill反復不能、同scope重複worker危険、出所不明cache、repo外/外部誤配置、scope escape、
別persona/別batch由来fileでは追加変更/recoverをせず停止し、session/scope/batch/manifest差分/leaseを
具体的に報告します。

完了時はp15 home exactly 8,000 regular files、M1 200不変、Full 7,800/7,800、checkpoint116/116、
assignment外0、symlink/nonregular0、SHA-256 unique8,000、全skill/構造QA合格、最後のafter manifestが
current homeと一致、p15全scopeとparent lease不在を確認します。Git add/commit/push、root manifest、
worktree、Kio書込み、Kio index/history/search/chunk/performance評価は実行しません。
```
