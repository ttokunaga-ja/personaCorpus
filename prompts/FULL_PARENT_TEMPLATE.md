# Full production parent task template

Replace every placeholder before starting a Local parent task.

```text
`personaCorpus` の <PERSONA_ID>（<ROLE>）について、既存M1を保持したFull追加
production batchを担当してください。このタスクは
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus を直接使用する
Localタスクです。別Git worktree・別workspace・canonical再生成はしません。

受入済み成果物とportable evidenceはGit管理されていますが、このproductionタスクは
git add／commit／pushを実行しません。他personaの未commit変更を正常な並列状態として扱い、
Git cleanを開始・完了条件にしません。自分のpersona以外とroot corpus manifestは編集せず、
wave coordinatorの受入を待ってください。

最初にAGENTS.md、README.md、runbooks/COMMON_RULES.md、
runbooks/FULL_PRODUCTION_PROTOCOL.md、runbooks/FULL_LEDGER_SPEC.md、
対象personaのcontent spine、fixed Full assignment、<BRIEF_PATH>を完全に読んでください。
別PC bundleを扱う場合だけrunbooks/CROSS_PC_PERSONA_BUNDLES.mdも読んでください。
canonical/persona-plan.jsonとworkspace/persona-workspace-owner.jsonがauthorityです。
Full protocol/ledgerに未確定の項目があれば制作せず、coordinatorへ報告してください。

<PERSONA_ID> の既存M1 200 final filesのSHA-256 baselineを制作前に検証し、batch
完了後にも同一性を検証してください。M1ファイルを上書き、移動、削除、改名せず、
fixed Full assignmentにある追加行だけを作成してください。開始前に次をcreate-onlyで
生成・検証します。既存なら上書きせず内容を照合してください。

- `./bin/full-ledger inventory` → canonical source inventory
- Full開始時に一度だけ`./bin/full-ledger manifest` → immutable 200-file
  `progress/<PERSONA_ID>/full/manifests/m1-baseline.jsonl`（後続batchの累積beforeで置換しない）
- frozen M1 assignment ledger
  `progress/<PERSONA_ID>/m1-assignment-ledger.jsonl`（header + 200 artifact rows、拡張子推測は禁止）
- `./bin/full-ledger reserve-m1 --m1-assignment ...` → 200-row M1 reservation ledger
- mass ledger全体への`./bin/full-ledger verify --m1-assignment ...` → inventoryからM1予約を
  除いた全sourceの完全割当とbaseline整合

mass productionではcontent spine、全Full追加の固定名・形式・scope・依存関係、
100--500件を標準とするbatchをtoken-free assignmentとしてpersona-owned progressへ
確定してください。p01の最初の12件だけは追跡済みpilot assignmentを使う例外です。
ledgerはcanonicalの代替ではありません。

親は一意session名で <PERSONA_ID> parent leaseを取得し、scopeごとにclaimしてから
Subagentへ委譲します。workerは自分の固定scope leafだけに、checkpointで定めた追加
ファイルだけを作ります。親は各scope batchをQAし、`./bin/full-ledger manifest`の
`progress/<PERSONA_ID>/full/manifests/`へ累積before/after JSONLをcreate-onlyで作成し、
完全master assignment、current `--batch-id`、
immutable 200-file `--baseline`、`--before`、`--after`を指定した
`full-ledger verify`でM1・既存Full不変と今回の追加pathを確定してから
token-free checkpointを記録し、通常のscope releaseを完了してから
次batchをclaimします。release tokenをprompt、成果物、progress、ログ、
Subagentメッセージへ書いてはいけません。

異なるscopeを並列制作する場合は、全worker開始前に1つのshared beforeを取得し、
全workerと親QA完了後、各memberの`--batch-id`を繰り返してunionを1つのafterに対して
atomicに検証します。group全体が合格するまでmemberを個別accept/releaseしません。

他personaのactive leaseは並列productionでは正常です。開始・完了で検査するのは
自分のpersonaのparent/scope leaseだけであり、他personaをshow、claim、recover、
release、検証対象にしません。個別完了条件にglobal active_leases=0を使わず、
全体coordinatorだけがparallel wave終了後に確認します。

DOCX/PDF/XLSX/PPTX/画像は対応するDocuments/PDF/Spreadsheets/Presentations/
ImageGen skillで実ファイルを作り、skill所定のrender/visual QAを完了してください。
CSV等の基礎データと同一の数値・日付・IDを関連Office/PDF/画像成果物で整合させます。
temporaryとrender出力はscratch/<PERSONA_ID>/<scope-id>/に置きます。完全合成の
内容だけを使い、PII、秘密、資格情報、実在の非公開資料を入れません。

中断・token喪失・stale leaseは、対象scopeと停止したwriterを特定してcoordinatorへ
報告し、明示承認なしにrecoverしません。通常releaseできないまま次のclaimを行いません。
Kio index/history/search/chunk/performanceの達成は生成・QA結果から主張しません。
終了時は自分の全scopeとparentを通常releaseし、before/after manifest差分、M1
baseline不変、追加batch件数、skill QA、未完了項目を報告してください。
```
