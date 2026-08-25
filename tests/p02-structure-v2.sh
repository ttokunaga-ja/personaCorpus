#!/bin/sh
# Isolated fixture coverage; it never opens the production p02 home or ledgers.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tool="$root/bin/p02-structure-v2"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p02-structure-v2.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

# The executable is production-strict (14,800 + 200).  Importing it with
# reduced in-memory constants gives the same plan/apply/verify code a small,
# fast fixture without adding a production CLI bypass.
run_tool() {
  python3 - "$tool" "$@" <<'PY'
import importlib.machinery,sys
m=importlib.machinery.SourceFileLoader('p02v2',sys.argv[1]).load_module()
m.FULL_COUNT=4
m.TOTAL_COUNT=204
sys.argv=[sys.argv[1],*sys.argv[2:]]
m.main()
PY
}

python3 - "$tool" <<'PY'
import importlib.machinery, sys
m=importlib.machinery.SourceFileLoader('p02v2', sys.argv[1]).load_module()
assert m.safe_path('documents/operations/runbooks/2026-q3/report.md')
assert not m.safe_path('documents//operations/report.md')
assert not m.safe_path('documents\\operations/report.md')
assert not m.safe_path('../report.md')
PY

python3 - "$tmp" <<'PY'
import hashlib, json, sys
from pathlib import Path

t=Path(sys.argv[1]); home=t/'home'; scope='documents/operations/runbooks'
rows=[]
for number in range(1,5):
    # The shared natural stem deliberately exercises deterministic -2/-3
    # collision suffixing after the terminal tracking token is removed.
    ext='csv' if number % 3 else 'md'
    family='csv_tsv' if ext == 'csv' else 'md'
    bucket='supporting-data' if ext == 'csv' else 'procedures'
    name=f'2026-07-13-reliability-report-{ext}-s{number:06d}.{ext}'
    p=home/scope/bucket/name; p.parent.mkdir(parents=True,exist_ok=True); p.write_text(f'full {number}')
    rows.append({'kind':'artifact','persona':'p02','artifact_id':f'p02-full-{number:06d}',
                 'source_id':f'p02-src-{number:06d}','scope_id':'p02-primary-01','scope_path':scope,
                 'family':family,'physical_extension':ext,'path':p.relative_to(home).as_posix()})
first=rows[0]['path']
m1=[]
for number in range(200):
    p=home/'archive/closed'/f'm1-{number:03d}.txt'; p.parent.mkdir(parents=True,exist_ok=True)
    # A retained M1 basename/path reference must force a compatibility route.
    p.write_text(first if number == 0 else f'm1 {number}')
    data=p.read_bytes()
    m1.append({'kind':'file','persona':'p02','path':p.relative_to(home).as_posix(),
               'bytes':len(data),'sha256':'sha256:'+hashlib.sha256(data).hexdigest()})
with (t/'assignment.jsonl').open('w') as out:
    out.write(json.dumps({'schema':'persona-corpus.full-addition-assignment/v1','kind':'header','persona':'p02','artifact_count':4,'assignment_id':'fixture'})+'\n')
    for row in rows: out.write(json.dumps(row)+'\n')
with (t/'m1.jsonl').open('w') as out:
    out.write(json.dumps({'schema':'persona-corpus.full-manifest/v1','kind':'header','persona':'p02','file_count':200})+'\n')
    for row in m1: out.write(json.dumps(row)+'\n')
files=[]
for p in sorted(home.rglob('*')):
    if p.is_file():
        data=p.read_bytes(); files.append({'kind':'file','persona':'p02','path':p.relative_to(home).as_posix(),
                                            'bytes':len(data),'sha256':'sha256:'+hashlib.sha256(data).hexdigest()})
assert len(files)==204
with (t/'predecessor.jsonl').open('w') as out:
    out.write(json.dumps({'kind':'header','persona':'p02','file_count':204})+'\n')
    for row in files: out.write(json.dumps(row)+'\n')
PY

assignment="$tmp/assignment.jsonl"
home="$tmp/home"
baseline="$tmp/m1.jsonl"
predecessor="$tmp/predecessor.jsonl"
mapping="$tmp/mapping.jsonl"
first_full=$(python3 - "$assignment" <<'PY'
import json,sys
count=0
for row in map(json.loads,open(sys.argv[1])):
 if row.get('kind')=='artifact':
  count+=1
  if count == 2: print(row['path']); break
PY
)
# A staging hardlink outside the persona home is legitimate and must not be
# confused with duplicate paths inside home.
ln "$home/$first_full" "$tmp/staging-original"
ln "$home/archive/closed/m1-002.txt" "$home/archive/closed/m1-duplicate.txt"
if python3 - "$tool" "$home" >/dev/null 2>&1 <<'PY'
import importlib.machinery,sys
from pathlib import Path
m=importlib.machinery.SourceFileLoader('p02v2',sys.argv[1]).load_module()
m.home_inode_guard(Path(sys.argv[2]))
PY
then
  echo 'duplicate inode inside home accepted' >&2; exit 1
fi
rm "$home/archive/closed/m1-duplicate.txt"
run_tool plan --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --out "$mapping"
run_tool verify --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$mapping" --state before >/dev/null

# The target named by retained M1 stays at its old path; all other generated
# collision names receive deterministic suffixes by immutable artifact id.
python3 - "$mapping" <<'PY'
import json,sys
rows=[json.loads(x) for x in open(sys.argv[1])]
assert rows[1]['compatibility_path_preserved'] is True
assert rows[1]['new_path']==rows[1]['old_path']
assert any(row.get('new_path','').endswith('-2.csv') for row in rows[2:])
PY

# A forged mapping with a refreshed digest still cannot cross scope or use an
# unsafe path.  It must be rejected before any live move occurs.
cp "$mapping" "$tmp/unsafe.jsonl"
python3 - "$tmp/unsafe.jsonl" <<'PY'
import hashlib,json,sys
p=sys.argv[1]; rows=[json.loads(x) for x in open(p)]
rows[2]['new_path']='../outside.csv'
rows[0]['mapping_rows_sha256']='sha256:'+hashlib.sha256(''.join(json.dumps(x,sort_keys=True,separators=(',',':'))+'\n' for x in rows[1:]).encode()).hexdigest()
open(p,'w').write('\n'.join(json.dumps(x,sort_keys=True,separators=(',',':')) for x in rows)+'\n')
PY
if run_tool apply --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$tmp/unsafe.jsonl" >/dev/null 2>&1; then
  echo 'unsafe mapping accepted' >&2; exit 1
fi

cp "$mapping" "$tmp/collision.jsonl"
python3 - "$tmp/collision.jsonl" <<'PY'
import hashlib,json,sys
p=sys.argv[1]; rows=[json.loads(x) for x in open(p)]
rows[4]['new_path']=rows[2]['new_path']
rows[0]['mapping_rows_sha256']='sha256:'+hashlib.sha256(''.join(json.dumps(x,sort_keys=True,separators=(',',':'))+'\n' for x in rows[1:]).encode()).hexdigest()
open(p,'w').write('\n'.join(json.dumps(x,sort_keys=True,separators=(',',':')) for x in rows)+'\n')
PY
if run_tool apply --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$tmp/collision.jsonl" >/dev/null 2>&1; then
  echo 'collision mapping accepted' >&2; exit 1
fi

# A same-inode destination simulates interruption between hard-link and unlink;
# the normal apply resumes without copying bytes or clobbering other files.
python3 - "$mapping" "$tmp/resume.tsv" <<'PY'
import json,sys
for row in map(json.loads, open(sys.argv[1])):
 if row.get('kind')=='move' and not row['compatibility_path_preserved']:
  open(sys.argv[2],'w').write(row['old_path']+'\t'+row['new_path']+'\n'); break
PY
read -r old new < "$tmp/resume.tsv"
mkdir -p "$(dirname "$home/$new")"
ln "$home/$old" "$home/$new"
# Even a coordinated live-M1 and baseline edit is rejected on partial resume:
# the mapping header remains bound to the original baseline and predecessor.
cp "$baseline" "$tmp/m1-original.jsonl"
cp "$home/archive/closed/m1-001.txt" "$tmp/m1-001-original.txt"
printf 'tampered M1' > "$home/archive/closed/m1-001.txt"
python3 - "$baseline" "$home/archive/closed/m1-001.txt" <<'PY'
import hashlib,json,sys
p,file=sys.argv[1:]; rows=[json.loads(x) for x in open(p)]
data=open(file,'rb').read()
for row in rows:
 if row.get('path')=='archive/closed/m1-001.txt':
  row['bytes']=len(data); row['sha256']='sha256:'+hashlib.sha256(data).hexdigest()
open(p,'w').write('\n'.join(json.dumps(x) for x in rows)+'\n')
PY
if run_tool apply --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$mapping" >/dev/null 2>&1; then
  echo 'coordinated M1/baseline mutation accepted' >&2; exit 1
fi
mv "$tmp/m1-original.jsonl" "$baseline"
mv "$tmp/m1-001-original.txt" "$home/archive/closed/m1-001.txt"
run_tool apply --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$mapping"
run_tool verify --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --mapping "$mapping" --state after >/dev/null
[ -f "$home/archive/closed/m1-000.txt" ]
[ -f "$tmp/staging-original" ]
[ "$(find "$home" -type f | wc -l | tr -d ' ')" -eq 204 ]
echo 'p02 structure v2 tests: ok'
