---
path: Google Drive/My Drive/Learner-support/October dispatch review.xlsx
kind: elmwood-outcome-reconciliation.xlsx
author: Heather Callahan
derived_from:
---

# October dispatch review

Sheet: Dispatch review

| Cell | Entry |
|---|---|
| A2 | October dispatch review |
| C3 | ELM-CRW-2024-REC-001 v1.0 |
| G3 | Heather Callahan |
| C4 | 2024-10-21 (date value, displayed as 21 Oct 2024) |
| G4 | Reading Conditions at Work |

Row 6 contains the seven column headings below. Row numbers at left are worksheet addresses.

| Row | A: Anonymous ID | B: Session | C: Absence confirmed | D: Next-day deadline | E: Delivered to learner | F: Days late | G: Delay reason |
|---|---|---|---|---|---|---|---|
| 7 | F01 | 1 | 2024-09-10 | =C7+1 | 2024-09-10 | =IF(E7="","",MAX(0,E7-D7)) | |
| 8 | F02 | 1 | 2024-09-10 | =C8+1 | 2024-09-11 | =IF(E8="","",MAX(0,E8-D8)) | |
| 9 | F03 | 1 | 2024-09-10 | =C9+1 | 2024-09-13 | =IF(E9="","",MAX(0,E9-D9)) | Material identification: regular notice and group entry instruction had the same abbreviation |
| 10 | F04 | 1 | 2024-09-10 | =C10+1 | 2024-09-11 | =IF(E10="","",MAX(0,E10-D10)) | |
| 11 | F05 | 1 | 2024-09-10 | =C11+1 | 2024-09-11 | =IF(E11="","",MAX(0,E11-D11)) | |
| 12 | F06 | 2 | 2024-09-17 | =C12+1 | 2024-09-17 | =IF(E12="","",MAX(0,E12-D12)) | |
| 13 | F07 | 2 | 2024-09-17 | =C13+1 | 2024-09-18 | =IF(E13="","",MAX(0,E13-D13)) | |
| 14 | F08 | 2 | 2024-09-17 | =C14+1 | 2024-09-20 | =IF(E14="","",MAX(0,E14-D14)) | Material identification: “Tuesday worksheet” could mean the projected example or the individual exercise |
| 15 | F09 | 2 | 2024-09-17 | =C15+1 | 2024-09-18 | =IF(E15="","",MAX(0,E15-D15)) | |
| 16 | F10 | 2 | 2024-09-17 | =C16+1 | 2024-09-18 | =IF(E16="","",MAX(0,E16-D16)) | |
| 17 | F11 | 3 | 2024-09-24 | =C17+1 | 2024-09-24 | =IF(E17="","",MAX(0,E17-D17)) | |
| 18 | F12 | 3 | 2024-09-24 | =C18+1 | 2024-09-25 | =IF(E18="","",MAX(0,E18-D18)) | |
| 19 | F13 | 3 | 2024-09-24 | =C19+1 | 2024-09-27 | =IF(E19="","",MAX(0,E19-D19)) | Distribution: sorting tray held the exercise without the notice extract |
| 20 | F14 | 3 | 2024-09-24 | =C20+1 | 2024-09-25 | =IF(E20="","",MAX(0,E20-D20)) | |
| 21 | F15 | 3 | 2024-09-24 | =C21+1 | 2024-09-25 | =IF(E21="","",MAX(0,E21-D21)) | |
| 22 | F16 | 4 | 2024-10-01 | =C22+1 | 2024-10-01 | =IF(E22="","",MAX(0,E22-D22)) | |
| 23 | F17 | 4 | 2024-10-01 | =C23+1 | 2024-10-02 | =IF(E23="","",MAX(0,E23-D23)) | |
| 24 | F18 | 4 | 2024-10-01 | =C24+1 | 2024-10-04 | =IF(E24="","",MAX(0,E24-D24)) | Material identification: “Tuesday” bundle did not distinguish Booking slips from the previous week's material |
| 25 | F19 | 4 | 2024-10-01 | =C25+1 | 2024-10-02 | =IF(E25="","",MAX(0,E25-D25)) | |
| 26 | F20 | 4 | 2024-10-01 | =C26+1 | 2024-10-02 | =IF(E26="","",MAX(0,E26-D26)) | |
| 27 | F21 | 5 | 2024-10-08 | =C27+1 | 2024-10-08 | =IF(E27="","",MAX(0,E27-D27)) | |
| 28 | F22 | 5 | 2024-10-08 | =C28+1 | 2024-10-09 | =IF(E28="","",MAX(0,E28-D28)) | |
| 29 | F23 | 5 | 2024-10-08 | =C29+1 | 2024-10-10 | =IF(E29="","",MAX(0,E29-D29)) | Distribution: viewing-link permissions blocked access before delivery by attachment |
| 30 | F24 | 5 | 2024-10-08 | =C30+1 | 2024-10-09 | =IF(E30="","",MAX(0,E30-D30)) | |
| 31 | F25 | 6 | 2024-10-15 | =C31+1 | 2024-10-15 | =IF(E31="","",MAX(0,E31-D31)) | |
| 32 | F26 | 6 | 2024-10-15 | =C32+1 | 2024-10-16 | =IF(E32="","",MAX(0,E32-D32)) | |
| 33 | F27 | 6 | 2024-10-15 | =C33+1 | 2024-10-18 | =IF(E33="","",MAX(0,E33-D33)) | Material identification: “board exercise” did not identify which projected material to send |
| 34 | F28 | 6 | 2024-10-15 | =C34+1 | 2024-10-16 | =IF(E34="","",MAX(0,E34-D34)) | |

Use actual Excel dates in C and E and display D as a date. D is the deadline at the end of the next calendar day in Toronto. E is blank until the learner has received the exercise. The formula in F returns a blank for an open delivery and a whole number of calendar days for a recorded delivery.

| Row | Label in A | Result in F |
|---|---|---|
| 35 | Confirmed absences | =COUNTA(A7:A34) |
| 36 | Delivered by end of next day | =COUNTIFS(E7:E34,">0",F7:F34,0) |
| 37 | Delivered after deadline | =COUNTIF(F7:F34,">0") |
| 38 | Total days late on delivered records | =SUM(F7:F34) |

Row 39 note: Checked with Arjun against absence-dispatch.csv for Sessions 1–6 (10 Sep to 15 Oct). One confirmed absence per learner. Use the date the learner received the exercise. The 21 Oct check gives 28 confirmed absences, 22 delivered by the end of the next day, six delivered late and 11 calendar days late in total.
