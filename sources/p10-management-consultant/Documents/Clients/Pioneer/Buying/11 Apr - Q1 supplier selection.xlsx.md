---
path: Documents/Clients/Pioneer/Buying/11 Apr - Q1 supplier selection.xlsx
kind: holt-pioneer-buying.xlsx
author: Nicholas Hargreaves
derived_from:
---

## Sheet: Supplier selection

| Cell | Entry |
|---|---|
| A2 | Pioneer — Q1 supplier selection (v1.0) |
| A4 | HRA-PIO-BUY-SEL-2025-001 |
| D4 | 2025-04-11 |
| F4 | Nicholas Hargreaves |

| Rank | Supplier | Trade reference | Net spend | Currency | Selection | Basis period | Selection evidence / reason |
|---|---|---|---|---|---|---|---|
| =1+COUNTIF($D$7:$D$48,">"&D7)+COUNTIFS($D$7:$D$48,D7,$B$7:$B$48,"<"&B7) | S008 | AP-47981, AP-48682 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B7,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A7<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D8)+COUNTIFS($D$7:$D$48,D8,$B$7:$B$48,"<"&B8) | S001 | AP-48087, AP-48731, AP-49427 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B8,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A8<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D9)+COUNTIFS($D$7:$D$48,D9,$B$7:$B$48,"<"&B9) | S022 | AP-48863, AP-49363 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B9,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A9<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D10)+COUNTIFS($D$7:$D$48,D10,$B$7:$B$48,"<"&B10) | S036 | AP-48079, AP-48872, AP-49405 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B10,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A10<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D11)+COUNTIFS($D$7:$D$48,D11,$B$7:$B$48,"<"&B11) | S011 | AP-49317 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B11,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A11<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D12)+COUNTIFS($D$7:$D$48,D12,$B$7:$B$48,"<"&B12) | S009 | AP-48931 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B12,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A12<=12,"In diagnostic","Population only") | Q1 2025 | Equal net spend with S027. S009 precedes S027 by supplier ID. |
| =1+COUNTIF($D$7:$D$48,">"&D13)+COUNTIFS($D$7:$D$48,D13,$B$7:$B$48,"<"&B13) | S027 | AP-48046, AP-49188 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B13,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A13<=12,"In diagnostic","Population only") | Q1 2025 | Equal net spend with S009. S027 follows S009 by supplier ID. |
| =1+COUNTIF($D$7:$D$48,">"&D14)+COUNTIFS($D$7:$D$48,D14,$B$7:$B$48,"<"&B14) | S016 | AP-48710, AP-49412 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B14,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A14<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D15)+COUNTIFS($D$7:$D$48,D15,$B$7:$B$48,"<"&B15) | S003 | AP-48429, AP-49232 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B15,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A15<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D16)+COUNTIFS($D$7:$D$48,D16,$B$7:$B$48,"<"&B16) | S020 | AP-48401 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B16,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A16<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D17)+COUNTIFS($D$7:$D$48,D17,$B$7:$B$48,"<"&B17) | S042 | AP-48106, AP-49271, AP-49308 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B17,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A17<=12,"In diagnostic","Population only") | Q1 2025 | Both invoice names combined under S042. AP-49308 deducts USD 9,000 from the two invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D18)+COUNTIFS($D$7:$D$48,D18,$B$7:$B$48,"<"&B18) | S018 | AP-48962 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B18,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A18<=12,"In diagnostic","Population only") | Q1 2025 | Final included supplier. The margin over S021 is shown below. |
| =1+COUNTIF($D$7:$D$48,">"&D19)+COUNTIFS($D$7:$D$48,D19,$B$7:$B$48,"<"&B19) | S021 | AP-49115 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B19,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A19<=12,"In diagnostic","Population only") | Q1 2025 | First supplier outside the review. Keep in the Q1 population. |
| =1+COUNTIF($D$7:$D$48,">"&D20)+COUNTIFS($D$7:$D$48,D20,$B$7:$B$48,"<"&B20) | S007 | AP-49126 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B20,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A20<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D21)+COUNTIFS($D$7:$D$48,D21,$B$7:$B$48,"<"&B21) | S013 | AP-48560 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B21,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A21<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D22)+COUNTIFS($D$7:$D$48,D22,$B$7:$B$48,"<"&B22) | S019 | AP-48517, AP-49070 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B22,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A22<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D23)+COUNTIFS($D$7:$D$48,D23,$B$7:$B$48,"<"&B23) | S033 | AP-49396 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B23,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A23<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D24)+COUNTIFS($D$7:$D$48,D24,$B$7:$B$48,"<"&B24) | S004 | AP-49036 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B24,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A24<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D25)+COUNTIFS($D$7:$D$48,D25,$B$7:$B$48,"<"&B25) | S041 | AP-48369, AP-49331 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B25,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A25<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D26)+COUNTIFS($D$7:$D$48,D26,$B$7:$B$48,"<"&B26) | S024 | AP-48767 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B26,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A26<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D27)+COUNTIFS($D$7:$D$48,D27,$B$7:$B$48,"<"&B27) | S030 | AP-48897 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B27,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A27<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D28)+COUNTIFS($D$7:$D$48,D28,$B$7:$B$48,"<"&B28) | S010 | AP-48348 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B28,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A28<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D29)+COUNTIFS($D$7:$D$48,D29,$B$7:$B$48,"<"&B29) | S037 | AP-48795 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B29,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A29<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D30)+COUNTIFS($D$7:$D$48,D30,$B$7:$B$48,"<"&B30) | S014 | AP-49264 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B30,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A30<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D31)+COUNTIFS($D$7:$D$48,D31,$B$7:$B$48,"<"&B31) | S025 | AP-48203 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B31,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A31<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D32)+COUNTIFS($D$7:$D$48,D32,$B$7:$B$48,"<"&B32) | S006 | AP-48943, AP-49372 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B32,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A32<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D33)+COUNTIFS($D$7:$D$48,D33,$B$7:$B$48,"<"&B33) | S038 | AP-49137 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B33,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A33<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D34)+COUNTIFS($D$7:$D$48,D34,$B$7:$B$48,"<"&B34) | S002 | AP-49002 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B34,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A34<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D35)+COUNTIFS($D$7:$D$48,D35,$B$7:$B$48,"<"&B35) | S015 | AP-48477, AP-49168 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B35,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A35<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D36)+COUNTIFS($D$7:$D$48,D36,$B$7:$B$48,"<"&B36) | S029 | AP-48579 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B36,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A36<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D37)+COUNTIFS($D$7:$D$48,D37,$B$7:$B$48,"<"&B37) | S035 | AP-49213 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B37,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A37<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D38)+COUNTIFS($D$7:$D$48,D38,$B$7:$B$48,"<"&B38) | S039 | AP-48149 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B38,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A38<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D39)+COUNTIFS($D$7:$D$48,D39,$B$7:$B$48,"<"&B39) | S005 | AP-48259 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B39,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A39<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D40)+COUNTIFS($D$7:$D$48,D40,$B$7:$B$48,"<"&B40) | S026 | AP-49462 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B40,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A40<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D41)+COUNTIFS($D$7:$D$48,D41,$B$7:$B$48,"<"&B41) | S032 | AP-48276 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B41,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A41<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D42)+COUNTIFS($D$7:$D$48,D42,$B$7:$B$48,"<"&B42) | S012 | AP-48004 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B42,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A42<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D43)+COUNTIFS($D$7:$D$48,D43,$B$7:$B$48,"<"&B43) | S040 | AP-49279 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B43,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A43<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D44)+COUNTIFS($D$7:$D$48,D44,$B$7:$B$48,"<"&B44) | S017 | AP-49349 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B44,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A44<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D45)+COUNTIFS($D$7:$D$48,D45,$B$7:$B$48,"<"&B45) | S034 | AP-48448, AP-49084 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B45,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A45<=12,"In diagnostic","Population only") | Q1 2025 | Posted invoices net of credit. |
| =1+COUNTIF($D$7:$D$48,">"&D46)+COUNTIFS($D$7:$D$48,D46,$B$7:$B$48,"<"&B46) | S031 | AP-49159 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B46,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A46<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D47)+COUNTIFS($D$7:$D$48,D47,$B$7:$B$48,"<"&B47) | S023 | AP-49224 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B47,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A47<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |
| =1+COUNTIF($D$7:$D$48,">"&D48)+COUNTIFS($D$7:$D$48,D48,$B$7:$B$48,"<"&B48) | S028 | AP-49289 | =SUMIF('Q1 posted lines'!$C$2:$C$59,B48,'Q1 posted lines'!$F$2:$F$59) | USD | =IF(A48<=12,"In diagnostic","Population only") | Q1 2025 | Posted tax-exclusive invoices. |

Row 49: A:C — Q1 population total. D — `=SUM(D7:D48)`. E — USD. F:H — Tax-exclusive posted invoices less credits.

Notes in A50:H52, one per row.

50. Meera Kulkarni agreed on 7 April to read the records of the first 12 suppliers. Ranks 13–42 remain in the population. The scope decision does not authorise a supplier approach.
51. Include only USD invoices and credits posted from 1 January to 31 March 2025. Amounts exclude tax and credits are signed negative. Group by supplier ID. Sort net purchases largest first and resolve equal values by ascending ID.
52. The five lines in Q1 invoice lines around the cut reconcile to the lines with the same AP keys on Q1 posted lines. Connor Atkinson’s 9 April alias check confirms both display names on S042.

## Sheet: Q1 posted lines

The line key identifies the posted AP entry. Column F carries the signed amount excluding tax. Posting date governs inclusion in the quarter. The supplier name is the display name on the posted document.

The header and detail below occupy A1:G59 with row 1 as the header.

| Line key | Posting date | Supplier ID | Invoice display name | Document type | Net of tax amount | Currency |
|---|---|---|---|---|---:|---|
| AP-47981 | 2025-01-09 | S008 | Supplier 008 | Invoice | 301820 | USD |
| AP-48004 | 2025-01-10 | S012 | Supplier 012 | Invoice | 35865 | USD |
| AP-48046 | 2025-01-13 | S027 | Supplier 027 | Invoice | 156400 | USD |
| AP-48079 | 2025-01-15 | S036 | Supplier 036 | Invoice | 197580 | USD |
| AP-48087 | 2025-01-15 | S001 | Supplier 001 | Invoice | 286250 | USD |
| AP-48106 | 2025-01-17 | S042 | Pioneer Supplier 042 | Invoice | 121000 | USD |
| AP-48149 | 2025-01-20 | S039 | Supplier 039 | Invoice | 57390 | USD |
| AP-48203 | 2025-01-21 | S025 | Supplier 025 | Invoice | 91640 | USD |
| AP-48259 | 2025-01-22 | S005 | Supplier 005 | Invoice | 51680 | USD |
| AP-48276 | 2025-01-23 | S032 | Supplier 032 | Invoice | 42255 | USD |
| AP-48348 | 2025-01-24 | S010 | Supplier 010 | Invoice | 113870 | USD |
| AP-48369 | 2025-01-27 | S041 | Supplier 041 | Invoice | 137400 | USD |
| AP-48401 | 2025-01-28 | S020 | Supplier 020 | Invoice | 207560 | USD |
| AP-48429 | 2025-01-29 | S003 | Supplier 003 | Invoice | 148750 | USD |
| AP-48448 | 2025-01-29 | S034 | Supplier 034 | Invoice | 21630 | USD |
| AP-48477 | 2025-01-30 | S015 | Supplier 015 | Invoice | 63520 | USD |
| AP-48517 | 2025-02-03 | S019 | Supplier 019 | Invoice | 155100 | USD |
| AP-48560 | 2025-02-04 | S013 | Supplier 013 | Invoice | 167690 | USD |
| AP-48579 | 2025-02-05 | S029 | Supplier 029 | Invoice | 67800 | USD |
| AP-48682 | 2025-02-07 | S008 | Supplier 008 | Invoice | 127440 | USD |
| AP-48710 | 2025-02-10 | S016 | Supplier 016 | Invoice | 224980 | USD |
| AP-48731 | 2025-02-10 | S001 | Supplier 001 | Invoice | 97480 | USD |
| AP-48767 | 2025-02-11 | S024 | Supplier 024 | Invoice | 128590 | USD |
| AP-48795 | 2025-02-12 | S037 | Supplier 037 | Invoice | 109450 | USD |
| AP-48863 | 2025-02-13 | S022 | Supplier 022 | Invoice | 306240 | USD |
| AP-48872 | 2025-02-13 | S036 | Supplier 036 | Invoice | 94710 | USD |
| AP-48897 | 2025-02-14 | S030 | Supplier 030 | Invoice | 119220 | USD |
| AP-48931 | 2025-02-17 | S009 | Supplier 009 | Invoice | 240000 | USD |
| AP-48943 | 2025-02-17 | S006 | Supplier 006 | Invoice | 89675 | USD |
| AP-48962 | 2025-02-18 | S018 | Supplier 018 | Invoice | 186400 | USD |
| AP-49002 | 2025-02-21 | S002 | Supplier 002 | Invoice | 72492 | USD |
| AP-49036 | 2025-02-24 | S004 | Supplier 004 | Invoice | 142950 | USD |
| AP-49070 | 2025-02-26 | S019 | Supplier 019 | Credit | -2120 | USD |
| AP-49084 | 2025-02-27 | S034 | Supplier 034 | Credit | -1970 | USD |
| AP-49115 | 2025-03-03 | S021 | Supplier 021 | Invoice | 182900 | USD |
| AP-49126 | 2025-03-04 | S007 | Supplier 007 | Invoice | 173460 | USD |
| AP-49137 | 2025-03-05 | S038 | Supplier 038 | Invoice | 76420 | USD |
| AP-49159 | 2025-03-07 | S031 | Supplier 031 | Invoice | 16960 | USD |
| AP-49168 | 2025-03-10 | S015 | Supplier 015 | Invoice | 8395 | USD |
| AP-49188 | 2025-03-11 | S027 | Supplier 027 | Invoice | 83600 | USD |
| AP-49213 | 2025-03-13 | S035 | Supplier 035 | Invoice | 59210 | USD |
| AP-49224 | 2025-03-14 | S023 | Supplier 023 | Invoice | 15900 | USD |
| AP-49232 | 2025-03-17 | S003 | Supplier 003 | Invoice | 67490 | USD |
| AP-49264 | 2025-03-18 | S014 | Supplier 014 | Invoice | 98540 | USD |
| AP-49271 | 2025-03-18 | S042 | Supplier 042 Trading | Invoice | 78000 | USD |
| AP-49279 | 2025-03-19 | S040 | Supplier 040 | Invoice | 32550 | USD |
| AP-49289 | 2025-03-19 | S028 | Supplier 028 | Invoice | 11670 | USD |
| AP-49308 | 2025-03-24 | S042 | Supplier 042 Trading | Credit | -9000 | USD |
| AP-49317 | 2025-03-24 | S011 | Supplier 011 | Invoice | 263410 | USD |
| AP-49331 | 2025-03-25 | S041 | Supplier 041 | Credit | -3600 | USD |
| AP-49349 | 2025-03-25 | S017 | Supplier 017 | Invoice | 22175 | USD |
| AP-49363 | 2025-03-26 | S022 | Supplier 022 | Credit | -11950 | USD |
| AP-49372 | 2025-03-26 | S006 | Supplier 006 | Credit | -7150 | USD |
| AP-49396 | 2025-03-27 | S033 | Supplier 033 | Invoice | 148705 | USD |
| AP-49405 | 2025-03-27 | S036 | Supplier 036 | Credit | -7780 | USD |
| AP-49412 | 2025-03-27 | S016 | Supplier 016 | Credit | -6350 | USD |
| AP-49427 | 2025-03-28 | S001 | Supplier 001 | Credit | -5360 | USD |
| AP-49462 | 2025-03-31 | S026 | Supplier 026 | Invoice | 47520 | USD |

## Sheet: Scope checks

| Check | Result |
|---|---|
| Suppliers with posted Q1 entries | =COUNTA('Supplier selection'!B7:B48) |
| Diagnostic suppliers | =COUNTIF('Supplier selection'!F7:F48,"In diagnostic") |
| Other suppliers in population | =COUNTIF('Supplier selection'!F7:F48,"Population only") |
| Share of suppliers in diagnostic | =B3/B2 |
| Rank 12 less rank 13, USD | =SUMIF('Supplier selection'!A7:A48,12,'Supplier selection'!D7:D48)-SUMIF('Supplier selection'!A7:A48,13,'Supplier selection'!D7:D48) |
| Supplier totals less signed AP entries, USD | ='Supplier selection'!D49-SUM('Q1 posted lines'!F2:F59) |

The scope share is 28.6% of the supplier IDs. The boundary margin is USD 3,500. The AP reconciliation difference is zero.
