# Vanessa Nguyen の仕事の事実

## 短文受付意図分類の研究計画

- 2024-04-15 14:00（サクラメント）、Vanessa Nguyen と Michael Serrano は受付意図12分類、初年度 macro-F1 目標0.82で合意した。用途への採用は Brandon Romero と別に検討する。
- 計画は `MDR-STI-20240416-RP-01-R01`（4月16日発行）。文書単位で学習・開発・評価区画を設計し、根拠のある正解が付き、12分類に支持件数がある固定評価区画で多数クラス基準と単語頻度の線形分類器を比較する。この時期の資料にモデルの実測 F1 はない。
- 12分類の英語名は New work request、Quote request、Schedule check、Reschedule、Cancel request、Job status、Send document、Request document、Correct record、Access issue、Report defect、Invoice question。

## 受付文書の取り込みと判定用の本文

- 5月2日に Vanessa と Joshua Ramirez が照合した4月末時点の文書集は Web form 2,160件、Shared inbox 1,440件、Service desk relay 1,200件、計4,800件。1件は受信した受付本文1通で、案件数・採点数とは別。
- 収録範囲を示す説明書は `MDR-STI-20240502-DD-01-R01`（5月2日発行）。5月7日に決めた判定用の抜粋は照合IDと受信本文だけで、件名、窓口名、routing tag、処理結果、担当者が後から付けた転送コメントを含めない。

## 5月の独立読解と6月の手順見直し

- 初版基準は `MDR-STI-20240513-GD-01-R01`（5月13日発行）。12分類の候補と本文の根拠を記録し、競合するときは裁定に回す。空欄と保留の区別や初回一致時の根拠再確認は、この版では未規定。
- 5月23日、Vanessa と Fatima Rahman が5月試行240件を確認。Web form は95一致・13不一致（108件）、Shared inbox は63・9（72件）、Service desk relay は49・11（60件）。合計207一致、33不一致。240件は4,800件の内数。
- 6月13日、Vanessa と Emily Chen が5月の不一致33件から18件を選び、最終文だけを読む規則を退けた。6月27日に全4,800件の独立再読を割り付け、5月の240件も新たに読み、候補が一致した場合も本文の根拠句を残すことにした。

## 7月18日の裁定と判定基準の改訂

- 7月18日14:00にサクラメントの研究会室で Vanessa と Fatima が独立再読を進めながら付け合わせてきた記録をもとに、残った境界例を照合して全件の処置を確定した。4,800件のうち、初回一致4,488件は正解候補4,456・保留32、初回不一致312件は正解候補218・保留94。
- 初回不一致の内訳（不一致／確定／保留）は日程70／54／16、取消56／42／14、文書送付・請求48／34／14、訂正46／32／14、進捗53／40／13、同格の二要求39／16／23。1件は代表となる境界1つにだけ数える。
- 4,800件の最終処置は正解ラベルが付いた評価区画への候補4,674件と、文面から意図を特定できず Hold とした126件。Hold は処置欄に記録し、正解ラベル欄は空のまま採点母集団から外す。初回読解の候補欄は読解履歴として保持する。
- 改訂基準は `MDR-STI-20240513-GD-01-R02`（7月18日発行）。12分類は同じで、一致でも本文の根拠を点検し、一つに定まらなければ Hold。Fatima が返した変更履歴付き控えは5月初版の発行情報を持つレビュー用の写し。
- 日程の対例は “Move the visit to Thursday”／“Thursday visit?”、取消は “Cancel the Friday visit”／“Can you do something about the Friday visit?”。文書の対例は “Could you send the invoice again?”／“Could you take care of the invoice again?”。各対の後者は要求された処理が不明のため Hold とする。

## 研究計画の回覧と改訂基準の引き渡し

- 1通目 “Short intake plan for Monday” は Michael Serrano → Vanessa Nguyen、4月10日10:18 PDT。2通目 “Re: Short intake plan for Monday” は Vanessa → Michael、4月16日11:02 PDT。1通目を引用し “Short intake intent research plan.pdf” を添付する。
- “July intake label guide for the handoff” は Vanessa → Fatima Rahman・Joshua Ramirez、7月19日10:12 PDT。“Short intake annotation guide July.docx” を添付し、保留126件を維持した状態で7月22日までの照合を頼む。
- Mercer 内部メールのアドレスは Vanessa `vanessa@mercer-document.example`、Michael `michael@mercer-document.example`、Fatima `fatima@mercer-document.example`、Joshua `joshua@mercer-document.example`。
