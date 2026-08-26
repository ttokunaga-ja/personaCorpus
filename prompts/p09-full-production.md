# p09 Full mass production task

Open one new Codex **Local** task directly at the `personaCorpus` repository
root, then paste the following instructions. This is a production parent task,
not the allocation task that created this prompt.

```text
このLocalタスクでp09（UX researcher）のFull physical-artifact productionを担当する。
既存M1 200件をbyte-for-byte保持したまま、Full追加8,800件を制作・QA・checkpoint受入し、
最終9,000 regular filesまで通常の制作とQA修正を自律的に進める。別worktree、別workspace、
別runtime、Git stage/commit/push、Kio評価、canonical/owner/runtime/root manifestの変更はしない。

最初に `AGENTS.md`、`README.md`、`runbooks/COMMON_RULES.md`、
`runbooks/FULL_PRODUCTION_PROTOCOL.md`、`runbooks/FULL_LEDGER_SPEC.md`、
`runbooks/SESSION_HANDOFF.md`、`runbooks/personas/p09-ux-researcher.md`、
`progress/p09/full/content-spine.md`、`progress/p09/full/full-assignment-ledger.jsonl`、既存checkpointを読む。
canonical plan/render/materialization/ownerが唯一のauthorityであり、再生成・再解釈しない。

開始時に次の固定値をSHA-256で照合する。異なればp09 final artifactを変更せず停止・報告する。

- M1 machine ledger: `92c53b2f54052827d5ad61beda6ce4487f2bd4733856abb954e5774a403f9892`
- inventory（9,000 source / 9,001 JSONL lines）: `4af41ba028cbee0cb3995771bdff9bd00dbf8e71361154b66d8607d0a18cff45`
- immutable M1 baseline（200 files、61,354,069 bytes）: `9887ce0ed530fb144d4bab778b510e56844c920e084f08e9e70d6a9c64ba7003`
- M1 strict reservations（200 reservations）: `fe2f6eda52bc5f9a5c6fe87209ca1a55f1fafecde5a5af452bf29d6828dc36d9`
- content spine: `37d06034f0cf58f1bd2fbfd65a6fe5f6474d62ba321a05184d67c0f5860d82bb`
- complete Full master（8,800 rows / 102 batches）: `e2111c8986e4d74d1a04c48d26f1c57b3089f1383ec93bac4e4236f8113147b4`

p09はstrict reservationでありreconciliation recordは存在しない。追加例外、family推測、
reservationの再生成・編集・置換をしない。次のpreflightが `result: ok` になることを確認する。

./bin/full-ledger verify \
  --persona p09 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/manifests/m1-baseline.jsonl

初回はp09 homeが200 regular files、symlink/nonregular 0、現在manifestがM1 baselineと
byte-for-byte一致することを確認する。resume時はcheckpointとそのbefore/after manifestが完全な
batchだけをacceptedとして再構成する。現在manifestはM1 baselineとaccepted assignment pathsの
正確な和でなければならない。checkpointなしpath、assignment外path、M1またはaccepted Fullの
bytes/SHA変更、symlink、nonregular、pyc、`__pycache__`を発見したら停止し、勝手に削除・recover・
再claimしない。

preflight後にp09自身のparent/scope leaseが不在であることだけを確認する。他personaのleaseや
working-tree変更は正常な共有状態であり、show/claim/recover/release/検証しない。`check-ready`の
global active_leasesとGit cleanは開始/完了条件ではない。親parent leaseを一意sessionで正式CLIから
claimし、release tokenは親プロセス内だけに保持する。tokenをprompt、subagent message、artifact、
progress、scratch、log、Gitに絶対書かない。token/sessionを失ったら停止し、recoverには明示承認を得る。

残りが0になるまで次を繰り返す。

1. checkpointからaccepted batchを再計算する。
2. 各scopeについて、未acceptedかつ外部dependencyがaccepted済みの先頭batchを最大1つ選ぶ。
3. readyな異なるscope batchを品質を保てる全Subagent枠へ並列化する。同一scopeは常に1 writer・1 batch。
4. group IDを`p09-full-wave-NNN`で採番し、worker起動前にcumulative before manifestをcreate-onlyで作る。
5. 親だけが選択scopeのleaseをclaimし、1 batchを1 Subagentに委譲する。workerへrelease tokenやlease CLI権限を渡さない。
6. workerはcomplete masterから自分のbatch rowsだけを制作し、そのscope_path配下以外へ書かない。
7. 全worker完了後、親が構造・content spine・family・視覚QAを行う。不備は同workerへ差戻してgroup leaseを維持する。
8. 合格member全体についてscratchにcandidate-after manifestをcreate-onlyで作り、complete master/baseline/before/全batch IDを一回のatomic verifyで検証する。
9. candidateが合格した場合だけ公式after manifestをcreate-onlyで生成し、candidateとbyte-equal、公式afterでもverify合格を確認する。
10. 各batchのtoken-free checkpointをcreate-onlyで書き、shared acceptance_group と before/after相対path、family QA、UTC時刻、M1不変を記録する。
11. checkpoint再読込後に各scope leaseを通常releaseする。親leaseは全child scopeが不在になってから通常releaseする。

masterのdependenciesはすべて同一scopeかつstrictly earlier batchである。batchのdependencyに従い、
CSV/structured factsを先に作り、XLSX/DOCX/searchable PDF/PPTXはその数値・日付・IDを共有する。
ImageGenのscope-local final PNGを先に作り、scan PDFは同一scope imageに画素整合した派生物にする。
全scan rowはmasterで宣言された同一scope・先行batchのfinal image dependencyを正確に1件使う。
この条件を満たさないrow、`scratch_input`、ad hocなscan sourceを検出した場合は制作せず停止する。

DOCXはDocuments、PDF text/scanはPDF、XLSXはSpreadsheets、PPTXはPresentations、PNG/scan sourceは
ImageGenで実ファイルとして制作し、各skillのSKILL.mdを読んでから使う。PDFはpage render/text-layer
またはscan OCR suitability、DOCXはrender、XLSXはsheet/formula/error/type、PPTXはrender/overflow、
imageはdimensions/pixels/visual inspectionを必ず確認する。temporary input/render/QAは
`scratch/p09/<scope>/<batch>/`または`scratch/p09/full/<group>/`だけに置く。

`Beacon Field Research`、`Study Alpha/Beta`、`2026-07-13`、`session UX-184`、`n=24`、`83%`、`0.8`をcontent spineどおり
整合させる。各batchの作成前に、そのrowの`content_spine_ids`を読み、Study Alpha/Beta は対応する
programme/relationshipだけを使う。survey/design/transcript/consentはspineのcoverage contractに従って同一の
安全な測定意味をcross-formatで再利用し、未定義の数値・study・sessionを導入しない。実在人物・参加者・音声・顔・連絡先・同意書・資格情報・秘密・PHI/PII・顧客資料・
routable IP・placeholder・TODO/TBD・lorem ipsum・text substitute with rich extensionを作らない。
Python QAはAST等のread-only解析を使い、`py_compile`やimport実行で`__pycache__`/`.pyc`を生成しない。

完了条件は9,000 regular files、M1 200件のbytes/SHA不変、Full 8,800件すべてexactly once、全102 batch
checkpoint/manifest/verify合格、actual SHA重複0、assignment外/symlink/nonregular/cache 0、p09 parent/scope
lease不在である。他personaを操作せず、Git操作・Kio達成主張をせず結果と証跡pathだけを報告する。

### Mandatory v2 acceptance and resume gate

The following executable rules supersede any weaker initial/resume wording above. Before **any** lease
claim or final-tree write, before every acceptance group, and again after writing the v2 checkpoints but
before a scope release, fail closed on the pinned gate and run it exactly as follows (a missing checkpoint
directory is the valid initial state):

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p09 \
  --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p09-ux-researcher/home \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/manifests/m1-baseline.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/full-assignment-ledger.jsonl \
  --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/checkpoints \
  --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p09/full/manifests \
  --require-scan-provenance
```

Before every initial, candidate, and official verification, pin the shared verifier SHA
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` and invoke the exact absolute-path
`./bin/full-ledger verify` command shown above (with the relevant `--before`, every group `--batch-id`, and
`--after` arguments). A mismatch stops without accepting or releasing. Each exact master batch has exactly
one create-only `progress/p09/full/checkpoints/<batch-id>.json`, schema
`persona-corpus.full-batch-checkpoint/v2`, with its exact scope/batch, shared group before/after manifests,
plan_digest, exact counts and family_counts, all-true QA keys `{structure,family,content_spine,dependencies,skill_required,secrets_pii}`,
UTC `accepted_at`, before/after manifest byte-SHA fields, and `scan_provenance`.

For every `pdf_scan`, acceptance requires exactly one master-declared final image dependency accepted in an
earlier group (never `scratch_input`), and one provenance record binding artifact/path/file SHA plus
`pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}` and source/rendered normalized-RGB
digests. Any mismatch stops. Completion requires the checkpoint batch-ID set to equal the complete master
batch-ID set exactly, not merely the same count.
```
