---
path: Documents/Clients/Pioneer/Buying/11 Apr - Q1 supplier cutoff review v0.1.pptx
kind: holt-diagnosis.pptx
author: Nicholas Hargreaves
derived_from:
---

## Slide 1 — Title

The cutoff after the alias check  
Pioneer buying diagnostic | HRA-PIO-BUY-BRF-2025-003 | v0.1  
11 Apr 2025 | Nicholas Hargreaves

Speaker notes: For the next progress discussion with Meera and Connor. This is the scope of our diagnostic, not an instruction to reopen terms.

## Slide 2 — Two columns

Title: One supplier appeared under two names

Left — Display-name view: Pioneer Supplier 042: invoice USD 121,000. Supplier 042 Trading: invoice USD 78,000 less credit USD 9,000 = USD 69,000. Reading names as separate suppliers suggests an extra candidate.

Right — Supplier-ID view: S042 combines both names. USD 121,000 + USD 78,000 − USD 9,000 = USD 190,000. The credit stays with the supplier to which it belongs.

Speaker notes: Connor's alias check uses AP-48106, AP-49271 and AP-49308. The boundary extract is not the full supplier population.

## Slide 3 — Evidence table

Title: The boundary is between twelve and thirteen

| Rank | Supplier ID | Q1 net purchases, USD | Treatment |
|---:|---|---:|---|
| 11 | S042 | 190,000 | In diagnostic |
| 12 | S018 | 186,400 | In diagnostic |
| 13 | S021 | 182,900 | Remains in population |

Q1 means invoices posted 01 Jan–31 Mar 2025, excluding tax and net of credits. Group by supplier ID; rank net amount descending, with supplier ID ascending for ties.

Speaker notes: The ID-based provisional order at the 7 April meeting is unchanged after the alias check. The selected twelve define whose transaction materials we read in detail; they are not a list of suppliers with whom terms have been reopened.

## Slide 4 — Title and body

Title: Keep the other thirty visible

- Retain ranks 13–42 in the 42-supplier population and preserve the selection basis.
- Do not expand the diagnostic merely because one supplier has more than one display name.
- Before considering renegotiation, ask what an urgent request tells us about when the requirement was known and what reached Buying.

Speaker notes: Meera supplied the current exception guidance, PTD-BUY-EXC-2025-001 Rev 2, on 9 April. Read its approval route and ask how request reasons can be matched to underlying demand evidence; the guidance itself does not establish that timing. No conclusion on causes yet.
