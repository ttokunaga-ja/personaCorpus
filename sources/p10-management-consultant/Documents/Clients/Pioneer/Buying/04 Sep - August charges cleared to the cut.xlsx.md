---
path: Documents/Clients/Pioneer/Buying/04 Sep - August charges cleared to the cut.xlsx
kind: Working sheet
author: Deepa Desai
derived_from:
---

# Sheet: Cleared August

For Nicholas, 04 Sep 2026. August receipts and linked returns posted through 04 Sep; actual net quantities valued at the approved comparison baseline, excluding tax and freight. Each cleared range is distinct from the held ranges on Queries. References identify the Buying receipt register ranges, applicable approved terms and Finance posting controls used for the reconciliation.

| Row | Supplier and cleared receipt range | Baseline USD | Confirmed net purchase USD | Difference USD | Evidence |
|---|---|---:|---:|---:|---|
| 2 | S018, R-AUG-018-01–08 and linked returns | 180000 | 168000 | =C2-D2 | Buying receipt register R-AUG-018-01–08; approved terms T-AUG-018 rev 1; Finance posting control PC-AUG-018 through 04 Sep, including linked returns |
| 3 | S042, R-AUG-042-01–06 and linked returns | 160000 | 152000 | =C3-D3 | Receipt register R-AUG-042-01–06; T-AUG-042 rev 1; posting control PC-AUG-042 through 04 Sep |
| 4 | S008, R-AUG-008-01–05 and linked returns | 140000 | 133000 | =C4-D4 | Receipt register R-AUG-008-01–05; T-AUG-008 rev 1; posting control PC-AUG-008 through 04 Sep |
| 5 | S036, R-AUG-036-01–04 and linked returns | 200000 | 187000 | =C5-D5 | Receipt register R-AUG-036-01–04; approved terms T-AUG-036 rev 1 effective for these receipts; posting control PC-AUG-036 through 04 Sep. Excludes R-AUG-036-05–06. |
| 6 | S011, R-AUG-011-01–03 and linked returns | 120000 | 112000 | =C6-D6 | Receipt register R-AUG-011-01–03; approved terms T-AUG-011 rev 1; posting control PC-AUG-011 through 04 Sep. Excludes R-AUG-011-04–05. |
| 7 | Total confirmed | =SUM(C2:C6) | =SUM(D2:D6) | =SUM(E2:E6) | =C7-D7-E7 |
| 8 | Realised / baseline | | | =E7/C7 | 6.0%; check in F7 should be zero. |

# Sheet: Queries

| Supplier | Receipt and invoice references | Missing evidence | Route | Difference USD | Boundary |
|---|---|---|---|---:|---|
| S036 | R-AUG-036-05–06; invoices INV-AUG-036-05–06 | Approved terms version effective for these August receipts | Meera / Buying |  | Separate from R-AUG-036-01–04; excluded from numerator and denominator. |
| S011 | R-AUG-011-04–05; invoices INV-AUG-011-04–05 | Approval and Finance posting of the August commercial credit | Meera / Buying and Finance |  | Separate from R-AUG-011-01–03; excluded from numerator and denominator. |

The January 5.6% checkpoint and its USD 5,500 S011 residual refer to Q4 and remain there; neither is an August posting.
