"""Disposable source coverage and duplicate-check regressions (no corpus writes)."""
import importlib.machinery
import importlib.util
from email.message import EmailMessage
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

    def test_csv_numbers_come_from_data_rows_not_prose_totals(self):
        manuscript = ('列は受注番号,金額。行の作り方：1 日 214 行、合計 7,520,000 円、有効 7,000,000 円、取消 520,000 円。\n\n'
                      '```csv\n受注番号,金額\nOR-1001,33000\nOR-1002,35000\nOR-1003,40000\nOR-1004,41000\nOR-1005,42000\n```\n')
        built = '受注番号,金額\nOR-1001,33000\nOR-1002,35000\nOR-1003,40000\nOR-1004,41000\nOR-1005,42000\nOR-1006,50000\n'
        self.assertIsNone(check.source_reason(manuscript, built, ext='csv'))
        dropped = built.replace('OR-1002,35000\n', '').replace('OR-1003,40000\n', '')
        self.assertIn('missing source values', check.source_reason(manuscript, dropped, ext='csv'))
        table = '| 受注番号 | 金額 |\n|---|---:|\n| OR-1001 | 33000 |\n| OR-1002 | 35000 |\n| OR-1003 | 40000 |\n| OR-1004 | 41000 |\n| OR-1005 | 42000 |\n\n合計 7,520,000 円'
        self.assertIsNone(check.source_reason(table, built, ext='tsv'))
        self.assertIn('missing source values', check.source_reason('合計 7,520,000 円、有効 7,000,000 円', built, ext='csv'))

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


class EmailHeaders(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.home = Path(self.tmp.name)/'people'; self.home.mkdir()
        self.sources = Path(self.tmp.name)/'sources'; self.sources.mkdir()

    def email(self, headers, body='案件の判断を確認しました。'):
        path = self.home/'message.eml'
        path.write_text('From: sender@example.test\nTo: recipient@example.test\nSubject: 確認\nMIME-Version: 1.0\nContent-Type: text/plain; charset=utf-8\nContent-Transfer-Encoding: 8bit\n'+headers+'\n\n'+body)
        return path

    def test_identifiers_must_match_headers_even_when_present_in_body(self):
        for field in ('Message-ID', 'In-Reply-To', 'References'):
            with self.subTest(field=field):
                expected = 'From: sender@example.test\n'+field+': <fixed@example.test>\n\n案件の判断を確認しました。'
                path = self.email(field+': <changed@example.test>', expected)
                (self.sources/'message.eml.md').write_text('---\nkind: text\n---\n'+expected)
                issues = check.inspect((path, self.home, {}, None, self.sources))[1]
                self.assertIn('email-headers', issues)
                self.assertIn(field, issues['email-headers'])

    def test_missing_identifier_is_rejected(self):
        path = self.email('Message-ID: <own@example.test>')
        reason = check.email_source_reason('From: sender@example.test\nIn-Reply-To: <parent@example.test>\n\n本文', path)
        self.assertIn('In-Reply-To', reason)

    def test_folded_references_and_fixed_ids_are_preserved(self):
        path = self.email('Message-ID: <study-postpone-260127@example.test>\nReferences: <first@example.test>\n <second@example.test>')
        expected = 'From: sender@example.test  \nMessage-ID: <study-postpone-260127@example.test>  \nReferences: <first@example.test> <second@example.test>\n\n本文'
        self.assertIsNone(check.email_source_reason(expected, path))

    def test_references_order_is_significant(self):
        path = self.email('References: <second@example.test> <first@example.test>')
        self.assertIn('References', check.email_source_reason('References: <first@example.test> <second@example.test>\n\n本文', path))

    def test_body_quotes_and_absent_reference_notes_are_not_headers(self):
        path = self.email('Message-ID: <own@example.test>')
        for expected in ('From: sender@example.test\nCc: なし\nIn-Reply-To: なし\nReferences: なし\n\nMessage-ID: <quoted@example.test>',
                         '本文の説明\n\nMessage-ID: <quoted@example.test>'):
            with self.subTest(expected=expected):
                self.assertIsNone(check.email_source_reason(expected, path))


class EmailAttachments(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.home = Path(self.tmp.name)/'people'; self.home.mkdir()
        self.sources = Path(self.tmp.name)/'sources'; self.sources.mkdir()

    def issues(self, manuscript, attachments=(), forwarded=False, attached_email=False):
        message = EmailMessage()
        message['From'] = 'sender@example.test'
        message['To'] = 'recipient@example.test'
        message['Subject'] = '資料の送付'
        message.set_content(manuscript)
        for filename, payload in attachments:
            message.add_attachment(payload, maintype='application', subtype='octet-stream', filename=filename)
        if forwarded:
            nested = EmailMessage()
            nested.set_content('前のメール')
            nested.add_attachment(b'old report', maintype='application', subtype='pdf', filename='report.pdf')
            if attached_email:
                message.add_attachment(nested, filename='forwarded.eml')
            else:
                message.add_attachment(nested)
        path = self.home/'message.eml'; path.write_bytes(message.as_bytes())
        (self.sources/'message.eml.md').write_text('---\nkind: text\n---\n'+manuscript)
        return check.inspect((path, self.home, {}, None, self.sources))[1]

    def test_filename_in_body_cannot_substitute_for_attachment(self):
        issues = self.issues('資料を送ります。\n添付: report.pdf')
        self.assertNotIn('source', issues)
        self.assertIn('email-attachments', issues)
        self.assertIn('report.pdf', issues['email-attachments'])

    def test_each_declared_attachment_is_required(self):
        issues = self.issues('添付：report.pdf、明細.xlsx', [('report.pdf', b'PDF')])
        self.assertIn('明細.xlsx', issues['email-attachments'])

    def test_empty_or_wrong_name_is_rejected(self):
        for attachments in ([('report.pdf', b'')], [('other.pdf', b'PDF')]):
            with self.subTest(attachments=attachments):
                self.assertIn('email-attachments', self.issues('添付: report.pdf', attachments))

    def test_paths_multiline_and_names_with_spaces(self):
        for manuscript in ('添付: Documents/報告.pdf', '添付（実ファイル）：\n- Documents/報告.pdf',
                           'Attachments:\n- `Monthly Financial Review.pdf`'):
            filename = 'Monthly Financial Review.pdf' if 'Monthly' in manuscript else '報告.pdf'
            with self.subTest(manuscript=manuscript):
                self.assertNotIn('email-attachments', self.issues(manuscript, [(filename, b'contents')]))
                self.assertIn('email-attachments', self.issues(manuscript))

    def test_quoted_and_prose_mentions_are_not_declarations(self):
        for manuscript in ('添付: なし\n> 添付: report.pdf', 'Attachment: none\n> Attachment: report.pdf',
                           'report.pdf を共有フォルダで参照してください。',
                           '添付: なし\n\n- report.pdf',
                           'Attachments: first.pdf, second.pdf',
                           '添付: report.pdf は前便で送付済みです。'):
            with self.subTest(manuscript=manuscript):
                self.assertNotIn('email-attachments', self.issues(manuscript))

    def test_forwarded_email_attachment_does_not_satisfy_current_message(self):
        self.assertIn('email-attachments', self.issues('添付: report.pdf', forwarded=True))

    def test_attached_email_itself_is_a_valid_attachment(self):
        self.assertNotIn('email-attachments', self.issues('添付: forwarded.eml', forwarded=True, attached_email=True))


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
