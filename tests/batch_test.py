"""Batch integration tests. Everything, including Git commits, stays in /tmp."""
import fcntl
import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SOURCE = Path(__file__).resolve().parents[1]
FAKE = r'''#!/usr/bin/env python3
import json, os, pathlib, sys, fcntl
root=pathlib.Path.cwd()
prompt=sys.stdin.read()
assert sys.argv[-1]=='-'
assert all('BATCH_CONTEXT' not in arg for arg in sys.argv)
meta=json.loads(prompt.split('BATCH_CONTEXT\n',1)[1].split('\nEND_BATCH_CONTEXT',1)[0])
s=meta['stage']; scratch=root/meta['scratch']; scratch.mkdir(parents=True,exist_ok=True)
with (scratch/'argv.jsonl').open('a') as f: f.write(json.dumps(sys.argv)+'\n')
with (scratch/'prompts.jsonl').open('a') as f: f.write(json.dumps(prompt)+'\n')
counter=scratch/'calls.json'
with (scratch/'calls.lock').open('a') as lock:
 fcntl.flock(lock,fcntl.LOCK_EX)
 calls=json.loads(counter.read_text()) if counter.exists() else {}
 calls[s]=calls.get(s,0)+1; counter.write_text(json.dumps(calls))
mode=os.environ.get('FAKE_MODE','ok')
with (scratch/'contexts.jsonl').open('a') as f: f.write(json.dumps(meta)+'\n')
if mode=='fail' and s=='build':
 print('rate limit: retry tomorrow'); sys.exit(3)
if s=='templates':
 p=root/'templates'/meta['persona']/'README.md'; p.parent.mkdir(parents=True,exist_ok=True); p.write_text('型の一覧\n')
if s=='plan':
 p=root/meta['plan']; p.parent.mkdir(parents=True,exist_ok=True)
 names=['Documents/記録.txt','Documents/連絡.txt']
 if mode in ('dependency','parent-fail'): names.reverse()
 if mode=='badplan' and calls[s]==1: names[0]='Documents/不正.py'
 if mode=='badplan-always': names[0]='../外.txt'
 p.write_text('| パス | 日付 | 作り方 | 内容 | 関連 |\n|---|---|---|---|---|\n'+''.join('| '+n+' | 2026-03-06 | text | 案件の判断と相手への連絡。 | '+('Documents/記録.txt' if mode in ('dependency','parent-fail') and '連絡' in n else '')+' |\n' for n in names))
if s=='draft':
 import random, string
 for i,row in enumerate(meta['rows']):
  p=root/'sources'/meta['persona']/(row['パス']+'.md'); p.parent.mkdir(parents=True,exist_ok=True)
  rng=random.Random(row['パス'])
  body=('月次の資金繰りについて山田が承認。前月比の差額1240円は入金遅延で説明がついた。' if '記録' in row['パス'] else '研修担当の木下へ参加日程の変更を通知した。火曜日から水曜日へ移すことで回答を得た。') if len(meta['rows'])<3 else ' '.join(''.join(rng.choices(string.ascii_lowercase,k=9)) for _ in range(40))
  if i==0 and mode=='machine-once' and meta['round']==0: body='placeholder'
  if mode in ('dependency','parent-fail') and '連絡' in row['パス']: body+='\n\n添付: Documents/記録.txt'
  p.write_text('---\npath: '+row['パス']+'\nkind: '+row['作り方']+'\nauthor: 山田 花子\n'+('device: Office Scanner 42\n' if mode=='device' else '')+'derived_from:\n---\n'+body+'\n')
if s=='build':
 for row in meta['rows']:
  if '記録' in row['パス'] and (mode=='skip-build-always' or mode=='skip-build-once' and meta['round']==0): continue
  source=root/'sources'/meta['persona']/(row['パス']+'.md')
  p=root/'people'/meta['persona']/row['パス']; p.parent.mkdir(parents=True,exist_ok=True); p.write_text(source.read_text().split('\n---\n',1)[1])
  if mode=='number-once' and meta['round']==0 and '記録' in row['パス']: p.write_text(p.read_text().replace('1240',''))
  if mode=='mutate-build': source.write_text(source.read_text()+'改変')
if s=='templates-review':
 output=root/meta['review_output']; output.parent.mkdir(parents=True,exist_ok=True)
 reject=mode=='template-always' or mode=='template-once' and calls[s]==1
 output.write_text('README.md\t'+('reject\tother\tフッタ欠落' if reject else 'pass\t\t説明と実物を確認')+'\n')
if s in ('draft-review','build-review'):
 output=root/meta['review_output']; output.parent.mkdir(parents=True,exist_ok=True)
 lines=[]
 for i,row in enumerate(meta['rows']):
  draft=s=='draft-review'
  reject=i==0 and ((mode in ('draft-reject-always','draft-final-fail') or mode=='draft-parent-fail' and '記録' in row['パス'] or mode=='draft-reject-once' and meta['round']==0) if draft else (mode in ('reject-always','build-final-fail') or mode=='parent-fail' and '記録' in row['パス'] or mode=='reject-once' and meta['round']==0))
  if mode=='mutate' and not draft: (root/'people'/meta['persona']/row['パス']).write_text('改変')
  if mode=='mutate-build-review-source' and not draft: (root/'sources'/meta['persona']/(row['パス']+'.md')).write_text('改変')
  if mode=='mutate-draft' and draft: (root/'sources'/meta['persona']/(row['パス']+'.md')).write_text('改変')
  if mode=='missing-review' and i==0 and not draft: continue
  lines.append(row['パス']+'\t'+('reject' if reject else 'pass')+'\t'+('数値不整合' if reject else '本文と計画を確認'))
 output.write_text('\n'.join(lines)+'\n')
pathlib.Path(sys.argv[sys.argv.index('-o')+1]).write_text('Documents/記録.txt: 写真生成ができなかった' if s=='build' and mode.startswith('skip-build') else '全体所見：途中のメモも含め自然。')
'''
CHECK = r'''#!/usr/bin/env python3
import os,pathlib,sys
args=sys.argv
if '--report' in args:
 p=pathlib.Path(args[args.index('--report')+1])
 mode=os.environ.get('FAKE_MODE','')
 missing=mode.startswith('skip-build') and not pathlib.Path('people/p01-test/Documents/記録.txt').is_file()
 if missing:
  p.write_text('p01-test\tDocuments/記録.txt\tplan\tplanned file missing\n'); sys.exit(1)
 fail=mode=='auto-reject' and p.name=='check-r0.tsv'
 p.write_text('p01-test\tDocuments/記録.txt\topens\t壊れた文書\n' if fail else '')
 if fail: sys.exit(1)
print('passed')
'''
JOB = r'''#!/usr/bin/env python3
import json,pathlib,sys
p=pathlib.Path('job-args.json')
if sys.argv[1]=='status':
 print('REPO\tJOB\tSTATE\tDETAIL')
 print('personaCorpus\tp01-b001\tdone\tfinished')
 print('personaCorpus\tp01-b001-r2\tfailed\tfailed')
else: p.write_text(json.dumps(sys.argv[1:]))
'''
MANIFEST = r'''#!/usr/bin/env python3
import pathlib,sys
p=pathlib.Path('manifests')/(sys.argv[1]+'.tsv'); p.parent.mkdir(exist_ok=True)
p.write_text(''.join(str(x.relative_to(pathlib.Path('people')/sys.argv[1]))+'\thash\t1\t2026-03-06T12:00:00Z\n' for x in sorted((pathlib.Path('people')/sys.argv[1]).rglob('*')) if x.is_file()))
'''

class BatchTest(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='batch-test-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name)
        for name in ('bin','prompts','personas','progress/p01-test','people/p01-test'):
            (self.root/name).mkdir(parents=True,exist_ok=True)
        shutil.copy2(SOURCE/'bin/batch',self.root/'bin/batch')
        if (SOURCE/'bin/stamp').exists(): shutil.copy2(SOURCE/'bin/stamp',self.root/'bin/stamp')
        shutil.copytree(SOURCE/'prompts',self.root/'prompts',dirs_exist_ok=True)
        for name,body in [('fake-codex',FAKE),('bin/check',CHECK),('bin/manifest',MANIFEST)]:
            p=self.root/name; p.write_text(body); p.chmod(0o755)
        (self.root/'personas/p01-test.md').write_text('---\npersona: p01-test\nperiod: 2026-01-01..2026-12-31\ntimezone: Asia/Tokyo\n---\n## 形式の目安\n- txt：100%\n')
        (self.root/'progress/p01-test/b001.request.md').write_text('# 依頼\n- 範囲：月次報告\n- 件数の目安：2\n')
        self.env=dict(os.environ,CODEX_BIN=str(self.root/'fake-codex'),PYTHONDONTWRITEBYTECODE='1')
        self.git('init','-q'); self.git('config','user.email','test@example.invalid'); self.git('config','user.name','Batch Test'); self.git('config','core.quotePath','false')
        self.git('add','.'); self.git('commit','-qm','Fixture')
    def git(self,*args):
        r=subprocess.run(['git',*args],cwd=self.root,text=True,capture_output=True)
        self.assertEqual(r.returncode,0,r.stderr); return r.stdout
    def cli(self,*args,mode='ok',ok=True):
        r=subprocess.run(['python3',str(self.root/'bin/batch'),*args],cwd=self.root,env=dict(self.env,FAKE_MODE=mode),text=True,capture_output=True)
        if ok: self.assertEqual(r.returncode,0,r.stdout+r.stderr)
        else: self.assertNotEqual(r.returncode,0,r.stdout+r.stderr)
        return r
    @property
    def scratch(self): return self.root/'scratch/p01-test/b001'
    def state(self): return json.loads((self.scratch/'state.json').read_text())
    def run_batch(self,**kw): return self.cli('run','p01','b001',**kw)
    def prompt_batch(self):
        mod=self.load()
        return mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())

    def test_redraft_only_draft_discards_and_failed_batch(self):
        self.run_batch()
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        b.state['draft_failures']={rows[0]['パス']:'原稿不合格'}
        b.discard_rows(b.state['draft_failures'])
        kept=b.source(rows[1]['パス']).read_bytes()
        artifact=(b.folder/rows[1]['パス']).read_bytes()
        b.state['status']='failed'; b.store()
        self.cli('run','p01','b001','--redraft')
        self.assertEqual(self.state()['status'],'done')
        self.assertEqual(self.state()['redraft_paths'],[rows[0]['パス']])
        self.assertEqual(b.source(rows[1]['パス']).read_bytes(),kept)
        self.assertEqual((b.folder/rows[1]['パス']).read_bytes(),artifact)
        self.assertNotIn('破棄：',b.plan.read_text())

    def test_redraft_preserves_coordinator_drop_and_unknown(self):
        self.run_batch(mode='draft-reject-always')
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        mod.mark_discard(b.plan,{rows[0]['パス']:'統括の抜き取りで不合格',rows[1]['パス']:'理由不明'})
        before=b.plan.read_bytes()
        result=self.cli('run','p01','b001','--redraft')
        self.assertEqual(b.plan.read_bytes(),before)
        self.assertIn(rows[1]['パス'],result.stdout)
        self.assertNotIn('redraft_paths',self.state())

    def test_draft_parent_failure_still_writes_child(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        rows[1]['先に要るもの']=rows[0]['パス']
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        b.state['completed']['plan']=rows; b.store()
        self.cli('run','p01','b001',mode='draft-parent-fail')
        state=self.state()
        self.assertEqual(set(state['draft_failures']),{rows[0]['パス']})
        contexts=[json.loads(x) for x in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        child=next(c for c in contexts if c['stage']=='draft' and c['rows'][0]['パス']==rows[1]['パス'])
        self.assertEqual(child['missing_prerequisites'],{rows[1]['パス']:[rows[0]['パス']]})
        self.assertIn(rows[1]['パス'],(b.result).read_text().split('## 先に要る原稿が欠けたまま書いた行')[1])

    def test_redraft_accepted_batch_and_findings_only(self):
        self.run_batch(mode='draft-parent-fail')
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        self.cli('accept','p01','b001')
        # Acceptance removes scratch/state; reconstruct from the persisted ledger.
        self.assertFalse((self.scratch/'state.json').exists())
        self.cli('run','p01','b001','--redraft')
        self.assertEqual(self.state()['redraft_paths'],[rows[0]['パス']])
        self.assertEqual(self.state()['status'],'done')
        self.cli('accept','p01','b001')

    def test_redraft_resumes_after_build_failure(self):
        self.run_batch(mode='draft-parent-fail')
        self.cli('run','p01','b001','--redraft',mode='fail',ok=False)
        self.assertTrue(self.state()['redraft_active'])
        self.cli('run','p01','b001','--redraft')
        self.assertEqual(self.state()['status'],'done')
        self.assertFalse(self.state()['redraft_active'])
        self.assertEqual(self.state()['redraft_generation'],1)

    def test_real_check_empty_persona_and_final_check(self):
        self.run_batch(mode='draft-reject-always')
        shutil.copy2(SOURCE/'bin/check',self.root/'bin/check')
        mod=self.load(); b=self.prompt_batch()
        # Explicitly discard the other row too, leaving no artifacts to inspect.
        mod.mark_discard(b.plan,{r['パス']:'原稿不合格' for r in mod.plan_rows(b.plan)})
        shutil.rmtree(b.folder)
        b.final_check()
        b.folder.mkdir(parents=True)
        b.final_check()
        result=subprocess.run(['python3',str(self.root/'bin/check'),'p99', '--batch','b001'],cwd=self.root,text=True,capture_output=True)
        self.assertNotEqual(result.returncode,0)
        self.assertIn('persona not found',result.stdout+result.stderr)
        b.folder.rmdir()
        b.folder.symlink_to(self.root/'progress',target_is_directory=True)
        with self.assertRaisesRegex(ValueError,'symlink'):
            b.final_check()

    def test_redraft_does_not_touch_other_build_pending_rows(self):
        self.run_batch(mode='draft-parent-fail')
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        rel=rows[1]['パス']
        mod.mark_discard(b.plan,{rel:'既存の成形待ち'},'成形待ち')
        b.state['failures'][rel]='既存の成形待ち'; b.store()
        artifact=(b.folder/rel).read_bytes()
        source=b.source(rel).read_bytes()
        self.cli('run','p01','b001','--redraft')
        self.assertEqual((b.folder/rel).read_bytes(),artifact)
        self.assertEqual(b.source(rel).read_bytes(),source)
        self.assertIn('成形待ち：既存の成形待ち。',b.plan.read_text())

    def test_missing_derived_original_waits_for_build(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=self.prompt_batch(); rows=mod.plan_rows(b.plan)
        rows[1]['先に要るもの']=rows[0]['パス']
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        mod.mark_discard(b.plan,{rows[0]['パス']:'原稿不合格'})
        # Set metadata directly after fake production, so validation sees a planned,
        # absent original while the child still proceeds through the draft review.
        original=b.codex
        def codex(stage,targets=(),*args,**kwargs):
            output=original(stage,targets,*args,**kwargs)
            if stage=='draft':
                path=b.source(rows[1]['パス'])
                path.write_text(path.read_text().replace('derived_from:', 'derived_from: '+rows[0]['パス']))
            return output
        with patch.dict(os.environ,self.env),patch.object(b,'codex',codex):
            self.assertEqual(b.draft_phase([rows[1]],rows),{})
            failures=b.build_phase([rows[1]],rows)
        self.assertIn('が未完成',failures[rows[1]['パス']])

    def test_zero_build_full_pipeline_with_real_check(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=self.prompt_batch()
        rows=mod.plan_rows(b.plan)[:1]
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        b.state['completed']['plan']=rows; b.store()
        shutil.copy2(SOURCE/'bin/check',self.root/'bin/check')
        shutil.rmtree(b.folder)
        self.cli('run','p01','b001',mode='draft-reject-always')
        self.assertEqual(self.state()['status'],'done')
        self.assertEqual(self.state()['count'],0)
        self.assertFalse(b.folder.exists())
        self.assertIn('passed: 0, failed: 0',(self.scratch/'logs/final-check.log').read_text())

    def test_prompt_uses_stdin_and_file_references(self):
        b=self.prompt_batch()
        documents=[b.definition,b.templates,b.plan,b.plan.parent/'facts.md']
        for i,p in enumerate(documents):
            p.parent.mkdir(parents=True,exist_ok=True)
            p.write_text(f'PRIVATE_DOCUMENT_BODY_{i}')
        other=b.plan.with_name('b002.request.md'); other.write_text('OTHER_REQUEST_BODY')
        other_plan=other.with_name('b002.md'); other_plan.write_text('OTHER_PLAN_BODY')
        state=self.root/'scratch/p01-test/b002/state.json'
        state.parent.mkdir(parents=True); state.write_text('{}')
        with patch.dict(os.environ,self.env): b._codex('plan')
        prompt=json.loads((self.scratch/'prompts.jsonl').read_text().splitlines()[0])
        args=json.loads((self.scratch/'argv.jsonl').read_text().splitlines()[0])
        self.assertEqual(args[-1],'-')
        self.assertFalse(any('BATCH_CONTEXT' in arg for arg in args))
        for i,p in enumerate(documents):
            self.assertNotIn(f'PRIVATE_DOCUMENT_BODY_{i}',prompt)
            self.assertIn(str(p.relative_to(self.root)),prompt)
        for p,body in [(other,'OTHER_REQUEST_BODY'),(other_plan,'OTHER_PLAN_BODY')]:
            self.assertIn(str(p.relative_to(self.root)),prompt)
            self.assertNotIn(body,prompt)
        self.assertIn(str(b.request.relative_to(self.root)),prompt)
        self.assertIn('範囲：月次報告',prompt)

    def test_large_facts_do_not_enlarge_prompt(self):
        b=self.prompt_batch()
        (b.plan.parent/'facts.md').write_text('F'*200_000)
        with patch.dict(os.environ,self.env): b._codex('templates')
        prompt=json.loads((self.scratch/'prompts.jsonl').read_text().splitlines()[0])
        self.assertLess(len(prompt.encode('utf-8')),60*1024)
        self.assertNotIn('F'*100,prompt)

    def test_oversized_prompt_fails_before_codex(self):
        b=self.prompt_batch()
        b.request.write_text('依'*21000)
        with patch.dict(os.environ,self.env):
            with self.assertRaisesRegex(ValueError,r'60 KiB.*依頼本文=.*BATCH_CONTEXT='):
                b._codex('templates')
        self.assertFalse((self.scratch/'calls.json').exists())

    def test_prompt_size_checks_template_and_context(self):
        b=self.prompt_batch()
        template=self.root/'prompts/templates.md'
        template.write_text('T'*61441)
        with patch.dict(os.environ,self.env):
            with self.assertRaisesRegex(ValueError,'ひな形=61441 bytes'):
                b._codex('templates')
        template.write_text('short')
        with patch.dict(os.environ,self.env):
            with self.assertRaisesRegex(ValueError,'BATCH_CONTEXT='):
                b._codex('templates',reasons={'reason':'大'*21000})
        self.assertFalse((self.scratch/'calls.json').exists())

    def test_review_never_receives_producer_output(self):
        b=self.prompt_batch(); b.plan.write_text('PLAN_BODY')
        self.scratch.mkdir(parents=True,exist_ok=True)
        (self.scratch/'producer.log').write_text('PRODUCER_SECRET')
        with patch.dict(os.environ,self.env):
            b._codex('draft-review',reasons={'producer':'PRODUCER_SECRET'},
                     review_output=self.scratch/'review.tsv')
        prompt=json.loads((self.scratch/'prompts.jsonl').read_text().splitlines()[0])
        self.assertNotIn('PRODUCER_SECRET',prompt)
        self.assertNotIn('PLAN_BODY',prompt)
        self.assertIn(str(b.plan.relative_to(self.root)),prompt)

    def test_missing_explanation_selects_the_row_section(self):
        mod=self.load()
        rows=[{'パス':'Pictures/未作成.jpg'},{'パス':'Documents/完了.txt'}]
        output='## 作れなかった行\n- Pictures/未作成.jpg\n  画像生成が利用できない。\n- Documents/完了.txt: 完了\n別の所見'
        reason=mod.missing_explanation(output,'Pictures/未作成.jpg',rows)
        self.assertIn('画像生成が利用できない',reason)
        self.assertNotIn('完了.txt',reason)
        self.assertNotIn('別の所見',reason)
        self.assertEqual(mod.missing_explanation('全体が利用上限','Pictures/未作成.jpg',rows),'全体が利用上限')

    def test_device_is_forwarded_to_stamp(self):
        stamp=self.root/'bin/stamp'
        stamp.write_text('#!/usr/bin/env python3\nimport json,sys\nwith open("stamp-args.jsonl","a") as f: f.write(json.dumps(sys.argv[1:])+"\\n")\n')
        stamp.chmod(0o755)
        self.run_batch(mode='device')
        calls=[json.loads(line) for line in (self.root/'stamp-args.jsonl').read_text().splitlines()]
        self.assertEqual(len(calls),2)
        self.assertTrue(all(args[args.index('--device')+1]=='Office Scanner 42' for args in calls))

    def test_draft_retry_receives_previous_rejection(self):
        self.run_batch(mode='draft-reject-once')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        c=next(c for c in contexts if c['stage']=='draft-review' and c['round']==1)
        self.assertEqual(c['previous_verdicts']['Documents/記録.txt'][:2],['reject','数値不整合'])
        self.assertEqual(c['reasons']['Documents/記録.txt'],'数値不整合')

    def test_re_review_receives_previous_pass(self):
        self.run_batch()
        self.cli('run','p01','b001','--review-again')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        c=[c for c in contexts if c['stage']=='build-review'][-1]
        self.assertEqual(len(c['previous_verdicts']),2)
        self.assertTrue(all(v[0]=='pass' for v in c['previous_verdicts'].values()))
        self.assertEqual(c['reasons'],{})

    def test_missing_only_final_reason_is_recorded(self):
        self.run_batch(mode='skip-build-once')
        self.assertNotIn('planned file missing',(self.root/'progress/findings.tsv').read_text())
        self.assertEqual(self.state()['pending'],0)

    def test_missing_rebuild_restores_original_plan_content(self):
        self.run_batch(mode='skip-build-always')
        self.cli('accept','p01','b001')
        self.cli('run','p01','b001','--rebuild')
        rows=self.load().plan_rows(self.root/'progress/p01-test/b001.md')
        self.assertTrue(all(row['内容']=='案件の判断と相手への連絡。' for row in rows),rows)
        self.assertEqual(self.state()['pending'],0)

    def test_missing_final_includes_producer_output(self):
        self.run_batch(mode='skip-build-always')
        events=[e for e in self.state()['events'] if e[5]=='plan']
        self.assertEqual(len(events),1)
        self.assertIn('写真生成ができなかった',events[0][-1])
        self.assertIn('写真生成ができなかった',(self.root/'progress/p01-test/b001.md').read_text())
        self.assertIn('写真生成ができなかった',(self.root/'progress/p01-test/b001.result.md').read_text())

    def test_review_previous_verdicts_and_notes_once(self):
        self.run_batch(mode='reject-once')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        c=next(c for c in contexts if c['stage']=='build-review' and c['round']==1)
        self.assertEqual(c['previous_verdicts']['Documents/記録.txt'][0],'reject')
        self.assertEqual(c['reasons']['Documents/記録.txt'],'数値不整合')
        report=(self.root/'progress/p01-test/b001.result.md').read_text()
        self.assertEqual(report.count('全体所見：途中のメモも含め自然。'),3)

    def test_draft_related_versions_not_self_duplicates(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        rows=b.state['completed']['plan']; a,z=rows
        for row,n in [(a,'100'),(z,'200')]:
            p=b.source(row['パス']); p.parent.mkdir(parents=True,exist_ok=True)
            p.write_text('---\npath: '+row['パス']+'\nkind: text\nauthor: 花子\nderived_from:\n---\n入金予定の確認結果を記録。銀行照会と取引先回答を照合して入金額'+n+'円を翌営業日に計上する。')
        self.assertEqual(set(b.draft_check(rows)),{a['パス'],z['パス']})
        for rel, reason in b.draft_check(rows).items():
            self.assertNotEqual(reason.rsplit(': ',1)[-1],rel)
        plan=b.plan.read_text().replace('| '+z['パス']+' | 2026-03-06 | text | 案件の判断と相手への連絡。 |  |', '| '+z['パス']+' | 2026-03-06 | text | 案件の判断と相手への連絡。 | '+a['パス']+' |')
        b.plan.write_text(plan)
        self.assertEqual(b.draft_check(rows),{})
        p=b.source(z['パス']); p.write_text(p.read_text().replace('200','100'))
        self.assertEqual(set(b.draft_check(rows)),{a['パス'],z['パス']})
        for rel, reason in b.draft_check(rows).items():
            self.assertNotEqual(reason.rsplit(': ',1)[-1],rel)

    def test_dependency_layers_and_blocked_child(self):
        self.run_batch(mode='dependency')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        builds=[c for c in contexts if c['stage']=='build']
        self.assertEqual([[r['パス'] for r in c['rows']] for c in builds],
                         [['Documents/記録.txt'],['Documents/連絡.txt']])
        first_review=next(i for i,c in enumerate(contexts) if c['stage']=='build-review')
        child=next(i for i,c in enumerate(contexts) if c['stage']=='build' and '連絡' in c['rows'][0]['パス'])
        self.assertLess(first_review,child)

    def test_failed_parent_never_builds_child(self):
        self.run_batch(mode='parent-fail')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        self.assertTrue(all('連絡' not in r['パス'] for c in contexts if c['stage']=='build' for r in c['rows']))
        self.assertNotIn('\tdependency\t', (self.root/'progress/findings.tsv').read_text())
        self.assertEqual(self.state()['pending'],2)
        self.assertIn('元のファイル Documents/記録.txt が未完成。',(self.root/'progress/p01-test/b001.md').read_text())
        self.assertEqual(len(list((self.root/'sources/p01-test').rglob('*.md'))),2)

    def test_derived_duplicates_and_attachment_dependencies(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        rows=b.state['completed']['plan']
        base=b.source('Documents/元.txt'); base.parent.mkdir(parents=True,exist_ok=True)
        base.write_text('---\npath: Documents/元.txt\nkind: text\nauthor: 花子\nderived_from:\n---\n元となる文書の中身')
        for row in rows:
            b.source(row['パス']).write_text('---\npath: '+row['パス']+'\nkind: text\nauthor: 花子\nderived_from: Documents/元.txt\n---\n'+('部長の承認印を右下へ追加する。' if '記録' in row['パス'] else '研修担当の確認署名を末尾へ追加する。'))
        self.assertEqual(b.draft_check(rows),{})
        originals={row['パス']:b.source(row['パス']).read_text() for row in rows}
        for row in rows:
            b.source(row['パス']).write_text('---\npath: '+row['パス']+'\nkind: text\nauthor: 花子\nderived_from: Documents/元.txt\n---\n違いなし')
        self.assertEqual(b.draft_check(rows),{})
        for path,text in originals.items(): b.source(path).write_text(text)
        p=b.source(rows[0]['パス']); p.write_text(p.read_text()+'\n添付:\n- Documents/欠けた資料.pdf\n')
        self.assertIn('Documents/欠けた資料.pdf', b.dependencies(rows)[rows[0]['パス']])
        p.write_text(p.read_text().replace('Documents/欠けた資料.pdf','欠けた資料.pdf'))
        self.assertIn('Documents/欠けた資料.pdf', b.dependencies(rows)[rows[0]['パス']])

    def test_accepted_pending_rebuild(self):
        self.run_batch(mode='reject-always')
        self.cli('accept','p01','b001')
        self.assertTrue(self.scratch.exists())
        accepted=self.state()
        self.cli('run','p01','b001')
        self.assertEqual(self.state(),accepted)
        self.cli('run','p01','b001','--rebuild')
        self.cli('accept','p01','b001')
        self.assertTrue((self.root/'people/p01-test/Documents/記録.txt').exists())
        self.assertFalse(self.scratch.exists())

    def test_accepted_failed_rebuild_can_start_a_fresh_attempt(self):
        self.run_batch(mode='reject-always')
        self.cli('accept','p01','b001')
        self.cli('run','p01','b001','--rebuild',mode='reject-always')
        self.cli('accept','p01','b001')
        self.cli('run','p01','b001','--rebuild')
        self.assertEqual(self.state()['pending'],0)
        self.assertTrue((self.root/'people/p01-test/Documents/記録.txt').is_file())

    def test_rebuild_final_failure_resumes(self):
        self.run_batch(mode='reject-always')
        p=self.root/'bin/check'
        p.write_text(p.read_text().replace("print('passed')", "if '--report' not in args and os.environ.get('FAKE_MODE')=='final-fail': sys.exit(1)\nprint('passed')"))
        self.cli('run','p01','b001','--rebuild',mode='final-fail',ok=False)
        self.cli('run','p01','b001','--rebuild')
        self.assertEqual(self.state()['status'],'done')
        self.assertNotIn('成形待ち：',(self.root/'progress/p01-test/b001.md').read_text())

    def test_stamp_failure_is_per_file(self):
        p=self.root/'bin/stamp'
        p.write_text('#!/usr/bin/env python3\nimport sys\nsys.exit(1)\n'); p.chmod(0o755)
        self.run_batch()
        self.assertEqual(self.state()['status'],'done')
        self.assertEqual(self.state()['pending'],2)
        self.assertEqual(len(list((self.root/'sources/p01-test').rglob('*.md'))),2)
        self.assertIn('metadata',(self.root/'progress/findings.tsv').read_text())

    def test_four_column_review_and_improvement_summary(self):
        p=self.root/'fake-codex'
        p.write_text(p.read_text().replace("+'\\t'+('数値不整合'", "+'\\t'+('D5' if draft else 'V1')+'\\t'+('数値不整合'"))
        self.run_batch(mode='draft-reject-once')
        import csv
        records=list(csv.DictReader((self.root/'progress/findings.tsv').read_text().splitlines(),delimiter='\t'))
        self.assertEqual(records[0]['criterion'],'D5')
        review=list(csv.reader((self.scratch/'draft-review-r0.tsv').read_text().splitlines(),delimiter='\t'))
        self.assertEqual(len(review[0]),4)
        self.assertEqual(review[0][2],'D5')
        (self.root/'progress/improvements.md').write_text('## 2020-01-01\n- D5: 合計検査\n')
        summary=self.cli('findings').stdout
        self.assertIn('2020-01-01\tD5\t0\t1',summary)
        self.assertNotIn('draft-review',self.cli('findings','--since','2999-01-01').stdout)

    def test_failed_accept_keeps_done_ledger(self):
        self.run_batch()
        p=self.root/'bin/check'; p.write_text('#!/usr/bin/env python3\nimport sys\nsys.exit(1)\n'); p.chmod(0o755)
        self.cli('accept','p01','b001',ok=False)
        self.assertNotIn('accepted',(self.root/'progress/batches.tsv').read_text())

    def test_template_retry_and_fix(self):
        self.run_batch(mode='template-once')
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['templates'],2)
        self.assertEqual(calls['templates-review'],2)
        self.assertIn('templates-review',(self.root/'progress/findings.tsv').read_text())
        self.cli('templates','p01','--fix','フッタ欠落')
        self.assertTrue((self.root/'templates/p01-test/README.md').exists())

    def test_template_review_stops(self):
        self.run_batch(mode='template-always',ok=False)
        self.assertEqual(json.loads((self.scratch/'calls.json').read_text())['templates'],3)

    def test_pending_rebuild(self):
        self.run_batch(mode='reject-always')
        source=self.root/'sources/p01-test/Documents/記録.txt.md'
        self.assertTrue(source.exists())
        before=source.read_bytes()
        self.cli('run','p01','b001','--rebuild')
        self.assertEqual(source.read_bytes(),before)
        self.assertNotIn('成形待ち：',(self.root/'progress/p01-test/b001.md').read_text())
        self.assertTrue((self.root/'people/p01-test/Documents/記録.txt').exists())

    def test_findings_idempotence_and_summary(self):
        self.run_batch(mode='draft-reject-once')
        path=self.root/'progress/findings.tsv'
        before=path.read_bytes()
        self.run_batch()
        self.assertEqual(before,path.read_bytes())
        self.assertIn('other',path.read_text())
        summary=self.cli('findings','--since','2020-01-01','--persona','p01').stdout
        self.assertIn('draft-review',summary)
        self.assertIn('b001',summary)
        self.cli('accept','p01','b001','--drop','Documents/記録.txt')
        self.assertIn('coordinator',path.read_text())
        self.assertIn('accepted',(self.root/'progress/batches.tsv').read_text())

    def test_pending_accept(self):
        self.run_batch(mode='reject-always')
        self.assertIn('成形待ち1',self.cli('status','p01','b001').stdout)
        self.cli('accept','p01','b001')
        self.assertIn('成形待ち1',self.cli('status','p01','b001').stdout)
        self.assertIn('受入済み',self.cli('status','p01','b001').stdout)
        self.assertIn('sources/p01-test/Documents/記録.txt.md',self.git('show','--format=','--name-only','HEAD'))

    def test_happy(self):
        r=self.run_batch(); self.assertNotIn('固有の実務',r.stdout)
        self.assertEqual(self.state()['status'],'done')
        self.assertEqual(len(list((self.root/'people/p01-test').rglob('*.txt'))),2)
        self.assertTrue((self.root/'progress/p01-test/b001.result.md').exists())
        self.assertEqual(json.loads((self.scratch/'calls.json').read_text()),{'templates':1,'templates-review':1,'plan':1,'draft':1,'draft-review':1,'build':1,'build-review':1})
        self.assertTrue((self.scratch/'review-r0.tsv').exists())
    def test_draft_machine_retry(self):
        self.run_batch(mode='machine-once')
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['draft'],2)
        self.assertEqual(calls['build'],1)
    def test_draft_review_retry(self):
        self.run_batch(mode='draft-reject-once')
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['draft'],2)
        self.assertEqual(calls['draft-review'],2)
        self.assertEqual(calls['build'],1)
        self.assertTrue((self.scratch/'draft-review-r0.tsv').exists())
    def test_draft_reject_never_built(self):
        self.run_batch(mode='draft-reject-always')
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['draft'],3)
        self.assertFalse((self.root/'people/p01-test/Documents/記録.txt').exists())
        self.assertFalse((self.root/'sources/p01-test/Documents/記録.txt.md').exists())
        self.assertTrue((self.root/'people/p01-test/Documents/連絡.txt').exists())
    def test_draft_hash_guards(self):
        for mode in ('mutate-draft','mutate-build','mutate-build-review-source'):
            with self.subTest(mode=mode):
                self.run_batch(mode=mode,ok=False)
                self.assertIn('変更',self.state()['reason'])
                shutil.rmtree(self.scratch)
                for name in ('people/p01-test','sources/p01-test'):
                    shutil.rmtree(self.root/name,ignore_errors=True)
    def test_draft_review_again(self):
        self.run_batch()
        self.cli('run','p01','b001','--draft-review-again')
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['draft'],1)
        self.assertEqual(calls['draft-review'],2)
    def test_build_review_keeps_source(self):
        self.run_batch()
        before={p:p.read_bytes() for p in (self.root/'sources/p01-test').rglob('*.md')}
        self.cli('run','p01','b001','--review-again',mode='reject-once')
        self.assertEqual(before,{p:p.read_bytes() for p in before})

    def test_draft_machine_contract(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        rows=b.state['completed']['plan']
        def manuscript(row, body, **metadata):
            values=dict(path=row['パス'],kind=row['作り方'],author='山田 花子',derived_from='')
            values.update(metadata)
            p=b.source(row['パス']); p.parent.mkdir(parents=True,exist_ok=True)
            p.write_text('---\n'+''.join(k+': '+v+'\n' for k,v in values.items())+'---\n'+body)
        first,second=rows
        body='資金調達の条件について経理部長が確認し、銀行への返済期限を変更する方針を決めた。'
        manuscript(first,body)
        manuscript(second,'研修の会場を東館へ変更するため、参加者へ新しい入館経路を通知した。')
        self.assertEqual(b.draft_check(rows),{})
        for text,meta in [('',{}),('placeholder',{}),(body,{'kind':'other'}),(body,{'path':'wrong.txt'})]:
            with self.subTest(text=text,meta=meta):
                manuscript(first,text,**meta)
                self.assertIn(first['パス'],b.draft_check(rows))
        manuscript(first,body)
        b.source(second['パス']).unlink()
        self.assertIn(second['パス'],b.draft_check(rows))
        manuscript(second,body)
        self.assertEqual(set(b.draft_check(rows)),{r['パス'] for r in rows})
        manuscript(second,'研修の日程は参加希望者の回答を踏まえて、水曜日から木曜日へ動かす。')
        previous=dict(first,パス='Documents/過去.txt')
        manuscript(previous,body)
        self.assertIn(first['パス'],b.draft_check(rows))

    def test_plan_retry(self):
        self.run_batch(mode='badplan'); self.assertEqual(json.loads((self.scratch/'calls.json').read_text())['plan'],2)
    def test_bad_plan_stops(self):
        self.run_batch(mode='badplan-always',ok=False); self.assertEqual(self.state()['stage'],'plan')
    def test_reject_rebuilt(self):
        self.run_batch(mode='reject-once'); self.assertEqual(self.state()['rebuilds'],1)
        self.assertEqual(len(list((self.root/'people/p01-test').rglob('*.txt'))),2)
    def test_reject_discarded(self):
        self.run_batch(mode='reject-always'); self.assertEqual(self.state()['rebuilds'],2)
        self.assertEqual(len(list((self.root/'people/p01-test').rglob('*.txt'))),1)
        self.assertIn('成形待ち：数値不整合。',(self.root/'progress/p01-test/b001.md').read_text())
    def test_resume(self):
        self.run_batch(mode='fail',ok=False); self.assertEqual(self.state()['reason'],'利用上限')
        self.run_batch(); calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['plan'],1); self.assertEqual(calls['build'],3)
        self.assertEqual(calls['draft'],1); self.assertEqual(calls['draft-review'],1)
    def test_review_mutation(self):
        self.run_batch(mode='mutate',ok=False); self.assertIn('変更',self.state()['reason'])
    def test_missing_review_rejected(self):
        self.run_batch(mode='missing-review'); self.assertIn('判定なし',(self.root/'progress/p01-test/b001.md').read_text())
    def test_plan_only_then_continue(self):
        self.cli('run','p01','b001','--plan-only'); self.assertFalse(list((self.root/'people/p01-test').rglob('*.txt')))
        self.run_batch(); self.assertEqual(json.loads((self.scratch/'calls.json').read_text())['plan'],1)
    def test_lock(self):
        p=self.root/'scratch/p01-test/.runtime-locks/b001.lock'; p.parent.mkdir(parents=True)
        with p.open('w') as f:
            fcntl.flock(f,fcntl.LOCK_EX|fcntl.LOCK_NB)
            self.run_batch(ok=False)
            self.cli('start','p01','b001',ok=False)
    def load(self):
        loader=importlib.machinery.SourceFileLoader('batch_under_test',str(self.root/'bin/batch'))
        spec=importlib.util.spec_from_loader(loader.name,loader); mod=importlib.util.module_from_spec(spec); loader.exec_module(mod); return mod
    def test_real_check_and_manifest_contract(self):
        shutil.copy2(SOURCE/'bin/check',self.root/'bin/check')
        shutil.copy2(SOURCE/'bin/manifest',self.root/'bin/manifest')
        self.run_batch()
        self.assertEqual(self.state()['count'],2)
        self.cli('accept','p01','b001')
        rows=(self.root/'manifests/p01-test.tsv').read_text().splitlines()
        self.assertEqual(len(rows),2)
        self.assertEqual(len(rows[0].split('\t')),4)
    def test_real_source_failure_rebuilds(self):
        shutil.copy2(SOURCE/'bin/check',self.root/'bin/check')
        self.run_batch(mode='number-once')
        self.assertEqual(self.state()['count'],2)
        calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['draft'],1)
        self.assertEqual(calls['build'],2)
        self.assertIn('1240',(self.root/'people/p01-test/Documents/記録.txt').read_text())

    def test_finished_resume_is_idempotent(self):
        self.run_batch(mode='reject-always')
        before=(self.root/'progress/p01-test/b001.result.md').read_text()
        calls=(self.scratch/'calls.json').read_text()
        self.run_batch()
        self.assertEqual((self.scratch/'calls.json').read_text(),calls)
        self.assertEqual((self.root/'progress/p01-test/b001.result.md').read_text(),before)
    def test_resume_after_final_check_failure(self):
        p=self.root/'bin/check'
        p.write_text(p.read_text().replace("print('passed')", "if '--report' not in args and os.environ.get('FAKE_MODE')=='final-fail': sys.exit(1)\nprint('passed')"))
        self.run_batch(mode='final-fail',ok=False)
        self.assertEqual(self.state()['stage'],'final-check')
        self.run_batch()
        self.assertEqual(json.loads((self.scratch/'calls.json').read_text())["build"],1)
    def test_resume_final_failure_preserves_discards(self):
        check=self.root/'bin/check'
        check.write_text(check.read_text().replace("print('passed')", "if '--report' not in args and os.environ.get('FAKE_MODE','').endswith('final-fail'): sys.exit(1)\nprint('passed')"))
        for mode in ('draft-final-fail','build-final-fail'):
            with self.subTest(mode=mode):
                self.run_batch(mode=mode,ok=False)
                self.assertEqual(self.state()['stage'],'final-check')
                calls=(self.scratch/'calls.json').read_text()
                self.run_batch()
                self.assertEqual((self.scratch/'calls.json').read_text(),calls)
                self.assertEqual(self.state()['count'],1)
                self.assertEqual((self.root/'sources/p01-test/Documents/記録.txt.md').exists(),mode=='build-final-fail')
                shutil.rmtree(self.scratch)
                for name in ('people/p01-test','sources/p01-test'):
                    shutil.rmtree(self.root/name,ignore_errors=True)

    def test_failures_are_only_in_private_logs(self):
        r=self.run_batch(mode='fail',ok=False)
        self.assertNotIn('retry tomorrow',r.stdout+r.stderr)
        self.assertIn('retry tomorrow',self.state()['failure_tail'])
    def test_automatic_reject(self):
        self.run_batch(mode='auto-reject'); self.assertEqual(self.state()['rebuilds'],1)
        calls=json.loads((self.scratch/'calls.json').read_text()); self.assertEqual(calls['build'],2)
    def test_default_models(self):
        self.run_batch()
        args=list(map(json.loads,(self.scratch/'argv.jsonl').read_text().splitlines()))
        self.assertEqual([a[a.index('-m')+1] for a in args],
                         ['gpt-6.1-sol','gpt-6.1-sol','gpt-6.1-sol','gpt-6.1-terra','gpt-6.1-sol','gpt-6.1-terra','gpt-6.1-sol'])
    def test_status_and_separate_report(self):
        self.run_batch(mode='draft-reject-once')
        text=self.cli('status','p01','b001').stdout
        self.assertIn('原稿',text)
        self.assertIn('成形',text)
        self.assertIn('合格',text)
        report=(self.root/'progress/p01-test/b001.result.md').read_text()
        self.assertIn('原稿',report)
        self.assertIn('成形',report)
        self.assertIn('数値不整合',report)
        self.assertIn('原稿の所要時間：',report)
        self.assertIn('成形の所要時間：',report)

    def test_recalc_then_stamp_and_frontmatter(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001',review_again=False))())
        row=dict(b.state['completed']['plan'][0], パス='Documents/計算.xlsx')
        source=b.source(row['パス']); source.parent.mkdir(parents=True,exist_ok=True)
        source.write_text('---\npath: '+row['パス']+'\nkind: office:xlsx\nauthor: 堀口\nmodified_by: 千田\ncreated: 2026-02-01\n---\n本文')
        from openpyxl import Workbook
        path=b.folder/row['パス']; path.parent.mkdir(parents=True,exist_ok=True)
        def produce(*args,**kwargs):
            book=Workbook(); book.active['A1']='=1+1'; book.save(path)
        calls=[]
        def command(args, **kwargs):
            calls.append(list(map(str,args)))
            return subprocess.CompletedProcess(args,0,'')
        with patch.object(b,'codex',side_effect=produce), patch.object(mod,'command',side_effect=command), patch.object(b,'check',return_value={}), patch.object(b,'review',return_value={row['パス']:('pass','確認','')}):
            self.assertEqual(b.phase('build',[row],[row]),{})
        self.assertEqual([Path(c[0]).name for c in calls], ['recalc','stamp'])
        self.assertEqual(calls[1][calls[1].index('--modified-by')+1], '千田')
        self.assertEqual(calls[1][calls[1].index('--created')+1], '2026-02-01')
        # Value-only XLSX bypasses Calc; optional metadata uses stamp defaults.
        source.write_text('---\npath: '+row['パス']+'\nkind: office:xlsx\nauthor: 堀口\n---\n本文')
        def value_only(*args,**kwargs):
            book=Workbook(); book.active['A1']=2; book.save(path)
        b.state['completed']={}; b.state['attempted']=[]; calls.clear()
        with patch.object(b,'codex',side_effect=value_only), patch.object(mod,'command',side_effect=command), patch.object(b,'check',return_value={}), patch.object(b,'review',return_value={row['パス']:('pass','確認','')}):
            self.assertEqual(b.phase('build',[row],[row]),{})
        self.assertEqual([Path(c[0]).name for c in calls], ['stamp'])
        self.assertNotIn('--modified-by',calls[0]); self.assertNotIn('--created',calls[0])
        # Recalculation failures must be isolated and prevent stamping/review.
        real_command=mod.command
        def fail(args,**kwargs):
            if Path(args[0]).name=='recalc':raise ValueError('conversion failed')
            if Path(args[0]).name=='stamp':self.fail('stamp called after failed recalc')
            return real_command(args,**kwargs)
        b.state['completed']={}; b.state['attempted']=[]
        with patch.object(b,'codex',side_effect=produce), patch.object(mod,'command',side_effect=fail), patch.object(b,'check',return_value={}), patch.object(b,'review',return_value={}):
            failures=b.phase('build',[row],[row])
        self.assertIn('conversion failed',failures[row['パス']])

    def test_model_flags_and_dates(self):
        self.cli('run','p01','b001','--producer','gpt-6.1-sol')
        for args in map(json.loads,(self.scratch/'argv.jsonl').read_text().splitlines()):
            self.assertEqual(args[args.index('-m')+1],'gpt-6.1-sol')
            self.assertEqual(args[args.index('-s')+1],'workspace-write')
            self.assertIn('model_reasoning_effort="medium"',args)
        from datetime import datetime, timezone
        expected=datetime(2026,3,6,3,tzinfo=timezone.utc).timestamp()
        self.assertEqual((self.root/'people/p01-test/Documents/記録.txt').stat().st_mtime,expected)
    def test_ten_file_groups(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load()
        args=type('Args',(),dict(persona='p01',batch='b001',producer=None,review_again=False,plan_only=False))()
        b=mod.Batch(args); row=b.state['completed']['plan'][0]
        rows=[dict(row,パス=f'Documents/連絡-{i}.txt') for i in range(23)]
        b.plan.write_text('| パス | 日付 | 作り方 | 内容 | 関連 |\n|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        b.state['completed']['plan']=rows; b.store()
        self.run_batch(); calls=json.loads((self.scratch/'calls.json').read_text())
        self.assertEqual(calls['build'],3); self.assertEqual(calls['build-review'],3)
        self.assertEqual(calls['draft'],3); self.assertEqual(calls['draft-review'],3)
    def test_naming_contract_matches_checker(self):
        import runpy
        checker=runpy.run_path(str(SOURCE/'bin/check'),run_name='checker_contract')
        mod=self.load()
        self.assertEqual(mod.EXT,checker['EXT'])
        self.assertEqual(mod.BAD_PATH.pattern,checker['BAD_PATH'].pattern)
        self.assertEqual(mod.TRACE.pattern,checker['TRACE'].pattern)
    def test_plan_validation(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); args=type('Args',(),dict(persona='p01',batch='b001'))()
        b=mod.Batch(args); original=b.plan.read_text()
        invalid=[('Documents/記録.txt','../外.txt'),('Documents/記録.txt','Documents//連絡.txt'),('Documents/記録.txt','Documents/連絡.txt'),
                 ('Documents/記録.txt','Documents/.隠し.txt'),('Documents/記録.txt','Documents/full.txt'),
                 ('Documents/記録.txt','Documents/batch.txt'),('2026-03-06','2027-03-06'),
                 ('Documents/記録.txt','Documents/記録.docx'),(' | text | ',' | 不明 | ')]
        for old,new in invalid:
            with self.subTest(new=new):
                b.plan.write_text(original.replace(old,new))
                with self.assertRaises(ValueError): b.validate_plan()
        b.plan.write_text(original)
        p=self.root/'people/p01-test/Documents/記録.txt'; p.parent.mkdir(parents=True); p.write_text('既存')
        with self.assertRaisesRegex(ValueError,'既存'): b.validate_plan()
    def test_status_grouped_formats(self):
        (self.root/'personas/p01-test.md').write_text('---\nperiod: 2026-01-01..2026-12-31\n---\n## 形式の目安\n| pdf（テキスト） | 16% |\n| pdf（スキャン） | 9% |\n| png・jpg | 4% |\n| txt・md | 3% |\n')
        (self.root/'manifests').mkdir(); (self.root/'manifests/p01-test.tsv').write_text('file.pdf\thash\n')
        output=self.cli('status','p01').stdout
        self.assertIn('差 +75.0 pt',output); self.assertIn('jpg・png',output)
    def test_committed_files_protected(self):
        self.run_batch(); self.cli('accept','p01','b001')
        self.cli('discard','p01','b001','--reason','誤操作',ok=False)
        self.assertEqual(len(list((self.root/'people/p01-test').rglob('*.txt'))),2)
    def test_start_resume_jobs(self):
        p=self.root/'ops/bin/codex-job'; p.parent.mkdir(parents=True); p.write_text(JOB); p.chmod(0o755)
        mod=self.load()
        with patch.object(mod.shutil,'disk_usage',return_value=shutil._ntuple_diskusage(100_000_000_000,1,99_999_999_999)):
            mod.main(['start','p01','b001','--plan-only'])
            args=json.loads((self.root/'job-args.json').read_text())
            self.assertEqual(args[args.index('--name')+1],'p01-b001-r3')
            self.assertIn('--plan-only',args[-1]); self.assertEqual(args[-2],'--command')
            mod.main(['resume','p01','b001','--producer','gpt-6.1-sol','--rebuild'])
            self.assertIn('--rebuild',json.loads((self.root/'job-args.json').read_text())[-1])
            self.assertIn('--producer gpt-6.1-sol',json.loads((self.root/'job-args.json').read_text())[-1])
            mod.main(['resume','p01','b001','--redraft'])
            self.assertIn('--redraft',json.loads((self.root/'job-args.json').read_text())[-1])
    def test_launch_lock_handoff(self):
        p=self.root/'ops/bin/codex-job'; p.parent.mkdir(parents=True)
        p.write_text("""#!/usr/bin/env python3
import pathlib,subprocess,sys,shlex,time
if sys.argv[1]=='status': print('REPO\\tJOB\\tSTATE\\tDETAIL')
else:
 with pathlib.Path('child.log').open('w') as f:
  subprocess.Popen([sys.executable,*shlex.split(sys.argv[-1])],stdout=f,stderr=subprocess.STDOUT,start_new_session=True)
 time.sleep(.2)
"""); p.chmod(0o755)
        mod=self.load()
        with patch.object(mod.shutil,'disk_usage',return_value=shutil._ntuple_diskusage(100_000_000_000,1,99_999_999_999)), patch.dict(os.environ,self.env):
            mod.main(['start','p01','b001'])
        import time
        deadline=time.monotonic()+10
        while time.monotonic()<deadline:
            if (self.scratch/'state.json').exists() and self.state()['status'] in ('done','failed'): break
            time.sleep(.05)
        self.assertEqual(self.state()['status'],'done',(self.root/'child.log').read_text())
    def test_queued_job_exclusion(self):
        p=self.root/'ops/bin/codex-job'; p.parent.mkdir(parents=True)
        p.write_text(JOB.replace('p01-b001\\tdone','p01-b002\\trunning')); p.chmod(0o755)
        mod=self.load()
        with patch.object(mod.shutil,'disk_usage',return_value=shutil._ntuple_diskusage(100_000_000_000,1,99_999_999_999)):
            with self.assertRaisesRegex(ValueError,'進行中'): mod.main(['start','p01','b001'])
    def test_low_space(self):
        mod=self.load()
        with patch.object(mod.shutil,'disk_usage',return_value=shutil._ntuple_diskusage(40,20,20)):
            with self.assertRaisesRegex(ValueError,'30'): mod.main(['start','p01','b001'])
    def test_accept_drop(self):
        self.run_batch(); self.cli('accept','p01','b001','--drop','Documents/記録.txt')
        self.assertFalse(self.scratch.exists()); self.assertFalse((self.root/'people/p01-test/Documents/記録.txt').exists())
        self.assertFalse((self.root/'sources/p01-test/Documents/記録.txt.md').exists())
        self.assertIn('連絡.txt',(self.root/'manifests/p01-test.tsv').read_text())
        self.assertIn('Accept p01-test b001: 月次報告 (1 files)',self.git('log','-1','--format=%s'))
        self.assertEqual(set(self.git('show','--format=','--name-only','HEAD').splitlines()),{'people/p01-test/Documents/連絡.txt','progress/p01-test/b001.md','progress/p01-test/b001.result.md','manifests/p01-test.tsv','templates/p01-test/README.md','sources/p01-test/Documents/連絡.txt.md','progress/findings.tsv','progress/batches.tsv'})
        self.assertIn('受入済み',self.cli('status','p01').stdout)
    def test_unrelated_index_preserved(self):
        self.run_batch(); (self.root/'unrelated').write_text('keep'); self.git('add','unrelated')
        before=self.git('diff','--cached'); self.cli('accept','p01','b001','--drop','Documents/記録.txt',ok=False)
        self.assertEqual(before,self.git('diff','--cached')); self.assertTrue((self.root/'people/p01-test/Documents/記録.txt').exists())
        self.cli('discard','p01','b001','--reason','品質',ok=False)
    def test_discard(self):
        self.run_batch(); self.cli('discard','p01','b001','--reason','品質')
        self.assertFalse(self.scratch.exists()); self.assertFalse(list((self.root/'people/p01-test').rglob('*.txt')))
        self.assertFalse(list((self.root/'sources/p01-test').rglob('*.md')))
        self.assertIn('破棄：品質。',(self.root/'progress/p01-test/b001.md').read_text())
        self.assertIn('coordinator',(self.root/'progress/findings.tsv').read_text())
        self.assertIn('discarded',(self.root/'progress/batches.tsv').read_text())
        self.assertTrue(all(p in ('progress/findings.tsv','progress/batches.tsv') or p.startswith('progress/p01-test/b001.') for p in self.git('show','--format=','--name-only','HEAD').splitlines()))
    def test_review_again_keeps_discard_history(self):
        self.run_batch(mode='reject-always')
        self.cli('run','p01','b001','--review-again')
        self.assertIn('数値不整合',(self.root/'progress/p01-test/b001.result.md').read_text())
        self.assertEqual(self.state()['count'],1)
    def test_review_again(self):
        self.run_batch(); self.cli('run','p01','b001','--review-again')
        calls=json.loads((self.scratch/'calls.json').read_text()); self.assertEqual(calls['build'],1); self.assertEqual(calls['build-review'],2)

if __name__=='__main__': unittest.main()
