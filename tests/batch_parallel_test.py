"""Parallel scheduling regressions; all processes and fixtures live in /tmp."""
import concurrent.futures
import json
import os
import time
from unittest.mock import patch
import unittest
import batch_test as fixtures

class ParallelTest(unittest.TestCase):
    setUp = fixtures.BatchTest.setUp
    git = fixtures.BatchTest.git
    cli = fixtures.BatchTest.cli
    load = fixtures.BatchTest.load
    scratch = fixtures.BatchTest.scratch
    state = fixtures.BatchTest.state
    run_batch = fixtures.BatchTest.run_batch
    def prepare_parallel(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load()
        b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001',producer=None,review_again=False,plan_only=False))())
        first=mod.plan_rows(b.plan)[0]
        rows=[dict(first, パス=f'Documents/連絡-{i}.txt') for i in range(23)]
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        fake=fixtures.FAKE.replace("s=meta['stage'];", """import time, atexit
log=root/'events.jsonl'
def event(kind):
 with log.open('a') as handle: handle.write(json.dumps([kind,time.monotonic(),os.getpid(),meta['stage'],[r['パス'] for r in meta['rows']]])+'\\n')
event('start'); atexit.register(event,'end'); time.sleep(.15)
if os.environ.get('FAIL_GROUP')=='yes' and meta['rows'] and meta['rows'][0]['パス'].endswith('連絡-0.txt'):
 sys.exit(3)
s=meta['stage'];""")
        (self.root/'fake-codex').write_text(fake)
        b.draft_check=lambda targets: {}
        return mod,b,rows

    def events(self):
        return [json.loads(line) for line in (self.root/'events.jsonl').read_text().splitlines()]

    def test_real_fake_review_parallel_and_global_limit(self):
        mod,b,rows=self.prepare_parallel()
        with patch.dict(os.environ,self.env | {'CORPUS_MAX_CODEX':'2'}):
            self.assertEqual(b.phase('draft',rows,rows),{})
        events=self.events(); active=maximum=0
        for event in events:
            active+=1 if event[0]=='start' else -1
            maximum=max(maximum,active)
        self.assertEqual(maximum,2)
        reviews=[e for e in events if e[3]=='draft-review']
        self.assertEqual(len([e for e in reviews if e[0]=='start']),3)
        self.assertLess(reviews[1][1],reviews[2][1])
        self.assertEqual(reviews[0][0], 'start')
        self.assertEqual(reviews[1][0], 'start')
        contexts=[json.loads(line) for line in (self.scratch/'contexts.jsonl').read_text().splitlines()]
        work=[c['work'] for c in contexts if c['stage'] in ('draft','draft-review')]
        self.assertEqual(len(work),len(set(work)))

    def test_real_fake_failure_joins_peers_and_resume(self):
        mod,b,rows=self.prepare_parallel()
        with patch.dict(os.environ,self.env | {'FAIL_GROUP':'yes'}):
            with self.assertRaisesRegex(ValueError,'codex exit'):
                b.phase('draft',rows,rows)
        events=self.events()
        self.assertEqual(sum(e[0]=='start' for e in events),sum(e[0]=='end' for e in events))
        completed=[k for k in b.state['completed'] if k.startswith('draft-r0-')]
        self.assertEqual(len(completed),2)
        with patch.dict(os.environ,self.env | {'FAIL_GROUP':'no'}):
            self.assertEqual(b.phase('draft',rows,rows),{})
        starts=[e for e in self.events() if e[0]=='start' and e[3]=='draft']
        self.assertEqual(sum(e[4][0].endswith('連絡-10.txt') for e in starts),1)
        self.assertEqual(sum(e[4][0].endswith('連絡-20.txt') for e in starts),1)

    def test_real_fake_dependency_layers_and_review_again(self):
        mod,b,rows=self.prepare_parallel()
        rows=rows[:3]
        rows[-1]['先に要るもの']=rows[0]['パス']
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        b.state['completed']['plan']=rows
        b.a.draft_review_again=False
        b.a.rebuild=False
        with patch.dict(os.environ,self.env):
            b.run()
            starts=[e for e in self.events() if e[0]=='start' and e[3]=='draft']
            self.assertEqual([len(e[4]) for e in starts],[2,1])
            first_review_end=next(e[1] for e in self.events() if e[0]=='end' and e[3]=='draft-review')
            self.assertLess(first_review_end,starts[1][1])
            b.a.draft_review_again=True
            b.run()
        self.assertEqual(len([e for e in self.events() if e[0]=='start' and e[3]=='draft']),2)

    def test_real_fake_rejected_parent_does_not_skip_descendants(self):
        mod,b,rows=self.prepare_parallel()
        rows=rows[:3]
        rows[1]['先に要るもの']=rows[0]['パス']
        rows[2]['先に要るもの']=rows[1]['パス']
        b.plan.write_text('| '+' | '.join(mod.COLUMNS)+' |\n|---|---|---|---|---|---|\n'+''.join('| '+' | '.join(r[c] for c in mod.COLUMNS)+' |\n' for r in rows))
        with patch.dict(os.environ,self.env | {'FAKE_MODE':'draft-reject-always'}):
            failures=b.draft_phase(rows,rows)
        self.assertTrue(all(reason != '先に要る原稿が不合格' for reason in failures.values()))
        self.assertEqual({e[4][0] for e in self.events() if e[0]=='start' and e[3]=='draft'}, {r['パス'] for r in rows})

    def test_overlapping_in_progress_plan_rejected(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        other=b.plan.with_name('b002.md'); other.write_text(b.plan.read_text())
        state=self.root/'scratch/p01-test/b002/state.json'
        state.parent.mkdir(parents=True); state.write_text('{"status":"running"}')
        with self.assertRaisesRegex(ValueError,'進行中の batch と重複パス'): b.validate_plan()

    def test_dependency_column_old_plan_and_invalid_graphs(self):
        self.cli('run','p01','b001','--plan-only')
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        self.assertEqual(mod.plan_rows(b.plan)[0]['先に要るもの'], '')
        original=b.plan.read_text()
        extended=original.replace('関連 |','関連 | 先に要るもの |').replace('|---|---|---|---|---|','|---|---|---|---|---|---|')
        lines=extended.splitlines()
        lines[2]=lines[2]+' Documents/外.txt |'
        lines[3]=lines[3]+' |'
        b.plan.write_text('\n'.join(lines)+'\n')
        with self.assertRaisesRegex(ValueError,'同じ batch'): b.validate_plan()
        lines[2]=lines[2].replace('Documents/外.txt','Documents/連絡.txt')
        lines[3]=lines[3]+' Documents/記録.txt |' if False else lines[3][:-1]+' Documents/記録.txt |'
        b.plan.write_text('\n'.join(lines)+'\n')
        with self.assertRaisesRegex(ValueError,'循環'): b.validate_plan()

    def test_parallel_steps_wait_and_resume(self):
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        ended=[]
        def bad(): time.sleep(.05); raise ValueError('failed group')
        def good(): time.sleep(.15); ended.append('done'); return 42
        with self.assertRaisesRegex(ValueError,'failed group'):
            b.parallel_steps([('bad',bad,1),('good',good,1)])
        self.assertEqual(ended,['done'])
        self.assertEqual(b.parallel_steps([('good',lambda: self.fail('rerun'),1)]),[42])
        self.assertEqual(json.loads(b.state_path.read_text())['completed']['good'],42)

    def test_slot_limit_and_release(self):
        mod=self.load(); active=0; maximum=0
        import threading
        lock=threading.Lock()
        def work():
            nonlocal active,maximum
            with mod.codex_slot():
                with lock: active+=1; maximum=max(maximum,active)
                time.sleep(.08)
                with lock: active-=1
        with patch.dict(os.environ,CORPUS_MAX_CODEX='2'):
            with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool: list(pool.map(lambda _:work(),range(6)))
            self.assertEqual(maximum,2)
            self.assertEqual(mod.slot_usage(),0)

    def test_shared_metadata_changes_after_planning_are_rejected(self):
        mod,b,rows=self.prepare_parallel()
        fake=self.root/'fake-codex'
        fake.write_text(fake.read_text().replace("s=meta['stage'];", "(root/'progress'/meta['persona']/'facts.md').write_text('unauthorized change')\ns=meta['stage'];"))
        with patch.dict(os.environ,self.env):
            with self.assertRaisesRegex(ValueError,'型・台帳・persona'):
                b.codex('draft',rows[:1])

    def test_slots_release_on_process_death(self):
        import subprocess
        mod=self.load()
        script="""import runpy, time
m=runpy.run_path('bin/batch', run_name='slot_test')
with m['codex_slot']():
 print('locked', flush=True)
 time.sleep(30)
"""
        with patch.dict(os.environ,CORPUS_MAX_CODEX='1'):
            process=subprocess.Popen(['python3','-c',script],cwd=self.root,env=dict(os.environ),stdout=subprocess.PIPE,text=True)
            try:
                self.assertEqual(process.stdout.readline().strip(),'locked')
                self.assertEqual(mod.slot_usage(),1)
            finally:
                process.kill(); process.wait(); process.stdout.close()
            self.assertEqual(mod.slot_usage(),0)

    def test_draft_waves_and_rejected_prerequisite(self):
        mod=self.load(); b=mod.Batch(type('Args',(),dict(persona='p01',batch='b001'))())
        rows=[{'パス':'a.txt','先に要るもの':''},{'パス':'b.txt','先に要るもの':'a.txt'},{'パス':'c.txt','先に要るもの':'b.txt'}]
        calls=[]
        def phase(stage,targets,all_rows):
            calls.append([r['パス'] for r in targets]); return {'a.txt':'bad'}
        b.phase=phase
        failed=b.draft_phase(rows,rows)
        self.assertEqual(calls,[['a.txt'],['b.txt'],['c.txt']])
        self.assertEqual(failed,{'a.txt':'bad'})

if __name__=='__main__':
    import unittest
    unittest.main()
