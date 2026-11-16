"""Disposable source coverage and duplicate-check regressions (no corpus writes)."""
import importlib.machinery
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import zipfile

BASE = Path(__file__).resolve().parents[1]
loader = importlib.machinery.SourceFileLoader('check_regression', str(BASE/'bin/check'))
spec = importlib.util.spec_from_loader(loader.name, loader)
check = importlib.util.module_from_spec(spec)
loader.exec_module(check)


class Coverage(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.base = Path(self.tmp.name)
        self.home = self.base/'people'; self.home.mkdir()
        self.sources = self.base/'sources'; self.sources.mkdir()

    def docx(self, markup, comments=''):
        path = self.home/'review.docx'
        with zipfile.ZipFile(path, 'w') as archive:
            archive.writestr('[Content_Types].xml', '<Types/>')
            archive.writestr('_rels/.rels', '<Relationships/>')
            archive.writestr('word/document.xml', '<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>'+markup+'</w:body></w:document>')
            if comments:
                archive.writestr('word/comments.xml', '<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:comment><w:p><w:r><w:t>'+comments+'</w:t></w:r></w:p></w:comment></w:comments>')
        return path

    def test_ocr_threshold_has_pass_and_fail_boundaries(self):
        expected = 'alpha beta gamma delta epsilon zeta eta theta iota kappa'
        self.assertIsNone(check.source_reason(expected, 'alpha beta gamma', from_ocr=True))
        self.assertIn('< 0.3', check.source_reason(expected, 'alpha beta', from_ocr=True))
        self.assertIsNotNone(check.source_reason(expected, 'alpha beta gamma'))

    def test_non_ocr_derived_retains_half_coverage_and_numeric_tolerance(self):
        expected = 'alpha beta gamma delta epsilon zeta eta theta iota 9876'
        self.assertIsNone(check.source_reason(expected, 'alpha beta gamma delta epsilon', derived=True))
        self.assertIn('< 0.5', check.source_reason(expected, 'alpha beta gamma delta', derived=True))
        self.assertIn('< 0.6', check.source_reason(expected, 'alpha beta gamma delta epsilon', derived=True, reviewed=True, ext='docx'))
        self.assertIsNone(check.source_reason(expected, 'alpha beta gamma delta epsilon zeta', derived=True, reviewed=True, ext='docx'))
        self.assertIsNone(check.source_reason(expected, 'alpha beta gamma', derived=True, from_ocr=True))
        (self.sources/'base.txt.md').write_text('---\nkind: text\n---\n'+expected)
        (self.sources/'derived.txt.md').write_text('---\nkind: text\nderived_from: base.txt\n---\n')
        path = self.home/'derived.txt'
        for text, passes in [('alpha beta gamma delta', False), ('alpha beta gamma delta epsilon', True)]:
            path.write_text(text)
            issues = check.inspect((path, self.home, {}, None, self.sources))[1]
            self.assertEqual('source' not in issues, passes, issues)

    def test_deleted_text_and_comments_are_extracted(self):
        path = self.docx('<w:p><w:del><w:r><w:delText>original decision</w:delText></w:r></w:del><w:ins><w:r><w:t>revised decision</w:t></w:r></w:ins></w:p>', '確認コメント')
        text, _, _ = check.extract(path, 'docx')
        self.assertIn('original decision', text)
        self.assertIn('revised decision', text)
        self.assertIn('確認コメント', text)

    def test_review_docx_coverage_threshold(self):
        expected = 'alpha beta gamma delta epsilon zeta eta theta iota kappa'
        for markup, comments, passes in [
            ('<w:p><w:del><w:r><w:delText>alpha beta gamma delta epsilon zeta</w:delText></w:r></w:del></w:p>', '', True),
            ('<w:p><w:r><w:t>alpha beta gamma delta epsilon zeta</w:t></w:r></w:p>', 'review', True),
            ('<w:p><w:r><w:t>alpha beta gamma delta epsilon zeta</w:t></w:r></w:p>', '', False),
            ('<w:p><w:ins><w:r><w:t>alpha beta gamma delta epsilon</w:t></w:r></w:ins></w:p>', '', False),
        ]:
            with self.subTest(markup=markup, comments=comments):
                path = self.docx(markup, comments)
                (self.sources/'review.docx.md').write_text('---\nkind: office:review.docx\n---\n'+expected)
                issues = check.inspect((path, self.home, {}, None, self.sources))[1]
                self.assertEqual('source' not in issues, passes, issues)

    def test_visual_kinds_require_source_without_token_comparison(self):
        for kind in ('photo', 'handwriting', 'scan:handwriting', 'scan:handwriting:sheet'):
            with self.subTest(kind=kind):
                path = self.home/'paper.pdf'; path.write_bytes(b'PDF')
                source = self.sources/'paper.pdf.md'
                source.write_text('---\nkind: '+kind+'\n---\nVisible handwritten marks')
                with patch.object(check, 'extract', return_value=('unrelated OCR with sufficient characters', True, False)), patch.object(check, 'metadata', return_value=None):
                    self.assertNotIn('source', check.inspect((path, self.home, {}, 'eng', self.sources))[1])
                    source.unlink()
                    self.assertIn('source', check.inspect((path, self.home, {}, 'eng', self.sources))[1])
        source.write_text('---\nkind: scan:tex:sheet.cls\n---\nalpha beta gamma delta epsilon')
        with patch.object(check, 'extract', return_value=('unrelated OCR with sufficient characters', True, False)), patch.object(check, 'metadata', return_value=None):
            self.assertIn('source', check.inspect((path, self.home, {}, 'eng', self.sources))[1])


class Duplicate(unittest.TestCase):
    def result(self, path, grams, digest=None):
        return (path, {}, digest, frozenset(grams), [])

    def test_unrelated_pair_reports_the_other_path(self):
        results = [self.result('before.txt', range(20)), self.result('after.txt', range(19))]
        check.duplicates(results)
        self.assertTrue(results[0][1]['duplicate'].endswith('after.txt'))
        self.assertTrue(results[1][1]['duplicate'].endswith('before.txt'))

    def test_related_pairs_exempt_near_but_never_exact_bytes(self):
        for grams in (range(19), range(20)):
            results = [self.result('before.txt', range(20)), self.result('after.txt', grams)]
            check.duplicates(results, {'after.txt': {'before.txt', 'other.txt'}})
            self.assertFalse(any(r[1] for r in results))
        results = [self.result('before.txt', range(20), b'same'), self.result('after.txt', range(20), b'same')]
        check.duplicates(results, {'after.txt': {'before.txt'}})
        self.assertTrue(all('identical bytes' in r[1]['duplicate'] for r in results))

    def test_self_is_not_a_duplicate(self):
        results = [self.result('only.txt', range(20))]
        check.duplicates(results)
        self.assertEqual(results[0][1], {})

    def test_plan_relations_are_read_from_all_batches(self):
        with tempfile.TemporaryDirectory() as tmp:
            folder = Path(tmp)
            for batch, row in [('b001', '| before.txt | 2026-01-01 | 破棄：previous check | after.txt；third.txt | fourth.txt |'), ('b002', '| after.txt | 2026-01-02 | Text | | |')]:
                (folder/(batch+'.md')).write_text('| パス | 日付 | 内容 | 関連 | 先に要るもの |\n|---|---|---|---|---|\n'+row+'\n')
            relations = check.plan_relations(folder)
            self.assertEqual(relations['before.txt'], {'after.txt', 'third.txt', 'fourth.txt'})
            results = [self.result('before.txt', range(20)), self.result('after.txt', range(19))]
            check.duplicates(results, relations)
            self.assertFalse(any(r[1] for r in results))


if __name__ == '__main__':
    unittest.main()
