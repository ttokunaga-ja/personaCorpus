#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
PYTHONDONTWRITEBYTECODE=1 "$root/bin/p15-full-content-spine" verify | jq -e '.result == "ok" and .content_reference_duplicates == 0' >/dev/null
python3 - "$root/progress/p15/full/full-assignment-ledger.jsonl" <<'PY'
import json,sys
rows=[json.loads(x) for x in open(sys.argv[1],encoding='utf-8')][1:]
assert len(rows)==7800
assert all(len(x['content_spine_ids']) == len(set(x['content_spine_ids'])) for x in rows)
assert all('program.requisition-beta' not in x['content_spine_ids'] for x in rows if x['scope_id'] in {'p15-primary-01','p15-primary-02','p15-primary-05','p15-primary-10','p15-secondary-01','p15-secondary-05'})
assert all('program.requisition-alpha' not in x['content_spine_ids'] for x in rows if x['scope_id'] in {'p15-primary-03','p15-primary-04','p15-secondary-03','p15-secondary-07'})
PY
printf '%s\n' 'p15 full content spine tests: ok'
