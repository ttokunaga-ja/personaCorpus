#!/bin/sh
# Isolated fixture coverage: production home and ledgers are never opened.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tool="$root/bin/p01-structure-v2"
tmp=$(mktemp -d "${TMPDIR:-/tmp}/p01-structure-v2.XXXXXX")
trap 'rm -rf "$tmp"' EXIT HUP INT TERM

# Frozen paths may retain uppercase, but separator/control normalization is
# never accepted silently.
python3 - "$tool" <<'PY'
import importlib.machinery,sys
m=importlib.machinery.SourceFileLoader('p01v2',sys.argv[1]).load_module()
assert m.safe('docs/ADR-042.md')
assert not m.safe('docs//ADR-042.md')
assert not m.safe('docs\\ADR-042.md')
assert not m.safe('docs/ADR\x01.md')
PY

python3 - "$tmp" <<'PY'
import hashlib,json,sys,zipfile
from pathlib import Path
t=Path(sys.argv[1]); h=t/'home'
full=[
 ('p01-primary-01','documents/work/product-alpha/architecture','md','md','arch-src-000001.md','alpha'),
 ('p01-primary-01','documents/work/product-alpha/architecture','pdf_text','pdf','arch-src-000002.pdf','beta'),
 ('p01-secondary-07','mail/recent','html_eml','eml','mail-src-000003.eml','gamma'),
 ('p01-primary-06','repos/product-alpha/docs','code','py','doc-src-000004.py','delta'),
 ('p01-primary-01','documents/work/product-alpha/architecture','pdf_text','pdf','arch-src-000005.pdf','arch-src-000001.md')]
rows=[]
for i,(sid,scope,fam,ext,name,data) in enumerate(full,1):
 p=h/scope/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(data)
 rows.append({'kind':'artifact','persona':'p01','artifact_id':f'a{i}','source_id':f'p01-src-{i:06d}','scope_id':sid,'scope_path':scope,'family':fam,'physical_extension':ext,'path':f'{scope}/{name}'})
# A real OOXML container is a referrer to the Full PDF target.  The test's
# pdftotext mock below makes the PDF referrer test deterministic as well.
p=h/'documents/work/product-alpha/architecture/arch-src-000006.docx'
with zipfile.ZipFile(p,'w') as z:z.writestr('word/document.xml','<w:t>arch-src-000002.pdf</w:t>')
rows.append({'kind':'artifact','persona':'p01','artifact_id':'a6','source_id':'p01-src-000006','scope_id':'p01-primary-01','scope_path':'documents/work/product-alpha/architecture','family':'docx','physical_extension':'docx','path':'documents/work/product-alpha/architecture/arch-src-000006.docx'})
for i in range(12):
 name=f'natural-pilot-{i:02d}.md';p=h/'documents/work/product-alpha/architecture'/name;p.write_text('pilot'+str(i))
 rows.append({'kind':'artifact','persona':'p01','artifact_id':f'pilot{i}','source_id':f'p01-src-{12000-i:06d}','scope_id':'p01-primary-01','scope_path':'documents/work/product-alpha/architecture','family':'md','physical_extension':'md','path':f'documents/work/product-alpha/architecture/{name}'})
# Complete the exact production cardinality with tiny isolated fixture files.
for i in range(7,11789):
 name=f'doc-src-{i:06d}.py';p=h/'repos/product-alpha/docs'/name;p.write_text('x'+str(i))
 rows.append({'kind':'artifact','persona':'p01','artifact_id':f'a{i}','source_id':f'p01-src-{i:06d}','scope_id':'p01-primary-06','scope_path':'repos/product-alpha/docs','family':'code','physical_extension':'py','path':f'repos/product-alpha/docs/{name}'})
with (t/'assignment.jsonl').open('w') as f:
 f.write(json.dumps({'schema':'persona-corpus.full-addition-assignment/v1','kind':'header','persona':'p01','artifact_count':len(rows),'assignment_id':'fixture'})+'\n')
 for x in rows:f.write(json.dumps(x)+'\n')
m=[]
for i in range(200):
 name=('M1-000.txt' if i==0 else f'm1-{i:03d}.txt');p=h/'repos/product-alpha/docs'/name;p.write_text('m1'+str(i));m.append({'kind':'file','path':f'repos/product-alpha/docs/{name}','bytes':p.stat().st_size,'sha256':'sha256:'+hashlib.sha256(p.read_bytes()).hexdigest()})
with (t/'m1.jsonl').open('w') as f:
 f.write(json.dumps({'kind':'header'})+'\n')
 for x in m:f.write(json.dumps(x)+'\n')
allfiles=[]
for p in sorted(h.rglob('*')):
 if p.is_file():allfiles.append({'kind':'file','path':p.relative_to(h).as_posix(),'bytes':p.stat().st_size,'sha256':'sha256:'+hashlib.sha256(p.read_bytes()).hexdigest()})
assert len(allfiles)==12000,len(allfiles)
with (t/'predecessor.jsonl').open('w') as f:
 f.write(json.dumps({'kind':'header','file_count':12000})+'\n')
 for x in allfiles:f.write(json.dumps(x)+'\n')
PY

assignment="$tmp/assignment.jsonl"; home="$tmp/home"; mapping="$tmp/mapping.jsonl"; baseline="$tmp/m1.jsonl"; predecessor="$tmp/predecessor.jsonl"
mkdir -p "$tmp/bin"
printf '%s\n' '#!/bin/sh' 'cat "$1"' > "$tmp/bin/pdftotext"
chmod +x "$tmp/bin/pdftotext"
PATH="$tmp/bin:$PATH" "$tool" plan --assignment "$assignment" --home "$home" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --out "$mapping"
PATH="$tmp/bin:$PATH" "$tool" verify --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --state before >/dev/null

# The map is digest-bound; tampering it is rejected.
cp "$mapping" "$tmp/bad.jsonl"
python3 - "$tmp/bad.jsonl" <<'PY'
import json,sys
p=sys.argv[1];a=[json.loads(x) for x in open(p)];a[0]['mapping_rows_sha256']='sha256:0';open(p,'w').write('\n'.join(json.dumps(x) for x in a)+'\n')
PY
if "$tool" apply --assignment "$assignment" --home "$home" --mapping "$tmp/bad.jsonl" >/dev/null 2>&1; then echo 'tampered mapping accepted' >&2; exit 1; fi

# A deterministic collision is rejected, even if a header digest is refreshed.
cp "$mapping" "$tmp/collision.jsonl"
python3 - "$tmp/collision.jsonl" <<'PY'
import hashlib,json,sys
p=sys.argv[1];a=[json.loads(x) for x in open(p)];a[2]['new_path']=a[1]['new_path']
rows=a[1:];a[0]['mapping_rows_sha256']='sha256:'+hashlib.sha256(''.join(json.dumps(x,sort_keys=True,separators=(',',':'))+'\n' for x in rows).encode()).hexdigest()
open(p,'w').write('\n'.join(json.dumps(x,sort_keys=True,separators=(',',':')) for x in a)+'\n')
PY
if "$tool" apply --assignment "$assignment" --home "$home" --mapping "$tmp/collision.jsonl" >/dev/null 2>&1; then echo 'collision mapping accepted' >&2; exit 1; fi

# A forged but rehashed extra row is rejected, and a post-plan source-byte
# mutation is rejected against the predecessor-bound mapping.
cp "$mapping" "$tmp/unknown-row.jsonl"
printf '%s\n' '{"kind":"note","unexpected":true}' >> "$tmp/unknown-row.jsonl"
if PATH="$tmp/bin:$PATH" "$tool" apply --assignment "$assignment" --home "$home" --mapping "$tmp/unknown-row.jsonl" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" >/dev/null 2>&1; then echo 'unknown mapping row accepted' >&2; exit 1; fi
first_old=$(sed -n '2p' "$mapping" | python3 -c 'import json,sys;print(json.load(sys.stdin)["old_path"])')
cp "$home/$first_old" "$tmp/original-first"
printf 'mutated' > "$home/$first_old"
if PATH="$tmp/bin:$PATH" "$tool" apply --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" >/dev/null 2>&1; then echo 'post-plan source mutation accepted' >&2; exit 1; fi
mv "$tmp/original-first" "$home/$first_old"

# A symlinked destination ancestor is rejected rather than followed outside
# the home root.  This fixture restores the directory before normal resume.
first_new=$(sed -n '2p' "$mapping" | python3 -c 'import json,sys;print(json.load(sys.stdin)["new_path"])')
first_parent=$(dirname "$home/$first_new")
mkdir -p "$tmp/outside" "$(dirname "$first_parent")"
ln -s "$tmp/outside" "$first_parent"
if PATH="$tmp/bin:$PATH" "$tool" apply --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" >/dev/null 2>&1; then echo 'symlinked destination ancestor accepted' >&2; exit 1; fi
rm "$first_parent"

# A race-created different-inode destination is never clobbered.
other_new=$(sed -n '3p' "$mapping" | python3 -c 'import json,sys;print(json.load(sys.stdin)["new_path"])')
mkdir -p "$(dirname "$home/$other_new")"; printf 'different' > "$home/$other_new"
if PATH="$tmp/bin:$PATH" "$tool" apply --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" >/dev/null 2>&1; then echo 'different inode destination accepted' >&2; exit 1; fi
rm "$home/$other_new"
# Simulate interruption after an exclusive link but before unlink; apply
# recognizes the same inode, removes only the old name, and resumes safely.
old="$first_old"
new="$first_new"
mkdir -p "$(dirname "$home/$new")"; ln "$home/$old" "$home/$new"
PATH="$tmp/bin:$PATH" "$tool" apply --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor"
PATH="$tmp/bin:$PATH" "$tool" verify --assignment "$assignment" --home "$home" --mapping "$mapping" --m1-baseline "$baseline" --predecessor-manifest "$predecessor" --state after >/dev/null
[ "$(jq -r 'select(.kind == "header") | .compatibility_name_preserved_count' "$mapping")" -eq 2 ]
[ "$(jq -r 'select(.kind == "header") | .reference_edge_count' "$mapping")" -eq 2 ]
[ -f "$home/documents/work/product-alpha/architecture/natural-pilot-00.md" ]
[ -f "$home/repos/product-alpha/docs/M1-000.txt" ]
# Only the two referenced targets retain compatibility basenames; all other
# selected rows were naturalized.
[ "$(find "$home" -name '*-src-??????*' | wc -l | tr -d ' ')" -eq 2 ]
echo 'p01 structure v2 tests: ok'
