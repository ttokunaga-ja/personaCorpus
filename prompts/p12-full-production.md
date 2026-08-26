# p12 Full mass-production task

Paste the following into one new Codex **Local** task opened at the
`personaCorpus` repository root. This is a production task, not the allocation
task that froze the records below. It is safe both for an initial run and a
resume from accepted p12 checkpoints.

```text
このLocalタスクではp12（Support / Success Lead）のFull physical-artifact productionを担当してください。
既存M1 200件を一切変更せず、Full追加15,800件を制作・受入し、最終16,000 regular filesまで自律的に
継続してください。すべて防御目的の完全な合成コーパスです。実在顧客、実在PII/PHI、資格情報、秘密、
未公開資料、実行可能な攻撃・侵入・回避手順を作成・使用しません。

rootは直接
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`
を使用します。別worktree、別workspace、別clone、repo外path、Kioは使用しません。

最初に `AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`、
`runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、
`runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p12-support-success-lead.md`、
`progress/p12/full/content-spine.md`、`progress/p12/full/full-assignment-ledger.jsonl`、
存在する`progress/p12/full/checkpoints/*.json`を読みます。canonical plan / render / materialization /
workspace ownerだけをcanonical authorityとし、再生成・再解釈しません。

開始時に以下のportable input SHA-256を照合し、編集・置換・移動・削除しません。

- frozen M1 ledger: `50f937d8dbf60894c60afe24c78a93d1888380c2c2276cbe65b1853a36a8929c`
- canonical source inventory: `436bcfe6092d176ac4c8355cf776b3597308758de490246c07d6faf8cde1fe95`
- immutable M1 baseline: `9d8c65999c364a6e5bf436049e79f6385e4776b1efae18b5de4f270226134fd5`
- strict M1 reservations: `01f8aa4ef2e574b1d045a751af885ad6ae314bc24dc310eb87c6e66f0a3d21d7`
- content spine: `36003b91b7a799c194e059646705000710009d65aac9b0f7a921f7d1928b1a79`
- complete Full master: `db34596709575b5cb79855f6d4f840ae441579c3452b5d1299466fe7f16cfa70`
- shared ledger CLI: `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`

固定状態はinventory 16,000、M1 baseline/reservation 200、Full assignment 15,800、100 batches、
dependency edge 2,054（すべて同一scopeのstrictly earlier batch）です。reconciliationは存在しません。
**常に共有 `./bin/full-ledger` を変更せずに使い、`--reconciliation`は渡しません。**

```sh
verify_shared_full_ledger() {
  actual="$(shasum -a 256 ./bin/full-ledger | awk '{print $1}')"
  [ "$actual" = "f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2" ] || {
    printf '%s\n' "shared bin/full-ledger SHA-256 mismatch: $actual" >&2
    exit 1
  }
}
verify_shared_full_ledger
./bin/full-ledger verify \
  --persona p12 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/manifests/m1-baseline.jsonl
```

global `active_leases`と他personaのworking-tree変更は情報です。p12以外をshow/claim/recover/release/
検証しません。p12自身のparentまたはscope leaseがすでにあるときだけ何も変更せずowner sessionを報告して
停止します。preflight合格後に一意なp12 parent leaseを正式CLIでclaimし、release tokenは親プロセス内だけに
保持します。tokenをprompt、subagent message、progress、scratch、log、artifact、Gitに記録しません。

初回はp12 homeが200 regular files、symlink/nonregular 0で、現在manifestがM1 baselineとbyte-for-byte
一致することを確認します。resumeでは、正しいtoken-free checkpointとcreate-only before/after manifestが
揃うbatchだけをacceptedとします。現在homeはM1 baselineとaccepted batch rowsの和にpath/bytes/SHA-256で
完全一致しなければなりません。checkpointのないassignment path、assignment外path、M1/accepted Fullの変更、
symlink/nonregularがあれば、追加変更・recoverなしで停止して証跡を報告します。

残りが0になるまで、各scopeで外部dependencyがaccepted済みの最初の未accepted batchを最大1件選びます。
異なるscopeのready batchを、品質を維持できる全Subagent枠までgroupとして並列化します。同一scopeは常に
1 writer/1 batchです。親はgroup開始前に一つのcumulative before manifestをcreate-onlyで作り、選択scopeの
leaseだけをclaimしてからworkerを起動します。workerにtokenを渡さず、workerはmasterの自batch rowsだけを
正しいscope leaf配下に作成し、Git、canonical、owner、runtime、root manifest、他persona/他scope、ledger、
manifest、checkpointを操作しません。

全workerの生成とQA後、親が全memberを独立QAします。合格したgroupだけscratchのcandidate-after manifestを
create-onlyで生成し、`verify_shared_full_ledger`を実行してから全memberの`--batch-id`を一回のatomic
`full-ledger verify`へ渡します。`result: ok`後だけ公式after manifestをcreate-onlyで生成しcandidateとbyte一致を
確認し、再度`verify_shared_full_ledger`を実行してから再verifyします。その後だけ各batchの
token-free checkpointをcreate-onlyで書き、再読込確認後にscope leaseを通常releaseします。partial accept、
partial release、後追いacceptはしません。最後にp12 parent leaseを通常releaseします。

品質規則：DocumentsでDOCX、PDFでsearchable/scan PDF、SpreadsheetsでXLSX、PresentationsでPPTX、ImageGenで
PNGとscan source artworkを実ファイルとして作成し、各SKILL.mdのrender-and-verifyを守ります。temporary
source/render/inspectionは`scratch/p12/<scope-id>/<batch-id>/`または`scratch/p12/full/`だけに置きます。
final leafは必ず
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p12-support-success-lead/home/<scope_path>/`
配下です。兄弟`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/workspace/`、Kio、repo外には絶対に出力しません。

content spineのscope relationship contractを必ず守ります。HelioDesk、Alpha Admin／Beta Sync、case
CS-260713-184、ticket TKT-18427、incident INC-260713-04、SLA 4h、retention 18m、first response 94.2%、
resolution 91.8%、adoption 78.6%、decision DEC-260713-006、Q3 2026と固定日付・measureを一貫させます。
特にQBRは該当productとadoption、case historyはproduct・case/ticket・incident・SLA・retention・response/
resolution、known issue/KBはincidentとdecisionを明示します。`p12-primary-01` ticket exportsは全rowで
`policy.sla-4h`と`rel.sla-to-escalation`を明示します。空テンプレート、TODO/TBD、lorem ipsum、壊れた拡張子、秘密、資格情報、
実在PII、実在未公開資料、routable IPを作りません。scan PDF 158件は全て同一scopeのfinal image artifactを
明示dependencyにし、画像面のみ・zero text layerで、元PNGの原寸・可読性・pixel/source-to-scan一致を確認します。
scratch-only sourceはこのmasterでは不要です。

全workerは存在・形式・構文・開封・content-spine/dependency整合を確認します。searchable PDFは全page/text layer、
scan PDFは全page/zero text layer、DOCXは全page、XLSXは全sheet/formula/type/error、PPTXは全slide/overflow、
PNGはpixel/dimension/原寸を確認します。親はworker自己申告だけに依存せず、path/family/scope、parse/open、
render、内容、M1/accepted Full不変、placeholder/secret/PII、unplanned path、exact SHA duplicateを独立確認します。
Pythonの構文QAは`ast.parse`またはメモリ内compileだけを用い、final treeをimport/実行せず、`py_compile`、
`compileall`、`python -m py_compile`を使用しません。`__pycache__`と`*.pyc`はfinal treeで0件です。

通常のrender/format/content failureは同じworker・同じscope lease内で修正して継続します。ただし、token喪失、
通常release失敗、recover必要、固定SHA/CLI/schema不一致、M1/accepted変更、assignment外file、symlink/nonregular、
出所を一意に証明できないcache、scope外/外部path、secret/PII、ledger revisionが必要なdependency不整合、
skill不可用が起きたら、追加変更・recover・手動`_control`編集をせず停止します。persona/session/scope/worker/
batch/group、最後のaccepted checkpoint、file count、manifest diff、残るp12 leaseを報告します。

最終ゲートは、p12 home exactly 16,000 regular files、M1 200件のbytes/SHA-256不変、Full 15,800/15,800、
checkpoint 100/100、scope/family/skill集計がmaster/canonical一致、16,000 SHA-256全一意、全skill QA合格、
placeholder/secret/credential/実在PII/routable IP 0、official after manifestが現在homeとbyte一致、p12 scopeと
parent lease不在です。Git add/commit/push、root corpus manifest、branch/worktree、Kio index/history/search/chunk/
performance評価は一切行いません。未完了なら完了を主張せず、安全に再開可能なbatchと停止証跡だけを報告します。

### Mandatory v2 acceptance and resume gate

This executable gate supersedes all earlier initial/resume prose. Run it before any lease/write, before every
group, and after writing checkpoints before scope release; an absent checkpoint directory is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p12 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p12-support-success-lead/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p12/full/manifests --require-scan-provenance
```

Before each initial/candidate/official verify, re-pin shared `bin/full-ledger` SHA
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` then run the exact absolute-path verifier
above with relevant before/all group batch-id/after. Exactly one create-only `<batch-id>.json` per master batch
uses `persona-corpus.full-batch-checkpoint/v2` and records exact scope/batch, shared group before/after,
plan_digest, counts/family_counts, all-true `{structure,family,content_spine,dependencies,skill_required,secrets_pii}`,
UTC accepted_at, before/after manifest byte SHA fields, and scan_provenance. Each pdf_scan needs exactly one
master-declared final image dependency accepted earlier, never scratch_input, with artifact/path/file SHA and
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` plus source/rendered normalized-RGB digests.
Stop on mismatch; release only after the post-checkpoint gate passes; final checkpoint IDs must equal master IDs.
```
