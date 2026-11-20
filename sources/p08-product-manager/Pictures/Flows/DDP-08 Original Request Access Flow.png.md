---
path: Pictures/Flows/DDP-08 Original Request Access Flow.png
kind: diagram
author: 石田 由美子
derived_from:
---

# When the original request cannot be opened

英語の分岐図。左から右に矢印で接続する。画面の図は置かない。

1. 開始箱： “User cannot open the original request”
2. 箱： “Identify the request and the person who needs to view it”
3. 箱： “Ask the site administrator to confirm whether that person can view that request”
4. 分岐： “Has the administrator answered about that person and request?”
5. No の矢印から終点： “Awaiting administrator's answer”
6. Yes の矢印から終点： “Access decision received — record the answer and date”

回答待ちと回答済みは別の終点。回答が可否を特定しなければ No に残る。別拠点の元票への線や販売窓口が権限を変更する線はない。
