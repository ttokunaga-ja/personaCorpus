# p13 Full mass production task

Paste the text below into **one new Codex Local task** opened at the
`personaCorpus` repository root. Do not continue the allocation task.

```text
このLocal taskでp13（corporate privacy counsel）のFull physical-artifact
productionを担当してください。既存M1 200件をbyte-for-byte保持したまま、Full追加6,800件を
制作・QA・受入し、最終7,000 regular filesまで自律的に継続します。通常の制作・QA修正は
ユーザー確認を求めず進めますが、下記の停止条件では追加変更をせず証跡を報告します。

作業rootは必ず
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`です。別worktree、別workspace、
別runtime、別PC bundleを使いません。まず`AGENTS.md`、`README.md`、
`runbooks/COMMON_RULES.md`、`runbooks/FULL_PRODUCTION_PROTOCOL.md`、
`runbooks/FULL_LEDGER_SPEC.md`、`runbooks/SESSION_HANDOFF.md`、
`runbooks/personas/p13-corporate-privacy-counsel.md`、
`progress/p13/full/content-spine.md`、
`progress/p13/full/full-assignment-ledger.jsonl`を完全に読んでください。

## 固定authorityとpreflight

canonical plan/render/materialization/workspace ownerだけがauthorityです。canonicalを再生成・
再解釈せず、次の固定inputをSHA-256照合して編集・置換・移動・削除しません。

- M1 human display: `a02171eca9778f2674bd1ef60d3c0b7d4d77eb51c2b4fa4712227f7cb05c7096`
- shared `bin/full-ledger`: `f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2`
- frozen M1 ledger: `a8c5680d8864f3051d3634a567e78008512f578aaffb430f8e83185279844ec2`
- source inventory: `f6a11d4d3bea53187d04d9eb0ef650d3af1ed6a9346558af59f60787eebbead7`
- immutable M1 baseline: `3f55fd69103091b5da732243ac0d357b8b858bf368524404b945a5b3fd1f147a`
- strict M1 reservations: `4f5bc261a550352c2adfb1f875705c819ba35c1dcfe4a88e99abcfc0b3a31c25`
- content spine: `8ebbe048ecb66bbcfdde3d62804c5df7a5ed5b8ed88250beff50c050aca54cfa`
- complete Full assignment: `728f65d075489d5dfdfaf4c9ada2b6427d92229dc382b6322da5efb6182b5158`

p13はreconciliationなしのstrict reservationです。inventory 7,000、M1 baseline/reservation
200、Full assignment 6,800、104 batchesが固定です。初回のFull残量は6,800、最終目標は
7,000 filesです。初回preflightと各acceptance groupのcandidate/official verifyの直前に、必ず
次の実行可能ファイルgateを通します。SHA不一致ならそのhelperを実行・編集・置換・回避せず停止します。

~~~sh
expected_full_ledger_sha=f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2
actual_full_ledger_sha=$(shasum -a 256 ./bin/full-ledger | awk '{print $1}')
if [ "$actual_full_ledger_sha" != "$expected_full_ledger_sha" ]; then
  printf '%s\n' "p13 preflight: bin/full-ledger SHA-256 mismatch" >&2
  exit 1
fi
~~~

gate合格後、次をread-only preflightとして実行して`result: ok`を確認します。

./bin/full-ledger verify \
  --persona p13 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/m1-assignment-ledger.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/m1-reservations.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/full-assignment-ledger.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/manifests/m1-baseline.jsonl

global `active_leases`と他personaのGit変更は情報に留め、他personaをshow/claim/recover/release/
検証しません。p13自身のparentまたはscope leaseが存在する場合だけ、何も変更せず所有sessionを
報告して停止します。preflight後に一意なp13 parent leaseをclaimし、release tokenは親プロセス
だけに保持します。tokenをprompt、subagent message、progress、scratch、log、成果物、Gitへ記録しません。

初回はp13 homeがexactly 200 regular files、symlink/nonregular 0で、現在manifestがM1 baselineと
byte-for-byte一致することを確認します。再開時はcheckpointとそのcreate-only before/after manifestが
すべて揃うbatchだけをacceptedとし、現在manifestがM1 baselineとaccepted rowsの和に完全一致することを
確認します。checkpointなしのdelta、assignment外path、M1/accepted file変更、symlink/nonregularが
1件でもあれば停止します。

## 自律production loop

Full完了まで、各scopeで未受入かつ外部dependencyがaccepted済みの最初のbatchを最大1件選び、
異なるscopeのready batchを利用可能なSubagent枠まで同時acceptance groupとして進めます。固定の
並列数上限は置きません。同一scopeは常に1 workerです。親だけがscope leaseをclaim/releaseし、
tokenは渡しません。workerはcomplete masterから自分のbatchのrowsだけを読み、指定scope_path配下だけに
制作します。canonical、assignment、manifest、checkpoint、Git、Kio、他persona/他scopeを操作させません。

group開始前にcreate-only cumulative before manifestを作成します。全workerの独立QA後、親はscratchで
candidate after manifestを作り、全memberの`--batch-id`を一つの`full-ledger verify`へ渡してatomicに
検証します。candidate合格後だけ公式afterをcreate-only作成し、同じverifyを再実行します。次に
token-free checkpointをbatchごとにcreate-only記録し、再読込してからscope leaseを通常releaseします。
groupの一件でも未合格なら公式after/checkpointを作らず、部分accept/releaseしません。

依存関係はmasterの同一scopeかつ厳密に先行するbatchのみです。structured/CSV factsをXLSX/DOCX/
searchable PDF/PPTXの共通根拠とし、ImageGen imageをscan PDFより先に作ります。scan PDFは1,020件で、
すべて同一scopeのfinal ImageGen source imageへのdependencyを持ち、text layerを含めません。

## 制作品質

workerは該当skillのSKILL.mdを読んで、DOCXはDocuments、PDF text/scanはPDF、XLSXはSpreadsheets、
PPTXはPresentations、PNGとscan原画はImageGenで実ファイルとして制作します。一時source/render/QAは
`scratch/p13/<scope-id>/<batch-id>/`、親集計QAは`scratch/p13/full/<group-id>/`だけに置きます。

content spineのAster Legal、Matter Alpha/Beta、AC-27、2026-06-30、2026-07-12、hold 42、evidence 184、
RISK-031を、そのscope契約にだけ従って一貫させます。`primary-01`/`primary-02`/`primary-11`だけが
Alpha preservation/holdを使え、`primary-03`〜`primary-10`だけがBeta contract/privacy/RISK-031を使えます。
`primary-12`と全secondary scopeは両matter anchorを使いません。この真理値表に反するcontent-spine ID、
文言、dependency由来のcross-carryを検出した時は、初回・resumeとも制作前に停止します。すべて合成の防御・ガバナンス目的の内容とし、実在PII/PHI、
資格情報、秘密、特権資料、実在未公開資料、法的助言、routable IP、placeholder/TODO/TBD、空テンプレート、
壊れた拡張子を作りません。Kio index/history/search/chunk/performanceの達成を主張しません。

worker/親とも、PDF全pageとtext layer/scan分類、DOCX全page、XLSX全sheet/formula/type/error、
PPTX全slide/overflow、PNGのpixel/dimension/原寸を確認します。Python sourceは文字列を`ast.parse`又は
in-memory `compile`するだけにし、final treeをimport/executeせず、py_compile/compileallを使いません。
全QA後にfinal treeの`__pycache__`と`*.pyc`が0件であることを確認します。

通常の生成/構文/render/数式/overflow/内容品質エラーは同じworkerと同じleaseで修正して継続します。
ただしtoken喪失・通常release失敗、stale/unknown p13 lease、固定input不一致、M1/accepted変更、
assignment外file、dependency不成立、secret/実在PII、必須skillの反復不能、重複worker危険では、recover・
手動_control編集・未受入final削除をせず停止し、session/scope/batch/checkpoint/manifest差分/leaseを報告します。

最終ゲートはp13 home 7,000 regular files、M1 200 files不変、Full 6,800/6,800、checkpoint 104/104、
assignment外0、symlink/nonregular 0、actual SHA-256 7,000 unique、family/skill集計一致、全QA合格、
最終official afterと現在manifest一致です。最後にp13 scope全部とparentを通常releaseして不在を確認します。
Git add/commit/push、root corpus manifest、branch/worktree、Kio書込み/評価は実行しません。

### Mandatory v2 acceptance and resume gate

The executable gate below supersedes initial/resume wording. Before lease/write, every group, and after
checkpoint writes before release (no checkpoint directory is valid initial state), fail closed:

```sh
test "$(shasum -a 256 ./bin/full-resume-gate | awk '{print $1}')" = 3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069 || exit 1
./bin/full-resume-gate --persona p13 --home /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace/people/p13-corporate-privacy-counsel/home --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/manifests/m1-baseline.jsonl --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/full-assignment-ledger.jsonl --checkpoints-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/checkpoints --manifests-dir /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p13/full/manifests --require-scan-provenance
```

Before every initial/candidate/official verify, re-pin shared `bin/full-ledger`
`f3481bded3b49133e66333004e170e63e406a8eb8ced77782ab3929d2e2059a2` and run the exact absolute-path command
above with before/all group batch-id/after. One create-only `persona-corpus.full-batch-checkpoint/v2` checkpoint `<batch-id>.json` per exact master
batch records scope/batch, shared before/after, plan_digest, counts/family_counts, all-true
`{structure,family,content_spine,dependencies,skill_required,secrets_pii}`, UTC accepted_at, before/after byte
SHA fields, and scan_provenance. Every pdf_scan has exactly one master-declared final image accepted earlier,
never scratch_input, with artifact/path/file SHA and `pixel_qa:{passed,width,height,method:"normalized-rgb-sha256/v1"}`
and source/rendered normalized-RGB digests. Stop on mismatch. Release only after the post-checkpoint gate;
completion requires checkpoint IDs equal master IDs exactly.
```
