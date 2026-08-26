#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
class="$root/progress/p14/full/m1-classification-reconciliation.jsonl"
inv="$root/progress/p14/full/canonical-source-inventory.jsonl"
m1="$root/progress/p14/m1-assignment-ledger.jsonl"
res="$root/progress/p14/full/m1-reservations.jsonl"
assignment="$root/progress/p14/full/full-assignment-ledger.jsonl"
base="$root/progress/p14/full/manifests/m1-baseline.jsonl"
run() { PYTHONDONTWRITEBYTECODE=1 "$root/bin/p14-full-ledger" verify --persona p14 --classification "$1" --inventory "$2" --m1-assignment "$3" --reservations "$4" --assignment "$5" --baseline "$6"; }
reject() { if run "$@" >/dev/null 2>&1; then echo "expected p14 gate rejection" >&2; exit 1; fi; }
run "$class" "$inv" "$m1" "$res" "$assignment" "$base" | jq -e '.result == "ok" and .assignment_artifacts == 8800' >/dev/null
[ "$(jq -s '[.[]|select(.kind=="artifact")|select(.content_spine_ids|length==6)]|length' "$assignment")" = 8800 ]
jq -s -e '
  [.[]|select(.kind=="artifact")]
  | all(.content_spine_ids[0:5] == ["org.cedar-works","project.fy2026-close","measure.cash-variance-jpy-12400000","control.dual-review","term.no-kio-claim"])
  and all(if (.scope_id | test("^p14-primary-0[1-3]$")) then .content_spine_ids[5] == "rel.close-to-forecast"
          elif (.scope_id | test("^p14-primary-0[4-6]$")) then .content_spine_ids[5] == "rel.assumption-to-outlook"
          elif (.scope_id | test("^p14-primary-0[7-8]$")) then .content_spine_ids[5] == "rel.transaction-to-control"
          elif (.scope_id | test("^p14-primary-(09|10)$")) then .content_spine_ids[5] == "rel.evidence-to-decision"
          elif (.scope_id | test("^p14-primary-(11|12)$")) then .content_spine_ids[5] == "rel.model-to-export"
          else .content_spine_ids[5] == "rel.reference-to-record" end)' "$assignment" >/dev/null
[ "$(jq -s '[.[]|select(.kind=="exception")]|length' "$class")" = 3 ]
[ "$(jq -s '[.[]|select(.kind=="artifact")]|length' "$assignment")" = 8800 ]
jq -s -e '[.[]|select(.kind=="artifact")]|[.[]|select(.family=="pdf_scan" and (.scratch_input? != null))]|all(.scratch_input.final_artifact == false and (.scratch_input.path|startswith("scratch/p14/")))' "$assignment" >/dev/null
tmp=$(mktemp -d)
cp "$class" "$tmp/class"; cp "$inv" "$tmp/inv"; cp "$m1" "$tmp/m1"; cp "$res" "$tmp/res"; cp "$assignment" "$tmp/assignment"; cp "$base" "$tmp/base"
# Byte-identical alternate copies are rejected for every fixed input.
reject "$tmp/class" "$inv" "$m1" "$res" "$assignment" "$base"
reject "$class" "$tmp/inv" "$m1" "$res" "$assignment" "$base"
reject "$class" "$inv" "$tmp/m1" "$res" "$assignment" "$base"
reject "$class" "$inv" "$m1" "$tmp/res" "$assignment" "$base"
reject "$class" "$inv" "$m1" "$res" "$tmp/assignment" "$base"
reject "$class" "$inv" "$m1" "$res" "$assignment" "$tmp/base"
ln -s "$class" "$tmp/class-link"; ln -s "$inv" "$tmp/inv-link"; ln -s "$m1" "$tmp/m1-link"; ln -s "$res" "$tmp/res-link"; ln -s "$assignment" "$tmp/assignment-link"; ln -s "$base" "$tmp/base-link"
reject "$tmp/class-link" "$inv" "$m1" "$res" "$assignment" "$base"
reject "$class" "$tmp/inv-link" "$m1" "$res" "$assignment" "$base"
reject "$class" "$inv" "$tmp/m1-link" "$res" "$assignment" "$base"
reject "$class" "$inv" "$m1" "$tmp/res-link" "$assignment" "$base"
reject "$class" "$inv" "$m1" "$res" "$tmp/assignment-link" "$base"
reject "$class" "$inv" "$m1" "$res" "$assignment" "$tmp/base-link"
# Deterministically mutate the private snapshot between digest and parse.
printf '%s\n' '#!/bin/sh' 'printf x > "$1"' > "$tmp/mutate-snapshot"; chmod +x "$tmp/mutate-snapshot"
if P14_FULL_LEDGER_TEST_HOOK="$tmp/mutate-snapshot" run "$class" "$inv" "$m1" "$res" "$assignment" "$base" >/dev/null 2>&1; then echo "expected snapshot mutation rejection" >&2; exit 1; fi
# The canonical record has the exact three approved observed classifications.
[ "$(jq -s '[.[]|select(.kind=="exception" and .declared_family=="pdf_scan" and .detected_family=="pdf_text")]|length' "$class")" = 3 ]
jq -s '.[0].classification_id="wrong" | .[]' "$class" > "$tmp/bad-id.jsonl"
jq -s '.[0:3][]' "$class" > "$tmp/missing-row.jsonl"
jq -s '.[1].m1_artifact_id="p14-m1-999999" | .[]' "$class" > "$tmp/swapped-row.jsonl"
for bad in "$tmp/bad-id.jsonl" "$tmp/missing-row.jsonl" "$tmp/swapped-row.jsonl"; do reject "$bad" "$inv" "$m1" "$res" "$assignment" "$base"; done
printf '%s\n' 'p14 full-ledger tests: ok'
