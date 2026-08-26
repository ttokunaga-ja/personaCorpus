# p16 Full physical-artifact production task

Paste this into one new Codex Local task opened at the `personaCorpus` root.

```text
p16（Clinical Researcher）のFull physical-artifact productionを担当してください。既存M1 200件は
byte-for-byte不変とし、固定masterのFull追加7,800件を制作・受入して最終8,000 regular filesまで
自律的に継続してください。完全に合成された防御的コーパスです。実在PHI/PII、患者・医療者・施設情報、
資格情報、秘密、実在資料、医療助言、攻撃手順を作成しません。Kioのindex/history/search/chunk/performanceを
実行・主張しません。

最初に`AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`、
`runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、
`runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p16-clinical-researcher.md`、
`progress/p16/full/content-spine.md`、`progress/p16/full/full-assignment-ledger.jsonl`、既存checkpointを読みます。

固定SHA-256を照合し、変更しません。
- M1 ledger `f10f054bad24fe3ba14d2429b917c13eac733cec7e7b086e9538c5bad2262b0e`
- inventory `952efae379123d77f100cc21d76845b552a3a490cab88183717c04a390ed4a8b`
- M1 baseline `d59e3eb8785141fc216db0ba5c2cc4865ddb443a58a67090eef7c14be2648a7a`
- reconciliation `4e601bc44dc54f7ceb294ec009fa4984a75ecfa06d9832b1a3244c547af714a4`
- reservations `d103a0c7936d56eeec7274bfcfff2afc8b492dc06161fda2e24b06ceacec6411`
- content spine `5ce9c3734da011c17ef2898e501003c41f561b5b7accf3e8f4f696576caf19a1`
- Full master `1a1f8e77787fe2b63b478ec269262fe06eb7f7cead5565be0fb701cdb6bfaee5`
- isolated helper `bin/p16-full-ledger` `4945749361f3f16d28c78e64195e4528aa1210198330b59c7e281adef39ccc7f`

常に`./bin/p16-full-ledger`を使用します。共有`bin/full-ledger`、canonical、owner、runtime、
assignment、reservations、content spine、masterを変更しません。全verifyには次のabsolute reconciliation
pathを渡します：
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/m1-reservation-reconciliation.jsonl`

初回preflightでp16 home=200 regular files、symlink/nonregular=0、baseline一致、次を確認します。
`./bin/p16-full-ledger verify --persona p16 --inventory "$PWD/progress/p16/full/canonical-source-inventory.jsonl" --m1-assignment "$PWD/progress/p16/m1-assignment-ledger.jsonl" --reservations "$PWD/progress/p16/full/m1-reservations.jsonl" --assignment "$PWD/progress/p16/full/full-assignment-ledger.jsonl" --baseline "$PWD/progress/p16/full/manifests/m1-baseline.jsonl" --reconciliation "/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/m1-reservation-reconciliation.jsonl"`

checkpointからaccepted batchだけを再構成し、current manifestがM1+accepted rowsと正確に一致するときだけresumeします。各groupでscopeごとにready batchを最大1つ選び、異なるscopeを可能な限りSubagentへ並列委譲します。親のみがp16 parent/scope leaseをclaim/releaseし、tokenは親プロセスだけに保持します。workerへ渡しません。他persona leaseには触れません。token喪失、recover必要、M1/accepted file変更、assignment外file、symlink/nonregular、unsafe external output、schema/digest不一致は追加変更せず停止報告します。

groupごとにcreate-only before manifest→scope lease→worker制作/QA→candidate after→全member `--batch-id`によるatomic verify→official after→token-free checkpoint→通常releaseの順にします。partial accept/releaseをしません。workerは自scopeのmaster rowsだけを正規repo内のhome leafへ書き、Git、Kio、worktree、別workspace、他scopeを操作しません。

DOCX=Documents、PDF text/scan=PDF、XLSX=Spreadsheets、PPTX=Presentations、PNG/scan source=ImageGenを用い、全skill artifactをrender/visual QAします。scan PDFはmasterで正確に1件宣言された同一scope・先行batchのfinal image dependencyだけから作り、text layer 0とsource/rendered pixel一致を確認します。`scratch_input`やad hocなscan sourceは使いません。Python構文QAは`ast.parse`またはメモリcompileだけで、py_compile/importを使わず、finalの`__pycache__`/`*.pyc`=0を保ちます。

semantic truth tableもFull masterから検証します。primary-01..03はStudy Alpha、primary-04..06はStudy Betaのみであり、shared scopeはstudy-neutralです。`cohort.synthetic-48`と`threshold.2-5mg`はcontent spineで指定された研究管理scopeだけに使用し、2.5 mgを医療助言として表現しません。

最終ゲートは8,000 regular files、M1 200不変、Full 7,800/7,800、全113 batch checkpoint、assignment外0、SHA-256全一意、全形式/skill QA合格、p16 scopeとparent lease不在です。Git add/commit/push、root corpus manifest、Kio、branch/worktreeは行いません。

### Mandatory v2 acceptance and resume gate

This executable gate is the sole initial/resume authority. Run before lease/write, before every group, and after
checkpoint writes before release; a nonexistent checkpoint directory is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p16 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p16-clinical-researcher/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p16/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, pin isolated `bin/p16-full-ledger`
`4945749361f3f16d28c78e64195e4528aa1210198330b59c7e281adef39ccc7f` and run its exact absolute-path verify
above including reconciliation plus before/all group batch-id/after. One create-only `persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` per
master batch records exact scope/batch, shared before/after, plan_digest, counts/family_counts, all-true
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after byte SHA
fields, and scan_provenance. Every pdf_scan requires exactly one earlier accepted master-declared final image,
not scratch_input, and artifact/path/file SHA with `pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}`
and source/rendered normalized-RGB digests. Stop on mismatch; gate precedes release; completion requires exact
checkpoint-ID/master-ID equality.
```
