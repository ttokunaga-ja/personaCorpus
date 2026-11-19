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

## 1月の短文評価の取り下げ

- 2025-01-16 の研究会資料 `MDR-STI-20250116-LM-01-R01` のスライド3で、Vanessa は研究班に文書単位分割の単語頻度線形分類器 macro-F1 0.851（評価940文書、12分類）を示した。旧割付の学習／開発／評価は2,800／934／940文書。
- 1月23日に Nicole が同一案件の複数区画への割付を指摘した（方法レビュー `HEI-20250123-MR-01-R01`）。旧区画ペア別の重複なしの案件数／文書数は、学習・評価12／42、学習・開発7／24、開発・評価5／18。合計24案件・84文書で、3区画すべてにまたがった案件は0。
- Vanessa は1月23日から旧値の再利用を止めた。1月27日14:00に Vanessa・Michael・Fatima・Joshua が旧値の撤回、12分類と7月改訂の判定基準の維持を確認し、1月28日に Vanessa が研究会参加者へ撤回通知 `MDR-STI-20250128-RM-01-R01` を発行した。旧資料は履歴として保管。
- 1通目 “Short intake split question” は Nicole Foster（`nicole@hartwell-evaluation.example`）→ Vanessa Nguyen、1月23日09:26 PST。“Short intake partition review.pdf” を添付する。2通目 “Re: Short intake split question” は Vanessa → Nicole、同日10:08 PST。1通目を引用し、返信自体に添付はない。

## 案件単位の再割付と採点条件

- 2月の既知24案件での照合試行では、窓口内キーで一束に戻ったのは17案件（旧区画ペアの上記順に8／5／4）、窓口横断キーでは24案件（12／7／5）。Joshua が受領原記録または確認済み別名表で裏付けた窓口横断キーを使う。
- 再評価条件 `MDR-STI-20250212-RM-01-R01` は2月12日の初稿。Fatima の2月14日の変更履歴付き控えは初稿の番号・発行情報を保持し、3月12日の承認版で同一起番の `MDR-STI-20250212-RM-01-R02` に進めた。
- 4月3日に固定した案件単位割付は学習2,804、開発935、評価935文書。全3ペアで共有案件は0。旧・新とも割付対象は同じ正解付き4,674文書（1月の84文書を含む）で、旧来の Hold 126文書は割付と採点の範囲外。案件は割付の単位、受信本文1通は採点の単位。
- 4月の固定評価の分類別支持は、研究計画の分類順で132／90／110／85／55／98／70／65／52／58／67／53文書（計935）。学習区画の最頻分類は New work request で、多数クラス基準は935文書すべてをこの分類と予測する。
- 両比較は同じ固定評価区画・12分類・7月改訂基準を使う。単語頻度の線形分類器の入力は受信本文のみ。案件キー・経路情報を特徴に使わず、特徴や設定の選択は開発区画で完了する。macro-F1 は各分類の未丸めの 2TP/(2TP+FP+FN) を等重みで平均し、最後に報告桁へ丸める。

## 4月17日の再評価報告

- 4月の案件分割の線形分類器 macro-F1 は0.832、多数クラス基準は0.021（各935文書）。旧0.851は比較値として撤回済みであり、丸め値の差0.019は別の割付・別の評価集合をまたぐため、漏洩の効果量には使わない。
- “April intake rerun for our review” は Vanessa → Michael（Cc Fatima）、4月17日11:48 PDT。報告 `MDR-STI-20250417-RM-01-R01` の “Case split reevaluation Apr 17.pdf” と “January score withdrawn April rerun.png” を添付する。14:00にこの3人で報告を確認し、0.832の引用に評価条件を付けると決めた。

## 5月の分割台帳の出所点検

- 5月23日14:00の週次研究会で Vanessa と Joshua は、次回比較への追加候補96文書（台帳96行）を点検した。4月固定評価の935文書とは別で、出所の直接確認68件と別名確認24件は候補のまま、出所不明4件は対象外。新しい比較の順位は保留し、4月の報告値はこの点検で再計算しない。
- 窓口別の直接確認／別名確認／出所不明は Web form 33／6／1、Shared inbox 19／11／2、Service desk relay 16／7／1（計40／32／24文書）。不明4件の理由は受領記録への参照欠落2件・別名対応の裏付け不足2件で、月末も対象外。
