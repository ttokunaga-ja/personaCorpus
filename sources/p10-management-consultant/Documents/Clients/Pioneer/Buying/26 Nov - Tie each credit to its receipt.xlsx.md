---
path: Documents/Clients/Pioneer/Buying/26 Nov - Tie each credit to its receipt.xlsx
kind: Working sheet
author: Connor Atkinson
derived_from:
---
# Tie each credit to its receipt

For Deepa | 26 Nov 2025. Working examples for the Q4 extract; a candidate is not a posted, matched return.

## Sheet: Credit links

| Row | A: Condition | B: Entry | C: Original receipt | D: Quantity (cases) | E: Unit price USD | F: Amount USD | G: Status / treatment |
|---|---|---|---|---:|---:|---|---|
| 2 | S018 | Receipt AP-62114 | AP-62114 | 1700 | 19.60 | =D2*E2 | Original receipt |
| 3 | S018 | Return candidate, 200 cases | Unconfirmed | | 19.60 | | Hold quantity and amount out of net until original link and credit evidence are established |
| 4 | S011 | Additional commercial allowance candidate | Not applicable | | | | No quantity; separate approval and posting evidence required |

Illustration only after a confirmed link: enter −200 cases against the original receipt and calculate −200 × 19.60 = −3,920 USD; do not count it a second time elsewhere. The S011 candidate is not a return and must not be assigned cases.

## Sheet: Check before allocation

| Row | A: Condition | B: Issue | C: Check | D: Consequence if ignored |
|---|---|---|---|---|
| 2 | S018 | 200-case return candidate; original link not yet evidenced | Obtain the credit and its original transaction key before reducing net quantity | An unmatched credit allocated to the item and later linked to a receipt can deduct the same 200 cases twice: −400 instead of −200. |
| 3 | S011 | Additional allowance candidate without quantity | Obtain approval and posting; reconcile separately | Allocating it to item quantities invents a return and can count an allowance again when its actual credit is posted. |

Reference: HRA-PIO-BUY-EV-2025-004, 22 Oct. No claim is made here that a candidate has been posted.
