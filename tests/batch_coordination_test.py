"""Admission and repository locks; no production jobs or Git mutations."""
import importlib.machinery
import importlib.util
import os
from pathlib import Path
import tempfile
import subprocess
import sys
from types import SimpleNamespace
import unittest
from unittest.mock import patch

class CoordinationTest(unittest.TestCase):
    def setUp(self):
        loader = importlib.machinery.SourceFileLoader('coordination', str(Path(__file__).resolve().parents[1] / 'bin/batch'))
        self.mod = importlib.util.module_from_spec(importlib.util.spec_from_loader(loader.name, loader))
        loader.exec_module(self.mod)
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.mod.ROOT = Path(temp.name)
        self.batch = SimpleNamespace(persona='p01-test', batch='b003')

    def accepted(self):
        path = self.mod.ROOT / 'progress/batches.tsv'
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('persona\tbatch\tstatus\np01-test\tb001\taccepted\n')
        self.mod.command(['git', 'init', '-q'])
        self.mod.command(['git', 'add', 'progress/batches.tsv'])
        self.mod.command(['git', '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-qm', 'accepted fixture'])

    def test_live_job_states_consume_capacity(self):
        jobs = '\n'.join(f'personaCorpus\tp01-b00{i}\t{s}\tdetail' for i, s in enumerate(('failed', 'done', 'stalled', 'limit', 'stdin-wait'), 1))
        self.assertEqual(self.mod.job_batches('p01-test', jobs), {'b003', 'b004', 'b005'})

    def test_second_batch_needs_accepted_predecessor(self):
        with self.mod.batch_lock('p01-test', 'b002'):
            with self.assertRaisesRegex(ValueError, '受入済み'):
                self.mod.admit_batch(self.batch)
            self.accepted()
            self.mod.admit_batch(self.batch)

    def test_configured_persona_limit(self):
        self.accepted()
        with self.mod.batch_lock('p01-test', 'b002'), patch.dict(os.environ, {'CORPUS_MAX_BATCHES_PER_PERSONA': '1'}):
            with self.assertRaisesRegex(ValueError, '上限'):
                self.mod.admit_batch(self.batch)

    def test_uncommitted_acceptance_does_not_enable_parallel_batches(self):
        path = self.mod.ROOT / 'progress/batches.tsv'
        path.parent.mkdir(parents=True)
        path.write_text('persona\tbatch\tstatus\np01-test\tb001\taccepted\n')
        with self.mod.batch_lock('p01-test', 'b002'):
            with self.assertRaisesRegex(ValueError, '受入済み'):
                self.mod.admit_batch(self.batch)

    def test_default_rejects_fourth_batch(self):
        self.accepted()
        self.batch.batch = 'b005'
        jobs = '\n'.join(f'personaCorpus\tp01-b00{i}\trunning\tactive' for i in (2, 3, 4))
        with patch.dict(os.environ, {}, clear=True):
            with self.assertRaisesRegex(ValueError, '上限: 3'):
                self.mod.admit_batch(self.batch, jobs)

    def serialized_children(self, operation):
        script = '''
import runpy, sys, time
from pathlib import Path
m = runpy.run_path(sys.argv[1])
m['Batch'].__init__.__globals__['ROOT'] = Path(sys.argv[2])
events = Path(sys.argv[2]) / 'events'
def action(self):
    with events.open('a') as f: f.write(self.batch + ':enter\\n')
    time.sleep(.15)
    with events.open('a') as f: f.write(self.batch + ':exit\\n')
if sys.argv[3] == 'finish':
    cls = m['Batch']; cls._finish = lambda self, discard=False: action(self)
    b = object.__new__(cls); b.persona = sys.argv[4]; b.batch = sys.argv[5]
    b.finish()
else:
    with m['persona_lock']('p01-test', wait=True):
        b = type('Fake', (), {'batch': sys.argv[5]})()
        action(b)
'''
        source = Path(__file__).resolve().parents[1] / 'bin/batch'
        processes = [subprocess.Popen([sys.executable, '-c', script, str(source), str(self.mod.ROOT), operation, f'p0{i}-test', f'b00{i}'], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                     for i in (1, 2)]
        try:
            for process in processes:
                stdout, stderr = process.communicate(timeout=10)
                self.assertEqual(process.returncode, 0, stdout + stderr)
        finally:
            for process in processes:
                if process.poll() is None:
                    process.kill()
                    process.wait()
        lines = (self.mod.ROOT / 'events').read_text().splitlines()
        self.assertEqual(len(lines), 4)
        self.assertEqual(lines[0].replace(':enter', ':exit'), lines[1])
        self.assertEqual(lines[2].replace(':enter', ':exit'), lines[3])

    def test_concurrent_accepts_serialize_repository_operations(self):
        self.serialized_children('finish')

    def test_same_persona_planning_stages_serialize(self):
        self.serialized_children('planning')

    def test_duplicate_batch_lock_and_reentrant_records(self):
        with self.mod.batch_lock('p01-test', 'b003'):
            with self.assertRaisesRegex(ValueError, '同じ batch'):
                with self.mod.batch_lock('p01-test', 'b003'):
                    pass
        with self.mod.repository_lock(), self.mod.repository_lock():
            self.assertIn('records', self.mod._repository_locks.held)
        self.assertFalse(self.mod._repository_locks.held)

if __name__ == '__main__':
    unittest.main()
