---
path: Documents/配車の見通し調査/Planning/2024-07-11 Dispatch prompts EN-JA.xlsx
kind: carver-prompt-review.xlsx
author: Jennifer Salazar
derived_from:
---
# Sheet: Prompt review

Dispatch prompts EN–JA
CWR-DR-20240711-XLT-01 | v1.0 | 2024-07-11 | Jennifer Salazar

| A: Order | B: Section | C: English | D: 日本語 | E: Old no. | F: Moderator note |
|---:|---|---|---|---|---|
| 1 | Recent handoff | Think of a recent run you handed over or received. Please walk me through what happened. | 最近、引き継いだ、または引き継ぎを受けた便を一つ思い浮かべてください。そのときのことを順に教えてください。 | 01 | Let the person start with their own account. |
| 2 | Recent handoff | What was your role in that handoff? | その引継ぎでは、あなたはどのような役割でしたか。 | 02 | |
| 3 | Recent handoff | What did you first use to orient yourself to that run? | その便について、最初に何を手掛かりにしましたか。 | 03 | Do not name a source. |
| 4 | State and reason | At that point, what did you understand the state of the run to be? | その時点で、その便についてどのような状態だと分かっていましたか。 | 05 | Use the person's words. |
| 5 | State and reason | Was there anything you learned about what was going on? If so, how did you learn it? | そのとき分かったことはありましたか。あれば、どうやって分かりましたか。 | 06 | Ask the route only if something was learned. |
| 6 | State and reason | At that point, was there anything you still wanted to check? | その時点で、まだ確かめたいことはありましたか。 | 08 | |
| 7 | Change and cross-check | Did anything about the run change before the handoff? | 引継ぎまでの間に、便について何か変わったことはありましたか。 | 09 | |
| 8 | Change and cross-check | If something changed, where or from whom did you learn about it? | 変更があった場合、それをどこで、または誰から知りましたか。 | 11 | |
| 9 | Change and cross-check | Did you compare that information with anything else to check it? If so, what? | その情報を確かめるために、ほかの情報と照らし合わせましたか。あれば何と照らし合わせましたか。 | 12 | |
| 10 | Passing on | If anything was out of the ordinary, what brought it to your attention? | 普段と違うことがあった場合、何を手掛かりに気づきましたか。 | 14 | |
| 11 | Passing on | What did you pass on to the next person? | 次の人に何を伝えましたか。 | 15 | Record content, including if nothing was passed. |
| 12 | Passing on | If the next person needed to check further, where or with whom could they do that? | 次の人が続きを確かめるとしたら、どこや誰に確認できましたか。 | 17 | |
| 13 | Passing on | After the handoff, was anything still unknown? | 引継ぎが終わった後も、まだ分からないままだったことはありましたか。 | 18 | Do not equate unknown with not checked. |
| =COUNT(A7:A19) | Adopted questions | | | | |

# Sheet: Removed

CWR-DR-20240711-XLT-01 | v1.0 | 2024-07-11 | Jennifer Salazar

| A: Order | B: Old no. | C: English | D: 日本語 | E: Decision | F: Reason |
|---:|---|---|---|---|---|
| 1 | 04 | Did you first look at the run list to find the run? | まず便一覧を見て、その便を探しましたか。 | Remove | Supplies the list as the first source. |
| 2 | 07 | Did you call someone to check the reason? | 理由を知るために電話で確認しましたか。 | Remove | Supplies a telephone call as the route. |
| 3 | 10 | Did you check the replacement-vehicle field to confirm the vehicle? | 代車欄を見て車両を確認しましたか。 | Remove | Supplies the vehicle field and conflates display with confirmation. |
| 4 | 13 | How did you hand over the details that had already been confirmed? | 確定済みの内容は、どのように引き継ぎましたか。 | Remove | Assumes confirmation has happened. |
| 5 | 16 | Did you read the Note and pass on what it said? | Noteを見て、そこに書かれたことを伝えましたか。 | Remove | Supplies the Note before the person identifies the information. |
| =COUNT(A7:A11) | Removed questions | | | | |
