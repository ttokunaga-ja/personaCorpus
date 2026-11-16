"""Weighted builds, same-round continuations and root stray quarantine (fake Codex only)."""
import json
import os
import concurrent.futures
import unittest
from unittest.mock import patch
import batch_test as fixtures


class BuildGroupsTest(unittest.TestCase):
    setUp = fixtures.BatchTest.setUp
    git = fixtures.BatchTest.git
    cli = fixtures.BatchTest.cli
    load = fixtures.BatchTest.load
    scratch = fixtures.BatchTest.scratch
    state = fixtures.BatchTest.state
    run_batch = fixtures.BatchTest.run_batch

    def contexts(self):
        return [json.loads(x) for x in (self.scratch/'contexts.jsonl').read_text().splitlines()]

    def test_weighted_fake_build_groups(self):
        self.cli('run', 'p01', 'b001', '--plan-only')
        mod = self.load()
        b = mod.Batch(type('Args', (), dict(persona='p01', batch='b001', review_again=False))())
        rows = [dict(パス=f'Documents/{i}.{ext}', 日付='2026-03-06', 作り方='text', 内容='内容', 関連='', 先に要るもの='')
                for i, ext in enumerate(['pptx']*2+['eml']*6)]
        for row in rows:
            p = b.source(row['パス']); p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text('---\nauthor: 山田\n---\n本文\n')
        b.check = lambda *args: {}
        original = mod.command
        def command(args, **kwargs):
            if str(args[0]).endswith('/bin/stamp'):
                return None
            return original(args, **kwargs)
        with patch.dict(os.environ, self.env), patch.object(mod, 'command', command):
            self.assertEqual(b.phase('build', rows, rows), {})
        builds = [c for c in self.contexts() if c['stage']=='build']
        self.assertEqual(sorted(len(c['rows']) for c in builds), [2, 6])
        self.assertEqual({tuple(r['パス'] for r in c['rows']) for c in builds}, {tuple(r['パス'] for r in rows[:2]), tuple(r['パス'] for r in rows[2:])})
        self.assertEqual(len(b.state['build_groups']), 2)
        # Review groups still contain up to ten rows.
        self.assertEqual([len(c['rows']) for c in self.contexts() if c['stage']=='build-review'], [8])

    def test_weights_override_and_oversize(self):
        mod = self.load()
        rows = [dict(パス='a.pptx', 作り方='office:t.pptx'), dict(パス='b.eml', 作り方='text')]
        (self.root/'config').mkdir()
        (self.root/'config/limits.json').write_text(json.dumps(dict(max_build_weight=2, build_weights=dict(pptx=9, other=.5))))
        self.assertEqual(mod.build_groups(rows), [[rows[0]], [rows[1]]])
        self.assertEqual(mod.build_weight(rows[0]), 9)
        self.assertEqual(mod.build_weight(rows[1]), .5)
        self.assertEqual(mod.build_groups([rows[1]]*6), [[rows[1]]*4, [rows[1]]*2])
        cases = [('a.xlsx','office:t.xlsx',2), ('a.png','handwriting',2), ('a.pdf','scan:t.pdf',2),
                 ('a.pdf','tex:t.tex',1.5), ('a.docx','office:t.docx',1.5), ('a.pdf','office:a.docx',.5)]
        for path, method, expected in cases:
            self.assertEqual(mod.build_weight(dict(パス=path, 作り方=method)), expected)

    def partial_fake(self):
        fake = fixtures.FAKE.replace("mode=='skip-build-once' and meta['round']==0", "mode=='skip-build-once' and not meta.get('continuation',0)")
        (self.root/'fake-codex').write_text(fake)

    def test_partial_success_continues_in_same_round(self):
        self.partial_fake()
        self.run_batch(mode='skip-build-once')
        builds = [c for c in self.contexts() if c['stage']=='build']
        self.assertEqual([len(c['rows']) for c in builds], [2,1])
        self.assertEqual([c['round'] for c in builds], [0,0])
        self.assertEqual(builds[1]['continuation'], 1)
        self.assertIn('写真生成ができなかった', builds[1]['continuation_output']['Documents/記録.txt'])
        self.assertEqual(builds[1]['reasons'], {})
        self.assertEqual(self.state()['build_retries'], 0)
        self.assertEqual(self.state()['build_history'][0]['failures'], {})
        self.assertEqual(set(self.state()['build_continuations']), {'Documents/記録.txt'})
        self.assertIn('Documents/記録.txt', (self.root/'progress/p01-test/b001.result.md').read_text().split('## 続きを使った行')[1])
        self.assertIn('成形の組 1 / 続きを使った行 1 件', self.cli('status','p01','b001').stdout)

    def test_exhausted_continuations_become_missing(self):
        self.run_batch(mode='skip-build-always')
        builds = [c for c in self.contexts() if c['stage']=='build']
        self.assertEqual([c['round'] for c in builds], [0]*3+[1]*3+[2]*3)
        self.assertEqual([c.get('continuation',0) for c in builds], [0,1,2]*3)
        self.assertIn('planned file missing', self.state()['failures']['Documents/記録.txt'])
        self.assertEqual(len([e for e in self.state()['events'] if e[5]=='plan']), 1)

    def test_strays_preserve_tracked_ignored_and_allowed(self):
        (self.root/'.gitignore').write_text('scratch/\n.venv/\nignored.txt\n')
        self.git('add','.gitignore'); self.git('commit','-qm','Ignore temporary files')
        (self.root/'before.txt').write_text('before')
        fake = fixtures.FAKE.replace("if s=='build':\n", """if s=='build':
 (root/'unexpected.txt').write_text('stray')
 (root/'unexpected-dir').mkdir(exist_ok=True)
 (root/'unexpected-dir/inside').write_text('stray directory')
 (root/'ignored.txt').write_text('ignored')
 (root/'.venv').mkdir(exist_ok=True)
 (root/'config').mkdir(exist_ok=True)
 (root/'config/new.json').write_text('{}')
""")
        (self.root/'fake-codex').write_text(fake)
        tracked = (self.root/'bin/batch').read_bytes()
        self.run_batch()
        self.assertFalse((self.root/'unexpected.txt').exists())
        self.assertFalse((self.root/'unexpected-dir').exists())
        self.assertTrue(list(self.scratch.glob('work/build-*/stray/unexpected.txt')))
        self.assertTrue(list(self.scratch.glob('work/build-*/stray/unexpected-dir/inside')))
        for name in ('before.txt','ignored.txt','.venv','config/new.json'):
            self.assertTrue((self.root/name).exists(), name)
        self.assertEqual((self.root/'bin/batch').read_bytes(), tracked)
        self.assertEqual((self.root/'fake-codex').read_text(), fake)
        report=(self.root/'progress/p01-test/b001.result.md').read_text()
        self.assertIn('repo の直下に作られたファイル：unexpected.txt（build、', report)
        self.assertTrue(any('repo の直下に作られたファイル：unexpected.txt' in p.read_text() for p in self.scratch.glob('logs/build-*.log')))

    def test_parallel_strays_list_all_candidate_executions(self):
        # Both fake processes enter the watcher before either creates the root file.
        fake = fixtures.FAKE.replace("if s=='build':\n", """if s=='build':
 import time
 ready=root/meta['work']/'ready'; ready.write_text('ready')
 deadline=time.monotonic()+5
 while len(list((root/meta['scratch']).glob('work/build-*/ready')))<2:
  if time.monotonic()>deadline: raise RuntimeError('peer did not start')
  time.sleep(.01)
 if str(meta['work']).endswith('0001'): (root/'shared-stray.txt').write_text('shared')
 time.sleep(.1)
""")
        (self.root/'fake-codex').write_text(fake)
        mod=self.load()
        b=mod.Batch(type('Args', (), dict(persona='p01', batch='b001'))())
        with patch.dict(os.environ, self.env | {'CORPUS_MAX_CODEX':'2'}):
            with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
                futures=[pool.submit(b._codex, 'build') for _ in range(2)]
                for future in futures:
                    future.result()
        records=[r for r in b.state['strays'] if r['name']=='shared-stray.txt']
        self.assertEqual(len(records), 1)
        self.assertEqual(len(records[0]['candidates']), 2)
        self.assertTrue((self.root/records[0]['destination']).is_file())

    def test_continuations_use_global_slot_and_resume_budget(self):
        self.partial_fake()
        fake=(self.root/'fake-codex').read_text().replace("if s=='build':\n", """if s=='build':
 import atexit, time
 def event(kind):
  with (scratch/'events.jsonl').open('a') as f: f.write(json.dumps([kind,os.getpid()])+'\\n')
 event('start'); atexit.register(event,'end'); time.sleep(.05)
""")
        (self.root/'fake-codex').write_text(fake)
        (self.root/'config').mkdir()
        (self.root/'config/limits.json').write_text('{"max_build_weight":1}')
        self.env['CORPUS_MAX_CODEX']='1'
        self.run_batch(mode='skip-build-once')
        active=maximum=0
        for kind, _ in map(json.loads, (self.scratch/'events.jsonl').read_text().splitlines()):
            active+=1 if kind=='start' else -1
            maximum=max(active,maximum)
        self.assertEqual((maximum,active), (1,0))
        self.assertEqual(self.state()['build_retries'], 0)
        # A completed production step may need re-entry after an interruption.
        # Keeping its successful-run journal must prevent extra continuations.
        self.run_batch(mode='skip-build-always')  # done is a no-op
        mod=self.load()
        b=mod.Batch(type('Args', (), dict(persona='p01',batch='b001',review_again=False))())
        rows=mod.plan_rows(b.plan)
        first=rows[0]['パス']
        (b.folder/first).unlink()
        b.state['completed'].pop('build-r0-g0')
        b.state['build_runs']['build-r0-g0']['completed']=3
        before=len([c for c in self.contexts() if c['stage']=='build'])
        with patch.dict(os.environ,self.env):
            b.phase('build', rows, rows)
        self.assertEqual(len([c for c in self.contexts() if c['stage']=='build']), before)

    def test_failed_execution_and_mixed_ignored_directory(self):
        (self.root/'.gitignore').write_text('scratch/\nmixed/keep.txt\n')
        self.git('add','.gitignore'); self.git('commit','-qm','Ignore scratch and protected child')
        fake=fixtures.FAKE.replace("if mode=='fail' and s=='build':", """if mode=='fail' and s=='build':
 (root/'mixed').mkdir(exist_ok=True)
 (root/'mixed/keep.txt').write_text('ignored')
 (root/'mixed/move.txt').write_text('stray')
""")
        (self.root/'fake-codex').write_text(fake)
        self.run_batch(mode='fail',ok=False)
        self.assertTrue((self.root/'mixed/keep.txt').is_file())
        self.assertFalse((self.root/'mixed/move.txt').exists())
        self.assertEqual(len(list(self.scratch.glob('work/build-*/stray/mixed/move.txt'))),2)
        self.assertEqual(len(self.state()['strays']),2)


if __name__ == '__main__':
    unittest.main()
