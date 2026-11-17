"""Autopilot integration tests: isolated Git repositories and fake job executors."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
P = 'p01-test'
HEADER = '| batch | 範囲 | 件数の目安 | 重点 | 含めたいもの | 避けるもの | 状態 |\n|---|---|---|---|---|---|---|\n'
PLAN = '| パス | 日付 | 作り方 | 内容 | 関連 | 先に要るもの |\n|---|---|---|---|---|---|\n'
FAKE = '''#!/usr/bin/env python3
import json, pathlib, sys
r=pathlib.Path.cwd(); a=sys.argv[1:]; f=r/'calls'
with f.open('a') as h:h.write(json.dumps([pathlib.Path(sys.argv[0]).name,*a],ensure_ascii=False)+'\\n')
jp=r/'jobs.json'; jobs=json.loads(jp.read_text())
if pathlib.Path(sys.argv[0]).name=='codex-job':
 if a[0]=='status':
  print('REPO\\tJOB\\tSTATE\\tDETAIL')
  for n,s in jobs.items():print('personaCorpus',n,s,'detail',sep='\\t')
 elif a[0]=='start':
  jobs[a[a.index('--name')+1]]='running';jp.write_text(json.dumps(jobs))
 elif a[0]=='stop':
  jobs[a[-1]]='failed';jp.write_text(json.dumps(jobs))
else:
 if a[0] in ('start','resume'):
  n=a[1].split('-')[0]+'-'+a[2]; base=n; i=1
  while n in jobs:i+=1;n=base+'-r'+str(i)
  jobs[n]='running';jp.write_text(json.dumps(jobs))
 elif a[0]=='accept':
  with (r/'progress/batches.tsv').open('a') as h:h.write(a[1]+'\\t'+a[2]+'\\taccepted\\n')
  import subprocess
  paths=['progress/batches.tsv','progress/'+a[1]+'/roadmap.md']
  paths=[p for p in paths if (r/p).exists()]
  subprocess.run(['git','add','--',*paths],check=True,stdout=subprocess.DEVNULL)
  subprocess.run(['git','commit','-qm','Accept fake batch'],check=True,stdout=subprocess.DEVNULL)
'''

class AutopilotTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.r = Path(self.tmp.name) / 'repo'; self.r.mkdir()
        for d in ('bin','ops/bin','personas','progress/'+P,'prompts','manifests'):
            (self.r/d).mkdir(parents=True, exist_ok=True)
        shutil.copy(ROOT/'bin/autopilot', self.r/'bin/autopilot')
        for p in ('bin/batch','ops/bin/codex-job'):
            (self.r/p).write_text(FAKE); (self.r/p).chmod(0o755)
        for p in ('audit','roadmap'):
            shutil.copy(ROOT/f'prompts/{p}.md',self.r/f'prompts/{p}.md')
        (self.r/'personas'/f'{P}.md').write_text('# test')
        (self.r/'jobs.json').write_text('{}')
        (self.r/'progress/batches.tsv').write_text('persona\tbatch\tstatus\n')
        self.env = dict(os.environ, AUTOPILOT_MIN_FREE_BYTES='0', PYTHONDONTWRITEBYTECODE='1')
        self.git('init','-qb','main'); self.git('config','user.name','Test'); self.git('config','user.email','test@example.invalid')
        self.git('add','.'); self.git('commit','-qm','fixture')
        self.remote=Path(self.tmp.name)/'remote.git'
        subprocess.run(['git','init','--bare','-q',str(self.remote)],check=True)
        self.git('remote','add','origin',str(self.remote)); self.git('push','-q','origin','main')
    def git(self,*a):
        return subprocess.run(['git',*a],cwd=self.r,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,check=True).stdout
    def cli(self,*a):
        p=subprocess.run([sys.executable,str(self.r/'bin/autopilot'),*a],cwd=self.r,env=self.env,text=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        self.assertEqual(p.returncode,0,p.stdout+p.stderr); return p.stdout
    def once(self):return self.cli('once','--target','30')
    def jobs(self,**updates):
        p=self.r/'jobs.json'; j=json.loads(p.read_text());j.update(updates);p.write_text(json.dumps(j));return j
    def calls(self):
        p=self.r/'calls';return [json.loads(x) for x in p.read_text().splitlines()] if p.exists() else []
    def roadmap(self,rows=None):
        (self.r/f'progress/{P}/roadmap.md').write_text(HEADER+(rows or '| b001 | 月次締め | 30 | 差異調査 | 表とメール | 重複 | |\n'))
    def batch(self,status='done',reason='',pending=0,dropped=0):
        d=self.r/f'scratch/{P}/b001';d.mkdir(parents=True,exist_ok=True)
        paths={f'd{i}.txt':'原稿不合格' for i in range(dropped)}
        (d/'state.json').write_text(json.dumps(dict(status=status,stage='draft',reason=reason,draft_failures=paths)))
        rows=[(f'd{i}.txt','破棄：原稿不合格。内容') for i in range(dropped)]+[(f'w{i}.txt','成形待ち：失敗。内容') for i in range(pending)]+[('ok.txt','内容')]
        (self.r/f'progress/{P}/b001.md').write_text(PLAN+''.join(f'| {p} | 2026-01-01 | text | {v} | | |\n' for p,v in rows))
        for base,ext in (('people',''),('sources','.md')):
            p=self.r/f'{base}/{P}/ok.txt{ext}';p.parent.mkdir(parents=True,exist_ok=True);p.write_text('original')
        return d/'state.json'
    def audit(self,verdict='accept',drop='[]'):
        self.jobs(**{'p01-b001-audit':'done'})
        judgment = '破棄' if drop != '[]' else '合格'
        (self.r/f'progress/{P}/b001.audit.md').write_text(f'verdict: {verdict}\ndrop: {drop}\n\n| パス | 判定 | 理由 |\n|---|---|---|\n| ok.txt | {judgment} | 確認 |\n\n所見：確認\n')
    def holds(self):
        p=self.r/'progress/holds.md';return p.read_text() if p.exists() else ''
    def test_roadmap_job_retry_and_hold(self):
        self.once();self.assertIn('p01-roadmap',self.jobs())
        self.jobs(**{'p01-roadmap':'done'});self.once();self.assertIn('p01-roadmap-r2',self.jobs())
        self.jobs(**{'p01-roadmap-r2':'done'});self.once();self.assertIn('roadmap',self.holds())
    def test_request_and_no_duplicate_or_target_launch(self):
        self.roadmap();self.once();self.once()
        self.assertEqual(sum(c[:2]==['batch','start'] for c in self.calls()),1)
        self.assertIn('範囲：月次締め',(self.r/f'progress/{P}/b001.request.md').read_text())
        self.assertIn('b001',(self.r/f'progress/{P}/roadmap.md').read_text())
    def test_target_reached(self):
        self.roadmap();(self.r/f'manifests/{P}.tsv').write_text('x\thash\t1\tdate\n'*30)
        self.once();self.assertFalse(any(c[:2]==['batch','start'] for c in self.calls()))
    def test_redraft_then_two_rebuilds_then_audit(self):
        self.batch(pending=1,dropped=3);self.once();self.assertTrue(any('--redraft' in c for c in self.calls()))
        for n in ('p01-b001','p01-b001-r2'):
            self.jobs(**{n:'done'});self.once();self.assertTrue(any('--rebuild' in c for c in self.calls()))
        self.jobs(**{'p01-b001-r3':'done'});self.once();self.assertIn('p01-b001-audit',self.jobs())
        self.assertEqual(sum('--rebuild' in c for c in self.calls()),2)
    def test_accept_drop_push_roadmap(self):
        self.roadmap('| b001 | 月次 | 30 | 表 | メール | 重複 | 進行中 b001 |\n');self.batch();self.once();self.audit(drop='["ok.txt"]');self.once()
        self.assertTrue(any(c[:2]==['batch','accept'] and c[-2:]==['--drop','ok.txt'] for c in self.calls()))
        self.assertIn('受入済み',(self.r/f'progress/{P}/roadmap.md').read_text())
        self.assertEqual(self.git('rev-parse','main').strip(),self.git('ls-remote','origin','refs/heads/main').split()[0])
    def test_hold_and_mutation(self):
        self.batch();self.once();self.audit('hold');self.once();self.assertIn('監査',self.holds());self.assertFalse(any(c[:2]==['batch','accept'] for c in self.calls()))
    def test_hash_mutation(self):
        self.batch();self.once();self.audit();(self.r/f'people/{P}/ok.txt').write_text('changed');self.once();self.assertIn('sha256',self.holds())
    def test_bad_audit_retries_once(self):
        self.batch();self.once();self.jobs(**{'p01-b001-audit':'done'});self.once();self.assertIn('p01-b001-audit-r2',self.jobs())
        self.jobs(**{'p01-b001-audit-r2':'done'});self.once();self.assertIn('監査',self.holds())
    def test_failed_resume_once(self):
        self.batch('failed','error');self.once();self.jobs(**{'p01-b001':'failed'});self.once();self.assertIn('再失敗',self.holds())
    def test_limit_wait(self):
        self.batch('failed','利用上限');self.once();self.assertFalse(any(c[:2]==['batch','resume'] for c in self.calls()))
        state=self.r/'scratch/autopilot/state.json';s=json.loads(state.read_text());s['batches'][P+'/b001']['limit_since']-=1801;state.write_text(json.dumps(s));self.once();self.assertTrue(any(c[:2]==['batch','resume'] for c in self.calls()))
    def test_stop_drains_acceptance(self):
        self.batch();self.once();self.audit();self.cli('stop');self.once();self.assertTrue(any(c[:2]==['batch','accept'] for c in self.calls()))
        self.assertFalse(any(c[:2]==['batch','start'] for c in self.calls()))
    def test_low_space_and_five_holds(self):
        self.roadmap();self.env['AUTOPILOT_MIN_FREE_BYTES']=str(10**30);self.once();self.assertFalse(any(c[:2]==['batch','start'] for c in self.calls()))
        self.env['AUTOPILOT_MIN_FREE_BYTES']='0';(self.r/'progress/holds.md').write_text('| persona | batch | 理由 | 日時 |\n|---|---|---|---|\n'+''.join(f'| p{i:02}-test | b001 | 失敗 | now |\n' for i in range(2,7)));self.once();self.assertFalse(any(c[:2]==['batch','start'] for c in self.calls()))
    def test_stop_now_and_start_deduplication(self):
        self.cli('start');self.cli('start');self.assertEqual(sum(c[:2]==['codex-job','start'] for c in self.calls()),1)
        self.cli('stop','--now');self.assertIn(['codex-job','stop','personaCorpus','autopilot'],self.calls())
    def test_exception_isolated_and_status(self):
        (self.r/f'scratch/{P}/b001').mkdir(parents=True);(self.r/f'scratch/{P}/b001/state.json').write_text('broken')
        p='p02-other';(self.r/f'personas/{p}.md').write_text('# persona');self.once();self.assertIn('p02-roadmap',self.jobs());self.assertIn('error',(self.r/'scratch/autopilot/log.tsv').read_text())
        out=self.cli('status');self.assertIn('Codex',out);self.assertIn(P,out);self.assertIn('30',out)
    def test_push_failure_three_times_blocks_acceptance(self):
        self.git('remote','set-url','origin',str(self.r/'missing'))
        self.batch();self.once();self.audit();self.once();self.once();self.once()
        self.assertIn('push',self.holds());s=json.loads((self.r/'scratch/autopilot/state.json').read_text());self.assertGreaterEqual(s['push_failures'],3)

    def test_failed_job_without_scratch_resumes_then_holds(self):
        self.jobs(**{'p01-b001': 'failed'})
        self.once()
        self.assertTrue(any(c[:2] == ['batch', 'resume'] for c in self.calls()))
        self.jobs(**{'p01-b001-r2': 'failed'})
        self.once()
        self.assertIn('再失敗', self.holds())

    def test_invalid_roadmap_fields_and_target(self):
        self.roadmap('| b001 | | 30 | 重点 | 内容 | 回避 | |\n')
        self.once()
        self.assertIn('p01-roadmap', self.jobs())
        self.roadmap('| b001 | 月次 | 20 | 重点 | 内容 | 回避 | |\n')
        self.jobs(**{'p01-roadmap': 'done'})
        self.once()
        self.assertIn('p01-roadmap-r2', self.jobs())

    def test_valid_generated_roadmap_launches_next_number(self):
        self.once()
        self.jobs(**{'p01-roadmap': 'done'})
        (self.r/f'progress/{P}/b012.request.md').write_text('# 過去の依頼')
        self.roadmap()
        self.once()
        self.assertTrue((self.r/f'progress/{P}/b013.request.md').exists())
        self.assertTrue(any(c[:4] == ['batch', 'start', P, 'b013'] for c in self.calls()))

    def test_live_limit_never_resumed_or_stopped(self):
        self.batch('failed', '利用上限')
        self.jobs(**{'p01-b001': 'limit'})
        self.once()
        self.assertFalse(any(c[:2] == ['batch', 'resume'] for c in self.calls()))
        self.assertFalse(any(c[:2] == ['codex-job', 'stop'] for c in self.calls()))

    def test_both_hash_trees_are_protected(self):
        self.batch()
        self.once()
        self.audit()
        (self.r/f'sources/{P}/ok.txt.md').unlink()
        self.once()
        self.assertIn('sha256', self.holds())

    def test_bad_drop_and_report_never_accept(self):
        self.batch()
        self.once()
        self.audit(drop='["../outside.txt"]')
        self.once()
        self.assertFalse(any(c[:2] == ['batch', 'accept'] for c in self.calls()))
        self.assertIn('p01-b001-audit-r2', self.jobs())

    def test_push_recovery_and_no_reacceptance(self):
        self.git('remote', 'set-url', 'origin', str(self.r/'missing'))
        self.batch()
        self.once()
        self.audit()
        for _ in range(3):
            self.once()
        self.assertIn('push', self.holds())
        self.git('remote', 'set-url', 'origin', str(self.remote))
        self.once()
        self.assertNotIn('| 全体 | push |', self.holds())
        self.once()  # The hold-removal commit is pushed on the following round.
        self.assertEqual(sum(c[:2] == ['batch', 'accept'] for c in self.calls()), 1)
        self.assertEqual(self.git('rev-parse', 'main').strip(), self.git('ls-remote', 'origin', 'refs/heads/main').split()[0])

    def test_graceful_stop_still_starts_existing_batch_audit(self):
        self.batch()
        self.cli('stop')
        self.once()
        self.assertIn('p01-b001-audit', self.jobs())
        self.assertNotIn('p01-roadmap', self.jobs())
        self.assertFalse(any(c[:2] == ['batch', 'start'] for c in self.calls()))
        self.audit()
        self.once()
        self.assertTrue(any(c[:2] == ['batch', 'accept'] for c in self.calls()))

    def test_stop_conditions_still_accept_completed_audit(self):
        self.batch()
        self.once()
        self.audit()
        self.env['AUTOPILOT_MIN_FREE_BYTES'] = str(10**30)
        self.once()
        self.assertTrue(any(c[:2] == ['batch', 'accept'] for c in self.calls()))

    def test_stop_now_prevents_once_and_preserves_batch_jobs(self):
        self.roadmap()
        self.jobs(**{'p01-b001': 'running', 'autopilot': 'running'})
        self.cli('stop', '--now')
        self.once()
        self.assertEqual(self.jobs()['p01-b001'], 'running')
        self.assertFalse(any(c[:2] == ['batch', 'start'] for c in self.calls()))

    def test_run_recovers_after_refresh_exception(self):
        # One failed supervisor query must not kill the persistent run loop.
        job = self.r/'ops/bin/codex-job'
        job.write_text(FAKE.replace("if a[0]=='status':", "if a[0]=='status':\n  flag=r/'fail-first'\n  if not flag.exists():flag.touch();sys.exit(4)"))
        self.env['AUTOPILOT_INTERVAL_SECONDS'] = '0.02'
        process = subprocess.Popen([sys.executable, str(self.r/'bin/autopilot'), 'run'], cwd=self.r, env=self.env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            import time
            deadline = time.monotonic() + 5
            while time.monotonic() < deadline and 'p01-roadmap' not in self.jobs():
                time.sleep(.02)
            self.assertIn('p01-roadmap', self.jobs())
            self.cli('start')
            self.assertNotIn('autopilot', self.jobs())
            (self.r/'scratch/autopilot/stop-now').touch()
            out, err = process.communicate(timeout=5)
            self.assertEqual(process.returncode, 0, out+err)
        finally:
            if process.poll() is None:
                process.kill()
                process.communicate()


class AuxiliaryTest(unittest.TestCase):
    def test_auxiliary_codex_uses_production_slot_and_astra(self):
        import fcntl
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            (root/'bin').mkdir()
            shutil.copy(ROOT/'bin/batch', root/'bin/batch')
            (root/'scratch/.slots').mkdir(parents=True)
            brief, output = root/'scratch/brief.md', root/'scratch/out.md'
            brief.write_text('audit instruction')
            fake = root/'scratch/fake-codex'
            fake.write_text('#!/usr/bin/env python3\nimport json,pathlib,sys\npathlib.Path(sys.argv[sys.argv.index("-o")+1]).write_text(json.dumps(dict(args=sys.argv,prompt=sys.stdin.read())))\n')
            fake.chmod(0o755)
            subprocess.run(['git','init','-q',str(root)],check=True)
            env = dict(os.environ, CODEX_BIN=str(fake), CORPUS_MAX_CODEX='1', PYTHONDONTWRITEBYTECODE='1')
            with (root/'scratch/.slots/0').open('a') as slot:
                fcntl.flock(slot, fcntl.LOCK_EX)
                process = subprocess.Popen([sys.executable, str(root/'bin/batch'), 'codex-once', '--brief', str(brief), '--output', str(output)], cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                try:
                    with self.assertRaises(subprocess.TimeoutExpired):
                        process.communicate(timeout=.2)
                    self.assertFalse(output.exists())
                    fcntl.flock(slot, fcntl.LOCK_UN)
                    out, err = process.communicate(timeout=5)
                    self.assertEqual(process.returncode,0,out+err)
                finally:
                    if process.poll() is None:
                        process.kill()
                        process.communicate()
            data = json.loads(output.read_text())
            self.assertEqual(data['args'][data['args'].index('-m')+1], 'gpt-6-astra')
            self.assertEqual(data['prompt'], 'audit instruction')
            self.assertEqual(data['args'][data['args'].index('-C')+1], str(root))

if __name__=='__main__':unittest.main()
