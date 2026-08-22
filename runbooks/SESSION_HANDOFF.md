# Cross-session handoff

The coordinator has already generated the accepted Rust artifacts and fresh
workspace. Use this handoff only in a Local task opened directly at
`/Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus`. The plan and
Rust records remain authoritative.

```text
この personaCorpus project で `<PERSONA_ID>` 1人分の完全合成 corpus production を
担当してください。最初に AGENTS.md、README.md、runbooks/COMMON_RULES.md、
runbooks/BATCH_PROTOCOL.md、runbooks/PERSONA_INDEX.md、
runbooks/SESSION_HANDOFF.md、runbooks/PERSONA_PC_CONTRACT.md、および
runbooks/personas/ の該当briefを読んでください。
accepted Rust plan と Rust が作成した workspace-owner record だけを persona、scope ID、
home path、配分の authority としてください。別の runtime で plan を解析・再生成・scaffold
してはいけません。

workspace は次の共有absolute rootです。別worktreeや別workspaceを作ってはいけません。
親は固定済みRust CLI wrapperだけでparent leaseを取得し、owner recordのdigestを
計算・指定してはいけません:

./bin/persona lease claim \
  --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
  --persona <PERSONA_ID> \
  --session <unique-parent-session>

返された release_token は親だけが保持し、prompt、metadata、home files に書かないでください。
各 worker を spawn する直前に、plan row の Rust scope ID（home path ではない）で scope
lease を取ります:

./bin/persona lease scope claim \
  --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
  --persona <PERSONA_ID> \
  --scope-id <rust-scope-id> \
  --parent-session <parent-session> --worker-session <worker-session>

worker は共有workspace内の
`people/<persona>-<role>/home/<scope-path>/` にだけfinal
artifact を置きます。`<scope-path>` は Rust scope ID とは別物です。worker は `_control/`
を編集せず、scope token も受け取りません。Documents/PDF/Spreadsheets/Presentations/
ImageGen の該当 skill を読んで生成・検査し、完了または具体的な blocked checkpoint を親に
報告してください。親は `./bin/persona lease scope release` で同じ `--scope-id` と
返されたrelease tokenを使ってleaseを解放します。全scope完了後に
`./bin/persona lease release` でparent leaseを解放します。

Kio prepare/index/replay/search の実行、chunk 数、history readiness はこの手順の成果では
ありません。Rust filesystem attestation は bounded bytes-only observation であり、Kio
evidence と history readiness の claim を true にしません。
```
