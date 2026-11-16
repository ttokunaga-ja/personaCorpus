"""Formatting regressions; all fixtures are disposable local files."""
import importlib.machinery
import importlib.util
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import xml.etree.ElementTree as ET
import zipfile

from openpyxl import Workbook, load_workbook
from openpyxl.styles import Font, PatternFill, Border, Side, Alignment
from pptx import Presentation

ROOT = Path(__file__).resolve().parents[1]
NS = '{http://schemas.openxmlformats.org/spreadsheetml/2006/main}'


def module(name):
    loader = importlib.machinery.SourceFileLoader('hardening_' + name, str(ROOT / 'bin' / name))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    result = importlib.util.module_from_spec(spec)
    loader.exec_module(result)
    return result


check = module('check')


class Hardening(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='hardening2-')
        self.addCleanup(self.tmp.cleanup)
        self.work = Path(self.tmp.name)

    def test_source_normalization(self):
        for heading in ('行', '#', 'No.'):
            body = f'| {heading} | 項目 | 差額 |\n|:---:|---|---:|\n| 7001 | 売上 | `=E7-D7` |\n| 7002 | 費用 | =SUM(E8:D8) |'
            self.assertIsNone(check.source_reason(body, '項目 差額 売上 費用', ext='xlsx'))
        # An ordinary business column must not disappear, even with a matching heading.
        self.assertIn('7654', check.source_reason('| 行 | 金額 |\n|---|---|\n| 7654円 | 12345 |', '金額 12345', ext='xlsx'))
        self.assertIn('12345', check.source_reason('| No. | 金額 |\n|---|---|\n| 1 | 12345 |', '金額', ext='xlsx'))
        self.assertIn('9876', check.source_reason('合計9876、式 `=SUM(E1:E2)`', '合計', ext='xlsx'))

    def test_thresholds(self):
        body = 'alpha bravo charlie delta echo foxtrot golf hotel india juliet'
        for ext, minimum in [('docx',8),('pdf',8),('txt',8),('md',8),('log',8),('eml',8),('html',8),('pptx',7),('xlsx',6),('csv',6),('tsv',6)]:
            with self.subTest(ext=ext):
                self.assertIsNone(check.source_reason(body, ' '.join(body.split()[:minimum]), ext=ext))
                self.assertIsNotNone(check.source_reason(body, ' '.join(body.split()[:minimum-1]), ext=ext))
        self.assertIsNone(check.source_reason(body, ' '.join(body.split()[:3]), from_ocr=True))
        self.assertIsNotNone(check.source_reason(body, ' '.join(body.split()[:2]), from_ocr=True))

    def test_notes_extracted(self):
        p = self.work/'deck.pptx'
        deck = Presentation(); slide = deck.slides.add_slide(deck.slide_layouts[0])
        slide.shapes.title.text = '決算会議'
        slide.notes_slide.notes_text_frame.text = '発表者だけの説明 金額12400000'
        deck.save(p)
        text = check.extract(p, 'pptx')[0]
        self.assertIn('発表者だけの説明 金額12400000', text)
        body = '決算会議 発表者だけの説明 金額12400000'
        self.assertIsNone(check.source_reason(body,text,ext='pptx'))
        slide.notes_slide.notes_text_frame.text = ''; deck.save(p)
        self.assertIsNotNone(check.source_reason(body,check.extract(p,'pptx')[0],ext='pptx'))

    def test_formula_cache(self):
        p = self.work/'book.xlsx'; book = Workbook(); book.active['A1'] = '=1+1'; book.save(p)
        def issues():
            return check.inspect((p,self.work,{},None,self.work/'sources'))[1]
        self.assertIn('formula', issues())
        raw = p.read_bytes()
        for value, kind, fails in [('2','n',False),('0','n',False),('','str',False),('','n',True),('false','b',False), *[(e,'e',True) for e in ('#REF!','#DIV/0!','#VALUE!','#NAME?','#N/A')]]:
            import io
            output = io.BytesIO()
            with zipfile.ZipFile(io.BytesIO(raw)) as old, zipfile.ZipFile(output, 'w') as new:
                for item in old.infolist():
                    data = old.read(item)
                    if item.filename == 'xl/worksheets/sheet1.xml':
                        tree = ET.fromstring(data); cell = tree.find('.//'+NS+'c'); cell.set('t',kind)
                        cell.find(NS+'v').text = value; data = ET.tostring(tree)
                    new.writestr(item,data)
            p.write_bytes(output.getvalue())
            self.assertEqual('formula' in issues(), fails, (value,issues()))

    def test_recalc_preserves_workbook(self):
        p = self.work/'計算.xlsx'; book = Workbook(); sheet = book.active; sheet.title = '資金繰り'
        sheet['A1'] = 12400000; sheet['B1'] = '=SUM(A1:A2)'; sheet['A2'] = 0
        sheet['B2'] = '=IF(A2=0,"",A1/A2)'
        sheet['B1'].font = Font(name='Arial',bold=True,color='FF125678')
        sheet['B1'].fill = PatternFill('solid',fgColor='FFABCDEF')
        sheet['B1'].border = Border(bottom=Side(style='thin',color='FF123456'))
        sheet['B1'].alignment = Alignment(horizontal='right'); sheet['B1'].number_format = '#,##0;[Red]-#,##0'
        sheet.column_dimensions['B'].width = 27.5; sheet.row_dimensions[1].height = 26
        sheet.freeze_panes = 'B2'; sheet.print_area = 'A1:B2'; sheet.print_title_rows = '1:1'
        sheet.page_setup.orientation = 'landscape'; sheet.page_setup.paperSize = sheet.PAPERSIZE_A4
        sheet.page_margins.left = .3
        book.create_sheet('補足')['A1'] = '=\'資金繰り\'!B1'
        book.save(p)
        with zipfile.ZipFile(p) as z: before = {n:z.read(n) for n in z.namelist()}
        result = subprocess.run([str(ROOT/'bin/recalc'),str(p)],capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stdout+result.stderr)
        self.assertEqual(load_workbook(p,data_only=False).active['B1'].value, '=SUM(A1:A2)')
        values = load_workbook(p,data_only=True)
        self.assertEqual(values.active['B1'].value,12400000)
        self.assertEqual(values['補足']['A1'].value,12400000)
        self.assertIsNone(check.formula_reason(p))
        # Every original XML attribute and node except cached values is identical.
        with zipfile.ZipFile(p) as z:
            for n, raw in before.items():
                if n.startswith('xl/worksheets/') and n.endswith('.xml'):
                    a,b = ET.fromstring(raw), ET.fromstring(z.read(n))
                    for tree in (a,b):
                        for cell in tree.iter(NS+'c'):
                            if cell.find(NS+'f') is not None:
                                cell.attrib.pop('t',None)
                                for child in list(cell):
                                    if child.tag in (NS+'v',NS+'is'):cell.remove(child)
                    self.assertEqual(ET.tostring(a),ET.tostring(b),n)
                else:self.assertEqual(raw,z.read(n),n)
        # Errors are cached for inspection, never treated as successful formulas.
        book = Workbook(); book.active['A1'] = '=1/0'; book.save(p)
        result = subprocess.run([str(ROOT/'bin/recalc'),str(p)],capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertIn('#DIV/0!',check.formula_reason(p))

    def test_recalc_refreshes_stale_manual_cache(self):
        import io
        p=self.work/'stale.xlsx'; book=Workbook(); book.active['A1']=5; book.active['B1']='=A1*2'
        book.calculation.calcMode='manual'; book.save(p)
        raw=p.read_bytes()
        with zipfile.ZipFile(io.BytesIO(raw)) as old, zipfile.ZipFile(p,'w') as new:
            for item in old.infolist():
                data=old.read(item)
                if item.filename=='xl/worksheets/sheet1.xml':
                    data=data.replace(b'<f>A1*2</f><v></v>',b'<f>A1*2</f><v>999</v>')
                new.writestr(item,data)
        self.assertEqual(load_workbook(p,data_only=True).active['B1'].value,999)
        result=subprocess.run([str(ROOT/'bin/recalc'),str(p)],capture_output=True,text=True)
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertEqual(load_workbook(p,data_only=True).active['B1'].value,10)
        self.assertEqual(load_workbook(p).calculation.calcMode,'manual')

    def test_recalc_failure_keeps_original(self):
        recalc = module('recalc')
        p = self.work/'bad.xlsx'; book = Workbook(); book.active['A1'] = '=1+1'; book.save(p)
        before = p.read_bytes()
        with patch.object(recalc.subprocess,'run',return_value=subprocess.CompletedProcess([],1,'','failure')):
            with self.assertRaises(ValueError):recalc.recalc(p)
        self.assertEqual(p.read_bytes(),before)


if __name__ == '__main__':unittest.main()
