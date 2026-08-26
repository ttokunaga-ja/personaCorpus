#!/bin/sh
set -eu

# Exercise the shared Full ledger against p09, whose accepted M1 home is still
# exactly 200 files and whose complete allocation package is immutable. This
# keeps the regression test independent from completed p01--p03 Full homes.
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
persona=p09
home="$root/workspace/people/p09-ux-researcher/home"
m1_assignment="$root/progress/p09/m1-assignment-ledger.jsonl"
frozen_inventory="$root/progress/p09/full/canonical-source-inventory.jsonl"
m1_baseline="$root/progress/p09/full/manifests/m1-baseline.jsonl"
frozen_reservations="$root/progress/p09/full/m1-reservations.jsonl"
frozen_assignment="$root/progress/p09/full/full-assignment-ledger.jsonl"
ledger="$root/bin/full-ledger"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/full-ledger-test.XXXXXX")
test_lock="$root/scratch/p09/.full-ledger-test.lock"
lock_held=false
lease_held=false
release_token=
claim_json=
created_test_root=false
created_link=false
created_fifo=false
created_file_b1=false
created_file_b2=false
created_dir_b1=false
created_dir_b2=false
test_root=
test_link=
test_fifo=
test_file_b1=
test_file_b2=
test_dir_b1=
test_dir_b2=

release_test_lease() {
  [ "$lease_held" = true ] || return 0
  if [ -z "$release_token" ] && [ -n "$claim_json" ]; then
    release_token=$(printf '%s\n' "$claim_json" | jq -er '.release_token | select(type == "string" and length > 0)') || return 1
  fi
  [ -n "$release_token" ] || return 1
  if "$root/bin/persona" lease release \
    --root "$root/workspace" --persona "$persona" \
    --release-token "$release_token" >/dev/null 2>&1; then
    lease_held=false
    release_token=
    claim_json=
    return 0
  fi
  return 1
}

cleanup() {
  cleanup_status=0
  [ "$created_link" = true ] && rm -f "$test_link"
  [ "$created_fifo" = true ] && rm -f "${test_fifo:-}"
  [ "$created_file_b1" = true ] && rm -f "${test_file_b1:-}"
  [ "$created_file_b2" = true ] && rm -f "${test_file_b2:-}"
  [ "$created_dir_b2" = true ] && rmdir "$test_dir_b2" 2>/dev/null || true
  [ "$created_dir_b1" = true ] && rmdir "$test_dir_b1" 2>/dev/null || true
  [ "$created_test_root" = true ] && rmdir "$test_root" 2>/dev/null || true
  if ! release_test_lease; then
    echo "could not normally release p09 full-ledger test lease; lock retained" >&2
    cleanup_status=1
  else
    [ "$lock_held" = true ] && rmdir "$test_lock" 2>/dev/null || true
    lock_held=false
  fi
  rm -rf "$tmp"
  return "$cleanup_status"
}
trap cleanup EXIT HUP INT TERM

[ -d "$root/scratch/p09" ] || { echo "missing p09 scratch directory" >&2; exit 1; }
if ! mkdir "$test_lock" 2>/dev/null; then
  echo "full-ledger test is already running or its lock is stale: $test_lock" >&2
  exit 1
fi
lock_held=true
parent_lease="$root/workspace/_control/personas/$persona/lease.json"
scope_control="$root/workspace/_control/scopes/$persona"
if [ -e "$parent_lease" ] || [ -L "$parent_lease" ] || find "$scope_control" -mindepth 2 -maxdepth 2 -name lease.json -print -quit | grep -q .; then
  echo "refusing to claim the p09 full-ledger test lease while p09 is active" >&2
  exit 1
fi
if ! claim_json=$("$root/bin/persona" lease claim \
  --root "$root/workspace" --persona "$persona" \
  --session "p09-full-ledger-test-$$" --label "tests/full-ledger.sh"); then
  echo "could not atomically claim the p09 full-ledger test lease" >&2
  exit 1
fi
lease_held=true
release_token=$(printf '%s\n' "$claim_json" | jq -er '.release_token | select(type == "string" and length > 0)') || {
  echo "p09 full-ledger test claim returned no release token" >&2
  exit 1
}
claim_json=

"$ledger" inventory --persona "$persona" --out "$tmp/inventory.jsonl"
[ "$(wc -l < "$tmp/inventory.jsonl" | tr -d ' ')" = 9001 ]
cmp -s "$tmp/inventory.jsonl" "$frozen_inventory"
if "$ledger" inventory --persona "$persona" --out "$tmp/inventory.jsonl" >/dev/null 2>&1; then
  echo "inventory overwrite was accepted" >&2
  exit 1
fi

"$ledger" manifest --persona "$persona" --out "$tmp/current.jsonl"
[ "$(jq -r 'select(.kind == "header") | .file_count' "$tmp/current.jsonl")" = 200 ]
cmp -s "$tmp/current.jsonl" "$m1_baseline"
if "$ledger" manifest --persona "$persona" --out "$tmp/current.jsonl" >/dev/null 2>&1; then
  echo "manifest overwrite was accepted" >&2
  exit 1
fi

test_root=$(mktemp -d "$home/.full-ledger-test.XXXXXX") || {
  echo "cannot create unique test root" >&2
  exit 1
}
created_test_root=true
test_component=$(basename "$test_root")
test_link="$test_root/symlink"
ln -s "../research/alpha" "$test_link"
created_link=true
if "$ledger" manifest --persona "$persona" --out "$tmp/symlink-manifest.jsonl" >/dev/null 2>&1; then
  echo "symlink was accepted by manifest" >&2
  exit 1
fi
rm -f "$test_link"
created_link=false
if command -v mkfifo >/dev/null 2>&1; then
  test_fifo="$test_root/fifo"
  mkfifo "$test_fifo"
  created_fifo=true
  if "$ledger" manifest --persona "$persona" --out "$tmp/fifo-manifest.jsonl" >/dev/null 2>&1; then
    echo "FIFO was accepted by manifest" >&2
    exit 1
  fi
  rm -f "$test_fifo"
  created_fifo=false
  test_fifo=
fi

"$ledger" reserve-m1 --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --baseline "$m1_baseline" \
  --m1-assignment "$m1_assignment" --out "$tmp/reservations.jsonl"
cmp -s "$tmp/reservations.jsonl" "$frozen_reservations"
if "$ledger" reserve-m1 --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --baseline "$m1_baseline" \
  --m1-assignment "$m1_assignment" --out "$tmp/reservations.jsonl" >/dev/null 2>&1; then
  echo "reservation overwrite was accepted" >&2
  exit 1
fi
"$ledger" verify --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" \
  --reservations "$tmp/reservations.jsonl" --assignment "$frozen_assignment" \
  --baseline "$m1_baseline" | jq -e '.result == "ok" and .assignment_artifacts == 8800' >/dev/null

jq -c 'if .kind == "source" and .source_id == "p09-src-000001" then .planned_identity = "tampered-but-count-preserved" else . end' \
  "$tmp/inventory.jsonl" > "$tmp/inventory-mutated.jsonl"
if "$ledger" reserve-m1 --persona "$persona" \
  --inventory "$tmp/inventory-mutated.jsonl" --baseline "$m1_baseline" \
  --m1-assignment "$m1_assignment" --out "$tmp/reservations-mutated.jsonl" >/dev/null 2>&1; then
  echo "mutated inventory was accepted by reservation" >&2
  exit 1
fi

jq -n --slurpfile inv "$tmp/inventory.jsonl" --slurpfile res "$tmp/reservations.jsonl" --arg component "$test_component" '
  def skill($family):
    if $family == "docx" then "documents"
    elif $family == "xlsx" then "spreadsheets"
    elif $family == "pptx" then "presentations"
    elif $family == "pdf_text" or $family == "pdf_scan" then "pdf"
    elif $family == "image" then "imagegen" else null end;
  ($inv | map(select(.kind == "header"))[0]) as $h |
  ([ $res[] | select(.kind == "reservation") | .source_id ]) as $reserved |
  ($inv | map(select(.kind == "source" and (.source_id as $id | $reserved | index($id) | not)) | .) | sort_by(.source_id)) as $sources |
  {schema:"persona-corpus.full-addition-assignment/v1",kind:"header",persona:"p09",plan_digest:$h.plan_digest,render_digest:$h.render_digest,assignment_id:"p09-full-master-regression-v1",batch_id:"master",scope_id:null,artifact_count:($sources|length),pilot:false},
  ($sources | to_entries[] | .key as $i | .value |
   (if $i == 0 then "mass-b1" elif $i == 1 then "mass-b2" else "mass-b3-unexecuted" end) as $batch |
   {schema:"persona-corpus.full-addition-assignment/v1",kind:"artifact",persona:"p09",artifact_id:("test-" + .source_id),source_id:.source_id,scope_id:.scope_id,scope_path:.scope_path,family:.family,canonical_variant:.canonical_variant,canonical_extension:.canonical_extension,physical_extension:.canonical_extension,path:(.scope_path + "/" + $component + "/" + .source_id + "." + .canonical_extension),batch_id:$batch,dependencies:[],content_spine_ids:[],skill_required:skill(.family),state:"planned"})
' > "$tmp/master-base.jsonl"
[ "$(jq -r 'select(.kind == "header") | .artifact_count' "$tmp/master-base.jsonl")" = 8800 ]
b1_dep=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b1") | .artifact_id' "$tmp/master-base.jsonl")
b2_target=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b2") | .artifact_id' "$tmp/master-base.jsonl")
jq --arg target "$b2_target" --arg dep "$b1_dep" \
  'if .kind == "artifact" and .artifact_id == $target then .dependencies = [$dep] else . end' \
  "$tmp/master-base.jsonl" > "$tmp/master.jsonl"
"$ledger" verify --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" \
  --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" \
  --baseline "$m1_baseline" | jq -e '.result == "ok" and .assignment_artifacts == 8800' >/dev/null
omit=$(jq -r 'select(.kind == "artifact") | .artifact_id' "$tmp/master.jsonl" | head -1)
jq --arg omit "$omit" \
  'if .kind == "header" then .artifact_count -= 1 elif .kind == "artifact" and .artifact_id == $omit then empty else . end' \
  "$tmp/master.jsonl" > "$tmp/master-incomplete.jsonl"
if "$ledger" verify --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" \
  --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master-incomplete.jsonl" \
  --baseline "$m1_baseline" >/dev/null 2>&1; then
  echo "incomplete mass assignment was accepted" >&2
  exit 1
fi

b1_path=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b1") | .path' "$tmp/master.jsonl")
b2_path=$(jq -r 'select(.kind == "artifact" and .batch_id == "mass-b2") | .path' "$tmp/master.jsonl")
test_file_b1="$home/$b1_path"
test_file_b2="$home/$b2_path"
[ ! -e "$test_file_b1" ] && [ ! -L "$test_file_b1" ]
[ ! -e "$test_file_b2" ] && [ ! -L "$test_file_b2" ]
test_dir_b1=$(dirname "$test_file_b1")
test_dir_b2=$(dirname "$test_file_b2")
if [ ! -d "$test_dir_b1" ]; then mkdir -p "$test_dir_b1"; created_dir_b1=true; fi
if [ ! -d "$test_dir_b2" ]; then mkdir -p "$test_dir_b2"; created_dir_b2=true; fi
(set -C; : > "$test_file_b1")
created_file_b1=true
printf 'full-ledger regression batch one\n' >> "$test_file_b1"
"$ledger" manifest --persona "$persona" --out "$tmp/after-b1.jsonl"
"$ledger" verify --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" \
  --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" \
  --baseline "$m1_baseline" --before "$tmp/current.jsonl" --batch-id mass-b1 \
  --after "$tmp/after-b1.jsonl" | jq -e '.result == "ok" and .before_files == 200 and .after_files == 201 and .current_batch_artifacts == 1' >/dev/null
(set -C; : > "$test_file_b2")
created_file_b2=true
printf 'full-ledger regression batch two\n' >> "$test_file_b2"
"$ledger" manifest --persona "$persona" --out "$tmp/after-b2.jsonl"
"$ledger" verify --persona "$persona" \
  --inventory "$tmp/inventory.jsonl" --m1-assignment "$m1_assignment" \
  --reservations "$tmp/reservations.jsonl" --assignment "$tmp/master.jsonl" \
  --baseline "$m1_baseline" --before "$tmp/after-b1.jsonl" --batch-id mass-b2 \
  --after "$tmp/after-b2.jsonl" | jq -e '.result == "ok" and .before_files == 201 and .after_files == 202 and .current_batch_artifacts == 1' >/dev/null

reserved=$(jq -r 'select(.kind == "reservation") | .source_id' "$tmp/reservations.jsonl" | head -1)
jq --arg id "$reserved" 'if .kind == "artifact" and .batch_id == "mass-b1" then .source_id = $id else . end' \
  "$tmp/master.jsonl" > "$tmp/assignment-reserved.jsonl"
if "$ledger" verify --persona "$persona" --inventory "$tmp/inventory.jsonl" \
  --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" \
  --assignment "$tmp/assignment-reserved.jsonl" --baseline "$m1_baseline" >/dev/null 2>&1; then
  echo "reserved source was accepted" >&2
  exit 1
fi
jq 'if .kind == "artifact" and .batch_id == "mass-b1" then .dependencies += ["missing-dependency"] else . end' \
  "$tmp/master.jsonl" > "$tmp/assignment-dependency.jsonl"
if "$ledger" verify --persona "$persona" --inventory "$tmp/inventory.jsonl" \
  --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" \
  --assignment "$tmp/assignment-dependency.jsonl" --baseline "$m1_baseline" >/dev/null 2>&1; then
  echo "unknown dependency was accepted" >&2
  exit 1
fi
jq 'if .kind == "header" then .render_digest = "sha256:bad" else . end' \
  "$tmp/master.jsonl" > "$tmp/assignment-digest.jsonl"
if "$ledger" verify --persona "$persona" --inventory "$tmp/inventory.jsonl" \
  --m1-assignment "$m1_assignment" --reservations "$tmp/reservations.jsonl" \
  --assignment "$tmp/assignment-digest.jsonl" --baseline "$m1_baseline" >/dev/null 2>&1; then
  echo "bad render digest was accepted" >&2
  exit 1
fi
jq 'if .kind == "artifact" then .family = "txt_log" else . end' "$m1_assignment" > "$tmp/m1-assignment-changed.jsonl"
if "$ledger" reserve-m1 --persona "$persona" --inventory "$tmp/inventory.jsonl" \
  --baseline "$m1_baseline" --m1-assignment "$tmp/m1-assignment-changed.jsonl" \
  --out "$tmp/reservations-family-changed.jsonl" >/dev/null 2>&1; then
  echo "changed M1 assignment was accepted" >&2
  exit 1
fi

rm -f "$test_file_b1" "$test_file_b2"
created_file_b1=false
created_file_b2=false
[ "$created_dir_b2" = true ] && rmdir "$test_dir_b2" 2>/dev/null || true
[ "$created_dir_b1" = true ] && rmdir "$test_dir_b1" 2>/dev/null || true
created_dir_b1=false
created_dir_b2=false
test_file_b1=
test_file_b2=
test_dir_b1=
test_dir_b2=

if ! cleanup; then
  trap - EXIT HUP INT TERM
  exit 1
fi
trap - EXIT HUP INT TERM
printf '%s\n' 'full-ledger tests: ok'
