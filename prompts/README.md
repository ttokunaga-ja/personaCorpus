# Parent task prompts

1ファイルを1つの新しいCodexタスクへ貼り付けます。すべてのタスクは
`personaCorpus` プロジェクトの **Local** 環境で開始し、Worktreeは使いません。
開始前にルートで `./bin/check-ready` が成功することを確認してください。

今回のp08／p09／p10同時開始手順は
[MAC_PARALLEL_WAVE_01.md](MAC_PARALLEL_WAVE_01.md)です。

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
- 最初の12ファイルpilot: [p01-full-pilot.md](p01-full-pilot.md)
- pilot固定content spine: [p01-full-pilot-content-spine.md](p01-full-pilot-content-spine.md)
- pilot固定assignment: [p01-full-pilot-assignment.jsonl](p01-full-pilot-assignment.jsonl)

Fullタスクも1親タスク＝1 personaです。既存M1 200ファイルの事前SHA-256 baselineを
取得し、固定Full追加assignmentを検証してから、scope leaseをbatch境界で取得します。
