# p01 Full master allocation task (completed archive)

> The 11,800-row master, content spine, and 12,000-file p01 Full corpus are
> accepted. Do not paste or rerun this allocation prompt or its historical
> production prompt.

The archived text below froze allocation only; it must not be reused to create,
alter, or delete a final corpus artifact.

```text
このpersonaCorpus Localタスクで、p01 Full mass productionの前提となる完全master
assignmentを作成・検証してください。作業rootはCodexで開いた現在のpersonaCorpus
repository rootのままにし、別machine由来のabsolute path、別worktree／別workspace／
別runtimeを使わず、git add／commit／pushを実行しません。

最初にAGENTS.md、README.md、runbooks/COMMON_RULES.md、
runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、
runbooks/GIT_MANAGEMENT.md、runbooks/personas/p01-software-engineer.md、
prompts/p01-full-pilot-content-spine.md、prompts/p01-full-pilot-assignment.jsonl、
progress/p01/full/checkpoints/p01-primary-01-full-pilot-b001.jsonを完全に読んでください。

これはallocation-only taskです。workspace/people以下のfinal fileを1 byteも変更せず、
Documents/PDF/Spreadsheets/Presentations/ImageGenのartifact productionも開始しません。
別personaのleaseや未commit変更を正常な並列状態として扱い、他personaをshow／recover／
release／検証しません。p01自身のparent/scope leaseがないことと、p01が正確に212 filesで
あることを確認してください。

tracked evidenceを使い、最初に次を再検証してください。

- `progress/p01/full/manifests/m1-baseline.jsonl`: 200 files
- `progress/p01/full/manifests/p01-primary-01-full-pilot-b001.after.jsonl`: 212 files
- before SHA-256 `6b59e44cd520134928b710303f0ddaa2890f006d58d7c582dc07bc280f429bec`
- after SHA-256 `4dae33fac0543a2f692d55c5159e70d40f5d2708ae1429936ba16272db44affb`
- pilot assignment 12 rowsと実在追加12 filesの完全一致
- M1 200 filesのpath／bytes／SHA-256不変

p01に一意なparent sessionを使ってparent leaseだけを取得し、allocation作業中のp01
artifact productionを排除してください。final leafを書かないためscope leaseは取得しません。
release tokenはプロセス内だけに保持し、ファイル、prompt、ログ、Subagentメッセージへ
記録しません。

既存なら上書きせず停止して照合するcreate-only方針で、次を固定してください。

1. `progress/p01/full/content-spine.md`
   - pilot spineを保持・包含し、p01全20 scopeで再利用する完全合成のorganization、project、
     date、ID、measure、terminology、cross-format relationshipを定義する。
2. `progress/p01/full/full-assignment-ledger.jsonl`
   - schema `persona-corpus.full-addition-assignment/v1`
   - header 1行、`persona:"p01"`、`artifact_count:11800`、`pilot:false`
   - inventory 12,000 sourceからfrozen M1 reservation 200 sourceを除いた11,800 sourceを
     1回ずつ完全割当する。
   - 受入済みpilot 12 artifact rowはartifact_id、source_id、scope、path、family、extension、
     batch_id、dependencies、content_spine_ids、skill_requiredをfield-for-field保持する。
   - 残り11,788 rowはcanonical scope_path内の一意なhome-relative path、canonical physical
     extension、安定artifact ID、100--500件を標準とするscope-local batch IDを持つ。
   - 高コストのPDF／Office／画像batchはQA可能な小さい境界へ分割してよい。
   - CSV／structured facts → XLSX／DOCX／searchable PDF／PPTX、ImageGen PNG → scan PDFの
     dependencyを明示し、dependencyはmaster内で解決し、batch順序と矛盾させない。
   - 既存M1 path、pilot以外の既存path、path traversal、重複path、ad hoc拡張子推測を禁止する。

allocationをSubagentへ並列委譲する場合は、各Subagentを異なるRust scope IDのledger
draftだけに限定し、親だけが完全masterを統合します。Subagentはfinal file、lease、canonical、
root manifest、他scope draftを編集しません。

統合後、`./bin/full-ledger verify`をinventory、p01 M1 assignment v2、reservation v3、
tracked M1 baseline、complete master assignmentへ実行し、`result: ok`と11,800/11,800の
完全割当を確認してください。続けてpilot batchをtracked before/after manifestで再検証し、
現在のp01 212 filesと一致させてください。family／scope／batch／skill_required集計、ledger
SHA-256、pilot 12 row不変、final artifact差分0を報告します。

全検証後にp01 parent leaseを正式CLIで通常releaseし、自分のp01 parent/scope leaseが
残っていないことを確認してください。recover、final production、root corpus manifest更新、
Git操作、Kio index/history/search/chunk/performance評価は実行しません。
```
