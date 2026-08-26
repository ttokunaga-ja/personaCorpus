# p18 Full mass production task

Paste the following into one new Codex **Local** task opened at the
`personaCorpus` repository root. Do not continue the allocation task that
prepared this ledger. This prompt is safe for the initial run or for resuming
from accepted checkpoints.

```text
このLocalタスクでp18（manufacturing quality engineer）のFull physical-artifact
productionを担当してください。既存M1 200件をbyte-for-byteで維持したままFull追加11,800件を
制作・受入し、最終12,000 regular filesまで通常の制作、QA、差戻し、再開判定を自律的に継続
してください。これは完全に合成された防御的な品質システムコーパスです。実在組織、顧客、
従業員、患者、製造設備、資格情報、秘密、PII/PHI、未公開資料、実環境への操作手順を作りません。

これはallocationとは別のproduction親タスクです。rootは
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`を直接使い、別worktree、
別workspace、別runtime、別PC bundleを作りません。

## 最初に読むものと固定入力

最初に完全に読んでください。

- `AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`
- `runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`
- `runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p18-manufacturing-quality-engineer.md`
- `progress/p18/full/content-spine.md`、`progress/p18/full/full-assignment-ledger.jsonl`
- 存在する`progress/p18/full/checkpoints/*.json`

accepted Rust plan、materialization record、workspace ownerだけをcanonical authorityとし、
canonicalを再生成・再解釈しません。以下を開始時にSHA-256照合し、再生成、編集、置換、移動、
削除しません。

- M1 display: `58a7e217dcf5399656fba1c9d427697645bd5fe28c4f0accca7e1bf141070763`
- M1 ledger: `5954df03cedf3b56a143e4b251447c7d6f84c564cfa4033535fad9ebf86291bf`
- canonical inventory: `201c82607b7ba53bb3ec9b27b13ac3d87ba7efb59422125c1e833fc5a9323539`
- immutable M1 baseline: `0ee0d8bcc0d31dda7fd5e5ee92f79c01a172f8602379401acb70ff0ff9308c69`
- strict M1 reservations: `d6107bf1fd2ba9cce543ff1e5c92565bf8a7398faa664ef7b681d7beaf0d89c4`
- content spine: `13e745b0b2cbf69bc4054ac027387cf005b80d3693ac4b1b7dd85408cf3ee64b`
- complete Full assignment: `21e950dc8c311acc53b35e7e4348526c36bd80eb728d5f4a0cd8738cfc3f503e`

固定状態はinventory 12,000、M1 baseline/reservations 200、Full assignment 11,800、
122 batchesです。p18は**strict reservation**でありreconciliationはありません。作成・verifyの
すべてで共有`./bin/full-ledger`を使い、CLI、reservation、masterを変更しません。

初回read-only preflight:

```sh
./bin/full-ledger verify --persona p18 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/manifests/m1-baseline.jsonl
```

`result: ok`、12,000 / 200 / 11,800 / 200 を確認します。`check-ready`のglobal
`active_leases`、他personaのlease、Git cleanは条件にしません。他personaをinspect、claim、
recover、release、検証しません。p18自身のparent/scope leaseが既にあれば、何も変更せずownerを
報告して停止します。preflight合格後に一意なp18 parent leaseを通常claimし、release tokenは親
プロセス内だけに保持します。tokenをprompt、Subagent message、progress、scratch、log、artifact、
Gitに記録しません。

## Resume・並列制作・原子的受入

checkpoint 0件なら、p18 homeはexactly 200 regular files、symlink/nonregular 0でM1 baselineと
byte-for-byte一致することを確認します。checkpointがある場合は、正しいtoken-free checkpointと
そのcreate-only before/after manifestの揃ったbatchだけをacceptedとし、current manifestが
M1 baseline + accepted rowsと完全一致する場合だけ再開します。未checkpoint path、assignment外path、
symlink/nonregular、M1またはaccepted Fullの変更があれば変更せず停止します。

残り0まで次を繰り返します。

1. checkpointからaccepted batchを再構成する。
2. 各scopeの未受入batchから、外部dependencyがaccepted済みの最初のbatchを最大1件選ぶ。
3. 異なるscopeのready batchを利用可能で品質を保てるSubagent枠まで同じgroupへ並列委譲する。
   同一scopeは常に1 writer/1 batch。固定少数上限を置かない。
4. `p18-full-wave-NNN`のgroup IDで一つのcumulative before manifestをcreate-only作成する。
5. 親だけがscope leaseをclaimしてtokenを保持する。workerへtokenもlease CLI権限も渡さない。
6. workerは自batchのmaster rowsだけを、指定scope_path下にのみ制作する。別persona/別scope、
   Git、canonical、owner、runtime、root manifest、assignment、checkpointを変更しない。
7. 全workerのQA後、親が独立QAを実施する。不具合は同じworkerへ差戻し、leaseを保持して修正する。
8. 全member合格後、親scratchにcandidate-after manifestをcreate-only作成し、before/master/baseline/
   全member batchでatomic verifyする。
9. candidateがokの場合だけ公式after manifestをcreate-only作成し、candidateとbyte一致、公式after
   でも同じatomic verifyを行う。合格前にcheckpointやscope releaseをしない。
10. 合格後にmemberごとのtoken-free checkpointを書き、再読込後にscope leaseを通常releaseする。

batchのdependencyは同一scope内でstrictly earlier batchです。scan PDFはmasterで正確に1件宣言された
同一scope・先行batchのfinal PNG dependencyだけから作ります。`scratch_input`やad hocなscan sourceを
検出した場合は制作せず停止します。

## 品質・内容・停止条件

DocumentsでDOCX、PDFでsearchable/scan PDF、SpreadsheetsでXLSX、PresentationsでPPTX、ImageGenで
PNGとscan source artworkを実ファイルとして制作します。各skillを先に読み、全ページ/全sheet/
全slide/原寸画像をrenderして確認します。scan PDFはimage-onlyでtext layer 0、source pixelとの対応を
確認します。XLSXは数式・型・循環・error、PPTXはoverflow、DOCX/PDFはページレイアウトを検査します。

content spineのNorthforge Quality Labs、Orchid QMS、NCR-042、CAPA-018、ECO-009、evidence set 184、
2026-07-13〜20、97.9%、98.7%、184 msを関連資料間で整合させます。空テンプレート、placeholder、
TODO/TBD、壊れた拡張子、同一内容の水増し、secret、credential、実在PII/PHI、routable IP、実製造設備の
操作指示を作りません。

Python source QAは`ast.parse`またはメモリ内`compile(source_text, filename, "exec")`のみです。
final treeをimport/実行せず、`py_compile`、`compileall`、`python -m py_compile`を使いません。各group前後に
`__pycache__`と`*.pyc`が0件であることを確認します。

通常の生成/QA失敗は同じworker・scope leaseで診断、修正、再QAして継続します。ただしtoken喪失、通常
release失敗、recoverが必要なlease、固定input不一致、M1/accepted Full変更、assignment外file、
出所を一意に証明できないcache、secret/PII、ledger revisionが必要なdependency不一致、必須skillの反復不能
では、追加変更・recover・手動`_control`編集をせず停止し、session/scope/batch/current count/manifest diff/
last checkpoint/remaining p18 leasesを報告します。

## 完了ゲート

122 batchesを全受入後に、p18 home 12,000 regular files、Full 11,800/11,800、M1 200件不変、
checkpoint 122/122、assignment外/symlink/nonregular/pyc/cache 0、actual SHA-256全件一意、family/
skill集計とmaster一致、全skill QA合格、最終official after manifestがcurrent homeとbyte一致を確認します。
その後全scope、最後にparent leaseを通常releaseしてp18自身が不在であることを確認します。

Git add/commit/push、root corpus manifest、branch/worktree、Kioへの書込み、Kio index/history/search/chunk/
performance評価は行いません。完了時だけ、12,000件、122/122 batch/checkpoint、最終manifest SHA-256、
M1不変、QA集計、p18 lease不在を報告します。

### Mandatory v2 acceptance and resume gate

This executable gate overrides initial/resume prose. Execute before lease/write, before every group, and after
checkpoint writes before release; absence of the checkpoint directory is valid initial state:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p18 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p18-manufacturing-quality-engineer/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p18/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, pin shared `bin/full-ledger`
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` then use the exact absolute-path
verifier above with before/all group batch-id/after. Each exact master batch gets exactly one create-only
`persona-corpus.full-batch-checkpoint/v2` `<batch-id>.json` with scope/batch, shared before/after, plan_digest,
counts/family_counts, all-true `{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC
accepted_at, before/after byte SHA fields, and scan_provenance. Each pdf_scan needs exactly one master-declared
final image accepted earlier, never scratch_input, with artifact/path/file SHA and
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` plus source/rendered normalized-RGB digests.
Stop on mismatch; post-checkpoint gate before release; completion needs exact checkpoint/master batch-ID equality.
```
