# p14 Full mass production task

Open one new Codex Local task at the `personaCorpus` repository root and paste
the following. This is a new production parent task, not the allocation task.

```text
このLocalタスクでp14（finance controller）のFull physical-artifact productionを担当してください。
既存M1 200件をbyte-for-byte保持し、Full追加8,800件を制作して最終9,000 filesまで自律的に進めます。
rootは必ず`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`です。別worktree、別workspace、Git操作、Kio操作、root manifest更新、pushは行いません。

最初にAGENTS.md、README.md、runbooks/COMMON_RULES.md、runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、runbooks/SESSION_HANDOFF.md、runbooks/personas/p14-finance-controller.md、progress/p14/full/content-spine.md、progress/p14/full/full-assignment-ledger.jsonlを完全に読んでください。

固定入力を再生成・編集・置換せずSHA-256照合してください。
- M1 ledger `623de06df03967a547e5245148ee77bf1da007bebc4739e3bcb1716edf16719e`
- inventory `b2e1f81fb3486e1d62ef44fe9b78673b1abe83b26199b10377508dd3085debcc`
- M1 baseline `04c3c23f4ce88f2b6eddf7200f516dff14ee1f7667f8f342f8c03d26d46e906f`
- PDF classification reconciliation `55704ae6054138a29b214f40354e4488414001f9770e1d0e1cbf0a40ef47acf7`
- reservations `a03e5d979f58e3a6a9a995f2a47f10a9bcf69ab70a7aeba8fba76e502548d90e`
- content spine `113678951969396b222229cdaf888020a3064fec2bc66c5521e94d7fda298a58`
- master assignment `dacc16569d81c57307ff2bdbe54ac08b7506ccbc9e4ed98b7608836ffb1b9b67`
- p14-only helper `7981685413e46e25a024415f8bee781c1b3531e880630d2d834efa7f71ee0045`
- shared `bin/full-ledger` `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`

M1には実PDFのテキスト層と当初family表示が異なる受入済み3件があります。これを除く197件はdeclared/detectedが一致します。共有`bin/full-ledger`を変更・直接利用せず、p14のinventory/manifest/reserve/verifyには必ず`./bin/p14-full-ledger`を使い、`--classification /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/m1-classification-reconciliation.jsonl`を必ず渡してください。helperは全fixed inputをsnapshot/sha確認後、この3件だけを検証用の正規化viewで`pdf_text`として扱います。reservation headerのM1 digestはこの正規化viewを指すため、raw M1 ledger pinと異なるのが正しい状態です。M1/reservationを編集して合わせてはなりません。このfixed 3-row record以外の分類例外、別path、symlink、変更されたPDFは停止条件です。

初回preflightと各acceptance groupのcandidate/official verifyの直前に、次の2 executableを
fail-closedで照合します。不一致ならhelperを実行・編集・置換・回避せず停止します。

~~~sh
expected_p14_helper_sha=7981685413e46e25a024415f8bee781c1b3531e880630d2d834efa7f71ee0045
expected_shared_helper_sha=f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2
actual_p14_helper_sha=$(shasum -a 256 ./bin/p14-full-ledger | awk '{print $1}')
actual_shared_helper_sha=$(shasum -a 256 ./bin/full-ledger | awk '{print $1}')
if [ "$actual_p14_helper_sha" != "$expected_p14_helper_sha" ] || [ "$actual_shared_helper_sha" != "$expected_shared_helper_sha" ]; then
  printf '%s\n' "p14 preflight: verifier SHA-256 mismatch" >&2
  exit 1
fi
~~~

開始preflight:
`./bin/p14-full-ledger verify --persona p14 --classification /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/m1-classification-reconciliation.jsonl --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/canonical-source-inventory.jsonl --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/m1-assignment-ledger.jsonl --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/m1-reservations.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/full-assignment-ledger.jsonl --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/manifests/m1-baseline.jsonl`

`result: ok`、inventory 9,000、reservation/baseline 200、assignment 8,800、symlink/nonregular 0を確認します。homeがexactly 200 regular filesであることはcheckpoint 0件のinitial stateだけで要求し、resume時は実行可能gateがbaseline+accepted batchの正確なhome状態を判定します。他personaのlease・working treeは操作も検証もしません。p14自身のleaseが既にあれば何も変更せず停止します。合格後に一意なp14 parent leaseをclaimし、release tokenは親プロセスだけに保持し、prompt/worker/progress/scratch/Gitへ記録しません。

masterは126 batchです。checkpointからaccepted batchを再構成し、各scopeでdependencyがaccepted済みの最初の未受入batchを最大1件だけ選び、異なるscopeは利用可能な枠まで同一acceptance groupへ並列化します。同一scopeは常に1 writerです。group前にcreate-only before manifest、全member QA後にscratch candidate-after、atomic verify、公式after、member checkpoint、scope releaseの順に進め、member失敗時に部分accept/releaseしません。checkpointはtoken-freeでscope/batch/count/group/before/after/M1不変/QAを記録します。

workerは自分のscope/batchの固定ledger rowsだけを作成します。DOCX=Documents、PDF=PDF、XLSX=Spreadsheets、PPTX=Presentations、PNGとscan-source artwork=ImageGenを使用し、skillのrender/visual QAを省略しません。PDF scanはmasterで正確に1件宣言された同一scope・先行batchのfinal image dependencyだけから作り、text layer 0とsource/rendered pixel一致を確認します。`scratch_input`やad hocなscan sourceは使いません。CSV/structured dataをOffice/PDF/PPTXの前提にし、Cedar Works、FY2026 Q1、2026-03-31、JPY 12,400,000を整合させます。実在PII、秘密、資格情報、顧客/銀行/税データ、routable IP、placeholder、壊れた拡張子は禁止です。

Python構文QAはsource textへの`ast.parse`等のメモリ内検証のみです。final treeをimport/実行せず、py_compile/compileallを使わず、`__pycache__`/`*.pyc`を0件にします。外部pathへの生成・copy・削除はせず、finalは必ずp14 homeのassigned pathだけです。

通常の生成・render・formula・overflow・構文失敗は同worker/同scope leaseで自律修正します。ただしtoken喪失/release失敗、固定input不一致、M1/accepted Full変更、assignment外file/symlink/nonregular、未承認classification mismatch、ledger revision必要、secret/PII、skill不能、外部誤配置は追加変更/recoverをせず停止し証跡を報告します。

完了時は9,000 regular files、M1 200件不変、Full 8,800/8,800、126/126 checkpoints、family/skill集計、全artifact SHA unique、全skill QA合格、unplanned/secret/PII/cache 0、最後のafter=current homeを確認し、p14 scopeすべてとparent leaseを通常releaseしてください。global active_leases=0やGit cleanは要求しません。

### Mandatory v2 acceptance and resume gate

This executable gate supersedes earlier initial/resume prose. Run it before lease/write, before every group,
and after checkpoint writes before release; no checkpoint directory is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p14 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p14-finance-controller/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p14/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, pin both `bin/p14-full-ledger`
`7981685413e46e25a024415f8bee781c1b3531e880630d2d834efa7f71ee0045` and shared `bin/full-ledger`
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`, then run the exact p14 wrapper command
above with its required classification and the relevant before/all group batch-id/after; never bypass either.
Exactly one create-only `persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` per master batch records
scope/batch, shared before/after, plan_digest, counts/family_counts, all-true
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after byte SHA
fields, and scan_provenance. Each pdf_scan needs exactly one earlier accepted master-declared final image (not
scratch_input) with artifact/path/file SHA plus `pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}`
and source/rendered normalized-RGB digests. Stop on mismatch; post-checkpoint gate precedes release; checkpoint
IDs must equal master IDs exactly at completion.
```
