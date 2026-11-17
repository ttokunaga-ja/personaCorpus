"""Generate disposable Office fixtures; assertions live in Rust integration tests."""
import io
import sys
from pathlib import Path
import zipfile
from openpyxl import Workbook
from openpyxl.styles import Font, PatternFill, Border, Side, Alignment
from pptx import Presentation

kind, target = sys.argv[1], Path(sys.argv[2])
if kind.startswith('notes'):
    deck=Presentation();slide=deck.slides.add_slide(deck.slide_layouts[0]);slide.shapes.title.text='決算会議'
    slide.notes_slide.notes_text_frame.text='発表者だけの説明 金額12400000' if kind=='notes' else ''
    deck.save(target)
elif kind in ('formula','error','stale'):
    book=Workbook();sheet=book.active
    sheet['A1']='=1/0' if kind=='error' else '=1+1'
    if kind=='stale':
        sheet['A1']=5;sheet['B1']='=A1*2';book.calculation.calcMode='manual'
    book.save(target)
    if kind=='stale':
        raw=target.read_bytes()
        with zipfile.ZipFile(io.BytesIO(raw)) as original,zipfile.ZipFile(target,'w') as out:
            for entry in original.infolist():
                data=original.read(entry)
                if entry.filename=='xl/worksheets/sheet1.xml':data=data.replace(b'<f>A1*2</f><v></v>',b'<f>A1*2</f><v>999</v>')
                out.writestr(entry,data)
elif kind=='layout':
    book=Workbook();sheet=book.active;sheet.title='資金繰り'
    sheet['A1']=12400000;sheet['B1']='=SUM(A1:A2)';sheet['A2']=0;sheet['B2']='=IF(A2=0,"",A1/A2)'
    sheet['B1'].font=Font(name='Arial',bold=True,color='FF125678')
    sheet['B1'].fill=PatternFill('solid',fgColor='FFABCDEF')
    sheet['B1'].border=Border(bottom=Side(style='thin',color='FF123456'))
    sheet['B1'].alignment=Alignment(horizontal='right');sheet['B1'].number_format='#,##0;[Red]-#,##0'
    sheet.column_dimensions['B'].width=27.5;sheet.row_dimensions[1].height=26
    sheet.freeze_panes='B2';sheet.print_area='A1:B2';sheet.print_title_rows='1:1'
    sheet.page_setup.orientation='landscape';sheet.page_setup.paperSize=sheet.PAPERSIZE_A4;sheet.page_margins.left=.3
    book.create_sheet('補足')['A1']="='資金繰り'!B1";book.save(target)
else:
    raise ValueError(kind)
