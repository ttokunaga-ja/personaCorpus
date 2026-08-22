# Parent task prompt template

Replace every placeholder before starting a separate Local task.

```text
`personaCorpus` の <PERSONA_ID>（<ROLE>）1人分の合成ファイル環境を担当してください。
このタスクは必ず /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus を
直接使用するLocalタスクです。別Git worktreeへ移動したり、新しいworkspaceを作ったり
してはいけません。

最初に AGENTS.md、README.md、runbooks/COMMON_RULES.md、
runbooks/BATCH_PROTOCOL.md、runbooks/PERSONA_INDEX.md、
runbooks/SESSION_HANDOFF.md、runbooks/PERSONA_PC_CONTRACT.md、
<BRIEF_PATH> を完全に読んでください。canonical/persona-plan.json と
workspace/persona-workspace-owner.json をauthorityとし、planやscaffoldを再生成・編集
しないでください。

親として一意なsession名で <PERSONA_ID> のparent leaseを取得してください。返された
release tokenは親だけが保持し、ファイル、prompt、Subagentメッセージへ書かないでください。
briefの初回seed artifactsから開始し、各plan-defined leaf scopeをそれぞれ1 Subagentへ
排他的に割り当ててください。Subagentをspawnする直前に親がscope leaseを取得します。
spawn前にrunbooks/BATCH_PROTOCOL.mdの初回200件計算へ従って、全20 scopeの件数と
固定ファイル名・形式をprogress/<PERSONA_ID>/m1-assignments.mdへ確定してください。
Subagentにはscope ID、最終absolute folder、作成する固定ファイル名・形式・内容要件だけを
渡し、そのフォルダ以外を編集させないでください。同じscopeのSubagentを同時に2つ起動
しないでください。

DOCX/PDF/XLSX/PPTX/画像はDocuments/PDF/Spreadsheets/Presentations/ImageGenの
該当skillを必ず読み、skill所定のrender・視覚確認まで行ってください。独自のKio検索QAは
設計しません。高品質で現実的な完全合成ファイルを優先し、実在PII・秘密・資格情報・私文書
は使用しません。temporary/render出力はscratch/<PERSONA_ID>/<scope-id>/、最終成果物は
割当済みworkspace leafだけに置いてください。

各Subagent完了後に親が配置、形式、開封・視認性、brief anchorの整合を確認してscope leaseを
解放します。中断時はprogress/<PERSONA_ID>/へ秘密を含まないcheckpointを残します。
初回目標はseed完了後、合計200 final filesです。full profile件数、Kio index/history/search/
performance達成は主張しません。全child leaseを解放してからparent leaseを解放し、成果物数、
使用skill、未完scope、blocked事項を報告してください。
```
