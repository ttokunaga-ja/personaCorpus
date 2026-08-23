#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tmp=$(mktemp -d "${TMPDIR:-/tmp}/full-ledger-test.XXXXXX")
test_lock="$root/scratch/p01/.full-ledger-test.lock"
lock_held=false
lease_held=false
release_token=
claim_json=
release_test_lease() {
  [ "$lease_held" = true ] || return 0
  if [ -z "$release_token" ] && [ -n "$claim_json" ]; then
    release_token=$(printf '%s\n' "$claim_json" | jq -er '.release_token | select(type == "string" and length > 0)') || return 1
  fi
  [ -n "$release_token" ] || return 1
  if "$root/bin/persona" lease release \
    --root "$root/workspace" --persona p01 \
    --release-token "$release_token" >/dev/null 2>&1; then
    lease_held=false
    release_token=
    claim_json=
    return 0
  fi
  return 1
}
initial_cleanup() {
  if ! release_test_lease; then
    echo "could not normally release p01 full-ledger test lease; lock retained" >&2
  else
    [ "$lock_held" = true ] && rmdir "$test_lock" 2>/dev/null || true
    lock_held=false
  fi
  rm -rf "$tmp"
}
trap initial_cleanup EXIT HUP INT TERM
[ -d "$root/scratch/p01" ] || { echo "missing p01 scratch directory" >&2; exit 1; }
if ! mkdir "$test_lock" 2>/dev/null; then
  echo "full-ledger test is already running or its lock is stale: $test_lock" >&2
  exit 1
fi
lock_held=true
# Prove that the test-only lock serializes a second invocation.
if mkdir "$test_lock" 2>/dev/null; then
  rmdir "$test_lock"
  echo "full-ledger test lock did not serialize" >&2
  exit 1
fi
parent_lease="$root/workspace/_control/personas/p01/lease.json"
scope_control="$root/workspace/_control/scopes/p01"
[ -d "$scope_control" ] || { echo "missing p01 scope control directory" >&2; exit 1; }
if [ -e "$parent_lease" ] || [ -L "$parent_lease" ] || find "$scope_control" -mindepth 2 -maxdepth 2 -name lease.json -print -quit | grep -q .; then
  echo "refusing to claim the p01 full-ledger test lease while p01 is active" >&2
  exit 1
fi
if ! claim_json=$("$root/bin/persona" lease claim \
  --root "$root/workspace" --persona p01 \
  --session "p01-full-ledger-test-$$" --label "tests/full-ledger.sh"); then
  echo "could not atomically claim the p01 full-ledger test lease" >&2
  exit 1
fi
lease_held=true
release_token=$(printf '%s\n' "$claim_json" | jq -er '.release_token | select(type == "string" and length > 0)') || {
  echo "p01 full-ledger test claim returned no release token" >&2
  exit 1
}
claim_json=
ledger="$root/bin/full-ledger"
assignment_helper="$root/bin/p01-m1-assignment"
pilot="$root/prompts/p01-full-pilot-assignment.jsonl"
if [ -n "${P01_M1_ASSIGNMENT:-}" ]; then
  m1_assignment=$P01_M1_ASSIGNMENT
  [ -f "$m1_assignment" ] || { echo "P01_M1_ASSIGNMENT is not a file" >&2; exit 1; }
else
  m1_assignment="$tmp/m1-assignment.jsonl"
  "$assignment_helper" --out "$m1_assignment"
  [ "$(jq -r 'select(.kind == "header") | .artifact_count' "$m1_assignment")" = 200 ]
  if "$assignment_helper" --out "$m1_assignment" >/dev/null 2>&1; then echo "M1 assignment overwrite was accepted" >&2; exit 1; fi
fi

"$ledger" inventory --persona p01 --out "$tmp/inventory.jsonl"
[ "$(wc -l < "$tmp/inventory.jsonl" | tr -d ' ')" = 12001 ]
if "$ledger" inventory --persona p01 --out "$tmp/inventory.jsonl" >/dev/null 2>&1; then echo "inventory overwrite was accepted" >&2; exit 1; fi
"$ledger" manifest --persona p01 --out "$tmp/baseline.jsonl"
[ "$(jq -r 'select(.kind == "header") | .file_count' "$tmp/baseline.jsonl")" = 200 ]
if "$ledger" manifest --persona p01 --out "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "manifest overwrite was accepted" >&2; exit 1; fi
# A symlink must never be silently omitted from a manifest.  This path is
# scoped to the test and is removed explicitly even when a later assertion
# fails.
home="$root/workspace/people/p01-software-engineer/home"
test_root=$(mktemp -d "$home/.full-ledger-test.XXXXXX") || { echo "cannot create unique test root" >&2; exit 1; }
test_component=$(basename "$test_root")
created_test_root=true
test_link="$test_root/symlink"
test_fifo=
test_file_b1=
test_file_b2=
test_dir_b1=
test_dir_b2=
created_link=false
created_fifo=false
created_file_b1=false
created_file_b2=false
created_dir_b1=false
created_dir_b2=false
cleanup() {
  cleanup_status=0
  [ "$created_link" = true ] && rm -f "$test_link"
  [ "$created_fifo" = true ] && rm -f "${test_fifo:-}"
  [ "$created_file_b1" = true ] && rm -f "${test_file_b1:-}"
  [ "$created_file_b2" = true ] && rm -f "${test_file_b2:-}"
  [ "$created_dir_b1" = true ] && rmdir "$test_dir_b1" 2>/dev/null || true
  [ "$created_dir_b2" = true ] && rmdir "$test_dir_b2" 2>/dev/null || true
  [ "$created_test_root" = true ] && rmdir "$test_root" 2>/dev/null || true
  if ! release_test_lease; then
    echo "could not normally release p01 full-ledger test lease; lock retained" >&2
    cleanup_status=1
  else
    [ "$lock_held" = true ] && rmdir "$test_lock" 2>/dev/null || true
    lock_held=false
  fi
  rm -rf "$tmp"
  return "$cleanup_status"
}
trap cleanup EXIT HUP INT TERM
ln -s "../archive/closed/alpha-r6-cleanup.rs" "$test_link"
created_link=true
if "$ledger" manifest --persona p01 --out "$tmp/symlink-manifest.jsonl" >/dev/null 2>&1; then echo "symlink was accepted by manifest" >&2; exit 1; fi
rm -f "$test_link"
created_link=false
if [ -e "$test_link" ] || [ -L "$test_link" ]; then echo "test symlink was not removed" >&2; exit 1; fi
if command -v mkfifo >/dev/null 2>&1; then
  test_fifo="$test_root/fifo"
  mkfifo "$test_fifo"
  created_fifo=true
  if "$ledger" manifest --persona p01 --out "$tmp/fifo-manifest.jsonl" >/dev/null 2>&1; then echo "FIFO was accepted by manifest" >&2; exit 1; fi
  rm -f "$test_fifo"
  created_fifo=false
  test_fifo=
fi
"$ledger" reserve-m1 --persona p01 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --out "$tmp/reservations.jsonl"
[ "$(jq -r 'select(.kind == "header") | .reservation_count' "$tmp/reservations.jsonl")" = 200 ]
[ "$(jq -r 'select(.kind == "header") | .m1_assignment_sha256' "$tmp/reservations.jsonl")" = "sha256:$(shasum -a 256 "$m1_assignment" | awk '{print $1}')" ]
if "$ledger" reserve-m1 --persona p01 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --out "$tmp/reservations.jsonl" >/dev/null 2>&1; then echo "reservation overwrite was accepted" >&2; exit 1; fi
jq -c 'if .kind == "source" and .source_id == "p01-src-000001" then .planned_identity = "tampered-but-count-preserved" else . end' "$tmp/inventory.jsonl" > "$tmp/inventory-mutated.jsonl"
[ "$(wc -l < "$tmp/inventory-mutated.jsonl" | tr -d ' ')" = "$(wc -l < "$tmp/inventory.jsonl" | tr -d ' ')" ]
[ "$(jq -c 'select(.kind == "header")' "$tmp/inventory-mutated.jsonl")" = "$(jq -c 'select(.kind == "header")' "$tmp/inventory.jsonl")" ]
if "$ledger" reserve-m1 --persona p01 --inventory "$tmp/inventory-mutated.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$m1_assignment" --out "$tmp/reservations-mutated.jsonl" >/dev/null 2>&1; then echo "mutated inventory was accepted by reservation" >&2; exit 1; fi

"$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" | jq -e '.result == "ok" and .assignment_artifacts == 12' >/dev/null
if "$ledger" verify --persona p01 --inventory "$tmp/inventory-mutated.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "mutated inventory was accepted by verification" >&2; exit 1; fi
digest=$(jq -r 'select(.kind == "header") | .plan_digest' "$tmp/inventory.jsonl")
{
  jq -cn --arg d "$digest" '{schema:"persona-corpus.full-manifest/v1",kind:"header",persona:"p01",plan_digest:$d,file_count:212}'
  jq -c 'select(.kind == "file")' "$tmp/baseline.jsonl"
  jq -c 'select(.kind == "artifact") | (.artifact_id | capture("(?<n>[0-9]+)$").n) as $n | {schema:"persona-corpus.full-manifest/v1",kind:"file",persona:"p01",path:.path,bytes:100,sha256:("sha256:" + (("0" * (64 - ($n|length))) + $n))}' "$pilot"
} > "$tmp/after-valid.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" --after "$tmp/after-valid.jsonl" >/dev/null 2>&1; then echo "fabricated pilot after manifest was accepted" >&2; exit 1; fi

# A non-pilot ledger must be complete, but an individual batch validates only
# its own additions against the cumulative manifest that precedes it.
jq -n --slurpfile inv "$tmp/inventory.jsonl" --slurpfile res "$tmp/reservations.jsonl" --arg test_component "$test_component" '
  def skill($family):
    if $family == "docx" then "documents"
    elif $family == "xlsx" then "spreadsheets"
    elif $family == "pptx" then "presentations"
    elif $family == "pdf_text" or $family == "pdf_scan" then "pdf"
    elif $family == "image" then "imagegen" else null end;
  ($inv | map(select(.kind == "header"))[0]) as $h |
  ([ $res[] | select(.kind == "reservation") | .source_id ]) as $reserved |
  ($inv | map(select(.kind == "source" and (.source_id as $id | $reserved | index($id) | not)) | .) | sort_by(.source_id)) as $sources |
  {schema:"persona-corpus.full-addition-assignment/v1",kind:"header",persona:"p01",plan_digest:$h.plan_digest,render_digest:$h.render_digest,assignment_id:"p01-full-master-test-v1",batch_id:"master",scope_id:null,artifact_count:($sources|length),pilot:false},
  ($sources | to_entries[] | .key as $i | .value |
   (if $i == 0 then "mass-b1" elif $i == 1 then "mass-b2" else "mass-b3-unexecuted" end) as $batch |
   {schema:"persona-corpus.full-addition-assignment/v1",kind:"artifact",persona:"p01",artifact_id:("mass-" + .source_id),source_id:.source_id,scope_id:.scope_id,scope_path:.scope_path,family:.family,canonical_variant:.canonical_variant,canonical_extension:.canonical_extension,physical_extension:.canonical_extension,path:(.scope_path + "/" + $test_component + "/" + .source_id + "." + .canonical_extension),batch_id:$batch,dependencies:[],content_spine_ids:[],skill_required:skill(.family),state:"planned"})
' > "$tmp/master-base.jsonl"
[ "$(jq -r 'select(.kind == "header") | .artifact_count' "$tmp/master-base.jsonl")" = 11800 ]
b1_dep=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b1") | .artifact_id' "$tmp/master-base.jsonl")
b2_target=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b2") | .artifact_id' "$tmp/master-base.jsonl")
jq --arg target "$b2_target" --arg dep "$b1_dep" 'if .kind == "artifact" and .artifact_id == $target then .dependencies = [$dep] else . end' "$tmp/master-base.jsonl" > "$tmp/master.jsonl"
"$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" | jq -e '.result == "ok" and .assignment_artifacts == 11800' >/dev/null
omit=$(jq -r 'select(.kind == "artifact") | .artifact_id' "$tmp/master.jsonl" | head -1)
jq --arg omit "$omit" 'if .kind == "header" then .artifact_count -= 1 elif .kind == "artifact" and .artifact_id == $omit then empty else . end' "$tmp/master.jsonl" > "$tmp/master-incomplete.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master-incomplete.jsonl" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "incomplete mass assignment was accepted" >&2; exit 1; fi
b1_path=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b1") | .path' "$tmp/master.jsonl")
b2_path=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b2") | .path' "$tmp/master.jsonl")
test_file_b1="$home/$b1_path"
test_file_b2="$home/$b2_path"
[ ! -e "$test_file_b1" ] && [ ! -L "$test_file_b1" ] || { echo "test batch-one path already exists" >&2; exit 1; }
[ ! -e "$test_file_b2" ] && [ ! -L "$test_file_b2" ] || { echo "test batch-two path already exists" >&2; exit 1; }
test_dir_b1=$(dirname "$test_file_b1")
test_dir_b2=$(dirname "$test_file_b2")
if [ ! -d "$test_dir_b1" ]; then mkdir -p "$test_dir_b1"; created_dir_b1=true; fi
if [ ! -d "$test_dir_b2" ]; then mkdir -p "$test_dir_b2"; created_dir_b2=true; fi
(set -C; : > "$test_file_b1") || { echo "cannot exclusively create batch-one test file" >&2; exit 1; }
created_file_b1=true
printf 'full-ledger regression mass batch one\n' >> "$test_file_b1"
"$ledger" manifest --persona p01 --out "$tmp/after-b1.jsonl"
"$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --before "$tmp/baseline.jsonl" --batch-id mass-b1 --after "$tmp/after-b1.jsonl" | jq -e '.result == "ok" and .current_batch_artifacts == 1 and .selected_batch_ids == ["mass-b1"]' >/dev/null
(set -C; : > "$test_file_b2") || { echo "cannot exclusively create batch-two test file" >&2; exit 1; }
created_file_b2=true
printf 'full-ledger regression mass batch two\n' >> "$test_file_b2"
"$ledger" manifest --persona p01 --out "$tmp/after-b2.jsonl"
"$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --before "$tmp/after-b1.jsonl" --batch-id mass-b2 --after "$tmp/after-b2.jsonl" | jq -e '.result == "ok" and .before_files == 201 and .after_files == 202 and .current_batch_artifacts == 1' >/dev/null
"$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" --baseline "$tmp/baseline.jsonl" --before "$tmp/baseline.jsonl" --batch-id mass-b1 --batch-id mass-b2 --after "$tmp/after-b2.jsonl" | jq -e '.result == "ok" and .current_batch_artifacts == 2 and .selected_batch_ids == ["mass-b1","mass-b2"]' >/dev/null
rm -f "$test_file_b1" "$test_file_b2"
created_file_b1=false
created_file_b2=false
[ "$created_dir_b1" = true ] && rmdir "$test_dir_b1" 2>/dev/null || true
[ "$created_dir_b2" = true ] && rmdir "$test_dir_b2" 2>/dev/null || true
test_file_b1= test_file_b2= test_dir_b1= test_dir_b2= created_dir_b1=false created_dir_b2=false

reserved=$(jq -r 'select(.kind == "reservation") | .source_id' "$tmp/reservations.jsonl" | head -1)
jq --arg id "$reserved" 'if .kind == "artifact" then .source_id = $id else . end' "$pilot" > "$tmp/assignment-reserved.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/assignment-reserved.jsonl" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "reserved source was accepted" >&2; exit 1; fi
jq 'if .kind == "artifact" and .artifact_id == "p01-full-pilot-001" then .dependencies += ["missing-dependency"] else . end' "$pilot" > "$tmp/assignment-dependency.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/assignment-dependency.jsonl" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "unknown dependency was accepted" >&2; exit 1; fi
jq 'if .kind == "artifact" and .artifact_id == "p01-full-pilot-001" then .path |= sub("\\.md$"; ".txt") | .physical_extension = "txt" else . end' "$pilot" > "$tmp/assignment-extension.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/assignment-extension.jsonl" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "extension mismatch was accepted" >&2; exit 1; fi
jq 'if .kind == "header" then .render_digest = "sha256:bad" else . end' "$pilot" > "$tmp/assignment-digest.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$tmp/assignment-digest.jsonl" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "bad render digest was accepted" >&2; exit 1; fi
jq 'if .kind == "reservation" and .source_id == "p01-src-000001" then .m1_path = "archive/closed/alpha-r6-cleanup.rs" else . end' "$tmp/reservations.jsonl" > "$tmp/reservations-duplicate-path.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations-duplicate-path.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "duplicate reservation path was accepted" >&2; exit 1; fi
jq 'if .kind == "artifact" then .family = "txt_log" else . end' "$m1_assignment" > "$tmp/m1-assignment-changed.jsonl"
if "$ledger" reserve-m1 --persona p01 --inventory "$tmp/inventory.jsonl" --baseline "$tmp/baseline.jsonl" --m1-assignment "$tmp/m1-assignment-changed.jsonl" --out "$tmp/reservations-family-changed.jsonl" >/dev/null 2>&1; then echo "changed M1 assignment family was accepted by reservation" >&2; exit 1; fi
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$tmp/m1-assignment-changed.jsonl" --reservations "$tmp/reservations.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" >/dev/null 2>&1; then echo "changed M1 assignment was accepted" >&2; exit 1; fi
jq 'if .kind == "file" and .path == "archive/closed/alpha-r6-cleanup.rs" then .sha256 = "sha256:changed" else . end' "$tmp/after-valid.jsonl" > "$tmp/after-changed.jsonl"
if "$ledger" verify --persona p01 --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" --assignment "$pilot" --baseline "$tmp/baseline.jsonl" --after "$tmp/after-changed.jsonl" >/dev/null 2>&1; then echo "changed baseline was accepted" >&2; exit 1; fi
if ! cleanup; then
  trap - EXIT HUP INT TERM
  exit 1
fi
trap - EXIT HUP INT TERM
echo "full-ledger tests: ok"
