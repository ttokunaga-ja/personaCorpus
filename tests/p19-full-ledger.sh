#!/bin/sh
# Regression coverage for p19's reviewed, p19-only reconciliation bridge.
# It writes only below mktemp and never changes the active shared helper.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
ledger="$root/bin/p19-full-ledger"
shared="$root/bin/full-ledger"
m1="$root/progress/p19/m1-assignment-ledger.jsonl"
inventory="$root/progress/p19/full/canonical-source-inventory.jsonl"
baseline="$root/progress/p19/full/manifests/m1-baseline.jsonl"
reconciliation="$root/progress/p19/full/m1-reservation-reconciliation.jsonl"
reservations="$root/progress/p19/full/m1-reservations.jsonl"
home="$root/workspace/people/p19-educator-instructional-designer/home"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p19-full-ledger-test.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

sha() { shasum -a 256 "$1" | awk '{print $1}'; }
fail_if_accepted() {
  label=$1
  shift
  if "$@" >/dev/null 2>&1; then
    echo "$label was accepted" >&2
    exit 1
  fi
}
home_fingerprint() {
  find "$home" -type f -print | LC_ALL=C sort | while IFS= read -r path; do
    printf '%s  %s\n' "$(sha "$path")" "${path#"$home"/}"
  done | shasum -a 256 | awk '{print $1}'
}

[ -f "$ledger" ] && [ -f "$m1" ] && [ -f "$inventory" ] && [ -f "$baseline" ] &&
  [ -f "$reconciliation" ] && [ -f "$reservations" ]
[ ! -L "$reconciliation" ]

# These values bind the test to the reviewed p19 input set, rather than to a
# copied p03 assumption or to a merely self-consistent alternate mapping.
[ "$(sha "$m1")" = "8711998fd9a07dd0ff687dbf1fb5ba89515cab80220e76693b615d9e242340dd" ]
[ "$(sha "$inventory")" = "49cac673140f43c5a21b216799698ac9f1eaaa836b6ce91351fa9e08caa9b3d0" ]
[ "$(sha "$baseline")" = "171bec6dde0901f35f2874678c519b5f110867fa9ebdec8c828717824c1ef8dd" ]
[ "$(sha "$reconciliation")" = "31ca6e559b0022109014cce7142a599429add5f997e00d2fe702cc6746c56e4a" ]
[ "$(sha "$reservations")" = "01a06ec564be697ff389a0c053ac0be43b4a20802f08482725626bc6fc0b93bf" ]

jq -nce --slurpfile rec "$reconciliation" '
  ($rec | map(select(.kind == "header"))) as $headers |
  ($rec | map(select(.kind == "exception")) | sort_by(.m1_artifact_id)) as $rows |
  ($headers | length) == 1 and
  $headers[0].persona == "p19" and
  $headers[0].reconciliation_id == "p19-m1-same-scope-family-balance-v1" and
  $headers[0].exception_count == 3 and
  ($rows | map({m1_artifact_id, m1_path, source_id, canonical_scope_id,
                canonical_family, canonical_variant, canonical_extension})) == [
    {m1_artifact_id:"p19-m1-000001",m1_path:"archive/closed/archived-assignment-debrief.wav",source_id:"p19-src-000333",canonical_scope_id:"p19-secondary-08",canonical_family:"pdf_scan",canonical_variant:"pdf-scan",canonical_extension:"pdf"},
    {m1_artifact_id:"p19-m1-000002",m1_path:"archive/closed/archived-audio-sidecars-scan.pdf",source_id:"p19-src-001427",canonical_scope_id:"p19-secondary-08",canonical_family:"domain_binary",canonical_variant:"pcap",canonical_extension:"pcap"},
    {m1_artifact_id:"p19-m1-000021",m1_path:"cloud/team-shared/team-drive-sync-synthetic.pcap",source_id:"p19-src-000619",canonical_scope_id:"p19-secondary-06",canonical_family:"media",canonical_variant:"wav",canonical_extension:"wav"}
  ]
' >/dev/null

before_shared=$(sha "$shared")
before_ledger=$(sha "$ledger")
before_m1=$(sha "$m1")
before_inventory=$(sha "$inventory")
before_baseline=$(sha "$baseline")
before_reconciliation=$(sha "$reconciliation")
before_reservations=$(sha "$reservations")
before_home=$(home_fingerprint)
[ "$(find "$home" -type f | wc -l | tr -d ' ')" = 200 ]
[ "$(find "$home" -type l | wc -l | tr -d ' ')" = 0 ]

# Strict same-scope pairing must fail for the real p19 legacy shortage. The
# domain-binary shortfall is rejected before pair construction because that
# family has no p19-secondary-06 canonical source at all.
if "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --out "$tmp/strict.jsonl" >"$tmp/strict.out" 2>&1; then
  echo "strict p19 reservation unexpectedly succeeded" >&2
  exit 1
fi
grep -F 'M1 assignment has no canonical scope/family extension: cloud/team-shared/team-drive-sync-synthetic.pcap' "$tmp/strict.out" >/dev/null
jq -nce --slurpfile m "$m1" --slurpfile inv "$inventory" '
  ($m | map(select(.kind == "artifact")) |
   group_by(.scope_id + "|" + .family) |
   map({key:(.[0].scope_id + "|" + .[0].family), value:length}) | from_entries) as $m1_counts |
  ($inv | map(select(.kind == "source")) |
   group_by(.scope_id + "|" + .family) |
   map({key:(.[0].scope_id + "|" + .[0].family), value:length}) | from_entries) as $source_counts |
  [$m1_counts | to_entries[] | select(($source_counts[.key] // 0) < .value)] == [
    {key:"p19-secondary-06|domain_binary",value:1},
    {key:"p19-secondary-08|media",value:2}
  ]
' >/dev/null

# Re-derive twice from the immutable bridge. Both must equal the frozen p19
# reservations byte-for-byte; this also exercises the private snapshot path.
"$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$reconciliation" --out "$tmp/reservations-1.jsonl"
"$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$reconciliation" --out "$tmp/reservations-2.jsonl"
cmp -s "$tmp/reservations-1.jsonl" "$tmp/reservations-2.jsonl"
cmp -s "$tmp/reservations-1.jsonl" "$reservations"

# Admission is deliberately tied to this one canonical regular file and its
# reviewed bytes. Every mutable variant below is made only in $tmp.
cp "$reconciliation" "$tmp/wrong-path.jsonl"
fail_if_accepted "wrong reconciliation path" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/wrong-path.jsonl" --out "$tmp/x1.jsonl"
ln -s "$reconciliation" "$tmp/symlink.jsonl"
fail_if_accepted "symlinked reconciliation" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/symlink.jsonl" --out "$tmp/x2.jsonl"
jq 'if .kind == "header" then .persona = "p03" elif .kind == "exception" then .persona = "p03" else . end' "$reconciliation" > "$tmp/wrong-persona.jsonl"
fail_if_accepted "wrong persona/header" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/wrong-persona.jsonl" --out "$tmp/x3.jsonl"
jq 'if .kind == "header" then .reconciliation_id = "p19-unapproved" else . end' "$reconciliation" > "$tmp/wrong-id.jsonl"
fail_if_accepted "wrong reconciliation ID" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/wrong-id.jsonl" --out "$tmp/x4.jsonl"
jq 'if .kind == "header" then .test_digest_tamper = true else . end' "$reconciliation" > "$tmp/wrong-digest.jsonl"
fail_if_accepted "wrong reconciliation digest" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/wrong-digest.jsonl" --out "$tmp/x5.jsonl"
sed '$d' "$reconciliation" > "$tmp/row-deletion.jsonl"
fail_if_accepted "reconciliation row deletion" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/row-deletion.jsonl" --out "$tmp/x6.jsonl"
jq 'if .kind == "exception" and .m1_artifact_id == "p19-m1-000021" then .source_id = "p19-src-000333" else . end' "$reconciliation" > "$tmp/source-replacement.jsonl"
fail_if_accepted "reconciliation source replacement" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/source-replacement.jsonl" --out "$tmp/x7.jsonl"
jq 'if .kind == "exception" and .m1_artifact_id == "p19-m1-000021" then .source_id = "p19-src-000333" | .canonical_scope_id = "p19-secondary-08" | .canonical_scope_path = "archive/closed" | .canonical_family = "pdf_scan" | .canonical_variant = "pdf-scan" | .canonical_extension = "pdf" else . end' "$reconciliation" > "$tmp/alternate-self-consistent.jsonl"
fail_if_accepted "alternate self-consistent reconciliation" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$m1" --reconciliation "$tmp/alternate-self-consistent.jsonl" --out "$tmp/x8.jsonl"
jq 'if .kind == "artifact" and .artifact_id == "p19-m1-000021" then .path = "cloud/team-shared/replaced.pcap" else . end' "$m1" > "$tmp/m1-source-replacement.jsonl"
fail_if_accepted "M1 source replacement" "$ledger" reserve-m1 --persona p19 --inventory "$inventory" --baseline "$baseline" --m1-assignment "$tmp/m1-source-replacement.jsonl" --reconciliation "$reconciliation" --out "$tmp/x9.jsonl"

# A safe regression cannot replace the approved canonical file while its
# snapshot is read, so the helper's cmp/re-hash TOCTOU guard remains covered by
# code inspection; all mutable admission variants above are temp-only.
[ "$(sha "$shared")" = "$before_shared" ]
[ "$(sha "$ledger")" = "$before_ledger" ]
[ "$(sha "$m1")" = "$before_m1" ]
[ "$(sha "$inventory")" = "$before_inventory" ]
[ "$(sha "$baseline")" = "$before_baseline" ]
[ "$(sha "$reconciliation")" = "$before_reconciliation" ]
[ "$(sha "$reservations")" = "$before_reservations" ]
[ "$(home_fingerprint)" = "$before_home" ]
[ "$(find "$home" -type f | wc -l | tr -d ' ')" = 200 ]
[ "$(find "$home" -type l | wc -l | tr -d ' ')" = 0 ]

echo "p19 full-ledger reconciliation tests: ok"
