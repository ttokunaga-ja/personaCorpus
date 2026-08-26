#!/bin/sh
# Regression coverage for p03's reviewed same-scope, global-family-balanced
# M1 reconciliation.  This test writes only below mktemp.  It intentionally
# uses the p03-isolated ledger helper to preserve the accepted p03 trust anchor.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ledger="$root/bin/p03-full-ledger"
m1_assignment="$root/progress/p03/m1-assignment-ledger.jsonl"
reconciliation="$root/progress/p03/full/m1-reservation-reconciliation.jsonl"
accepted_inventory="$root/progress/p03/full/canonical-source-inventory.jsonl"
accepted_baseline="$root/progress/p03/full/manifests/m1-baseline.jsonl"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p03-full-ledger-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

fail_if_accepted() {
  label=$1
  shift
  if "$@" >/dev/null 2>&1; then
    echo "$label was accepted" >&2
    exit 1
  fi
}

[ -f "$m1_assignment" ] || { echo "missing p03 M1 assignment" >&2; exit 1; }
[ -f "$reconciliation" ] || { echo "missing p03 reconciliation" >&2; exit 1; }
[ ! -L "$reconciliation" ] || { echo "p03 reconciliation is symlinked" >&2; exit 1; }
[ "$(shasum -a 256 "$reconciliation" | awk '{print $1}')" = "5e723463a6322aad8f0c2c8248c851e33797504603dfbd7b0524e69168100d5d" ] || {
  echo "p03 reconciliation digest is not the reviewed value" >&2
  exit 1
}

# Inventory regeneration remains deterministic against current canonical
# authority.  The immutable 200-file baseline predates completed Full
# production; do not regenerate it from the current 10,000-file home.
"$ledger" inventory --persona p03 --out "$tmp/inventory.jsonl"
cmp -s "$accepted_inventory" "$tmp/inventory.jsonl"
cp "$accepted_baseline" "$tmp/baseline.jsonl"
[ "$(jq -r 'select(.kind == "header") | .file_count' "$tmp/baseline.jsonl")" = 200 ]

if "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --out "$tmp/strict-reservations.jsonl" >"$tmp/strict.out" 2>&1; then
  echo "strict p03 reservation unexpectedly succeeded" >&2
  exit 1
fi
grep -F 'canonical group shortage: p03-primary-12|pptx' "$tmp/strict.out" >/dev/null

"$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$reconciliation" --out "$tmp/reservations.jsonl"

jq -nce --slurpfile m1 "$m1_assignment" --slurpfile res "$tmp/reservations.jsonl" '
  ($m1 | map(select(.kind == "artifact"))) as $m1_rows |
  ($res | map(select(.kind == "header"))[0]) as $header |
  ($res | map(select(.kind == "reservation"))) as $rows |
  ($rows | map({key:.m1_path,value:.}) | from_entries) as $by_m1_path |
  $header.schema == "persona-corpus.full-m1-reservations/v1" and
  $header.persona == "p03" and $header.reservation_count == 200 and
  $header.reconciliation_sha256 == "sha256:5e723463a6322aad8f0c2c8248c851e33797504603dfbd7b0524e69168100d5d" and
  ($rows | length) == 200 and
  ([ $rows[].source_id ] | unique | length) == 200 and
  ([ $rows[].m1_path ] | unique | length) == 200 and
  ($by_m1_path["cloud/team-shared/ac27-control-owner-handoff.txt"].source_id == "p03-src-009973") and
  ($by_m1_path["meetings/security-reviews/vendor-risk-quarterly-review.pptx"].source_id == "p03-src-000307") and
  (($rows | group_by(.scope_id) | map({key:.[0].scope_id,value:length}) | from_entries) ==
   ($m1_rows | group_by(.scope_id) | map({key:.[0].scope_id,value:length}) | from_entries)) and
  (($rows | group_by(.family) | map({key:.[0].family,value:length}) | from_entries) ==
   ($m1_rows | group_by(.family) | map({key:.[0].family,value:length}) | from_entries))
' >/dev/null

# The reconciliation path and its exact bytes are the trust boundary.  These
# variants cover a wrong path, symlink, digest, ID, membership, mapping, and a
# structurally valid but unapproved alternate source selection.
cp "$reconciliation" "$tmp/reconciliation-wrong-path.jsonl"
fail_if_accepted "wrong reconciliation path" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-wrong-path.jsonl" --out "$tmp/wrong-path-reservations.jsonl"
ln -s "$reconciliation" "$tmp/reconciliation-symlink.jsonl"
fail_if_accepted "symlinked reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-symlink.jsonl" --out "$tmp/symlink-reservations.jsonl"
jq 'if .kind == "header" then .test_note = "digest-tamper" else . end' "$reconciliation" > "$tmp/reconciliation-digest-tamper.jsonl"
fail_if_accepted "digest-tampered reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-digest-tamper.jsonl" --out "$tmp/digest-reservations.jsonl"
jq 'if .kind == "header" then .reconciliation_id = "p03-wrong-id" else . end' "$reconciliation" > "$tmp/reconciliation-id-tamper.jsonl"
fail_if_accepted "ID-tampered reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-id-tamper.jsonl" --out "$tmp/id-reservations.jsonl"
jq 'if .kind == "header" then .persona = "p02" elif .kind == "exception" then .persona = "p02" else . end' "$reconciliation" > "$tmp/reconciliation-wrong-persona.jsonl"
fail_if_accepted "wrong-persona reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-wrong-persona.jsonl" --out "$tmp/wrong-persona-reservations.jsonl"
jq 'if .kind == "header" then .inventory_sha256 = "sha256:0000000000000000000000000000000000000000000000000000000000000000" else . end' "$reconciliation" > "$tmp/reconciliation-inventory-digest-tamper.jsonl"
fail_if_accepted "bound-input digest-tampered reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-inventory-digest-tamper.jsonl" --out "$tmp/input-digest-reservations.jsonl"
jq 'if .kind == "header" then .exception_count = 1 elif .kind == "exception" and .m1_artifact_id == "p03-m1-0094" then empty else . end' "$reconciliation" > "$tmp/reconciliation-omission.jsonl"
fail_if_accepted "incomplete reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-omission.jsonl" --out "$tmp/omission-reservations.jsonl"
jq 'if .kind == "exception" and .m1_artifact_id == "p03-m1-0011" then .source_id = "p03-src-000028" else . end' "$reconciliation" > "$tmp/reconciliation-mapping-tamper.jsonl"
fail_if_accepted "mapping-tampered reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-mapping-tamper.jsonl" --out "$tmp/mapping-reservations.jsonl"
jq 'if .kind == "exception" and .m1_artifact_id == "p03-m1-0094" then .source_id = "p03-src-000539" | .canonical_variant = "jsonl" | .canonical_extension = "jsonl" else . end' "$reconciliation" > "$tmp/reconciliation-valid-alternate.jsonl"
fail_if_accepted "valid-but-unapproved alternate reconciliation" "$ledger" reserve-m1 --persona p03 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --reconciliation "$tmp/reconciliation-valid-alternate.jsonl" --out "$tmp/alternate-reservations.jsonl"

# A flat, complete assignment is enough to exercise the full set algebra.  The
# allocation session owns scheduling, dependencies, and content quality; this
# regression test deliberately has no production artifact side effects.
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
  {schema:"persona-corpus.full-addition-assignment/v1",kind:"header",persona:"p03",plan_digest:$header.plan_digest,render_digest:$header.render_digest,assignment_id:"p03-full-ledger-test-v1",batch_id:"p03-full-ledger-test-b001",artifact_count:($sources|length),pilot:false},
  ($sources[] |
   {schema:"persona-corpus.full-addition-assignment/v1",kind:"artifact",persona:"p03",artifact_id:("p03-full-test-" + .source_id),source_id:.source_id,scope_id:.scope_id,scope_path:.scope_path,family:.family,canonical_variant:.canonical_variant,canonical_extension:.canonical_extension,physical_extension:.canonical_extension,path:(.scope_path + "/full-ledger-test/" + .source_id + "." + .canonical_extension),batch_id:"p03-full-ledger-test-b001",dependencies:[],content_spine_ids:[],skill_required:skill(.family),state:"planned"})
' > "$tmp/master.jsonl"
[ "$(jq -r 'select(.kind == "header") | .artifact_count' "$tmp/master.jsonl")" = 9800 ]
"$ledger" verify --persona p03 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --reconciliation "$reconciliation" | jq -e '.result == "ok" and .m1_reservations == 200 and .assignment_artifacts == 9800' >/dev/null

echo "p03 full-ledger reconciliation tests: ok"
