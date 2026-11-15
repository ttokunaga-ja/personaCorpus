"""Behavioral tests; unavailable system dependencies are failures, never skips."""
import concurrent.futures
import getpass
import importlib.machinery
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import sys
from unittest.mock import patch
import unittest

from PIL import Image, ImageDraw, ImageFont
from pypdf import PdfReader, PdfWriter
from reportlab.pdfgen.canvas import Canvas

ROOT = Path(__file__).resolve().parents[1]
loader = importlib.machinery.SourceFileLoader('corpus_check', str(ROOT / 'bin/check'))
spec = importlib.util.spec_from_loader(loader.name, loader)
check = importlib.util.module_from_spec(spec)
loader.exec_module(check)


class MakeTools(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='maketools-')
        self.addCleanup(self.temp.cleanup)
        self.work = Path(self.temp.name)
        self.image = self.work / 'original page.png'
        im = Image.new('RGB', (1240, 1754), 'white')
        draw = ImageDraw.Draw(im)
        font = ImageFont.truetype('DejaVuSans.ttf', 48)
        draw.text((110, 180), 'MONTHLY REPORT 2026', font=font, fill='black')
        draw.text((110, 270), 'Total sales 12345', font=font, fill='black')
        im.save(self.image)
        self.pdf = self.work / 'two pages.pdf'
        canvas = Canvas(str(self.pdf), invariant=1)
        for text in ('First monthly report', 'Second monthly report'):
            canvas.drawString(72, 720, text)
            canvas.showPage()
        canvas.save()

    def require(self, *names):
        missing = [n for n in names if not shutil.which(n)]
        if missing:
            self.fail('\n'.join(f'未実行：{n} なし' for n in missing))

    def run_tool(self, name, *args, ok=True):
        result = subprocess.run([str(ROOT / 'bin' / name), *map(str, args)],
                                capture_output=True, text=True)
        if ok:
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0)
        return result

    def metadata(self, path):
        reader = PdfReader(path)
        metadata = str(reader.metadata) + str(reader.xmp_metadata)
        self.assertNotIn('/home/', metadata)
        self.assertNotIn(getpass.getuser(), metadata)
        self.assertFalse(reader.metadata and reader.metadata.get('/CreationDate'))
        self.assertFalse(reader.metadata and reader.metadata.get('/ModDate'))
        return reader

    def scan(self, source, name='scan.pdf', *extra):
        out = self.work / name
        self.run_tool('scan', source, out, '--seed', 42, *extra)
        reader = self.metadata(out)
        for page in reader.pages:
            self.assertEqual(page.extract_text().strip(), '')
            images = page['/Resources']['/XObject'].get_object()
            self.assertTrue(images)
            for ref in images.values():
                self.assertIn('/DCTDecode', str(ref.get_object()['/Filter']))
        self.assertFalse(check.SECRET.search(out.read_bytes()))
        return out

    def test_scan_image_and_determinism(self):
        first = self.scan(self.image)
        other = self.scan(self.image, 'other.pdf')
        self.assertEqual(first.read_bytes(), other.read_bytes())
        self.assertEqual(len(PdfReader(first).pages), 1)
        gray = self.scan(self.image, 'gray.pdf', '--gray', '--quality', 70)
        obj = next(iter(PdfReader(gray).pages[0]['/Resources']['/XObject'].values())).get_object()
        self.assertEqual(obj['/ColorSpace'], '/DeviceGray')
        changed = self.work / 'changed.pdf'
        self.run_tool('scan', self.image, changed, '--seed', 43)
        self.assertNotEqual(first.read_bytes(), changed.read_bytes())

    def test_scan_pdf(self):
        self.require('pdftoppm')
        out = self.scan(self.pdf)
        self.assertEqual(len(PdfReader(out).pages), 2)
        again = self.scan(self.pdf, 'again.pdf')
        self.assertEqual(out.read_bytes(), again.read_bytes())

    def test_scan_rotated_pdf(self):
        self.require('pdftoppm')
        writer = PdfWriter()
        page = writer.add_blank_page(width=300, height=500)
        page.rotate(90)
        source = self.work / 'rotated.pdf'
        writer.write(source)
        out = self.scan(source)
        page = PdfReader(out).pages[0]
        self.assertEqual((float(page.mediabox.width), float(page.mediabox.height)), (500, 300))

    def test_scan_readable(self):
        self.require('tesseract', 'pdftoppm')
        out = self.scan(self.image)
        prefix = self.work / 'ocr'
        subprocess.run(['pdftoppm', '-singlefile', '-r', '150', '-png', str(out), str(prefix)], check=True)
        result = subprocess.run(['tesseract', str(prefix) + '.png', 'stdout', '-l', 'eng'],
                                check=True, capture_output=True, text=True)
        self.assertIn('MONTHLY REPORT 2026', result.stdout)
        self.assertIn('12345', result.stdout)

    def test_render(self):
        self.require('pdftoppm')
        out = self.work / 'renders'
        self.run_tool('render', self.pdf, out, '--dpi', 72)
        self.assertEqual(sorted(p.name for p in out.iterdir()), ['two pages-1.png', 'two pages-2.png'])
        for path in out.iterdir():
            with Image.open(path) as im:
                self.assertEqual(im.format, 'PNG')
                self.assertEqual(im.width, 596)
        limited = self.work / 'limited'
        self.run_tool('render', self.pdf, limited, '--max-pages', 1)
        self.assertEqual(len(list(limited.iterdir())), 1)
        copied = self.work / 'copied'
        self.run_tool('render', self.image, copied)
        self.assertEqual((copied / self.image.name).read_bytes(), self.image.read_bytes())

    def test_bad_arguments(self):
        bad = self.work / 'bad.txt'
        bad.write_text('not a supported document')
        for tool in ('render', 'tex', 'office-pdf', 'scan'):
            self.run_tool(tool, bad, self.work / 'out', ok=False)
            self.run_tool(tool, self.work / 'missing.pdf', self.work / 'out', ok=False)
            self.run_tool(tool, ok=False)
        for option, value in (('--dpi', 0), ('--max-pages', -1)):
            self.run_tool('render', self.pdf, self.work / 'bad-render', option, value, ok=False)
        for quality in (0, 101):
            self.run_tool('scan', self.image, self.work / 'bad.pdf', '--quality', quality, ok=False)
        self.run_tool('scan', self.image, self.image, ok=False)
        self.run_tool('office-pdf', self.pdf, self.work / 'out.pdf', ok=False)

    def test_office_profile_isolation_and_metadata(self):
        # Exercise our wrapper with a stand-in converter. Real Office export is
        # covered separately and must still fail when soffice is unavailable.
        fakebin = self.work / 'fakebin'
        fakebin.mkdir()
        trace = self.work / 'profiles.jsonl'
        private_pdf = self.work / 'metadata.pdf'
        writer = PdfWriter(clone_from=PdfReader(self.pdf))
        writer.add_metadata({'/Author': '/home/' + getpass.getuser(), '/CreationDate': 'D:20260101000000'})
        writer.xmp_metadata = b'<metadata>/home/private-user</metadata>'
        writer.write(private_pdf)
        fake = fakebin / 'soffice'
        fake.write_text('#!' + sys.executable + '\n' +
                        'import json, pathlib, shutil, sys\n' +
                        'args = sys.argv[1:]\n' +
                        'profile = next(a for a in args if a.startswith("-env:UserInstallation="))\n' +
                        'with open(' + repr(str(trace)) + ', "a") as f: f.write(json.dumps(profile) + "\\n")\n' +
                        'out = pathlib.Path(args[args.index("--outdir") + 1]) / (pathlib.Path(args[-1]).stem + ".pdf")\n' +
                        'shutil.copyfile(' + repr(str(private_pdf)) + ', out)\n')
        fake.chmod(0o755)
        source = self.work / 'document.docx'
        source.write_bytes(b'wrapper test; converter is stubbed')
        def convert(number):
            out = self.work / f'clean-{number}.pdf'
            self.run_tool('office-pdf', source, out)
            self.assertEqual(len(self.metadata(out).pages), 2)
        with patch.dict(os.environ, {'PATH': str(fakebin) + os.pathsep + os.environ['PATH']}):
            with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
                list(pool.map(convert, range(3)))
        profiles = [json.loads(line) for line in trace.read_text().splitlines()]
        self.assertEqual(len(set(profiles)), 3)
        for profile in profiles:
            self.assertIn('file://', profile)

    def test_office(self):
        self.require('soffice', 'pdftoppm')
        from docx import Document
        from openpyxl import Workbook
        from pptx import Presentation
        source = self.work / 'office files'
        source.mkdir()
        doc = Document()
        doc.add_paragraph('First monthly report')
        doc.add_page_break()
        doc.add_paragraph('Second monthly report')
        doc.core_properties.author = '/home/' + getpass.getuser()
        doc.save(source / 'report.docx')
        book = Workbook()
        book.active['A1'] = 'Monthly sales 12345'
        book.save(source / 'report.xlsx')
        slides = Presentation()
        for text in ('First monthly report', 'Second monthly report'):
            slide = slides.slides.add_slide(slides.slide_layouts[0])
            slide.shapes.title.text = text
        slides.save(source / 'report.pptx')

        def convert(ext, count):
            out = self.work / (ext + '.pdf')
            self.run_tool('office-pdf', source / ('report.' + ext), out)
            self.assertEqual(len(self.metadata(out).pages), count)
            self.assertIn('monthly' if ext != 'xlsx' else 'Monthly', PdfReader(out).pages[0].extract_text())
            rendered = self.work / ext
            self.run_tool('render', source / ('report.' + ext), rendered)
            self.assertEqual(len(list(rendered.glob('report-*.png'))), count)
        with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
            list(pool.map(lambda pair: convert(*pair), [('docx', 2), ('xlsx', 1), ('pptx', 2)]))

    def test_tex_english(self):
        self.require('lualatex')
        # A local class and relative input must be discoverable from the temporary build.
        (self.work / 'localreport.cls').write_text('\\ProvidesClass{localreport}\n\\LoadClass{article}\n')
        (self.work / 'body.tex').write_text('Monthly report \\label{monthly}\\newpage Page \\pageref{monthly}')
        source = self.work / 'report.tex'
        source.write_text('\\documentclass{localreport}\\begin{document}\\input{body}\\end{document}')
        out = self.work / 'english.pdf'
        self.run_tool('tex', source, out)
        reader = self.metadata(out)
        self.assertEqual(len(reader.pages), 2)
        self.assertIn('Page 1', reader.pages[1].extract_text())
        self.assertEqual(sorted(p.name for p in self.work.glob('report.*')), ['report.tex'])
        source.write_text('\\documentclass{article}\\begin{document}\\undefinedcommand\\end{document}')
        failure = self.run_tool('tex', source, self.work / 'failure.pdf', ok=False)
        self.assertIn('Undefined control sequence', failure.stderr)
        self.assertFalse((self.work / 'failure.pdf').exists())

    def test_tex_japanese(self):
        self.require('lualatex', 'fc-list')
        fonts = subprocess.run(['fc-list', ':lang=ja'], capture_output=True, text=True, check=True)
        if not fonts.stdout.strip():
            self.fail('未実行：日本語フォント なし')
        source = self.work / 'japanese.tex'
        source.write_text('\\documentclass{article}\\usepackage{luatexja}\\begin{document}日本語の月次報告書です。\\end{document}')
        out = self.work / 'japanese.pdf'
        self.run_tool('tex', source, out)
        reader = self.metadata(out)
        self.assertEqual(len(reader.pages), 1)
        self.assertIn('月次報告書', reader.pages[0].extract_text().replace(' ', ''))

    def test_japanese_fonts_available(self):
        self.require('fc-list')
        result = subprocess.run(['fc-list', ':lang=ja'], capture_output=True, text=True, check=True)
        self.assertTrue(result.stdout.strip(), '未実行：日本語フォント なし')


if __name__ == '__main__':
    unittest.main(verbosity=2)
