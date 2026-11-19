---
path: Documents/短文受付意図分類/Measurements/Working out which seconds count.xlsx
kind: mercer-adjudication.xlsx
author: Vanessa Nguyen
derived_from:
---

# Working out which seconds count — provisional

## Sheet: Adjudication

- A2: Working out which seconds count — provisional
- B3: MDR-STI-20241002-AW-01-R01
- E3: October 2, 2024
- B4: Vanessa Nguyen
- E4: Resident worker; 1,000 requests per run; comparison endpoint unresolved

| Row | A: Measure | B: Scope | C: Seconds | D: USD/hour | E: USD per 1,000 | F: Status / note |
|---:|---|---|---:|---:|---|---|
| 6 | Linear | Full path median | 100 | 1.20 | =C6/3600*D6 | First parse to final route |
| 7 | Larger | Forward-only median | 150 | =D6 | =C7/3600*D7 | Inner inference only; provisional comparison |
| 8 | Larger | Full path median | 210 | =D6 | =C8/3600*D8 | Same endpoint as linear? Confirm with Brandon |
| 9 | Provisional ratio | Forward-only / linear full path |  |  | =E7/E6 | 1.50; mixed endpoints |
| 10 | Decision | Application choice |  |  |  | Not determined |

The common worker rate is an input for exploring the arithmetic, not a resolution of the endpoint mismatch. Each median comes from three 1,000-request runs. Startup, external waiting and human review are outside the timing. Ask Brandon which endpoints belong in the use comparison before relying on the ratio.
