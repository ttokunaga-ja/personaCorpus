#!/usr/bin/env bash
# Temp-only regression checks for p20 production's fail-closed manifest gates.
set -euo pipefail

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
base="$root/progress/p20/full/manifests/m1-baseline.jsonl"
work=$(mktemp -d "$root/scratch/p20/full/p20-ledger-test.XXXXXX")
trap 'rm -rf "$work"' EXIT

test -f "$base" && test ! -L "$base"

# Production's initial manifest route: the helper creates a temp-only current
# manifest and cmp accepts it only when it is the immutable M1 baseline.
"$root/bin/full-ledger" manifest --persona p20 --out "$work/live-initial.jsonl"
cmp "$base" "$work/live-initial.jsonl"

python3 - "$base" "$work" <<'PY'
import json, os, pathlib, shutil, sys
base, work = map(pathlib.Path, sys.argv[1:])
lines = base.read_text(encoding="utf-8").splitlines()
header, rows = json.loads(lines[0]), [json.loads(x) for x in lines[1:]]
assert header["file_count"] == 200 and len(rows) == 200

def write(name, values):
    p = work / name
    p.write_text("\n".join(json.dumps(x, separators=(",", ":")) for x in [header, *values]) + "\n", encoding="utf-8")
    return p

write("changed.jsonl", [{**rows[0], "sha256": "0" * 64}, *rows[1:]])
write("missing.jsonl", rows[:-1])
write("extra.jsonl", [*rows, {**rows[-1], "path": "unexpected.txt", "sha256": "1" * 64}])
os.symlink(base, work / "baseline-link.jsonl")

# A minimal accepted chain has one checkpoint whose after manifest is the live
# manifest. The same gate must reject divergent live output and ambiguous chains.
after = write("after.jsonl", rows)
cp = {"schema":"persona-corpus.full-batch-checkpoint/v1", "persona":"p20", "batch_id":"p20-primary-01-full-b001-foundation", "scope_id":"p20-primary-01", "file_count":0, "acceptance_group":"p20-full-wave-001", "before_manifest":"manifests/m1-baseline.jsonl", "after_manifest":str(after)}
(work / "checkpoint-ok.json").write_text(json.dumps(cp), encoding="utf-8")
(work / "checkpoint-ambiguous.json").write_text(json.dumps({**cp, "batch_id":"p20-primary-02-full-b001-foundation", "acceptance_group":"p20-full-wave-002"}), encoding="utf-8")
PY

python3 - "$base" "$work" <<'PY'
import json, pathlib, sys
base, work = map(pathlib.Path, sys.argv[1:])
def rows(p):
    if not p.is_file() or p.is_symlink(): raise ValueError("unsafe manifest")
    x=[json.loads(z) for z in p.read_text().splitlines()]
    return {z["path"]:(z["sha256"], z.get("bytes")) for z in x[1:]}
def initial(current):
    return rows(base)==rows(current)
def resume(current, checkpoints):
    cps=[json.loads(p.read_text()) for p in checkpoints]
    if len({x["batch_id"] for x in cps}) != len(cps): return False
    # An initial-only test chain may contain zero additions, but never two
    # incomparable latest groups.
    groups={x["acceptance_group"] for x in cps}
    if len(groups)!=1: return False
    after=pathlib.Path(cps[0]["after_manifest"])
    return rows(current)==rows(after) and all(k in rows(current) and rows(current)[k]==v for k,v in rows(base).items())

assert initial(work / "live-initial.jsonl")
assert not initial(work / "changed.jsonl")
assert not initial(work / "missing.jsonl")
assert not initial(work / "extra.jsonl")
try: rows(work / "baseline-link.jsonl")
except ValueError: pass
else: raise AssertionError("symlink baseline accepted")
assert resume(work / "live-initial.jsonl", [work / "checkpoint-ok.json"])
assert not resume(work / "changed.jsonl", [work / "checkpoint-ok.json"])
assert not resume(work / "live-initial.jsonl", [work / "checkpoint-ok.json", work / "checkpoint-ambiguous.json"])
print("p20 full-ledger gates: ok")
PY
