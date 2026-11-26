---
path: Documents/Change-decisions/Ticket archive trial scope.docx
kind: brt-change-decision.docx
author: Anjali Shah
modified_by: Anjali Shah
created: 2024-09-17T14:33:00+01:00
derived_from: Desktop/Ticket archive trial - draft.docx
---

表題「Ticket archive trial scope」はそのまま使用し、表題直後の管理欄を次に更新する。

Reference  BRT-CD-20240919-001  |  Issued  19 Sep 2024  
Prepared by  Anjali Shah

v1.0 · Issued  
For Reliability and Customer Support

- 「Basis and restrictions」の本文と表を次に置き換える。

  The September observations support taking the 10-minute candidate into isolated load testing. The two flags identify starts during an open predecessor write. Both predecessors were followed to completion. Restorable record coverage at a fixed reference remains to be established.

| Item | September evidence | Action |
|---|---|---|
| 15-minute comparator | On 9 September, 0 of 24 starts had an open predecessor. | Repeat as the comparator under the agreed foreground load. |
| 10-minute candidate | On 11 September, 2 of 36 starts had an open predecessor. Both earlier finishes were located by write reference. | Retain for the isolated load trial and record starts and matching predecessor finishes. |
| Evidence needed next | BRT-IC-20240916-001 gives the overlap comparison. The sustained-load run and eligible-set readback are outstanding. | Use matched windows under the same sustained foreground load. Obtain a fixed-reference readback of restorable coverage against the existing 15-minute target, inclusive. |

残りの2節は既存の見出しを使い、それぞれの本文を以下で差し替える。

## Decision

Approved at the 19 September review for isolated load testing. Keep the 10-minute schedule as the trial candidate and run the 15-minute schedule as the comparator using the same sustained foreground load profile. The live archive schedule and the existing operator recovery terms remain in force.

## Review conditions

Sanjay Mehta will agree the matched windows and shared load profile with Patrick Sutcliffe before booking. Daniel Jennings will specify how to retain the paired start and finish evidence and read back the latest restorable coverage from an eligible set complete by the fixed reference. Review both results before considering any change to the live schedule or operator recovery point. Claire Hutchinson will continue to use the existing operator wording.
