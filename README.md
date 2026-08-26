# personaCorpus

20人の完全合成ペルソナについて、実際の利用環境に近いフォルダと高品質な
ファイル群を制作するための共有production projectです。Kioのコードリポジトリとは
分離されており、生成物そのものは特定の検索製品に限定しません。

## 現在の制作ルート

```text
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus
├── canonical/   # portable plan/scheduleはGit管理、render/receiptはローカル
├── workspace/   # people/**/homeの受入済み成果物はGit管理、_controlはローカル
├── runbooks/    # 共通規則と20人の個別brief
├── prompts/     # 別タスクへ貼り付ける親タスク用prompt
├── progress/    # portable assignment/manifest/checkpointのみGit管理
├── manifests/   # Git管理成果物の全件SHA-256 manifest
├── scratch/     # skillの一時入力・render・視覚確認（Git外）
├── bin/         # 固定runtimeを呼ぶ薄いwrapper
└── .runtime/    # 固定済みkio-eval binary（Git外）
```

## 別タスクを開始する方法

1. Codexでこのディレクトリをプロジェクトとして開きます。
2. タスク環境は必ず **Local** を選びます。Git Worktreeは選びません。
3. `prompts/README.md` から未担当のpersonaを選び、対応するpromptを新規タスクへ
   貼り付けます。
4. 親タスクはpersona leaseを取得し、異なるleaf scopeごとにSubagentを割り当てます。
5. Subagentは指定された1フォルダと固定ファイル一覧だけを制作します。

Fullの「READY」は、**物理Full成果物を作り終えた意味ではありません**。M1の200件を
変えずに、新しいLocal taskが安全にphysical Full productionを開始できるallocation/start
package（inventory、baseline、reservation、必要なreconciliation、content spine、complete master、
production prompt）が揃った状態を指します。現在のp04--p20のpersona homeは、監査時点では
いずれもM1の200件です。

| Persona | Full準備の現在地 | 新規Local taskの扱い |
|---|---|---|
| p01--p03 | Full完成・QA受入・Git管理済み（12,000／15,000／10,000件） | production promptは履歴。明示承認された新revisionなしに再実行しない。 |
| p04 | portable partial preflight（inventory＋M1 baseline）を共有済み。Full production未開始 | M1 ledger、reservations、Full assignmentが未作成のためREADYではない。 |
| p05--p06 | Full package未作成。Full production未開始 | allocation packageの作成から開始する。 |
| p07 | READY。Full production未開始 | persona専用Full promptを新しいLocal taskへ貼り付ける。 |
| p08／p10 | machine-readable M1 ledgerのみ。Full production未開始 | allocation packageの作成から開始する。 |
| p09 | READY。Full production未開始 | persona専用Full promptを新しいLocal taskへ貼り付ける。 |
| p11--p14 | READY。Full production未開始 | 各persona専用Full promptを新しいLocal taskへ貼り付ける。 |
| p15 | READY。Full production未開始 | persona専用Full promptを新しいLocal taskへ貼り付ける。 |
| p16--p20 | READY。Full production未開始 | 各persona専用Full promptを新しいLocal taskへ貼り付ける。 |

p01--p03のFull physical productionは完了しています。それぞれの
[`prompts/p01-full-production.md`](prompts/p01-full-production.md)、
[`prompts/p02-full-production.md`](prompts/p02-full-production.md)、
[`prompts/p03-full-production.md`](prompts/p03-full-production.md)は受入済みv1 chainの
履歴・再現用資料であり、現在のhomeへ再適用しません。

異なるpersonaの親タスクは同時に実行できます。他personaのleaseは正常な並列状態であり、
個別タスクは自分のpersonaのleaseと成果物だけを操作します。全体の
`active_leases=0`確認は、並列batch全体が終わった後にcoordinatorが行います。

cloneやworktreeには受入済み成果物は含まれますが、ローカルlease topology、workspace
owner、canonical render、固定runtimeは含まれません。productionは引き続きこのLocal
rootを共有し、別worktreeを作りません。

## 最初に読む文書

- [共通制作規則](runbooks/COMMON_RULES.md)
- [batch/lease手順](runbooks/BATCH_PROTOCOL.md)
- [20人の一覧](runbooks/PERSONA_INDEX.md)
- [workspace構造](runbooks/PRODUCTION_LAYOUT.md)
- [別タスク引き継ぎ](runbooks/SESSION_HANDOFF.md)
- [Rust契約](runbooks/PERSONA_PC_CONTRACT.md)
- [Full制作protocol](runbooks/FULL_PRODUCTION_PROTOCOL.md)
- [Full台帳仕様](runbooks/FULL_LEDGER_SPEC.md)
- [別PC persona bundle](runbooks/CROSS_PC_PERSONA_BUNDLES.md)
- [Git管理境界](runbooks/GIT_MANAGEMENT.md)

制作開始前のローカル確認は `./bin/check-ready` を実行します。成功時は20 personas、
400 scopes、full profile 195,000 files、active lease数を表示します。

## 制作範囲

初回は各briefに記載したseed artifactsから開始し、その後personaごとに200ファイル
（全体4,000ファイル）を最初のmilestoneとします。full profileは長期目標です。
この段階ではKioのindex、履歴、検索品質、性能達成を主張しません。

Full追加制作はM1の200ファイルをbyte-for-byte保持し、canonical source inventory、
凍結済みM1 assignment ledger／reservation、必要時のみdigest-bound same-scope reconciliation、content spine、追加assignment、batch manifestを先に確定してから
開始します。`prompts/p01-full-pilot.md`は、既に完了した12件だけのhistoricalかつ
persona-specificな検証記録であり、新しいFull productionの開始指示ではありません。Git cloneは受入済み成果物、portable plan、
台帳、manifestを共有できますが、runtime、owner、lease topology、100 MiB超のrenderを
持たないためclone単独では正式production環境になりません。別PCを使う場合はpersona
単位bundle手順に従い、同じpersonaを2台で同時に担当しません。

受入済み成果物の全件照合は次で行います。

```bash
./bin/corpus-manifest verify --manifest manifests/corpus-sha256.jsonl
```
