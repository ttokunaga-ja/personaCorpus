#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
tmp=$(mktemp -d)
export AGENT_OPS_HOME="$tmp" AGENT_WATCH_INTERVAL=0.2 AGENT_STALL_SECONDS=1 AGENT_TMUX_SOCKET="test-$$"
trap 'tmux -L "$AGENT_TMUX_SOCKET" kill-server 2>/dev/null || true; rm -rf "$tmp"' EXIT
brief="$tmp/brief"; printf test > "$brief"
job="$root/bin/codex-job"; alert="$root/bin/agent-alert"
# Validate the public invocation contract before launching real tmux jobs.
if "$job" start --repo personaCorpus --name missing > "$tmp/invalid" 2>&1; then
  echo 'missing --brief/--command accepted' >&2; exit 1
fi
if "$job" start --repo personaCorpus --name both --brief "$brief" --command true > "$tmp/invalid" 2>&1; then
  echo 'mutually exclusive inputs accepted' >&2; exit 1
fi
if "$job" start --repo personaCorpus --name retired --test-command true > "$tmp/invalid" 2>&1; then
  echo 'retired --test-command accepted' >&2; exit 1
fi
"$alert" wait --repo personaCorpus > "$tmp/waited" &
waiter=$!
sleep 0.5
kill -0 "$waiter"
"$job" start --repo personaCorpus --name wake --command 'echo wake'
wait "$waiter"
python3 - "$tmp/waited" <<'PYWAIT'
import json,sys
assert any(x["job"] == "wake" and x["event"] == "done" for x in map(json.loads, open(sys.argv[1])))
PYWAIT
"$job" start --repo kio --name success --command 'echo ok'
"$job" start --repo kio --name failure --command 'exit 4'
"$job" start --repo kio --name stalled --command 'sleep 15'
"$job" start --repo kio --name limit --command 'echo "error: rate limit exceeded"; sleep 15'
"$job" start --repo kio --name stdin --command 'echo "Reading additional input from stdin..."; sleep 15'
"$job" start --repo kio --name vanished --command 'sleep 15'
tmux -L "$AGENT_TMUX_SOCKET" kill-session -t cj-kio-vanished
sleep 3
"$alert" wait --repo kio > "$tmp/seen"
python3 - "$tmp/seen" <<'PY'
import json,sys
rows=[json.loads(x) for x in open(sys.argv[1])]
found={(x['job'],x['event']) for x in rows}
expected={('success','done'),('failure','failed'),('stalled','stalled'),('limit','limit'),('stdin','stdin-wait'),('vanished','failed')}
assert expected <= found, (expected-found,found)
PY
"$alert" list --repo kio | grep -q . && { echo 'cursor did not advance' >&2; exit 1; }
sleep 1
python3 - "$tmp/alerts/kio.jsonl" <<'PY'
import json,sys
rows=[json.loads(x) for x in open(sys.argv[1])]
pairs=[(r['job'],r['event']) for r in rows]
assert len(pairs)==len(set(pairs)), pairs
PY
tmux -L "$AGENT_TMUX_SOCKET" kill-session -t agent-watchd
"$job" status --repo kio > /dev/null
tmux -L "$AGENT_TMUX_SOCKET" has-session -t agent-watchd
"$job" stop kio stalled
"$job" stop kio limit
"$job" stop kio stdin
printf 'all tests passed\n'
