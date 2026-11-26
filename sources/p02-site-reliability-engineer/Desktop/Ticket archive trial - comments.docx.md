---
path: Desktop/Ticket archive trial - comments.docx
kind: brt-change-decision.docx
author: Anjali Shah
modified_by: Daniel Jennings
created: 2024-09-17T14:33:00+01:00
derived_from: Desktop/Ticket archive trial - draft.docx
---

## 変更履歴

- 種類：コメント
  - 人物：Daniel Jennings
  - 日時：2024-09-18T09:41:00+01:00
  - 対象の文：The shorter interval improves our recovery point
  - コメント本文：Anjali, the 2/36 measures concurrent writes: at two starts, the previous write was still open. We followed both to completion, but didn't check how far the restorable records went. Can we keep this to the isolated load trial and require a fixed-reference readback of the latest eligible set before we change any recovery statement?
