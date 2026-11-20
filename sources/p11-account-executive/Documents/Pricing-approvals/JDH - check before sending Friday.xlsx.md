---
path: Documents/Pricing-approvals/JDH - check before sending Friday.xlsx
kind: halstead-adoption.xlsx
author: Michelle Whitaker
derived_from:
---

# Sheet: Price Release Check

A3: Price Release Check
A5: As of
C4: HBC-JDH-20260717-REN-02 v1.0
C5: July 17, 2026
C6: Michelle Whitaker

| Row | A: No. | B: Package or revision | C: Annual price (USD) | D: Valid through (MDT) | E: Planned send (MDT) | F: Approval reference / release status |
|---:|---:|---|---:|---|---|---|
| 9 | 1 | First price window · 520 seats and 60 annual support hours | 248000 | 2026-07-10 17:00 | 2026-07-17 11:30 | Donald Ramirez, Jul 9 at 10:02, “JDH August terms - first price window.” Expired. |
| 10 | 2 | Reapproved price window · same annual package | 248000 | 2026-07-17 16:00 | 2026-07-17 11:30 | Donald Ramirez, Jul 16 at 10:03, “Re: JDH August terms - renew the price window.” Within the price window. Issue approval still to be checked before sending. |

A11:F11 note: JDH Route Services Inc · Annual term August 1, 2026 through July 31, 2027. The two rows show successive price windows for the same annual amount. The May–July extension was a separate contract.

| Row | B: Window timing | E: Time difference | F: Price decision at planned send |
|---:|---|---|---|
| 13 | First price window | `=TEXT(ABS(D9-E9),"[h]"" h ""mm"" min""")` | `=IF(E9>D9,"Expired - hold for price reapproval","Within price window")` |
| 14 | Reapproved price window | `=TEXT(ABS(D10-E10),"[h]"" h ""mm"" min""")` | `=IF(E10>D10,"Expired - hold for price reapproval","Within price window")` |

A15:F15 note: At the 11:30 a.m. plan, the first window is 162 hours 30 minutes past expiry and the reapproved window has 4 hours 30 minutes left. Update the planned send time if it slips. If the send would be after 4:00 p.m. MDT, hold the email and ask Donald to reapprove the price in writing.

A17:F17 note: Wording cleared separately by Jonathan Salgado, Jul 16 at 14:26, “JDH August renewal - signature conditions cleared.” August 1 starts the term and the existing 480 seats continue. Release the additional 40 in stages after Divya confirms readiness and JDH completes its roster, rehearsals and approvals.

A18:F18 note: Before sending, match Donald's written approval to the issue copy HBC-JDH-20260717-REN-01 v1.0 and verify the support scope reference (HBC-JDH-20260716-SVC-01 v1.0). Ask Janet to return written acceptance identifying both versions by 4:00 p.m. MDT. File it with Donald's issue approval when it comes back.
