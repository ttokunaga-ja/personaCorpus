# Parent task prompts

1ファイルを1つの新しいCodexタスクへ貼り付けます。すべてのタスクは
`personaCorpus` プロジェクトの **Local** 環境で開始し、Worktreeは使いません。
開始前にルートで `./bin/check-ready` が成功することを確認してください。

## Full production prompt index（監査時点）

ここでのREADYは、allocation/start packageが完成している意味です。Full physical artifactの
完成ではなく、下表のp04--p20 homesはすべてM1 200件のままです。各行は必ず**新しいLocal task**で
開始し、同じpersonaを別task・別PCで同時に扱いません。

| Persona | 状態 | Full promptの扱い |
|---|---|---|
| p01 | Full完成・Git管理済み（12,000件） | production／pilot promptは履歴。再実行しない。 |
| p02 | Full完成・Git管理済み（15,000件） | production promptは履歴。再実行しない。 |
| p03 | Full完成・Git管理・remote push済み（10,000件） | production promptは履歴。再実行しない。 |
| p04 | portable partial preflight共有済み、production未開始 | M1 ledger、reservations、Full assignmentが未作成。READYではない。 |
| p05--p06 | package未作成、production未開始 | allocation packageを作る。 |
| p07 | READY、production未開始 | [p07-full-production.md](p07-full-production.md)を新しいLocal taskへ貼り付ける。 |
| p08／p10 | M1 ledgerのみ、production未開始 | allocation packageを完成させる。 |
| p09 | READY、production未開始 | [p09-full-production.md](p09-full-production.md)を新しいLocal taskへ貼り付ける。 |
| p11--p14 | READY、production未開始 | 対応する`pNN-full-production.md`を新しいLocal taskへ貼り付ける。 |
| p15 | READY、production未開始 | [p15-full-production.md](p15-full-production.md)を新しいLocal taskへ貼り付ける。 |
| p16--p20 | READY、production未開始 | 対応する`pNN-full-production.md`を新しいLocal taskへ貼り付ける。 |

p01の12件pilot prompt／content spine／assignmentはhistoricalかつp01専用です。他personaの
開始やp01のmass productionに再利用しません。

| ID | Persona | Start prompt |
|---|---|---|
| p01 | Software engineer | [prompt](p01-software-engineer.md) |
| p02 | Site reliability engineer | [prompt](p02-site-reliability-engineer.md) |
| p03 | Security/GRC analyst | [prompt](p03-security-grc-analyst.md) |
| p04 | ML research engineer | [prompt](p04-ml-research-engineer.md) |
| p05 | BI/data analyst | [prompt](p05-bi-data-analyst.md) |
| p06 | Life-science researcher | [prompt](p06-life-science-researcher.md) |
| p07 | Humanities researcher | [prompt](p07-humanities-researcher.md) |
| p08 | Product manager | [prompt](p08-product-manager.md) |
| p09 | UX researcher | [prompt](p09-ux-researcher.md) |
| p10 | Management consultant | [prompt](p10-management-consultant.md) |
| p11 | Account executive | [prompt](p11-account-executive.md) |
| p12 | Support/success lead | [prompt](p12-support-success-lead.md) |
| p13 | Corporate privacy counsel | [prompt](p13-corporate-privacy-counsel.md) |
| p14 | Finance controller | [prompt](p14-finance-controller.md) |
| p15 | Recruiter/People Ops | [prompt](p15-recruiter-people-ops.md) |
| p16 | Clinical researcher | [prompt](p16-clinical-researcher.md) |
| p17 | Construction project manager | [prompt](p17-construction-project-manager.md) |
| p18 | Manufacturing quality engineer | [prompt](p18-manufacturing-quality-engineer.md) |
| p19 | Educator/instructional designer | [prompt](p19-educator-instructional-designer.md) |
| p20 | Investigative journalist | [prompt](p20-investigative-journalist.md) |

同時開始時も、各親タスクは自分のpersonaだけを所有します。親タスク内部では
異なるleaf scopeに限ってSubagentを並列化し、同一scopeは常に1 writerとします。
他personaのactive leaseは正常な並列状態です。個別親タスクは他personaのleaseを
show/recover/releaseせず、全体の`active_leases=0`を完了条件にしません。

## Full追加制作

M1完了済みpersonaをFullへ拡張するタスクでは、M1用promptを再利用しません。

- 共通template: [FULL_PARENT_TEMPLATE.md](FULL_PARENT_TEMPLATE.md)
- 完了済み12ファイルpilot（履歴、再実行禁止）:
  [p01-full-pilot.md](p01-full-pilot.md)
- pilot固定content spine: [p01-full-pilot-content-spine.md](p01-full-pilot-content-spine.md)
- pilot固定assignment: [p01-full-pilot-assignment.jsonl](p01-full-pilot-assignment.jsonl)
- p01 Full mass production（完了済み履歴）:
  [p01-full-production.md](p01-full-production.md)
- p02 Full mass production（完了済み履歴）:
  [p02-full-production.md](p02-full-production.md)
- p03 Full mass production（完了済み履歴）:
  [p03-full-production.md](p03-full-production.md)

READYなp04--p20のpromptは、p04--p20用のcheckpoint v2 resume gate
`bin/full-resume-gate`（SHA-256
`3c4889deadd8b0eabcb6494454501db0adc063e7166d61dafc5cf6490660d069`）を使います。
v2の具体的なcheckpoint／scan provenance契約は
[Full台帳仕様](../runbooks/FULL_LEDGER_SPEC.md)を参照してください。p01--p03の完了済み
accepted v1 protocolは閉じた履歴であり、このv2 gateへ変換・混在させません。

Fullタスクも1親タスク＝1 personaです。既存M1 200ファイルの事前SHA-256 baselineを
取得し、固定Full追加assignmentを検証してから、scope leaseをbatch境界で取得します。
現在のp01は12,000 filesで完成・受入済みです。11,800行の完全masterとpilotを含む
accepted chainは履歴として保持し、production promptやpilot promptを再実行しません。

p01--p03は完了済みaccepted v1 checkpointとの連続性を保持するため、上記v2の開始／再開規則を適用しません。
