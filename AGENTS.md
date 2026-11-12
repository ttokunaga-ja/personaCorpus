# 開発方針の適用範囲

- このディレクトリ配下では、現在の合意済み仕様を正本にし、不要な旧実装・互換層・依存関係・設定・資料の削除までを開発の完了範囲とする。個人開発・リリース前と指定されたプロジェクトでは後方互換性と旧データ移行は不要とする。
- KISS / YAGNIを優先し、変更行数より完成後の責務・分岐・依存関係の少なさを重視する。変更対象の仕様を先に揃え、振る舞いの変更は原則TDDで進め、正式なローカル検証を行う。
- 未コミット変更、確定済み成果物、個人データ、秘密情報、共有・本番資源、現在も必要な回帰基準を保持する。旧実装削除をこれらの破棄権限と解釈しない。下位のリポジトリ固有の保護条件と担当権限を維持する。
- 検証済みの論理的な単位で適切な粒度のコミットを作る。無関係な作業を含めない。
- 公開GitHubリポジトリへの通常のPushは毎回の確認不要。PrivateでCI/CD等のActionsが設定されている場合は月間利用枠のためPush前にユーザーの許可を得て、それまではコミットのみ。PrivateでActionsがない場合はタスクのPush許可に従い、公開範囲やActionsの有無が不明ならコミットまでにとどめる。
- Push前に送信対象コミット、差分、ワークフロー、hooks、ローカル検証を確認する。通常のPush許可はデプロイ・リリース・課金操作の許可を兼ねない。

## コミットと Push の権限・費用方針

- 実装を依頼された場合は、検証済みの論理的な変更単位で、適切な粒度のローカルコミットを作りながら進める。無関係な変更をまとめず、細かすぎるチェックポイントも量産しない。資料と直接関連するテストは対応する変更と一緒にコミットする。
- GitHub の公開リポジトリへの通常の Push はユーザーの包括的な許可があるため、毎回の確認は不要。ただし、既存の未送信コミットも送られることを確認し、無関係なユーザー作業を勝手に送らない。
- Private リポジトリに CI/CD 等の GitHub Actions が設定されている場合は、Actions の月間利用枠を守るため、対象リポジトリと送信する変更・起動するワークフローを示して、Push 前にユーザーの許可を得る。許可が得られるまではローカルコミットだけにとどめる。docs の変更、paths フィルタによるスキップ見込み、再 Push もこの条件から除外しない。
- Private で Actions がない場合はタスクの明示的な Push 許可に従う。公開範囲や Actions の有無を確認できない場合は、許可済みと推測せずコミットまでにとどめる。
- Push 前には GitHub の現在の公開範囲、ワークフロー、hooks、差分、送信対象コミット、ローカル検証結果を確認する。公開リポジトリへの Push 許可は、デプロイ・公開・リリース・課金資源の操作の許可を兼ねない。それらを自動実行する Push は対応する操作の許可を必要とする。
- personaCorpus の成果物制作など、担当者の stage/commit/push を禁止する固有の権限分離は維持する。担当範囲の変更は権限を持つ調整担当が適切な単位でコミットする。

# Persona corpus production instructions

## Purpose and authority

This repository coordinates production of a fully synthetic, twenty-persona
filesystem corpus. The accepted Rust plan at `canonical/persona-plan.json`
and the Rust workspace owner record at `workspace/persona-workspace-owner.json`
are authoritative for persona IDs, scope IDs, paths, formats, and counts.
Runbooks and prompts are operational guidance only.

## Mandatory session model

- Run every persona parent task in **Local / same-directory mode** against this
  exact repository. Do not use a separate Git worktree for production.
- One parent task owns exactly one persona (`p01` through `p20`).
- A parent may run several subagents concurrently only when every subagent owns
  a different plan-defined leaf scope. Never assign one scope twice at once.
- Parent tasks for different personas may run concurrently. Another persona's
  lease is expected shared-workspace state: do not inspect, recover, release,
  or use it as a reason to stop. Global `active_leases=0` is a coordinator-only
  batch-end check, not an individual persona completion condition.
- Each subagent writes final artifacts only below its assigned
  `workspace/people/<persona>-<role>/home/<scope-path>/` leaf folder.
- The parent alone invokes `./bin/persona lease ...`, retains release tokens,
  checks the output, and releases leases. Never put a release token in a
  prompt, artifact, progress file, Git file, or subagent message.
- A stopped folder worker should be continued through the same subagent when
  practical. Recovery commands require explicit user/coordinator confirmation.

## Artifact production

- Read `README.md`, all common files named by the selected parent prompt, and
  the matching `runbooks/personas/pXX-*.md` brief before producing anything.
- Use Documents for DOCX, PDF for PDF, Spreadsheets for XLSX, Presentations for
  PPTX, and ImageGen for images or scan source artwork whenever the brief calls
  for those formats. Read each applicable `SKILL.md` before using it.
- The user's priority is high-quality, realistic persona content. Do not design
  a separate search or Kio QA suite. Skill-required rendering and visual/file
  verification remain mandatory before an artifact is accepted.
- Text, code, CSV/TSV, JSON/YAML, HTML/EML, notebooks, and similar files may be
  created normally. Extensions must match real file contents.
- Use synthetic names and facts only. Never include real PII, PHI, credentials,
  secrets, private documents, customer data, or copyrighted source documents.
- Put temporary inputs and render/inspection output under
  `scratch/<persona-id>/<scope-id>/`, never inside the final workspace.

## Ownership and repository safety

### Shared monitoring responsibility

- The coordinator and central resource monitor own shared disk-capacity
  decisions, the all-persona reservation ledger, cross-persona scheduling,
  and whole-corpus stop/completion checks. Persona tasks must not independently
  recalculate global free-space floors or add their own reservation a second
  time to the all-persona aggregate.
- The user-approved capacity policy (2026-10-13) is a shared pool with one
  central free-space threshold of 30 GB (30,000,000,000 bytes, decimal). Do not add the former 48 GiB
  reservation aggregate or fixed per-persona allocations to this threshold.
  Fixed slot allocation and unused reservation amounts are not launch gates.
  Roll this policy into reviewed immutable admission successors; until a
  caller adopts the operational replacement, preserve its existing interface
  and report its legacy threshold rather than claiming it has migrated.
- Migrate existing pinned consumers through narrow immutable successors and
  a reviewed central admission interface. Do not disable their capacity checks
  before the central replacement is operational. A periodic monitoring report
  alone is not synchronous launch admission.
- A persona retains its own scope ownership, input/runtime identity,
  protected-artifact checks, finite per-operation write bounds, actual I/O error handling,
  process containment, durable exit recording, and artifact-quality checks.
  Global Git cleanliness, unrelated persona changes, unrelated shared metadata
  additions, and other personas' active leases are not local stop conditions.
- Central admission holds affect new launches. They must not kill healthy
  running children or prevent recording their actual exits and diagnostics.
  Preserve unresolved reservations after a crash until actual ownership and
  completion are reconciled; do not release them solely because time elapsed.

- Persona parents own only their persona's `workspace/people/.../` subtree and
  `progress/<persona-id>/`. Folder workers own only the exact leaf folder given
  to them by that parent.
- Do not directly edit `canonical/`, the Rust plan/owner records, `_control/`,
  runbooks, prompts, scripts, or another persona's files during production.
  Only the pinned Rust lease commands may mutate `_control/`.
- Accepted final artifacts below `workspace/people/**/home/**`, portable
  canonical plan/schedule records, and allowlisted token-free progress records
  are Git-managed. Production tasks leave their own accepted additions as
  ordinary working-tree changes; they never stage or commit them.
- Runtime binaries, workspace owner/lease topology, canonical render and
  materialization receipts, scratch renders, local logs, and release tokens
  remain ignored. Never force-add an ignored path.
- Do not require a globally clean Git status while parallel persona tasks are
  active: another persona's uncommitted artifacts are expected shared state.
  The coordinating task alone verifies the corpus manifest, stages only an
  accepted wave, and commits directly to `main`. Production tasks never push.

## Production milestones

The first launch targets the persona brief's named seed artifacts, then the
first 200 final files for that persona in bounded folder batches. Freeze the
machine-readable M1 assignment ledger before the first worker starts, then
derive its human assignment display; Full production must never infer an
accepted M1 family from extension alone. The full
profile (195,000 files total) proceeds only through
`runbooks/FULL_PRODUCTION_PROTOCOL.md` and a frozen, token-free Full addition
assignment. Full work must preserve the first 200 files byte-for-byte, use
bounded resumable batches, and never reinterpret the Rust plan. Do not claim
Kio indexing, history, search, measured chunks, or performance from either
milestone.

For M1, the sole machine-readable allocation authority is
`progress/<persona>/m1-assignment-ledger.jsonl` with schema
`persona-corpus.m1-assignment/v1`. It is frozen before the first worker starts;
the human `m1-assignments.md` is its derived display/narrative, never an
independent allocation. Full reservations consume the frozen ledger directly;
do not infer a family (especially `.sql`) from an extension.

# エージェント運用（2026-11-12〜、個人デスクトップ WSL）

この節は personaCorpus での Claude・Codex の役割、モデル指定、commit の粒度の正本であり、`~/.codex/AGENTS.md` の一般的なモデル規定と旧「中央調整チャット」の例外より優先する。下記の「調整担当（coordinator）」は、この節の統括（Opus）を指す。

- **作業場所**：個人デスクトップ WSL の `~/dev/github.com/ttokunaga-ja/personaCorpus` を正とする。Mac では作業しない。
- **セッション分離**：personaCorpus の Claude セッションは personaCorpus だけを扱う。kio のファイルは変更しない。kio の作業は kio のセッションが行う。
- **役割とモデル**（effort はすべて medium）

  | 役割 | 担当 |
  |---|---|
  | 制作方式・品質基準・運用規則の見直しと修正 | Claude Fable 5.2（`claude-fable-5-2`）のセッション |
  | 制作の指示・受入検証・commit・push の統括（調整担当） | Claude Opus 5.5（`claude-opus-5-5`） |
  | persona 制作（親タスクとその subagent） | Codex `gpt-6.1-terra` |
  | 確立済みの検証コマンドの実行、読み取り専用の集計・探索 | Codex `gpt-6-luna` |
  | 受入レビューの難しい判断、監査、Terra で品質が安定しない制作 | Codex `gpt-6.1-sol` |

- **Terra から Sol への切替え**：制作はまず `gpt-6.1-terra` で行う。同じ batch で受入検査の不合格が 2 回続く、または同種の品質問題が繰り返す場合、統括がその persona・batch を `gpt-6.1-sol` に切り替え、切替えと理由を `progress/<persona>/` に記録する。
- **委譲と監視**：Codex は `~/dev/github.com/ttokunaga-ja/agent-ops` の `codex-job start --repo personaCorpus ...` で起動する。統括は `agent-alert wait --repo personaCorpus` をバックグラウンドで待ち、通知を受けたときだけ処理する。LLM による定期監視（旧 Codex automation）は廃止した。
- **停止の基準**：Codex が作業全体を止めてよいのは、取り返しのつかない操作、外部への公開、秘密情報、課金に関わる判断だけである。それ以外の不明点はその項目だけ保留として理由と選択肢を記録し、残りを進める。保留は統括が解決する。Codex は新しい停止規則・承認手続きを自分で作らず、repo の資料にも追加しない。規則の追加・変更は Fable のセッションで行う。
- **commit の粒度**：
  - 受入済みの成果物は、persona × batch（または phase）ごとに 1 commit とし、受入のたびに commit・push する。複数 persona や複数 batch をまとめた一括 commit、`git add -A`・`git add workspace/` のような広い stage はしない。
  - 1 回の push で送る pack は 1 GiB 程度までに抑える（GitHub の 2 GiB 上限）。超えそうな batch は分割して commit する。
  - 成果物の commit と、資料・規則・スクリプトの変更の commit は分ける。
- **検証**：Codex の報告のうち、受入・一致・失敗原因などの重要な主張は、統括がコマンドで確認してから採用する。
