---
path: Documents/短文受付意図分類/Measurements/May ledger source check.xlsx
kind: mercer-adjudication.xlsx
author: Joshua Ramirez
derived_from:
---
# May ledger source check

## Sheet: Adjudication

A2: Additional candidate origin check  
B3: MDR-STI-20250523-AW-01-R01  
E3: May 23, 2025  
B4: Joshua Ramirez  
E4: 96 additional documents; separate from April fixed evaluation

Row 5 columns: A Ref. | B Channel | C Document ID | D Documents | E Disposition | F Evidence / notes

Rows 6–101 contain one document each, with D=1. The identifiers and evidence references follow these exact ranges and rules; the numbers are three digits. Each receipt reference identifies the receipt for that same numbered document. Alias evidence comprises both the alias-table entry and its linked receipt; the suffix identifies the same document in both records.

| Channel | Document IDs | Disposition | Evidence in F |
|---|---|---|---|
| Web form | WF-001–WF-033 | Direct | Receipt WF-R001–WF-R033, corresponding suffix |
| Web form | WF-034–WF-039 | Alias | Alias WF-A034–WF-A039 linked to receipt WF-R034–WF-R039, corresponding suffix |
| Web form | WF-040 | Exclude | Receipt reference missing; Joshua to trace original receipt |
| Shared inbox | SI-001–SI-019 | Direct | Receipt SI-R001–SI-R019, corresponding suffix |
| Shared inbox | SI-020–SI-030 | Alias | Alias SI-A020–SI-A030 linked to receipt SI-R020–SI-R030, corresponding suffix |
| Shared inbox | SI-031 | Exclude | Receipt reference missing; Joshua to trace original receipt |
| Shared inbox | SI-032 | Exclude | Alias correspondence unsupported; Joshua to check alias table and original record |
| Service desk relay | SD-001–SD-016 | Direct | Receipt SD-R001–SD-R016, corresponding suffix |
| Service desk relay | SD-017–SD-023 | Alias | Alias SD-A017–SD-A023 linked to receipt SD-R017–SD-R023, corresponding suffix |
| Service desk relay | SD-024 | Exclude | Alias correspondence unsupported; Joshua to check alias table and original record |

Rows are in the order above, with reference numbers 1–96. Representative entries:

| Row | A Ref. | B Channel | C Document ID | D Documents | E Disposition | F Evidence / notes |
|---:|---:|---|---|---:|---|---|
| 6 | 1 | Web form | WF-001 | 1 | Direct | Receipt WF-R001 |
| 38 | 33 | Web form | WF-033 | 1 | Direct | Receipt WF-R033 |
| 39 | 34 | Web form | WF-034 | 1 | Alias | Alias WF-A034 linked to receipt WF-R034 |
| 45 | 40 | Web form | WF-040 | 1 | Exclude | Receipt reference missing; Joshua to trace original receipt |
| 46 | 41 | Shared inbox | SI-001 | 1 | Direct | Receipt SI-R001 |
| 65 | 60 | Shared inbox | SI-020 | 1 | Alias | Alias SI-A020 linked to receipt SI-R020 |
| 76 | 71 | Shared inbox | SI-031 | 1 | Exclude | Receipt reference missing; Joshua to trace original receipt |
| 77 | 72 | Shared inbox | SI-032 | 1 | Exclude | Alias correspondence unsupported; Joshua to check alias table and original record |
| 78 | 73 | Service desk relay | SD-001 | 1 | Direct | Receipt SD-R001 |
| 101 | 96 | Service desk relay | SD-024 | 1 | Exclude | Alias correspondence unsupported; Joshua to check alias table and original record |

Summary below the detail, labels in A:C and counts in D:

| Row | Label | Formula | Expected documents |
|---:|---|---|---:|
| 102 | Total checked | =SUM(D6:D101) | 96 |
| 103 | Direct | =COUNTIF(E6:E101,"Direct") | 68 |
| 104 | Alias | =COUNTIF(E6:E101,"Alias") | 24 |
| 105 | Exclude | =COUNTIF(E6:E101,"Exclude") | 4 |
| 106 | Supported candidates | =D103+D104 | 92 |

Row 107 note: Direct and alias rows remain candidates, not an approved evaluation set. The four excluded rows require follow-up; no origin is inferred from the channel or a similar subject. The April 3 fixed evaluation is separate and is not recalculated here.
