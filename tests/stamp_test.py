"""Stamp changes document properties without changing document content."""
import datetime as dt
import importlib.machinery
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest
import zipfile
from xml.dom import minidom

from docx import Document
from openpyxl import Workbook
from pptx import Presentation
from PIL import Image, PngImagePlugin
from pypdf import PdfReader, PdfWriter
from reportlab.pdfgen.canvas import Canvas

ROOT = Path(__file__).resolve().parents[1]
loader = importlib.machinery.SourceFileLoader('stamp_check', str(ROOT / 'bin/check'))
spec = importlib.util.spec_from_loader(loader.name, loader)
check = importlib.util.module_from_spec(spec)
loader.exec_module(check)
DATE = '2026-03-06T18:42:10+09:00'
EPOCH = dt.datetime.fromisoformat(DATE).timestamp()

class Stamp(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='stamp-')
        self.addCleanup(self.tmp.cleanup)
        self.work = Path(self.tmp.name)

    def stamp(self, path, *extra):
        result = subprocess.run([str(ROOT / 'bin/stamp'), str(path), '--author', '山田 花子',
                                 '--date', DATE, *extra], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)

    def clean(self, path):
        self.assertIsNone(check.metadata(path, path.suffix[1:], {'period': '2026-01-01..2026-12-31', 'timezone':'Asia/Tokyo'}, EPOCH))

    def deterministic(self, path, *extra):
        original = path.read_bytes()
        self.stamp(path, *extra)
        first = path.read_bytes()
        path.write_bytes(original)
        self.stamp(path, *extra)
        self.assertEqual(path.read_bytes(), first)
        self.clean(path)

    def test_office(self):
        doc = Document(); doc.add_paragraph('Monthly sales 12345')
        doc.core_properties.comments = 'generated using python-docx'
        doc.save(self.work / 'report.docx')
        book = Workbook(); book.active['A1'] = 'Monthly sales 12345'; book.save(self.work / 'report.xlsx')
        slides = Presentation(); slides.core_properties.author = 'python-pptx'; slide = slides.slides.add_slide(slides.slide_layouts[0]); slide.shapes.title.text = 'Monthly sales 12345'; slides.save(self.work / 'report.pptx')
        for ext, application in [('docx','Microsoft Office Word'),('xlsx','Microsoft Excel'),('pptx','Microsoft Office PowerPoint')]:
            with self.subTest(ext=ext):
                path = self.work / ('report.'+ext)
                before = check.extract(path, ext)
                self.assertIsNotNone(check.metadata(path, ext, {'timezone':'Asia/Tokyo'}, EPOCH))
                self.deterministic(path)
                self.assertEqual(check.extract(path, ext), before)
                with zipfile.ZipFile(path) as archive:
                    core = minidom.parseString(archive.read('docProps/core.xml'))
                    values = {n.localName:n.firstChild.data if n.firstChild else '' for n in core.documentElement.childNodes if n.nodeType == n.ELEMENT_NODE}
                    self.assertEqual(values['creator'], '山田 花子')
                    self.assertEqual(values['lastModifiedBy'], '山田 花子')
                    self.assertEqual(values['created'], '2026-03-06T09:42:10Z')
                    self.assertEqual(values['modified'], values['created'])
                    self.assertNotIn('python', archive.read('docProps/core.xml').decode())
                    self.assertIn(application, archive.read('docProps/app.xml').decode())
                    self.assertTrue(all(i.date_time == (2026,3,6,18,42,10) for i in archive.infolist()))

    def test_pdf(self):
        path = self.work / 'report.pdf'
        c = Canvas(str(path)); c.drawString(72,720,'Monthly sales 12345'); c.save()
        before = [p.extract_text() for p in PdfReader(path).pages]
        self.assertIsNotNone(check.metadata(path,'pdf',{'timezone':'Asia/Tokyo'},EPOCH))
        self.deterministic(path, '--producer', 'Microsoft® Word for Microsoft 365')
        reader = PdfReader(path)
        self.assertEqual([p.extract_text() for p in reader.pages], before)
        self.assertEqual(reader.metadata.author,'山田 花子')
        self.assertEqual(reader.metadata.creation_date.timestamp(), EPOCH)
        self.assertEqual(reader.xmp_metadata.dc_creator,['山田 花子'])
        self.assertEqual(reader.xmp_metadata.xmp_create_date.replace(tzinfo=dt.timezone.utc).timestamp(), EPOCH)
        self.assertEqual(reader.metadata.producer,'Microsoft® Word for Microsoft 365')

    def test_images_keep_pixels(self):
        for ext in ('png','jpg'):
            path = self.work / ('photo.'+ext)
            image = Image.new('RGB',(12,12),'red')
            exif = Image.Exif(); exif[305]='Pillow'; exif[306]='2026:11:16 12:00:00'; exif[274]=6
            if ext=='png':
                info=PngImagePlugin.PngInfo(); info.add_text('Software','matplotlib'); image.save(path,pnginfo=info,exif=exif)
            else: image.save(path,exif=exif)
            with Image.open(path) as im: pixels=im.tobytes()
            self.assertIsNotNone(check.metadata(path,ext,{'timezone':'Asia/Tokyo'},EPOCH))
            self.deterministic(path)
            with Image.open(path) as im:
                self.assertEqual(im.tobytes(),pixels)
                self.assertEqual(im.getexif()[274],6)

    def test_revision_dates_and_timezone(self):
        path = self.work/'history.docx'
        doc = Document(); doc.add_paragraph('Monthly report'); doc.save(path)
        with zipfile.ZipFile(path) as source:
            entries = {i.filename:source.read(i) for i in source.infolist()}
        entries['word/comments.xml'] = b'<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:comment w:id="0" w:author="Reviewer" w:date="2026-01-01T08:00:00Z"/><w:comment w:id="1" w:author="Editor" w:date="2026-11-16T08:00:00Z"/></w:comments>'
        entries['word/document.xml'] = entries['word/document.xml'].replace(b'<w:body>', b'<w:body><w:ins w:id="1" w:author="Editor" w:date="2026-11-16T08:00:00Z"/>')
        with zipfile.ZipFile(path,'w') as target:
            for name,data in entries.items(): target.writestr(name,data)
        result = subprocess.run([str(ROOT/'bin/stamp'),str(path),'--author','山田 花子',
                                 '--date','2026-03-06 18:42:10','--timezone','Asia/Tokyo'],capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stderr)
        self.clean(path)
        with zipfile.ZipFile(path) as archive:
            text=archive.read('word/comments.xml').decode()
            self.assertIn('2026-01-01T08:00:00Z',text)
            self.assertIn('2026-03-06T09:42:10Z',text)
            self.assertIn('Reviewer',text)
            self.assertNotIn('2026-11-16',archive.read('word/document.xml').decode())

    def test_preserve_xmp_and_office_nonmetadata(self):
        path=self.work/'metadata.pdf'
        writer=PdfWriter(); writer.add_blank_page(width=100,height=100)
        writer.xmp_metadata=b'<x:xmpmeta xmlns:x="adobe:ns:meta/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:xmp="http://ns.adobe.com/xap/1.0/"><rdf:RDF><rdf:Description dc:title="Monthly report" xmp:CreatorTool="ReportLab"><dc:description>Important context</dc:description></rdf:Description></rdf:RDF></x:xmpmeta>'
        writer.write(path)
        self.deterministic(path)
        raw=PdfReader(path).trailer['/Root']['/Metadata'].get_object().get_data().decode()
        self.assertIn('Monthly report',raw)
        self.assertIn('Important context',raw)
        self.assertNotIn('ReportLab',raw)
        path=self.work/'comments.xlsx'
        from openpyxl.comments import Comment
        book=Workbook(); book.active['A1']='Monthly report'
        book.active['A1'].comment=Comment('Review this number','openpyxl')
        book.save(path)
        self.deterministic(path)
        from openpyxl import load_workbook
        self.assertEqual(load_workbook(path).active['A1'].comment.author,'山田 花子')
        path=self.work/'shapes.pptx'
        slides=Presentation(); slide=slides.slides.add_slide(slides.slide_layouts[0]); slide.shapes.title.text='Monthly report'
        slide.shapes.title.name='Python library discussion'
        slides.save(path)
        self.deterministic(path)
        self.assertEqual(Presentation(path).slides[0].shapes.title.name,'Python library discussion')

    def test_real_pdf_converters(self):
        # These wrappers already remove metadata; dirty a copy to test repair too.
        office = self.work/'document.docx'
        doc=Document(); doc.add_paragraph('Monthly sales 12345'); doc.save(office)
        tex = self.work/'document.tex'
        tex.write_text(r'\documentclass{article}\begin{document}Monthly sales 12345\end{document}')
        image=self.work/'document.png'; Image.new('RGB',(100,100),'white').save(image)
        for tool,source in [('office-pdf',office),('tex',tex),('scan',image)]:
            with self.subTest(tool=tool):
                path=self.work/(tool+'.pdf')
                result=subprocess.run([str(ROOT/'bin'/tool),str(source),str(path)],capture_output=True,text=True)
                self.assertEqual(result.returncode,0,result.stderr)
                before=[p.extract_text() for p in PdfReader(path).pages]
                self.stamp(path)
                self.clean(path)
                self.assertEqual([p.extract_text() for p in PdfReader(path).pages],before)
                writer=PdfWriter(clone_from=PdfReader(path))
                writer.add_metadata({'/Author':'python','/Producer':'reportlab','/CreationDate':'D:20261116080000Z'})
                writer.write(path)
                self.assertIsNotNone(check.metadata(path,'pdf',{'timezone':'Asia/Tokyo'},EPOCH))
                self.deterministic(path)
                self.assertEqual([p.extract_text() for p in PdfReader(path).pages],before)
                if tool=='scan': self.assertEqual(PdfReader(path).metadata.producer,'Canon iR-ADV C3830')

    def test_zip_and_unchanged_text(self):
        path=self.work/'bundle.zip'
        with zipfile.ZipFile(path,'w') as archive: archive.writestr('memo.txt','Monthly sales 12345')
        self.deterministic(path)
        with zipfile.ZipFile(path) as archive:
            self.assertEqual(archive.read('memo.txt'),b'Monthly sales 12345')
            self.assertEqual(archive.getinfo('memo.txt').date_time,(2026,3,6,18,42,10))
        path=self.work/'memo.txt'; path.write_text('Monthly sales 12345')
        self.deterministic(path)
        self.assertEqual(path.read_text(),'Monthly sales 12345')

if __name__=='__main__': unittest.main(verbosity=2)
