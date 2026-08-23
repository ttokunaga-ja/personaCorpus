# personaCorpus

20人の完全合成ペルソナについて、実際の利用環境に近いフォルダと高品質な
ファイル群を制作するための共有production projectです。Kioのコードリポジトリとは
分離されており、生成物そのものは特定の検索製品に限定しません。

## 現在の制作ルート

```text
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus
├── canonical/   # Rustが生成したplan/schedule/render/materialization（Git外）
├── workspace/   # 20人×20 leaf scopeの最終制作先（Git外）
├── runbooks/    # 共通規則と20人の個別brief
├── prompts/     # 別タスクへ貼り付ける親タスク用prompt
├── progress/    # persona別のローカルcheckpoint（Git外）
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

p08／p09／p10の最初の並列waveは
[`prompts/MAC_PARALLEL_WAVE_01.md`](prompts/MAC_PARALLEL_WAVE_01.md)に固定しています。

異なるpersonaの親タスクは同時に実行できます。他personaのleaseは正常な並列状態であり、
個別タスクは自分のpersonaのleaseと成果物だけを操作します。全体の
`active_leases=0`確認は、並列batch全体が終わった後にcoordinatorが行います。

タスクを別worktreeで開始すると、Git管理外の `workspace/` がコピー・共有されません。
20個の親タスクが同じ絶対パスを共有することが、この運用の前提です。

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

制作開始前のローカル確認は `./bin/check-ready` を実行します。成功時は20 personas、
400 scopes、full profile 195,000 files、active lease数を表示します。

## 制作範囲

初回は各briefに記載したseed artifactsから開始し、その後personaごとに200ファイル
（全体4,000ファイル）を最初のmilestoneとします。full profileは長期目標です。
この段階ではKioのindex、履歴、検索品質、性能達成を主張しません。

Full追加制作はM1の200ファイルをbyte-for-byte保持し、canonical source inventory、
凍結済みM1 assignment ledger／reservation、content spine、追加assignment、batch manifestを先に確定してから
開始します。最初の検証対象は
`prompts/p01-full-pilot.md`です。Windows clone単独はGit外のcanonical、workspace、
runtime、leaseを持たないためproduction環境ではありません。別PCを使う場合は
persona単位bundle手順に従い、同じpersonaを2台で同時に担当しません。
