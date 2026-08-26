#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
helper="$root/bin/p02-m1-assignment"
ledger="$root/bin/full-ledger"
accepted_inventory="$root/progress/p02/full/canonical-source-inventory.jsonl"
accepted_baseline="$root/progress/p02/full/manifests/m1-baseline.jsonl"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p02-m1-assignment-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

first="$tmp/first.jsonl"
second="$tmp/second.jsonl"
# This regression intentionally runs against the accepted post-Full home.  A
# helper that still requires home == 200 M1 files must fail here.
[ "$(find "$root/workspace/people/p02-site-reliability-engineer/home" -type f | wc -l | tr -d ' ')" -gt 200 ]
"$helper" --out "$first"
"$helper" --out "$second"
cmp -s "$first" "$second"
jq -se '
  . as $rows | ($rows[0]) as $h | ($rows[1:]) as $a |
  $h.schema == "persona-corpus.m1-assignment/v1" and $h.kind == "header" and $h.persona == "p02" and $h.artifact_count == 200 and $h.source_assignment_sha256 == "sha256:a3557153096edd1ef42ecae7a9a83e9832972918390b0b94b180e6e2efbd065b" and
  ($a|length == 200) and ([ $a[].artifact_id ]|unique|length == 200) and ([ $a[].path ]|unique|length == 200) and
  ([ $a[] | select(.schema != "persona-corpus.m1-assignment/v1" or .kind != "artifact" or .persona != "p02" or .state != "frozen") ]|length == 0)
' "$first" >/dev/null
if "$helper" --out "$first" >/dev/null 2>&1; then echo "overwrite was accepted" >&2; exit 1; fi
unsafe="$root/workspace/p02-m1-assignment-test-output.jsonl"
if "$helper" --out "$unsafe" >/dev/null 2>&1; then echo "unsafe workspace output was accepted" >&2; exit 1; fi
[ ! -e "$unsafe" ] || { echo "unsafe output was created" >&2; exit 1; }

# Exercise deterministic inventory generation without modifying final files or
# progress.  The 200-file baseline predates completed Full production and must
# be reused from its immutable accepted record; regenerating a manifest from
# the current 15,000-file home would not be an M1 baseline.
"$ledger" inventory --persona p02 --out "$tmp/inventory.jsonl"
cmp -s "$accepted_inventory" "$tmp/inventory.jsonl"
cp "$accepted_baseline" "$tmp/baseline.jsonl"
[ "$(jq -r 'select(.kind == "header") | .file_count' "$tmp/baseline.jsonl")" = 200 ]

# The legacy M1 display has a frozen, same-scope reconciliation; a bare
# reservation must reject that mismatch, while the exact frozen record makes
# the complete mapping reproducible.
if "$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --out "$tmp/reservations.jsonl" >"$tmp/reserve.out" 2>&1; then
  echo "reserve-m1 unexpectedly accepted the known p02 canonical mismatch" >&2
  exit 1
fi
grep -F 'M1 assignment has no canonical scope/family extension:' "$tmp/reserve.out" >/dev/null
grep -F 'meetings/operations/2026-07-13-operations-review.pptx' "$tmp/reserve.out" >/dev/null
grep -F 'meetings/operations/2026-Q3-reliability-planning.pptx' "$tmp/reserve.out" >/dev/null

reconciliation="$root/progress/p02/full/m1-reservation-reconciliation.jsonl"
[ -f "$reconciliation" ] || { echo "missing frozen reconciliation" >&2; exit 1; }
rec_digest="sha256:$(shasum -a 256 "$reconciliation" | awk '{print $1}')"
[ "$rec_digest" = "sha256:cb55ac00908354632520ae4fce6dae4c7c93bddf5ad129bfafcb18b61a56dbf7" ]
"$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --reconciliation "$reconciliation" --out "$tmp/reservations.jsonl"

jq -nce --slurpfile m1 "$first" --slurpfile rec "$reconciliation" --slurpfile res "$tmp/reservations.jsonl" --arg rec_digest "$rec_digest" '
  ($m1 | map(select(.kind == "artifact"))) as $m1_rows |
  ($rec | map(select(.kind == "exception"))) as $exceptions |
  ($res | map(select(.kind == "header"))[0]) as $header |
  ($res | map(select(.kind == "reservation"))) as $rows |
  ([
    {m1_path:"infrastructure/terraform/variables.tf",source_id:"p02-src-001055",m1_family:"code",family:"pptx",scope_id:"p02-primary-04"},
    {m1_path:"infrastructure/terraform/versions.tf",source_id:"p02-src-003656",m1_family:"code",family:"pptx",scope_id:"p02-primary-04"},
    {m1_path:"meetings/operations/2026-07-13-operations-review.pptx",source_id:"p02-src-002216",m1_family:"pptx",family:"code",scope_id:"p02-primary-12"},
    {m1_path:"meetings/operations/2026-Q3-reliability-planning.pptx",source_id:"p02-src-002227",m1_family:"pptx",family:"code",scope_id:"p02-primary-12"}
  ]) as $expected |
  ($m1_rows | map({key:.path,value:.}) | from_entries) as $m1_by_path |
  ($rows | map({key:.m1_path,value:.}) | from_entries) as $reservation_by_path |
  ($exceptions | map({key:.m1_path,value:.}) | from_entries) as $exception_by_path |
  $header.schema == "persona-corpus.full-m1-reservations/v1" and
  $header.persona == "p02" and $header.reservation_count == 200 and
  $header.reconciliation_sha256 == $rec_digest and
  ($rows|length == 200) and ([ $rows[].source_id ]|unique|length == 200) and
  ([ $rows[].m1_path ]|unique|length == 200) and
  ($exceptions|length == 4) and
  ([ $expected[] | . as $e |
     ($exception_by_path[$e.m1_path]) as $x |
     ($reservation_by_path[$e.m1_path]) as $r |
     ($m1_by_path[$e.m1_path]) as $m |
     $x != null and $r != null and $m != null and
     $x.source_id == $e.source_id and $x.m1_family == $e.m1_family and
     $x.canonical_family == $e.family and $x.m1_scope_id == $e.scope_id and
     $x.canonical_scope_id == $e.scope_id and $r.source_id == $e.source_id and
     $r.family == $e.family and $r.scope_id == $e.scope_id and
     $m.family == $e.m1_family and $m.scope_id == $e.scope_id
   ] | all) and
  (($rows | group_by(.scope_id) | map({key:.[0].scope_id,value:length}) | from_entries) ==
   ($m1_rows | group_by(.scope_id) | map({key:.[0].scope_id,value:length}) | from_entries)) and
  (($rows | group_by(.family) | map({key:.[0].family,value:length}) | from_entries) ==
   ($m1_rows | group_by(.family) | map({key:.[0].family,value:length}) | from_entries))
' >/dev/null

# A complete synthetic assignment exercises the full verifier without writing
# any production artifact.  It is intentionally flat: scheduling/content
# quality belong to the allocation task, while this test covers set algebra.
jq -n --slurpfile inv "$tmp/inventory.jsonl" --slurpfile res "$tmp/reservations.jsonl" '
  def skill($family):
    if $family == "docx" then "documents"
    elif $family == "xlsx" then "spreadsheets"
    elif $family == "pptx" then "presentations"
    elif $family == "pdf_text" or $family == "pdf_scan" then "pdf"
    elif $family == "image" then "imagegen" else null end;
  ($inv | map(select(.kind == "header"))[0]) as $header |
  ([ $res[] | select(.kind == "reservation") | .source_id ]) as $reserved |
  ($inv | map(select(.kind == "source" and (.source_id as $id | $reserved | index($id) | not))) | sort_by(.source_id)) as $sources |
  {schema:"persona-corpus.full-addition-assignment/v1",kind:"header",persona:"p02",plan_digest:$header.plan_digest,render_digest:$header.render_digest,assignment_id:"p02-full-test-master-v1",batch_id:"p02-full-test-b001",artifact_count:($sources|length),pilot:false},
  ($sources[] |
   {schema:"persona-corpus.full-addition-assignment/v1",kind:"artifact",persona:"p02",artifact_id:("p02-full-test-" + .source_id),source_id:.source_id,scope_id:.scope_id,scope_path:.scope_path,family:.family,canonical_variant:.canonical_variant,canonical_extension:.canonical_extension,physical_extension:.canonical_extension,path:(.scope_path + "/full-test/" + .source_id + "." + .canonical_extension),batch_id:"p02-full-test-b001",dependencies:[],content_spine_ids:[],skill_required:skill(.family),state:"planned"})
' > "$tmp/master.jsonl"
[ "$(jq -r 'select(.kind == "header") | .artifact_count' "$tmp/master.jsonl")" = 14800 ]
"$ledger" verify --persona p02 --inventory "$tmp/inventory.jsonl" --m1-assignment "$first" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --reconciliation "$reconciliation" | jq -e '.result == "ok" and .assignment_artifacts == 14800 and .m1_reservations == 200' >/dev/null

# The reconciliation is immutable input: changing its bytes, mapping, or
# membership must never validate against the frozen reservation output.
jq 'if .kind == "header" then .test_note = "digest-tamper" else . end' "$reconciliation" > "$tmp/reconciliation-digest-tamper.jsonl"
if "$ledger" verify --persona p02 --inventory "$tmp/inventory.jsonl" --m1-assignment "$first" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --reconciliation "$tmp/reconciliation-digest-tamper.jsonl" >/dev/null 2>&1; then
  echo "tampered reconciliation digest was accepted" >&2
  exit 1
fi
jq 'if .kind == "exception" and .m1_artifact_id == "p02-m1-0131" then .source_id = "p02-src-001056" else . end' "$reconciliation" > "$tmp/reconciliation-mapping-tamper.jsonl"
if "$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --reconciliation "$tmp/reconciliation-mapping-tamper.jsonl" --out "$tmp/reservations-mapping-tamper.jsonl" >/dev/null 2>&1; then
  echo "tampered reconciliation mapping was accepted" >&2
  exit 1
fi
# This alternative exists in the same canonical scope and family, so ordinary
# set-balance checks alone cannot distinguish it.  The only accepted policy is
# the repository's pinned reconciliation file and digest.
jq 'if .kind == "exception" and .m1_artifact_id == "p02-m1-0131" then .source_id = "p02-src-004057" else . end' "$reconciliation" > "$tmp/reconciliation-valid-alternate.jsonl"
if "$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --reconciliation "$tmp/reconciliation-valid-alternate.jsonl" --out "$tmp/reservations-valid-alternate.jsonl" >/dev/null 2>&1; then
  echo "valid-but-unapproved reconciliation mapping was accepted" >&2
  exit 1
fi
ln -s "$reconciliation" "$tmp/reconciliation-symlink.jsonl"
if "$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --reconciliation "$tmp/reconciliation-symlink.jsonl" --out "$tmp/reservations-symlink.jsonl" >/dev/null 2>&1; then
  echo "symlinked reconciliation was accepted" >&2
  exit 1
fi
jq 'if .kind == "header" then .exception_count = 3 elif .kind == "exception" and .m1_artifact_id == "p02-m1-0140" then empty else . end' "$reconciliation" > "$tmp/reconciliation-omission.jsonl"
if "$ledger" reserve-m1 --persona p02 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$first" --reconciliation "$tmp/reconciliation-omission.jsonl" --out "$tmp/reservations-omission.jsonl" >/dev/null 2>&1; then
  echo "reconciliation omission was accepted" >&2
  exit 1
fi

echo "p02 M1 assignment tests: ok"
