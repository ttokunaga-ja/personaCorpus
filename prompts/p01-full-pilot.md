# p01 Full pilot start prompt

以下を新しいCodex Localタスクへ貼り付けてください。

```text
このpersonaCorpus Localタスクでp01（software-engineer）のFull追加pilotを担当して
ください。作業ディレクトリは必ず
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus のまま使用し、Git
worktreeや別workspaceを作成しないでください。commit・pushもしません。

AGENTS.md、README.md、prompts/FULL_PARENT_TEMPLATE.md、
runbooks/COMMON_RULES.md、runbooks/FULL_PRODUCTION_PROTOCOL.md、
runbooks/FULL_LEDGER_SPEC.md、runbooks/personas/p01-software-engineer.md、
prompts/p01-full-pilot-content-spine.md、
prompts/p01-full-pilot-assignment.jsonlを完全に読んでください。今回のcontent spineと
12-row assignmentは追跡済みの固定入力です。内容を再設計・再割当・上書きせず、
assignmentにあるfull source ID、scope、path、family、依存関係をそのまま使います。

最初に `./bin/check-ready` を実行してください。p01自身のactive parent/scope leaseが
なく、p01 final rootがexactly 200 filesであることを確認します。他personaのactive
leaseは正常な並列作業なので、show、recover、release、停止理由にしません。

次のcreate-only記録用ディレクトリを用意し、Full ledger helperでpreflightします。
既存出力がある場合は削除・上書きせず、内容とcheckpointを確認して安全なresumeかを
判断してください。以下の各生成commandは対応する出力が存在しない場合だけ実行します。
coordinatorが事前生成済みなら、既存JSONLのdigest・header・全行を検証して再利用し、
create-only拒否をエラー回避のために迂回してはいけません。
旧`m1-family-map.jsonl`、`m1-reservations.jsonl`、`m1-reservations-v2.jsonl`が残っていても
変更しません。このpilotでは新しいM1 assignment digestを持つ
`m1-assignment-ledger-v2.jsonl`と`m1-reservations-v3.jsonl`だけを入力として使います。

mkdir -p progress/p01/full/checkpoints \
  scratch/p01/full/manifests scratch/p01/full/qa scratch/p01/p01-primary-01

./bin/full-ledger inventory --persona p01 \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/canonical-source-inventory.jsonl

./bin/full-ledger manifest --persona p01 \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.before.jsonl

./bin/p01-m1-assignment \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/m1-assignment-ledger-v2.jsonl

./bin/full-ledger reserve-m1 --persona p01 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/m1-assignment-ledger-v2.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.before.jsonl \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/m1-reservations-v3.jsonl

./bin/full-ledger verify --persona p01 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/m1-reservations-v3.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/prompts/p01-full-pilot-assignment.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.before.jsonl

preflightがinventory 12,000 sources、M1 reservations 200、assignment 12、baseline
200で合格した場合だけ制作へ進みます。parent leaseを一意sessionで取得し、今回唯一の
scope `p01-primary-01`（`documents/work/product-alpha/architecture`）をclaimしてから、
その1フォルダを1 Subagentへ排他的に委譲してください。他scope、他persona、canonical、
runtime、`_control`を直接編集しません。同じscopeへ並行workerを起動しません。

Subagentはassignmentの12 pathだけを追加します。JSON/CSVを基礎データとして先に作り、
XLSX/DOCX/searchable PDF/PPTXに同じ事実を反映してください。2つのPNGはImageGenで
実ファイルとして制作し、`p01-full-pilot-011`のscan PDFは
`p01-full-pilot-010`のPNGを元画像としてPDF skillで作ります。Documents、PDF、
Spreadsheets、Presentations、ImageGenの各該当skillを担当agent自身が完全に読み、
所定のrender、全ページ・全sheet・全slide・原寸画像の視覚QAまで行います。

親は12件の形式、開封、content-spine整合、依存関係、構造、視覚QAを確認します。その後
`./bin/full-ledger manifest`で同batch IDのcreate-only after JSONLを作成し、下記の
完全なafter検証commandを実行してください。単独の`--after`行で代用しません。

./bin/full-ledger manifest --persona p01 \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.after.jsonl

./bin/full-ledger verify --persona p01 \
  --inventory /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/canonical-source-inventory.jsonl \
  --m1-assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/m1-assignment-ledger-v2.jsonl \
  --reservations /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/progress/p01/full/m1-reservations-v3.jsonl \
  --assignment /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/prompts/p01-full-pilot-assignment.jsonl \
  --baseline /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.before.jsonl \
  --before /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.before.jsonl \
  --after /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p01/full/manifests/p01-primary-01-full-pilot-b001.after.jsonl

その自動検証と親QAで次を確認してください。

- afterはexactly 212 files
- beforeの200 pathはすべて同じSHA-256・bytesで残る
- 新規12 pathはassignmentの12 pathと完全一致する
- assignment外の変更・追加・削除、exact SHA-256 duplicateは0
- 全QAが合格し、placeholder、秘密、資格情報、実在PIIは0

合格後だけtoken-free checkpointを
`progress/p01/full/checkpoints/p01-primary-01-full-pilot-b001.json`へcreate-onlyで記録し、
scopeとparent leaseを正式CLIで通常releaseします。release tokenは成果物、prompt、
progress、scratch、ログ、Subagentメッセージへ記録しません。recoverは実際の中断が
起きても明示承認なしに実行しません。中断を意図的に発生させる試験もしません。

完了報告にはbefore/after manifestのSHA-256、200 M1不変、追加12件、各skill QA、
p01の残りFull 11,788件は未割当・未制作であること、自分のleaseが全解放済みであることを
含めてください。Kio index/history/search/chunk/performanceの達成を主張しません。
```
