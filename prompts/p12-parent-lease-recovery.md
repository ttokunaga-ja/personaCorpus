# p12 parent lease recovery and M1 acceptance

> **Historical resolved recovery instruction. The named lease was recovered and
> p12 M1 was accepted; do not execute this prompt again.**

Send the following to the stopped p12 Local task. If that task cannot be
continued, paste it into one new Local task opened directly at the
`personaCorpus` repository root. This is explicit authorization for the one
named parent-lease recovery only.

```text
p12 M1の成果物制作後に残ったparent leaseだけを是正し、最終受入まで進めてください。
この指示により、p12 session `codex-p12-20260824-01a02f78`のparent leaseを、下記の
正式CLIで1回だけrecoverすることを明示承認します。別session、scope lease、別personaの
recoverは承認しません。

作業rootはCodexで開いた現在のpersonaCorpus repository rootをそのまま使い、最初に
AGENTS.md、runbooks/COMMON_RULES.md、runbooks/BATCH_PROTOCOL.md、
runbooks/SESSION_HANDOFF.mdを完全に読んでください。別worktree／workspaceを作らず、
Git add／commit／pushを実行しません。

recover前にread-onlyで次を確認してください。

- 次の正式commandでp12 parentが存在し、sessionが正確に
  `codex-p12-20260824-01a02f78`である。

  ./bin/persona lease show \
    --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
    --persona p12

- canonicalのp12全20 Rust scope IDについてscope leaseがすべて不在である。
  各IDはplanから読み、次の正式commandで確認する。

  ./bin/persona lease scope show \
    --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
    --persona p12 --scope-id <canonical-p12-scope-id>

- `workspace/people/p12-support-success-lead/home`はexactly 200 regular filesで、symlink・
  nonregular entryが0である。
- `progress/p12/m1-assignment-ledger.jsonl`はheader 1行＋artifact 200行、20 scopeで、
  ledger pathと現在のhome file pathが完全一致する。
- scope件数とfamily件数がcanonicalのM1配分に一致し、actual SHA-256が200件すべて一意である。

lease確認commandは`set -e`へ無条件に流さず、stdout、stderr、exit statusを個別に取得して
判定してください。pre-recover parent `lease show`は、exit 0と上記sessionのJSONならrecover対象、
exit 1かつ下記No-such-fileなら既に不在としてrecoverをskipし、再検証へ進みます。
全20件のpre-recover `scope show`と、recover後のparent/scope `show`は、exit 1かつstderrが
`persona lease I/O: No such file or directory`であることを「lease不在」の正常結果として扱います。
scope showがexit 0ならactive childが残っているため停止します。これら以外のstatus／stderrは
不在として握りつぶさず、unexpected I/Oとして停止・報告してください。単純な`|| true`で
判定を省略しません。

recover前のhomeを次の正式commandでtoken-free cumulative manifestへ記録してください。

./bin/full-ledger manifest \
  --persona p12 \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p12/p12-parent-recovery-codex-p12-20260824-01a02f78.before.jsonl

出力はcreate-onlyで扱います。既存なら削除・上書きせず、現在homeとの一致を検証してください。
初回preflight時の期待SHA-256は
`9d8c65999c364a6e5bf436049e79f6385e4776b1efae18b5de4f270226134fd5`です。

上記すべてが一致する場合だけ、次を正確に1回実行してください。

./bin/persona lease recover \
  --root /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/workspace \
  --persona p12 \
  --session codex-p12-20260824-01a02f78 \
  --reason "parent task stopped after all p12 scope leases were normally released; in-memory release token was lost"

recoverがexit 0であることと、上記の引数をすべて指定した同じ`lease show`が、前述の
「exit 1＋No such file」条件でp12 parent不在を示すことを確認します。
exit非0、session相違、child lease検出時は再試行・別recover・手動`_control`編集をせず、
実行結果と現在のleaseだけを報告して停止してください。対象parentが確認前から既に不在なら
recoverは実行せず、以下の再検証だけを行います。

recover後、次の正式commandでafter manifestをcreate-only生成してください。

./bin/full-ledger manifest \
  --persona p12 \
  --out /Users/ttokunaga-ja/dev/github.com/ttokunaga-ja/personaCorpus/scratch/p12/p12-parent-recovery-codex-p12-20260824-01a02f78.after.jsonl

beforeとbyte-for-byte一致し、両方のSHA-256が上記期待値と一致することを確認してください。
さらに200 files、20 scope、ledgerとの
path/family/scope一致、SHA-256重複0、symlink/nonregular 0、p12 child 0、p12 parent不在を
再確認します。`./bin/check-ready`のglobal active leaseは他personaを含むため、p12完了条件へ
使いません。他personaのleaseをshow／recover／releaseしません。

成果物、ledger、assignment表示、canonical、owner、runtime、root corpus manifestを変更せず、
Subagentやartifact workerを起動しません。検証がすべて合格した場合だけ「p12 M1受入完了」と
報告し、recover exit、before/after manifest SHA-256と完全一致、200/200 files、20/20 scope、
duplicate 0、p12 parent/child lease不在を示してください。Kio index/history/search/chunk/
performanceの達成は主張しません。
```
