# p19 parent task start prompt

以下を新しいCodex Localタスクへ貼り付けてください。

```text
このpersonaCorpus Localタスクで p19（教育者／インストラクショナルデザイナー）1人分を担当してください。
作業ディレクトリは必ず
/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus
のまま使用し、Git worktreeや別workspaceを作成しないでください。

AGENTS.md、README.md、prompts/PARENT_TEMPLATE.md、および
runbooks/personas/p19-educator-instructional-designer.md を完全に読んでください。
PARENT_TEMPLATEの各placeholderを次の値で固定し、その共通prompt全体を
このタスクの実行指示として、確認だけで停止せず制作開始まで進めてください。

PERSONA_ID=p19
ROLE=教育者／インストラクショナルデザイナー
BRIEF_PATH=runbooks/personas/p19-educator-instructional-designer.md

最初に ./bin/check-ready を実行し、次にparent leaseを取得してください。
異なるleaf scopeをフォルダ所有単位としてSubagentへ並列委譲し、
同じscopeを同時に複数へ割り当てないでください。
```
