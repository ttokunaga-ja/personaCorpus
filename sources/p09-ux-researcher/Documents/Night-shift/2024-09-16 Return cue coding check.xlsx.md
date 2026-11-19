---
path: Documents/Night-shift/2024-09-16 Return cue coding check.xlsx
kind: carver-field-coding.xlsx
author: Brian Chen
derived_from:
---

# Return cue coding check

Document: CWR-NS-20240916-DEC-01 | Version: v1.0 | Date: 2024-09-16 | Author: Brian Chen

## Sheet: Return cues

| Row | A: Scene | B: Jennifer · first pass | C: Brian · first pass | D: Agreed cue | E: First pass match | F: Observation behind the call |
|---:|---|---|---|---|---|---|
| 7 | N01 | Paper | Paper | Paper | =IF(B7=C7,"Agree","Review") | After returning, used the B-14 check on own paper to find the next line, B-15. |
| 8 | N02 | Colleague | Colleague | Colleague | =IF(B8=C8,"Agree","Review") | Asked a colleague after returning; then reconciled C-08. |
| 9 | N03 | Terminal | Bin label | Bin label | =IF(B9=C9,"Agree","Review") | D-04 was entered but unsaved; after return the label identified D-04, then the count was repeated and saved. |
| 10 | N04 | Paper | Paper | Paper | =IF(B10=C10,"Agree","Review") | Found the B-21 check on paper from the fold before B-22. |
| 11 | N05 | No cue | Colleague | Colleague | =IF(B11=C11,"Agree","Review") | A colleague said C-12 on return; reconciliation began in the same recorded minute. |
| 12 | N06 | Paper | Paper | Paper | =IF(B12=C12,"Agree","Review") | Followed the paper from the top to the D-09 check before D-10. |
| 13 | First-pass matches | | | | =COUNTIF(E7:E12,"Agree") | 4 of 6 |
| 14 | Agreed: Paper | | | =COUNTIF(D7:D12,"Paper") | | 3 scenes |
| 15 | Agreed: Colleague | | | =COUNTIF(D7:D12,"Colleague") | | 2 scenes |
| 16 | Agreed: Bin label | | | =COUNTIF(D7:D12,"Bin label") | | 1 scene |

