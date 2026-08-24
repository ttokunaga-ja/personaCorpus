#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
fixture=$(mktemp -d "${TMPDIR:-/tmp}/p01-format-audit.XXXXXX")
trap 'rm -rf "$fixture"' EXIT HUP INT TERM

mkdir -p "$fixture/bin" "$fixture/progress/p01/full" \
  "$fixture/workspace/people/p01-software-engineer/home/documents"

# The fake text extractor makes PDF-family classification deterministic without
# depending on a host Poppler installation.  A scan name returns no text.
cat > "$fixture/bin/pdftotext" <<'EOF'
#!/bin/sh
case "$1" in *scan.pdf) exit 0;; *) printf 'extractable text\n';; esac
EOF
chmod +x "$fixture/bin/pdftotext"

FIXTURE="$fixture" python3 - <<'PY'
import json, os, zipfile
from pathlib import Path

root = Path(os.environ['FIXTURE'])
home = root / 'workspace/people/p01-software-engineer/home/documents'
rows = [
 ('p01-m1-0001','txt_log','txt','notes.txt'),
 ('p01-m1-0002','pdf_text','pdf','search.pdf'),
 ('p01-full-0003','pdf_scan','pdf','scan.pdf'),
 ('p01-full-0004','docx','docx','brief.docx'),
 ('p01-full-0005','xlsx','xlsx','plan.xlsx'),
 ('p01-full-0006','pptx','pptx','review.pptx'),
 ('p01-full-0007','image','png','image.png'),
 ('p01-full-0008','ipynb','ipynb','analysis.ipynb'),
 ('p01-full-0009','domain_binary','pcap','trace.pcap'),
 ('p01-full-0010','domain_binary','sqlite','state.sqlite'),
 ('p01-full-0011','domain_binary','zip','bundle.zip'),
 ('p01-full-0012','code','py','code.py'),
]
def row(identifier, family, ext, name, schema, state):
 result = {'schema':schema,'kind':'artifact','persona':'p01','artifact_id':identifier,'scope_id':'p01-primary-01','scope_path':'documents','family':family,'physical_extension':ext,'path':'documents/'+name,'state':state}
 if schema == 'persona-corpus.full-addition-assignment/v1': result['canonical_extension'] = ext
 return result
m1 = [{'schema':'persona-corpus.m1-assignment/v1','kind':'header','persona':'p01','artifact_count':2}]
full = [{'schema':'persona-corpus.full-addition-assignment/v1','kind':'header','persona':'p01','artifact_count':10}]
for index, item in enumerate(rows):
 target = m1 if index < 2 else full
 target.append(row(*item, 'persona-corpus.m1-assignment/v1' if index < 2 else 'persona-corpus.full-addition-assignment/v1', 'frozen' if index < 2 else 'planned'))
(root/'progress/p01/m1.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in m1))
(root/'progress/p01/full/full.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in full))
(root/'predecessor.jsonl').write_text('{"fixture":"predecessor"}\n')
(home/'notes.txt').write_text('notes\n')
(home/'search.pdf').write_bytes(b'%PDF-1.4 searchable')
(home/'scan.pdf').write_bytes(b'%PDF-1.4 scan')
packages = {
 'brief.docx': {
  '[Content_Types].xml':'<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>',
  '_rels/.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>',
  'word/document.xml':'<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p/></w:body></w:document>',
  'word/_rels/document.xml.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>',
  'word/styles.xml':'<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"/>',
 },
 'plan.xlsx': {
  '[Content_Types].xml':'<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/></Types>',
  '_rels/.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>',
  'xl/workbook.xml':'<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="rId1"/></sheets></workbook>',
  'xl/_rels/workbook.xml.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>',
  'xl/worksheets/sheet1.xml':'<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"/>',
 },
 'review.pptx': {
  '[Content_Types].xml':'<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/></Types>',
  '_rels/.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/></Relationships>',
  'ppt/presentation.xml':'<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:sldIdLst><p:sldId id="256" r:id="rId1"/></p:sldIdLst></p:presentation>',
  'ppt/_rels/presentation.xml.rels':'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="/ppt/slides/slide1.xml"/></Relationships>',
  'ppt/slides/slide1.xml':'<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"/>',
 },
}
for name, members in packages.items():
 with zipfile.ZipFile(home/name, 'w') as z:
  for member, content in members.items(): z.writestr(member, content)
(home/'image.png').write_bytes(b'\x89PNG\r\n\x1a\nbody')
(home/'analysis.ipynb').write_text('{"cells":[]}')
(home/'trace.pcap').write_bytes(b'\xd4\xc3\xb2\xa1body')
(home/'state.sqlite').write_bytes(b'SQLite format 3\x00body')
with zipfile.ZipFile(home/'bundle.zip', 'w') as z: z.writestr('item.txt','x')
(home/'code.py').write_text('print("ok")\n')
PY

counts='{"txt_log":1,"pdf_text":1,"pdf_scan":1,"docx":1,"xlsx":1,"pptx":1,"image":1,"ipynb":1,"domain_binary":3,"code":1}'
run() {
  PATH="$fixture/bin:$PATH" "$root/bin/p01-format-audit" --repo-root "$fixture" \
    --m1-ledger "$fixture/progress/p01/m1.jsonl" --full-ledger "$fixture/progress/p01/full/full.jsonl" \
    --home "$fixture/workspace/people/p01-software-engineer/home" --expected-count 12 \
    --expected-family-counts "$counts" --predecessor-manifest "$fixture/predecessor.jsonl" "$@"
}
reject_full() {
  if PATH="$fixture/bin:$PATH" "$root/bin/p01-format-audit" --repo-root "$fixture" \
    --m1-ledger "$fixture/progress/p01/m1.jsonl" --full-ledger "$1" \
    --home "$fixture/workspace/people/p01-software-engineer/home" --expected-count 12 \
    --expected-family-counts "$counts" >/dev/null 2>&1; then
    echo "expected rejected Full ledger: $1" >&2
    exit 1
  fi
}
run | jq -e '.result == "ok" and .artifact_count == 12 and .pdf.searchable == 1 and .pdf.scan == 1' >/dev/null
run --out "$fixture/report.json" >/dev/null
if run --out "$fixture/report.json" >/dev/null 2>&1; then
  echo 'expected create-only report rejection' >&2
  exit 1
fi
run --out "$fixture/report.json" --replace >/dev/null

# Ledger paths must be portable before they are compared to the filesystem, and
# mutable ledger fields cannot redefine canonical Office/PDF/image extensions.
FIXTURE="$fixture" python3 - <<'PY'
import json, os
from pathlib import Path
root = Path(os.environ['FIXTURE'])
rows = [json.loads(line) for line in (root/'progress/p01/full/full.jsonl').read_text().splitlines()]
def write(name, mutate):
 out = [dict(row) for row in rows]
 mutate(out)
 (root/name).write_text(''.join(json.dumps(row)+'\n' for row in out))
write('bad-canonical-extension.jsonl', lambda out: out[2].update(canonical_extension='pdf'))
write('missing-canonical-extension.jsonl', lambda out: out[2].pop('canonical_extension'))
write('bad-reserved-path.jsonl', lambda out: out[1].update(path='documents/CON.txt'))
write('bad-backslash-path.jsonl', lambda out: out[1].update(path='documents\\bad.txt'))
write('bad-trailing-path.jsonl', lambda out: out[1].update(path='documents/bad .txt'))
write('bad-repeated-slash.jsonl', lambda out: out[1].update(path='documents//bad.txt'))
write('bad-forbidden-char.jsonl', lambda out: out[1].update(path='documents/bad?.txt'))
write('bad-casefold-path.jsonl', lambda out: out[1].update(path='documents/NOTES.txt'))
write('bad-swapped-code-pdf.jsonl', lambda out: out[-1].update(physical_extension='pdf', canonical_extension='pdf', path='documents/code.pdf'))
PY
reject_full "$fixture/bad-canonical-extension.jsonl"
reject_full "$fixture/missing-canonical-extension.jsonl"
reject_full "$fixture/bad-reserved-path.jsonl"
reject_full "$fixture/bad-backslash-path.jsonl"
reject_full "$fixture/bad-trailing-path.jsonl"
reject_full "$fixture/bad-repeated-slash.jsonl"
reject_full "$fixture/bad-forbidden-char.jsonl"
reject_full "$fixture/bad-casefold-path.jsonl"
mv "$fixture/workspace/people/p01-software-engineer/home/documents/code.py" "$fixture/workspace/people/p01-software-engineer/home/documents/code.pdf"
reject_full "$fixture/bad-swapped-code-pdf.jsonl"
mv "$fixture/workspace/people/p01-software-engineer/home/documents/code.pdf" "$fixture/workspace/people/p01-software-engineer/home/documents/code.py"
mkdir "$fixture/workspace/people/p01-software-engineer/home/documents/empty"
if run >/dev/null 2>&1; then
  echo 'expected empty-directory rejection' >&2
  exit 1
fi
rmdir "$fixture/workspace/people/p01-software-engineer/home/documents/empty"

# An OOXML ZIP must also have its declared main QName and package relationship.
cp "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx" "$fixture/brief.valid.docx"
FIXTURE="$fixture" python3 - <<'PY'
import os, zipfile
from pathlib import Path
root = Path(os.environ['FIXTURE']); path = root/'workspace/people/p01-software-engineer/home/documents/brief.docx'
with zipfile.ZipFile(path) as old: members = {name: old.read(name) for name in old.namelist()}
members['word/document.xml'] = b'<wrong/>'
with zipfile.ZipFile(path, 'w') as new:
 for name, content in members.items(): new.writestr(name, content)
PY
if run >/dev/null 2>&1; then
  echo 'expected OOXML QName rejection' >&2
  exit 1
fi
cp "$fixture/brief.valid.docx" "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx"
cp "$fixture/workspace/people/p01-software-engineer/home/documents/plan.xlsx" "$fixture/plan.valid.xlsx"
FIXTURE="$fixture" python3 - <<'PY'
import os, zipfile
from pathlib import Path
root = Path(os.environ['FIXTURE']); path = root/'workspace/people/p01-software-engineer/home/documents/plan.xlsx'
with zipfile.ZipFile(path) as old: members = {name: old.read(name) for name in old.namelist()}
members['xl/_rels/workbook.xml.rels'] = b'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/decoy.xml"/></Relationships>'
members['xl/worksheets/decoy.xml'] = b'<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"/>'
with zipfile.ZipFile(path, 'w') as new:
 for name, content in members.items(): new.writestr(name, content)
PY
if run >/dev/null 2>&1; then
  echo 'expected unreferenced OOXML child relationship rejection' >&2
  exit 1
fi
cp "$fixture/plan.valid.xlsx" "$fixture/workspace/people/p01-software-engineer/home/documents/plan.xlsx"
FIXTURE="$fixture" python3 - <<'PY'
import os, zipfile
from pathlib import Path
root = Path(os.environ['FIXTURE']); path = root/'workspace/people/p01-software-engineer/home/documents/brief.docx'
with zipfile.ZipFile(path) as old: members = {name: old.read(name) for name in old.namelist()}
members['_rels/.rels'] = b'<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="wrong.xml"/></Relationships>'
with zipfile.ZipFile(path, 'w') as new:
 for name, content in members.items(): new.writestr(name, content)
PY
if run >/dev/null 2>&1; then
  echo 'expected OOXML relationship rejection' >&2
  exit 1
fi
cp "$fixture/brief.valid.docx" "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx"

# A revision changes only the effective expected path; the old path must not
# remain in the tree.  This is the migration-safe overlay contract.
mkdir -p "$fixture/workspace/people/p01-software-engineer/home/documents/revised"
FIXTURE="$fixture" python3 - <<'PY'
import hashlib, json, os
from pathlib import Path
root = Path(os.environ['FIXTURE'])
moves = [{"schema":"persona-corpus.p01-full-path-revision/v1","kind":"move","artifact_id":"p01-m1-0002","old_path":"documents/search.pdf","new_path":"documents/revised/search.pdf"}]
def dump(rows): return ''.join(json.dumps(row, sort_keys=True, separators=(',', ':')) + '\n' for row in rows)
def sha(path): return 'sha256:'+hashlib.sha256(path.read_bytes()).hexdigest()
header = {"schema":"persona-corpus.p01-full-path-revision/v1","kind":"header","persona":"p01","revision":"p01-full-structure-v2","selected_count":len(moves),"preserved_pilot_count":2,"assignment_sha256":sha(root/'progress/p01/full/full.jsonl'),"predecessor_manifest_sha256":sha(root/'predecessor.jsonl'),"mapping_rows_sha256":"sha256:"+hashlib.sha256(dump(moves).encode()).hexdigest()}
(root/'m1-overlay.jsonl').write_text(dump([header, *moves]))
moves[0]['artifact_id'] = 'p01-full-0004'
moves[0]['old_path'] = 'documents/brief.docx'
moves[0]['new_path'] = 'documents/revised/brief.docx'
header['selected_count'] = len(moves); header['mapping_rows_sha256'] = 'sha256:'+hashlib.sha256(dump(moves).encode()).hexdigest()
(root/'revision.jsonl').write_text(dump([header, *moves]))
header['selected_count'] = 2
(root/'bad-header.jsonl').write_text(dump([header, *moves]))
missing = dict(header); missing.pop('predecessor_manifest_sha256')
(root/'missing-binding.jsonl').write_text(dump([missing, *moves]))
PY
# M1 rows are never movable, even when the mapping otherwise has the official
# schema and a valid digest.
if run --path-revision "$fixture/m1-overlay.jsonl" >/dev/null 2>&1; then
  echo 'expected M1 overlay rejection' >&2
  exit 1
fi
if run --path-revision "$fixture/bad-header.jsonl" >/dev/null 2>&1; then
  echo 'expected revision header rejection' >&2
  exit 1
fi
if run --path-revision "$fixture/missing-binding.jsonl" >/dev/null 2>&1; then
  echo 'expected missing revision binding rejection' >&2
  exit 1
fi
mv "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx" "$fixture/workspace/people/p01-software-engineer/home/documents/revised/brief.docx"
run --path-revision "$fixture/revision.jsonl" >/dev/null
mv "$fixture/workspace/people/p01-software-engineer/home/documents/revised/brief.docx" "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx"

if run --path-revision "$fixture/revision.jsonl" >/dev/null 2>&1; then
  echo 'expected migration path mismatch rejection' >&2
  exit 1
fi
mv "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx" "$fixture/workspace/people/p01-software-engineer/home/documents/revised/brief.docx"
mv "$fixture/workspace/people/p01-software-engineer/home/documents/review.pptx" "$fixture/workspace/people/p01-software-engineer/home/documents/review.bad"
FIXTURE="$fixture" python3 - <<'PY'
import hashlib, json, os
from pathlib import Path
root = Path(os.environ['FIXTURE'])
moves = [
 {"schema":"persona-corpus.p01-full-path-revision/v1","kind":"move","artifact_id":"p01-full-0004","old_path":"documents/brief.docx","new_path":"documents/revised/brief.docx"},
 {"schema":"persona-corpus.p01-full-path-revision/v1","kind":"move","artifact_id":"p01-full-0006","old_path":"documents/review.pptx","new_path":"documents/review.bad"},
]
def dump(rows): return ''.join(json.dumps(row, sort_keys=True, separators=(',', ':')) + '\n' for row in rows)
def sha(path): return 'sha256:'+hashlib.sha256(path.read_bytes()).hexdigest()
header = {"schema":"persona-corpus.p01-full-path-revision/v1","kind":"header","persona":"p01","revision":"p01-full-structure-v2","selected_count":len(moves),"preserved_pilot_count":2,"assignment_sha256":sha(root/'progress/p01/full/full.jsonl'),"predecessor_manifest_sha256":sha(root/'predecessor.jsonl'),"mapping_rows_sha256":"sha256:"+hashlib.sha256(dump(moves).encode()).hexdigest()}
(root/'bad-extension-revision.jsonl').write_text(dump([header, *moves]))
PY
if run --path-revision "$fixture/bad-extension-revision.jsonl" >/dev/null 2>&1; then
  echo 'expected extension mismatch rejection' >&2
  exit 1
fi
mv "$fixture/workspace/people/p01-software-engineer/home/documents/review.bad" "$fixture/workspace/people/p01-software-engineer/home/documents/review.pptx"

printf 'not a zip\n' > "$fixture/workspace/people/p01-software-engineer/home/documents/brief.docx"
if run --path-revision "$fixture/revision.jsonl" >/dev/null 2>&1; then
  echo 'expected malformed OOXML rejection' >&2
  exit 1
fi

printf '%s\n' 'p01 format audit tests: ok'
