#!/bin/sh
# Persona-isolated regression coverage for p16's reviewed reconciliation.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ledger="$root/bin/p16-full-ledger"
shared="$root/bin/full-ledger"
m1="$root/progress/p16/m1-assignment-ledger.jsonl"
rec="$root/progress/p16/full/m1-reservation-reconciliation.jsonl"
before_shared=$(shasum -a 256 "$shared" | awk '{print $1}')
before_m1=$(shasum -a 256 "$m1" | awk '{print $1}')
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p16-full-ledger-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
bad() { if "$@" >/dev/null 2>&1; then echo "unexpected acceptance: $1" >&2; exit 1; fi; }
[ "$(shasum -a 256 "$rec" | awk '{print $1}')" = 4e601bc44dc54f7ceb294ec009fa4984a75ecfa06d9832b1a3244c547af714a4 ]
[ ! -L "$rec" ]
"$ledger" inventory --persona p16 --out "$tmp/inventory.jsonl"
"$ledger" manifest --persona p16 --out "$tmp/baseline.jsonl"
bad "$ledger" reserve-m1 --persona p16 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1" --out "$tmp/strict.jsonl"
"$ledger" reserve-m1 --persona p16 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1" --reconciliation "$rec" --out "$tmp/res1.jsonl"
"$ledger" reserve-m1 --persona p16 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1" --reconciliation "$rec" --out "$tmp/res2.jsonl"
cmp -s "$tmp/res1.jsonl" "$tmp/res2.jsonl"
jq -n -e --slurpfile r "$tmp/res1.jsonl" '([ $r[]|select(.kind=="reservation") ]|length)==200 and ([ $r[]|select(.kind=="reservation")|.source_id]|unique|length)==200 and ([ $r[]|select(.kind=="reservation" and .m1_path=="statistics/analysis/alpha-sensitivity-analysis.ipynb")|.source_id][0]=="p16-src-001462") and ([ $r[]|select(.kind=="reservation" and .m1_path=="archive/closed/alpha-archived-webinar.wav")|.source_id][0]=="p16-src-002249")' >/dev/null
# Production uses verify, not reserve-m1 alone. Exercise its complete master
# path and prove it re-applies the exact reconciliation admission boundary.
master="$root/progress/p16/full/full-assignment-ledger.jsonl"
baseline="$root/progress/p16/full/manifests/m1-baseline.jsonl"
reservations="$root/progress/p16/full/m1-reservations.jsonl"
"$ledger" verify --persona p16 --inventory "$root/progress/p16/full/canonical-source-inventory.jsonl" --m1-assignment "$m1" --reservations "$reservations" --assignment "$master" --baseline "$baseline" --reconciliation "$rec" | jq -e '.result=="ok" and .assignment_artifacts==7800' >/dev/null
# Semantic truth table: the three Beta paths must never inherit Alpha; shared
# paths are deliberately study-neutral; cohort/threshold are bounded to the
# research-administration scopes declared by the frozen spine.
jq -s -e '
  [ .[] | select(.kind == "artifact") ] as $a |
  ($a | map(select(.scope_id | IN("p16-primary-04","p16-primary-05","p16-primary-06")))) as $beta |
  ($a | map(select(.scope_id | IN("p16-primary-01","p16-primary-02","p16-primary-03")))) as $alpha |
  ($a | map(select(.scope_id | startswith("p16-secondary-")))) as $shared |
  ($a | map(select(.content_spine_ids | index("cohort.synthetic-48")))) as $cohort |
  ($a | map(select(.content_spine_ids | index("threshold.2-5mg")))) as $threshold |
  ($beta|length)==1638 and
  ($beta|all(.content_spine_ids|index("study.beta-2026"))) and
  ($beta|all((.content_spine_ids|index("study.alpha-2026"))|not)) and
  ($alpha|all(.content_spine_ids|index("study.alpha-2026"))) and
  ($alpha|all((.content_spine_ids|index("study.beta-2026"))|not)) and
  ($shared|all(((.content_spine_ids|index("study.alpha-2026")) or (.content_spine_ids|index("study.beta-2026")))|not)) and
  ($cohort|length)>0 and ($cohort|all(.scope_id|IN("p16-primary-02","p16-primary-03","p16-primary-05","p16-primary-06","p16-primary-11"))) and
  ($threshold|length)>0 and ($threshold|all(.scope_id|IN("p16-primary-01","p16-primary-04","p16-primary-07","p16-primary-09","p16-primary-10","p16-primary-11")))
' "$master" >/dev/null
cp "$reservations" "$tmp/reservations-tampered.jsonl"
sed -i.bak 's/"reservation_count":200/"reservation_count":199/' "$tmp/reservations-tampered.jsonl"
bad "$ledger" verify --persona p16 --inventory "$root/progress/p16/full/canonical-source-inventory.jsonl" --m1-assignment "$m1" --reservations "$tmp/reservations-tampered.jsonl" --assignment "$master" --baseline "$baseline" --reconciliation "$rec"
[ "$(shasum -a 256 "$shared" | awk '{print $1}')" = "$before_shared" ]
[ "$(shasum -a 256 "$m1" | awk '{print $1}')" = "$before_m1" ]
[ "$(find "$root/workspace/people/p16-clinical-researcher/home" -type f | wc -l | tr -d ' ')" = 200 ]
echo 'p16 full-ledger reconciliation tests: ok'
