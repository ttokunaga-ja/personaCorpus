#!/usr/bin/env bash
# Isolated regression coverage for bin/full-resume-gate; never touches a persona.
set -euo pipefail

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$(mktemp -d "${TMPDIR:-/tmp}/full-resume-gate.XXXXXX")
trap 'rm -rf "$work"' EXIT

python3 - "$work" <<'PY'
import hashlib, json, pathlib, sys, io
from PIL import Image
w=pathlib.Path(sys.argv[1]); h=w/'home'; c=w/'checkpoints'; m=w/'manifests'
for p in (h,c,m): p.mkdir()
(h/'m1.txt').write_text('m1\n')
img=Image.new('RGB',(2,3),(255,255,255)); img.save(h/'image.png')
img.save(h/'new.pdf','PDF',resolution=72.0)
plan='sha256:'+'a'*64
def sha(p): return 'sha256:'+hashlib.sha256(p.read_bytes()).hexdigest()
def row(p): return {'schema':'persona-corpus.full-manifest/v1','kind':'file','persona':'p99','path':p.name,'bytes':p.stat().st_size,'sha256':sha(p)}
def mf(name, rows):
 p=m/name; p.write_text('\n'.join(json.dumps(x,separators=(',',':')) for x in [{'schema':'persona-corpus.full-manifest/v1','kind':'header','persona':'p99','plan_digest':plan,'file_count':len(rows)},*rows])+'\n'); return p
def fd(p): return hashlib.sha256(p.read_bytes()).hexdigest()
base=mf('m1-baseline.jsonl',[row(h/'m1.txt')])
b1_after=mf('b1.after.jsonl',[row(h/'m1.txt'),row(h/'image.png')])
b2_before=m/'b2.before.jsonl'; b2_before.write_bytes(b1_after.read_bytes())
b2_after=mf('b2.after.jsonl',[row(h/'m1.txt'),row(h/'image.png'),row(h/'new.pdf')])
assign=w/'assignment.jsonl'
header={'schema':'persona-corpus.full-addition-assignment/v1','kind':'header','persona':'p99','plan_digest':plan,'artifact_count':2}
image={'schema':'persona-corpus.full-addition-assignment/v1','kind':'artifact','persona':'p99','artifact_id':'p99-full-image','scope_id':'p99-primary-01','batch_id':'p99-primary-01-image','path':'image.png','family':'image'}
scan={'schema':'persona-corpus.full-addition-assignment/v1','kind':'artifact','persona':'p99','artifact_id':'p99-full-scan','scope_id':'p99-primary-01','batch_id':'p99-primary-01-scan','path':'new.pdf','family':'pdf_scan','dependencies':['p99-full-image']}
assign.write_text('\n'.join(json.dumps(x,separators=(',',':')) for x in [header,image,scan])+'\n')
cp=lambda batch, group, before, after, rows: {'schema':'persona-corpus.full-batch-checkpoint/v2','kind':'checkpoint','persona':'p99','plan_digest':plan,'scope_id':'p99-primary-01','batch_id':batch,'assignment_count':1,'accepted_additions':1,'m1_file_count':1,'m1_sha256_unchanged':True,'acceptance_group':group,'before_manifest':before.name,'after_manifest':after.name,'before_manifest_sha256':fd(before),'after_manifest_sha256':fd(after),'family_counts':({'image':1} if batch.endswith('image') else {'pdf_scan':1}),'qa':{'structure':True,'family':True,'content_spine':True,'dependencies':True,'skill_required':True,'secrets_pii':True},'accepted_at':'2026-08-24T00:00:00Z','scan_provenance':rows}
pixels=(2).to_bytes(8,'big')+(3).to_bytes(8,'big')+b'RGB\0'+img.tobytes(); pixel_digest='sha256:'+hashlib.sha256(pixels).hexdigest()
prov=[{'scan_artifact_id':'p99-full-scan','scan_path':'new.pdf','scan_sha256':row(h/'new.pdf')['sha256'],'source_kind':'dependency','dependency_artifact_id':'p99-full-image','image_path':'image.png','image_sha256':row(h/'image.png')['sha256'],'pixel_qa':{'passed':True,'width':2,'height':3,'method':'normalized-rgb-sha256/v1'},'source_pixel_sha256':pixel_digest,'rendered_pixel_sha256':pixel_digest}]
(c/'p99-primary-01-image.json').write_text(json.dumps(cp('p99-primary-01-image','g1',base,b1_after,[])))
(c/'p99-primary-01-scan.json').write_text(json.dumps(cp('p99-primary-01-scan','g2',b2_before,b2_after,prov)))
PY

gate=("$root/bin/full-resume-gate" --persona p99 --home "$work/home" --baseline "$work/manifests/m1-baseline.jsonl" --assignment "$work/assignment.jsonl" --checkpoints-dir "$work/checkpoints" --manifests-dir "$work/manifests" --require-scan-provenance)
"${gate[@]}" >/dev/null

# A scan may not consume an image accepted in the same atomic group.
cp "$work/checkpoints/p99-primary-01-image.json" "$work/image-checkpoint.saved"
cp "$work/checkpoints/p99-primary-01-scan.json" "$work/scan-checkpoint.saved"
python3 - "$work/checkpoints/p99-primary-01-image.json" "$work/checkpoints/p99-primary-01-scan.json" "$work/manifests/m1-baseline.jsonl" "$work/manifests/b2.after.jsonl" <<'PY'
import hashlib,json,sys
image,scan,before,after=map(__import__('pathlib').Path,sys.argv[1:])
for p in (image,scan):
 x=json.loads(p.read_text()); x['acceptance_group']='same'; x['before_manifest']=before.name; x['after_manifest']=after.name; x['before_manifest_sha256']=hashlib.sha256(before.read_bytes()).hexdigest(); x['after_manifest_sha256']=hashlib.sha256(after.read_bytes()).hexdigest(); p.write_text(json.dumps(x))
PY
if "${gate[@]}" >/dev/null 2>&1; then echo 'expected same-group image dependency failure' >&2; exit 1; fi
mv "$work/image-checkpoint.saved" "$work/checkpoints/p99-primary-01-image.json"
mv "$work/scan-checkpoint.saved" "$work/checkpoints/p99-primary-01-scan.json"

# A missing checkpoint directory is the explicit zero-accepted-batch state;
# it passes only while the live home is still the baseline.
mv "$work/checkpoints" "$work/checkpoints.saved"
mv "$work/home/new.pdf" "$work/new.saved"
mv "$work/home/image.png" "$work/image.saved"
mkdir "$work/zero-manifests"
cp "$work/manifests/m1-baseline.jsonl" "$work/zero-manifests/m1-baseline.jsonl"
zero_gate=("$root/bin/full-resume-gate" --persona p99 --home "$work/home" --baseline "$work/zero-manifests/m1-baseline.jsonl" --assignment "$work/assignment.jsonl" --checkpoints-dir "$work/checkpoints" --manifests-dir "$work/zero-manifests" --require-scan-provenance)
"${zero_gate[@]}" >/dev/null
mv "$work/new.saved" "$work/home/new.pdf"
mv "$work/image.saved" "$work/home/image.png"
if "${zero_gate[@]}" >/dev/null 2>&1; then echo 'expected zero-checkpoint live mismatch failure' >&2; exit 1; fi
mv "$work/checkpoints.saved" "$work/checkpoints"

# Each mutation must fail closed, then be restored before the next case.
expect_fail() { if "$@" >/dev/null 2>&1; then echo "expected failure: $*" >&2; exit 1; fi; }
cp "$work/assignment.jsonl" "$work/assignment.saved"
python3 - "$work/assignment.jsonl" <<'PY'
import json,sys
p=sys.argv[1]; xs=[json.loads(x) for x in open(p)]; xs[1]['path']='../escape.png'; open(p,'w').write('\n'.join(json.dumps(x) for x in xs)+'\n')
PY
expect_fail "${gate[@]}"
mv "$work/assignment.saved" "$work/assignment.jsonl"
cp "$work/manifests/m1-baseline.jsonl" "$work/baseline.saved"
python3 - "$work/manifests/m1-baseline.jsonl" <<'PY'
import json,sys
p=sys.argv[1]; xs=[json.loads(x) for x in open(p)]; xs[0]['schema']='wrong'; open(p,'w').write('\n'.join(json.dumps(x) for x in xs)+'\n')
PY
expect_fail "${gate[@]}"
mv "$work/baseline.saved" "$work/manifests/m1-baseline.jsonl"
cp "$work/checkpoints/p99-primary-01-scan.json" "$work/scan.saved"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['persona']='p98'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['batch_id']='p99-unknown'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scope_id']='p99-primary-99'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['assignment_count']=2; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
cp "$work/checkpoints/p99-primary-01-scan.json" "$work/checkpoints/duplicate.json"
expect_fail "${gate[@]}"
rm "$work/checkpoints/duplicate.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['after_manifest']='m1-baseline.jsonl'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
# A second group rooted at the baseline is a fork, even when its references and
# checkpoint-bound digest are internally consistent.
python3 - "$work/checkpoints/p99-primary-01-scan.json" "$work/manifests/m1-baseline.jsonl" <<'PY'
import hashlib,json,sys
p,base=sys.argv[1:]; x=json.load(open(p)); x['before_manifest']='m1-baseline.jsonl'; x['before_manifest_sha256']=hashlib.sha256(open(base,'rb').read()).hexdigest(); open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
cp "$work/manifests/b2.after.jsonl" "$work/b2-after.saved"
python3 - "$work/manifests/b2.after.jsonl" <<'PY'
import json,sys
p=sys.argv[1]; xs=[json.loads(x) for x in open(p)]; xs[1]['sha256']='sha256:'+'c'*64; open(p,'w').write('\n'.join(json.dumps(x,separators=(',',':')) for x in xs)+'\n')
PY
expect_fail "${gate[@]}"
mv "$work/b2-after.saved" "$work/manifests/b2.after.jsonl"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scan_provenance'][0]['image_sha256']='sha256:'+'0'*64; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['release_token']='forbidden'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); del x['qa']; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['accepted_at']='not-utc'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scan_provenance'][0]['pixel_qa']['width']=99; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scan_provenance'][0]['scan_sha256']='sha256:'+'0'*64; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scan_provenance'][0]['dependency_artifact_id']='p99-full-scan'; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
python3 - "$work/checkpoints/p99-primary-01-scan.json" <<'PY'
import json,sys
p=sys.argv[1]; x=json.load(open(p)); x['scan_provenance']=[]; open(p,'w').write(json.dumps(x))
PY
expect_fail "${gate[@]}"
cp "$work/scan.saved" "$work/checkpoints/p99-primary-01-scan.json"
rm "$work/checkpoints/p99-primary-01-scan.json"
ln -s "$work/checkpoints/p99-primary-01-image.json" "$work/checkpoints/unsafe.json"
expect_fail "${gate[@]}"
rm "$work/checkpoints/unsafe.json"
mkdir "$work/home/__pycache__"
expect_fail "${gate[@]}"
rmdir "$work/home/__pycache__"
printf 'changed\n' > "$work/home/m1.txt"
expect_fail "${gate[@]}"

echo 'full-resume-gate: ok'
