"""Small disposable fixtures and CLI regression tests; no corpus artifacts are written."""
import csv
import datetime as dt
import gzip
import hashlib
import importlib.machinery
import importlib.util
import io
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile

BASE = Path(__file__).resolve().parents[1]
sys.dont_write_bytecode = True

class Fixture(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.name = 'p01-writer'
        self.root = self.base / 'people'
        self.home = self.root / self.name
        self.home.mkdir(parents=True)
        for d in ('personas', 'progress', 'manifests'):
            (self.base / d).mkdir()
        self.cfg('period: 2025-01-01..2026-12-31\nallowed_terms: batch payment, synthetic lease\nphoto_dirs: photos')
        self.args = sum(([f'--{n}', str(self.root if n == 'root' else self.base/n)] for n in ('root','personas','progress','manifests')), [])

    def cfg(self, text):
        (self.base/'personas'/f'{self.name}.md').write_text('---\n'+text+'\n---\n')

    def put(self, rel, content='Minutes: Eleanor approved the revised delivery schedule after the supplier inspection.'):
        p = self.home/rel
        p.parent.mkdir(parents=True, exist_ok=True)
        p.write_bytes(content.encode() if isinstance(content,str) else content)
        os.utime(p, (1767225600,1767225600))
        return p

    def cli(self, tool, *args):
        return subprocess.run([str(BASE/'bin'/tool), *map(str,args), *self.args], capture_output=True, text=True)

    def issues(self, *args):
        report=self.base/'report.tsv'
        result=self.cli('check','p01','--report',report,*args)
        self.assertIn(result.returncode,(0,1),result.stderr)
        self.assertTrue(report.exists(),result.stderr)
        rows=list(csv.reader(report.open(),delimiter='\t'))
        self.assertTrue(all(len(r)==4 for r in rows))
        return result, rows

    def pair(self, kind, good, bad):
        good()
        result,rows=self.issues()
        self.assertFalse(any(r[2]==kind for r in rows),rows)
        bad()
        result,rows=self.issues()
        self.assertEqual(result.returncode,1,result.stdout)
        self.assertTrue(any(r[2]==kind for r in rows),(result.stdout,result.stderr))

class Check(Fixture):
    def test_format(self):
        self.pair('format',lambda:self.put('a.txt'),lambda:self.put('bad.py','print(1)'))
        for rel in ('.hidden.txt','hidden/.a.txt','zero.txt'):
            self.put(rel,b'')
        _,rows=self.issues()
        self.assertTrue(all(any(r[1]==rel and r[2]=='format' for r in rows) for rel in ('.hidden.txt','hidden/.a.txt','zero.txt')))

    def test_opens(self):
        from docx import Document
        from openpyxl import Workbook
        from pptx import Presentation
        from PIL import Image
        from reportlab.pdfgen.canvas import Canvas
        import wave
        makers = {
            'txt':lambda p:p.write_text('A useful note'), 'md':lambda p:p.write_text('# Notes'),
            'log':lambda p:p.write_text('Meeting started'), 'csv':lambda p:p.write_text('a,b\n1,2\n'),
            'tsv':lambda p:p.write_text('a\tb\n1\t2\n'), 'json':lambda p:p.write_text('{"cost":42}'),
            'xml':lambda p:p.write_text('<cost>42</cost>'), 'html':lambda p:p.write_text('<p>Decision approved</p>'),
            'eml':lambda p:p.write_text('From: a@example.test\nTo: b@example.test\nSubject: Review\n\nApproved'),
            'docx':lambda p:Document().save(p), 'xlsx':lambda p:Workbook().save(p),
            'pptx':lambda p:Presentation().save(p),
            'png':lambda p:Image.new('RGB',(50,50),'white').save(p),
            'jpg':lambda p:Image.new('RGB',(50,50),'blue').save(p),
        }
        def pdf(p):
            c=Canvas(str(p),pageCompression=0); c.drawString(30,700,'Meeting approved for next Tuesday'); c.save()
        def wav(p):
            with wave.open(str(p),'wb') as w:
                w.setparams((1,2,8000,0,'NONE','not compressed')); w.writeframes(b'\0\0'*800)
        def zipped(p):
            with zipfile.ZipFile(p,'w') as z:z.writestr('meeting.txt','Decisions')
        makers.update(pdf=pdf,wav=wav,zip=zipped)
        for ext,make in makers.items():
            with self.subTest(ext=ext):
                p=self.home/f'a.{ext}'; make(p)
                _,rows=self.issues()
                self.assertFalse(any(r[1]==p.name and r[2]=='opens' for r in rows),rows)
                p.write_bytes(b'\xffbroken\x00')
                _,rows=self.issues()
                self.assertTrue(any(r[1]==p.name and r[2]=='opens' for r in rows),rows)
                p.unlink()
        self.put('wrong.wav',b'not audio')
        _,rows=self.issues(); self.assertTrue(any(r[2]=='opens' for r in rows))

    def test_path(self):
        self.pair('path',lambda:self.put('meeting-notes.txt'),lambda:self.put('foundation/notes.txt'))
        for rel in ('imagegen-assets/a.txt','a-p01.txt','memo-s123456.txt','memo-12345.txt','primary/a.txt'):
            self.put(rel,rel)
        _,rows=self.issues()
        self.assertEqual(len({r[1] for r in rows if r[2]=='path'}),6)

    def test_trace(self):
        self.pair('trace',lambda:self.put('a.txt','A BATCH PAYMENT under the synthetic lease. Batchwork and scoping are normal. Scope of work.'),lambda:self.put('b.txt','Batch payment then batch 42.'))
        terms=['Kio','kio-eval','personaCorpus','persona','corpus','fixture','content spine','synthetic','fictional','not a real','artifact id','source id','batch','scope X17','p14-finance','/Users/alice','scratch/x','lorem ipsum','placeholder']
        for i,term in enumerate(terms):self.put(f'term-{i}.txt',term)
        _,rows=self.issues()
        for i in range(len(terms)):self.assertTrue(any(r[1]==f'term-{i}.txt' and r[2]=='trace' for r in rows),terms[i])

    def test_trace_in_path_and_email_headers(self):
        eml=lambda extra:'From: a@x.example\nTo: b@x.example\nSubject: Delivery schedule\n'+extra+'\nThe supplier confirmed the revised delivery date for the oak panels.\n'
        self.pair('trace',lambda:self.put('mail/schedule.eml',eml('')),lambda:self.put('mail/tagged.eml',eml('X-Synthetic-Corpus: yes\n')))
        self.put('synthetic-audio/note.txt','The quarterly review covered staffing in the Osaka office and the new travel policy.')
        _,rows=self.issues()
        self.assertTrue(any(r[1]=='synthetic-audio/note.txt' and r[2]=='trace' for r in rows),rows)

    def test_duplicate(self):
        self.put('a.txt',' '.join(f'word{chr(97+i)}' for i in range(20))+' 2025-03-01 cost 42')
        self.put('b.txt','The rain damaged the northern retaining wall and delayed the station opening.')
        _,rows=self.issues(); self.assertFalse(any(r[2]=='duplicate' for r in rows))
        self.put('c.txt',(self.home/'a.txt').read_bytes())
        self.put('d.txt',(self.home/'a.txt').read_text().replace('2025-03-01','2026-08-02').replace('42','97')+' revised')
        _,rows=self.issues()
        self.assertEqual({r[1] for r in rows if r[2]=='duplicate'},{'a.txt','c.txt','d.txt'})

    def test_duplicate_japanese(self):
        body='三月の月次締めでは銀行残高と帳簿残高の差異を確認し、入金のタイミングと支払のタイミングに分けて原因を整理した。差異は{n}万円で、翌月初に解消する見込みである。'
        self.put('ja-a.txt',body.format(n=1240))
        self.put('ja-b.txt','静岡工場の棚卸では仕掛品の移動伝票に入力漏れがあり、管理課長と現物を照合して修正仕訳を起こした。再発防止として週次の点検を始める。')
        _,rows=self.issues(); self.assertFalse(any(r[2]=='duplicate' for r in rows),rows)
        self.put('ja-c.txt',body.format(n=380))
        _,rows=self.issues()
        self.assertEqual({r[1] for r in rows if r[2]=='duplicate'},{'ja-a.txt','ja-c.txt'})

    def test_office_runs_are_contiguous(self):
        from docx import Document
        d=Document();p=d.add_paragraph();p.add_run('syn');p.add_run('thetic')
        d.save(self.home/'runs.docx')
        d=Document();p=d.add_paragraph();p.add_run('ghp_');p.add_run('z'*36)
        d.save(self.home/'key.docx')
        d=Document();p=d.add_paragraph();p.add_run('fixture').add_break();p.add_run('reference')
        d.save(self.home/'break.docx')
        _,rows=self.issues()
        self.assertTrue(any(r[1]=='break.docx' and r[2]=='trace' for r in rows),rows)
        self.assertTrue(any(r[1]=='runs.docx' and r[2]=='trace' for r in rows),rows)
        self.assertTrue(any(r[1]=='key.docx' and r[2]=='secret' for r in rows),rows)

    def test_secret(self):
        self.pair('secret',lambda:self.put('a.txt','Authorization is pending.'),lambda:self.put('b.txt','AKIA'+'A'*16))
        tokens=['ghp_'+'a'*36,'github_pat_'+'a'*30,'xoxb-'+'1'*24,'xapp-'+'1'*24,'-----BEGIN PGP PRIVATE KEY BLOCK-----','hvs.'+'a'*24,'-----BEGIN PRIVATE KEY-----','sk-'+'a'*24,'eyJabc.eyJdef.abcdef','/ASCII85Decode']
        for i,t in enumerate(tokens):self.put(f'key-{i}.txt',t)
        from docx import Document
        d=Document();d.add_paragraph('ghp_'+'x'*36);d.save(self.home/'key.docx')
        _,rows=self.issues()
        self.assertEqual(len({r[1] for r in rows if r[2]=='secret'}),len(tokens)+2)

    def test_mtime(self):
        self.pair('mtime',lambda:self.put('a.txt'),lambda:os.utime(self.home/'a.txt',(0,0)))
        (self.base/'personas'/f'{self.name}.md').unlink()
        r,rows=self.issues();self.assertFalse(any(x[2]=='mtime' for x in rows));self.assertIn('定義なし',r.stderr)

    def make_plan(self,rows):
        d=self.base/'progress'/self.name;d.mkdir(exist_ok=True)
        (d/'b001.md').write_text('| パス | 日付 | 内容 | 関連 |\n|---|---|---|---|\n'+''.join(f'| {p} | {day} | {content} | |\n' for p,day,content in rows))

    def test_plan(self):
        self.put('a.txt')
        self.make_plan([('a.txt','2026-01-01','Meeting decision'),('discarded.txt','2026-01-01','破棄：bad')])
        r,rows=self.issues('--batch','b001');self.assertEqual(r.returncode,0,(r.stdout,r.stderr))
        self.put('extra.txt','Extra notes')
        (self.home/'a.txt').unlink()
        _,rows=self.issues('--batch','b001');self.assertEqual({r[1] for r in rows if r[2]=='plan'},{'a.txt','extra.txt'})

    def test_batch_dates_and_duplicates(self):
        p=self.put('a.txt');self.put('existing.txt',p.read_bytes())
        subprocess.run(['git','init','-q',str(self.base)],check=True)
        subprocess.run(['git','-C',str(self.base),'add','people'],check=True)
        self.make_plan([('a.txt','2026-01-02','Decision')])
        _,rows=self.issues('--batch','b001')
        self.assertEqual({r[2] for r in rows},{'mtime','duplicate'})
        self.assertEqual({r[1] for r in rows},{'a.txt'})
        # The pure Python index reader must also understand path-compressed v4.
        subprocess.run(['git','-C',str(self.base),'update-index','--index-version','4'],check=True)
        _,rows=self.issues('--batch','b001')
        self.assertFalse(any(r[2]=='plan' for r in rows),rows)

    def test_missing_ocr_is_not_opens_or_success(self):
        if shutil.which('tesseract'):
            return
        from PIL import Image
        Image.new('RGB',(80,80),'white').save(self.home/'scan.png')
        r,rows=self.issues()
        self.assertEqual(r.returncode,1)
        self.assertFalse(any(x[2] in ('opens','empty-scan') for x in rows),rows)
        self.assertIn('未実行：empty-scan',r.stderr)
        self.assertIn('passed: 0',r.stdout)
        self.assertIn('pending: 1',r.stdout)

    def test_duplicate_prefix_join_matches_bruteforce(self):
        import random
        loader=importlib.machinery.SourceFileLoader('test_check',str(BASE/'bin/check'))
        spec=importlib.util.spec_from_loader(loader.name,loader)
        module=importlib.util.module_from_spec(spec);loader.exec_module(module)
        rng=random.Random(42)
        sets=[]
        for _ in range(100):
            common=set(rng.sample(range(1000),rng.randrange(5,100)))
            sets.extend([frozenset(common),frozenset(common|set(rng.sample(range(1000),5)))])
        results=[(str(i),{},None,g,[]) for i,g in enumerate(sets)]
        expected={i for i,a in enumerate(sets) for j,b in enumerate(sets) if i!=j and len(a&b)/len(a|b)>=.8}
        module.duplicates(results)
        self.assertEqual({i for i,r in enumerate(results) if 'duplicate' in r[1]},expected)

    def test_empty_scan(self):
        if not shutil.which('tesseract'):
            self.fail('未実行：tesseract なし')
        from PIL import Image,ImageDraw,ImageFont
        im=Image.new('RGB',(1300,180),'white')
        ImageDraw.Draw(im).text((20,40),'Meeting approved after supplier inspection on Tuesday',font=ImageFont.truetype('DejaVuSans.ttf',32),fill='black')
        im.save(self.home/'readable.png')
        _,rows=self.issues();self.assertFalse(any(r[2]=='empty-scan' for r in rows),rows)
        Image.new('RGB',(150,150),'white').save(self.home/'blank.png')
        _,rows=self.issues();self.assertTrue(any(r[1]=='blank.png' and r[2]=='empty-scan' for r in rows),rows)
        (self.home/'blank.png').unlink()
        (self.home/'photos').mkdir();Image.new('RGB',(160,150),'white').save(self.home/'photos/blank.png')
        _,rows=self.issues();self.assertFalse(any(r[2]=='empty-scan' for r in rows),rows)
        Image.new('RGB',(150,150),'white').save(self.home/'blank.pdf','PDF')
        _,rows=self.issues();self.assertTrue(any(r[1]=='blank.pdf' and r[2]=='empty-scan' for r in rows),rows)

    def test_self_test(self):
        r=self.cli('check','--self-test')
        missing=[n for n in ('pdftotext','pdfinfo','pdftoppm','tesseract','soffice','ffprobe') if not shutil.which(n)]
        for n in missing:self.assertIn(n,r.stdout+r.stderr)
        self.assertEqual(r.returncode,0,'未実行／依存不足：'+', '.join(missing)+'\n'+r.stdout)

class Manifest(Fixture):
    def test_inventory_and_verify(self):
        self.put('b.txt');self.put('.hidden','hidden');self.put('a.py','code')
        r=self.cli('manifest','p01');self.assertEqual(r.returncode,0,r.stderr)
        path=self.base/'manifests'/f'{self.name}.tsv'
        rows=list(csv.reader(path.open(),delimiter='\t'));self.assertEqual([r[0] for r in rows],['.hidden','a.py','b.txt'])
        self.assertEqual(rows[2][3],'2026-01-01T00:00:00Z')
        self.assertEqual(self.cli('manifest','p01','--verify').returncode,0)
        for change in ('content','mtime','extra','missing','size'):
            original=(self.home/'b.txt').read_bytes()
            if change=='content':self.put('b.txt','Changed')
            elif change=='mtime':os.utime(self.home/'b.txt',(1767225700,1767225700))
            elif change=='extra':self.put('extra.txt','extra')
            elif change=='missing':(self.home/'b.txt').unlink()
            else:
                rows[2][2]='999';path.write_text('\n'.join('\t'.join(r) for r in rows)+'\n')
            self.assertEqual(self.cli('manifest','p01','--verify').returncode,1,change)
            self.put('b.txt',original)
            (self.home/'extra.txt').unlink(missing_ok=True)

class Pack(Fixture):
    def test_reproducible_and_gates(self):
        self.put('notes/a.txt')
        self.assertEqual(self.cli('manifest','p01').returncode,0)
        out=self.base/'out';out2=self.base/'out2'
        for dest in (out,out2):
            r=self.cli('pack',dest);self.assertEqual(r.returncode,0,(r.stdout,r.stderr))
        archive=out/f'{self.name}.tar.gz'
        self.assertEqual(archive.read_bytes(),(out2/archive.name).read_bytes())
        self.assertEqual(archive.read_bytes()[4:8],b'\0'*4)
        with tarfile.open(archive) as t:
            self.assertEqual(t.getnames(),[f'{self.name}/notes/a.txt'])
            info=t.getmembers()[0];self.assertEqual((info.uid,info.gid,info.mtime),(0,0,1767225600))
        self.assertIn(hashlib.sha256(archive.read_bytes()).hexdigest(),(out/'SHA256SUMS').read_text())
        self.assertIn('synthetic',(out/'README.md').read_text())
        self.put('bad.txt','placeholder')
        r=self.cli('pack',self.base/'bad');self.assertEqual(r.returncode,1);self.assertFalse((self.base/'bad').exists())
        (self.home/'bad.txt').unlink();self.put('notes/a.txt','A changed decision')
        r=self.cli('pack',self.base/'mismatch');self.assertEqual(r.returncode,1)
        self.assertFalse((self.base/'mismatch').exists())

    def test_output_location_and_manifest_gates(self):
        self.put('a.txt')
        # No manifest must prevent packaging, including an otherwise clean persona.
        r=self.cli('pack',self.base/'missing');self.assertEqual(r.returncode,1)
        self.assertFalse((self.base/'missing').exists())
        self.cli('manifest','p01')
        r=self.cli('pack',self.home/'nested');self.assertEqual(r.returncode,1)
        self.assertFalse((self.home/'nested').exists())
        r=self.cli('pack',BASE/'bad-distribution');self.assertEqual(r.returncode,1)
        self.assertFalse((BASE/'bad-distribution').exists())

    def test_subset_still_checks_all(self):
        self.put('a.txt');self.cli('manifest','p01')
        other=self.root/'p02-editor';other.mkdir();(other/'a.txt').write_text('placeholder')
        r=self.cli('pack',self.base/'out','--persona','p01');self.assertEqual(r.returncode,1)
        (other/'a.txt').write_text('The editor chose a new title after the review.')
        self.cli('manifest','p02')
        r=self.cli('pack',self.base/'out','--persona','p01');self.assertEqual(r.returncode,0,(r.stdout,r.stderr))
        self.assertEqual(len(list((self.base/'out').glob('*.tar.gz'))),1)

if __name__=='__main__':
    suite=unittest.defaultTestLoader.loadTestsFromTestCase({'check':Check,'manifest':Manifest,'pack':Pack}[sys.argv[1]])
    sys.exit(not unittest.TextTestRunner(verbosity=2).run(suite).wasSuccessful())
