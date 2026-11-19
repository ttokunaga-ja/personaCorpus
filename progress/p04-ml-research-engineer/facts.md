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

## 長文点検記録の対象確定と区画の固定

- 2025-07-28 14:00 PDT、Vanessa・Michael・Fatima・Joshua が対象を確認。Michael が長文点検記録2,400件・8分類の研究開始を承認し、短文課題と別の母数で管理する。開始提案は `MDR-LDI-20250725-RP-01-R01`、対象文書説明は `MDR-LDI-20250731-DD-01-R01`。
- 1通目 “Which inspection records count” は Vanessa → Michael、7月25日10:36 PDT。“Inspection classification for July 28.pdf” を添付。2通目 “Re: Which inspection records count” は Michael → Vanessa、7月28日15:16 PDT。1通目を引用し、返信側に新しい添付はない。アドレスは既出の `vanessa@mercer-document.example` と `michael@mercer-document.example`。
- 1記録は点検票1件。冒頭の状態、途中の所見、追記、最終処置、署名を含む本文を使い、台帳の系列キーと構造化正解ラベルは入力の外。正解は署名時に有効な処置を表す8分類のうち1つで、取り消された指示や過去の引用と区別する。
- 分類順は Repair required / Repeat inspection / Remove from service / Engineering review / Routine closeout / Monitor at next service / Record correction / No equipment finding。先頭4分類を要処置とする。判定手引きは `MDR-LDI-20250805-GD-01-R01`。
- Joshua と Fatima が8月7日に、同じ機器の原点検・再点検・訂正でつながる系列を分けず、学習1,440／開発480／評価480件で固定。区画間の共有系列は0で、固定後の窓の検討は開発区画で行う。
- 上記分類順の支持は、学習190／150／110／90／330／240／140／190、開発62／52／36／30／112／80／46／62、評価64／48／38／30／112／78／48／62。評価の要処置は180件。
- 本文長の帯を2,048以下／2,049〜2,560／2,561以上（トークン）とすると、学習402／278／760、開発133／94／253、評価136／92／252件。

## 点検記録の切詰め位置の読み合わせ

- 根拠対照メモの8月22日初稿は `MDR-LDI-20250822-RM-01-R01`。Fatima の8月26日の指摘入り控えは初稿の番号と発行情報を保持し、Vanessa の8月29日改訂版は `MDR-LDI-20250822-RM-01-R02`。改訂で窓の根拠残存と分類器の予測を別欄にした。
- 9月11日14:00 PDT の研究会は Vanessa・Fatima・Emily。板書は “First 2048” と “First 1536” + “Last 1024” の2本、下の帯の中段に空白を置き “Same budget?” と書いた。評価結果はこの時点で未取得。
- 10月の比較条件は冒頭2,048トークンと先頭1,536＋末尾1,024トークン。本文が各予算内なら順序どおり全体を1回だけ渡し、予算を超えた場合は各窓を重複なしで取り、両端条件では元の順に連結する。本文予算とは別の共通制御トークン、固定モデル、トークン化、評価区画、参照ラベルは同じ。

## 10月の見落とし比較と費用の確認

- 固定評価480件のうち、要処置4分類が正解の180件で、残り4分類を予測したときだけ見落としとする。冒頭のみ見落とし28件、両端のみ2件、両方9件、どちらも見落とさず141件。条件別の見落としは37件と11件。
- どちらかで見落とした39件について、全本文で有効な根拠句の末端を本文トークン位置の4帯（0〜25%、25%超〜50%、50%超〜75%、75%超〜100%）で集計。冒頭だけで見落とした28件は0／0／7／21、両端だけの2件は0／0／2／0、両方の9件は2／4／1／2で、帯計は2／4／10／23件。
- 両端で新たに見落とした2件は、修理指示自体が中段にある記録と、再点検の期限を決める条件節が中段にある記録。8月の開発例とは別の固定評価の記録。
- Brandon の費用比較 `MDR-LDI-20251013-AW-01-R01` は同じ480件を推論する各1回分、同じ機器・並列度・単価での相対指数。冒頭条件を100とし、アクセラレータ80→98、ホスト処理12→12、入力準備8→8で合計118（18%増）。
- 10月14日14:00 PDT、Vanessa と Brandon は37→11件、相対費用18%増を確認して用途採用を保留した。位置と予算を切り分けるため、各予算内で冒頭2,048対先頭1,536＋末尾512、冒頭2,560対先頭1,536＋末尾1,024を比べる。
- Emily は同一モデルで入力窓を点検、Fatima と Vanessa は逆方向の見落としも根拠位置別に読み、Brandon は同じ測定境界と単価で費用を取る。10月末時点で追加の2条件は未測定で、観測済みの2条件と合わせると比較は4条件。

## 短文の誤分類の読み合わせと考察の改稿

- 2026年1月12日14:00 PST、Vanessa Nguyen と Nicole Foster がサクラメントで160文書を読み合わせた。2025年4月の開発区画935文書について保存した線形分類器の開発時の予測から、正解と予測の組合せを見られるよう選んだ誤分類の部分集合。
- 根拠句と否定が掛かる箇所を、受信本文を読んでから正解・予測と突き合わせた。160件は固定評価935文書やその誤分類の件数、割合を表さず、既存ラベル・採点・特徴の変更はしない。
- 合議で否定の作用域に関わる41件は、動作の拒否18、属性の否定14、代わりの依頼9。明示された別の動作を求めるものを「代わりの依頼」、動作自体の拒否と日付など属性だけの拒否は別区分とし、1文書は1区分に数える。
- 合議後の上記3区分の、両者の当初の作用域フラグの一致／不一致は順に12／6、10／4、7／2（計29／12）。残る119件は102／17で、同じ原因とみなさず文面ごとの根拠を記録する。
- 考察の旧稿は `MDR-STI-20260108-RM-01-R01`（1月8日）、一括説明を撤回した改稿は同系列の `MDR-STI-20260108-RM-01-R02`（Issued 2月6日）。41件は読解上の観察で、モデル内部の原因を実証したものではない。
- 独立した連絡 “Reading the intake misses on Monday” は Vanessa → Nicole（Cc Fatima）、1月9日10:38 PST。Jan 12 reading packet notes.pdf を添付。
- 独立した連絡 “Scope examples from Monday” は Vanessa → Nicole、1月15日11:12 PST。What the no belongs to.pdf と Scope decisions by first read.tsv を添付。

## 編集者向け初稿の比較条件点検と送付日変更

- 初稿は `MDR-STI-20260420-RM-01-R01`（Issued 4月20日）。Emily Chen の4月23日のコメント付き控えは発行情報を保持し、6項目を本文に補った4月29日内部稿は `MDR-STI-20260420-RM-01-R02`。
- 4月23日14:00 PDTに Vanessa・Emily・Michael Serrano が点検し、欠けた比較条件を1シートで補うまで初稿の送付を3週間延ばすと決めた。当初4月24日、新しい受領予定5月15日。4月末は内部点検まで。
- 6項目の順は (1) 案件分割と非重複 (2) 共通の採点対象と参照ラベル (3) 受信本文のみの入力 (4) 開発区画だけでの特徴・設定選択 (5) 学習多数クラス基準 (6) 12分類の macro-F1 算定。値は既存の「案件単位の再割付と採点条件」「4月17日の再評価報告」に従う。
- 条件表は `MDR-STI-20260427-AW-01-R01`（Issued 4月27日）。Vanessa が根拠を照合して6行を埋め、Emily が4月30日までに改稿中の反映箇所を確認。固定評価の比較と開発誤分類の読解を分けた。
- 1通目 “Moving the short intake draft to May 15” は Vanessa → Eric Sandoval（`eric@ellis-technical.example`）、4月23日15:12 PDT。2通目 “Re: Moving the short intake draft to May 15” は Eric → Vanessa、4月24日09:34 PDT。1通目を引用して新日程を了承する。両方とも添付なし。
- 独立した連絡 “Six conditions to check in Methods” は Vanessa → Emily（`emily@mercer-document.example`、Cc Michael）、4月28日09:48 PDT。Comparison conditions for the editor copy.xlsx を添付。

## 6月の低信頼度分類の保留閾値と読解

- 2026年6月の比較は正解付き受付本文800文書（Web form 360、Shared inbox 264、Service desk relay 176）。同じ文書ID、参照ラベル、保存済みの最上位予測・未丸め信頼度を使用し、保留適用前の誤分類は100件。2024年7月の正解なし Hold 126件は含めない。
- cutoff 未満を保留、同値を自動判定。0.450ちょうどは4件で 0.60→0.45 で保留から自動へ、0.600ちょうどは4件で両条件とも自動。0.60も0.45も予測ラベルは振分けによって変わらない。
- 0.60条件は保留144/800（18%）、誤分類捕捉22/100（22%）、自動判定656件（誤り78）。0.45条件は保留72/800（9%）、誤分類捕捉15/100（15%）、自動判定728件（誤り85）。捕捉率の分母100は両条件で共通。
- 0.60から0.45に下げると保留継続72件（誤り15・正解57）、新規自動72件（誤り7・正解65）、自動継続656件（誤り78・正解578）、新規保留0件。新規自動の経路別は Web form 31/誤り2、Shared inbox 24/誤り4、Service desk relay 17/誤り1。
- 6月8日14:00 PDTに研究会室で Vanessa と Emily が800件の比較条件を確認し、新規自動72件のうち誤り全7件と各経路から正解の対照1件ずつ（計10件）を読んだ。二人は受信本文の根拠を先に選び、その後に保存予測と参照ラベルを突合した。
- 戻る誤りは参照 Reschedule／予測 Schedule check が3件（CA-281・CA-342 は Shared inbox、CA-718 は Web form）、参照 Job status／予測 Cancel request が2件（CA-614 は Service desk relay、CA-547 は Shared inbox）、参照 Quote request／予測 New work request が2件（CA-390 は Web form、CA-433 は Shared inbox）。
- CA-281 は “Thursday instead” が変更依頼で月曜の確認は背景。CA-614 は以前の取消に触れた後の “Has the pump repair been signed off?” が現在の照会。CA-390 は “what a replacement seal would cost” が見積依頼で作業は未発注。
- 条件メモの初稿は `MDR-CA-20260603-RM-01-R01`（Issued June 3）。6月4日の Emily のコメント付き控えは初稿の発行情報を保持し、境界値の包含と捕捉の分母を指摘。確認後の改稿は同系列の `MDR-CA-20260603-RM-01-R02`（Issued June 9）。
- 1通目 “Monday - read what comes off hold” は Vanessa → Emily（`emily@mercer-document.example`）、6月5日10:22 PDT。What counts as an error caught.pdf を添付。2通目 “Re: Monday - read what comes off hold” は Emily → Vanessa、同日11:08 PDT。1通目を引用し、添付なしで6月8日の読み方を確認する。

## 6月の用途判断と次回の確認条件

- 6月11日14:00 PDTに Vanessa、Michael Serrano、Emily、Brandon Romero が研究会。Michael は用途採用を保留し現行の振分けに変更を入れない。Emily は次回の比較条件、Brandon は誤振分けの復旧担当・計測点、Vanessa は独立した次回評価案を6月19日までに持ち寄る。
- 6月19日時点では、検知から正しい担当への受渡しまでを測る案を残した。正しい担当が受領した時刻の取得方法と許容する作業負担は未合意。次回の独立評価と復旧時間の計測はいずれも未実施。
- “June routing review - measures and movement” は Vanessa → Michael（`michael@mercer-document.example`）・Brandon（`brandon@mercer-document.example`）、Cc Emily、6月12日09:26 PDTの独立した連絡。What counts as an error caught.pdf と Where the decisions moved.tsv を添付。

## M-14の7月評価集合と候補の選択

- 2026年7月の正解付き6,000受信本文を、受領記録と確認済み別名で裏付けた窓口横断案件キーで学習3,600／開発1,200／評価1,200文書に分けた。7月7日に Joshua が3組の共有案件各0、キー欠落0文書、重複採点ID 0件を確認した。
- Web form／Shared inbox／Service desk relay の順で学習1,642／1,125／833、開発534／382／284、評価515／401／284文書。過去の評価区画、2025年5月の候補、2026年6月の保留比較から持ち込まない。
- 参照は2024年7月改訂の12分類基準。既定の分類順で評価の支持は160／120／110／100／80／120／90／85／75／70／95／95（計1,200）。配分単位は案件、採点単位は本文1通。正解不明の旧 Hold 126件は含めず、信頼度で評価文書を取り除かない。
- 同じ7月の学習集合・正則化付き線形分類器で、R173 は単語頻度のみ、R184 は単語頻度と文字3〜5グラムを使う。受信本文以外の案件キー・経路・件名・後付けの追記を特徴に使わない。いずれも評価文書に上位1分類を出す。
- 7月8日、Vanessa と Emily は同じ開発1,200文書の macro-F1（R173 0.848、R184 0.871、文字2〜5グラム案 R187 0.870）を確認。R187を開発で止め、評価を見る前に R173 を比較用、R184 を固定候補として設定を閉じた。

## 7月16日のR184固定と採点結果

- 2026年7月16日13:30 PDTに Vanessa と Fatima が分割・同一参照・採点を照合し、14:20に固定。各分類は 2TP/(2TP+FP+FN)、12分類の未丸め値を等重みで平均する。
- 同じ評価1,200文書で R184 0.873、R173 0.852、差+0.021 macro-F1。未丸め平均は約0.872784697／0.852005305、差は約0.020779392。報告値と差はそれぞれ最後に小数第3位に丸める。
- R184が下がった分類は Reschedule、Send document、Correct record。Reschedule は支持100、R173 0.888（FN 9）、R184 0.829（FN 20）。FNの差11は両ラン間で入れ替わった文書数を表さない。
- 条件表は `MDR-STI-20260716-AW-01-R01`、固定報告は `MDR-STI-20260716-RM-01-R01`（ともに Issued July 16, 2026）。2025年4月の0.832と当時の多数クラス基準0.021はいずれも別の935文書の結果。

## R184の口頭発表資料と許可

- 発表資料は `MDR-STI-20260715-LM-01-R01`（Issued July 15）の結果先行案から、Emily の7月15日14:00 PDTの読み合わせを経て、同系列 `MDR-STI-20260715-LM-01-R02`（Issued July 16）で集合・比較条件を先にした。
- 1通目 “R184 figure and wording for the talk” は Vanessa → Michael、Cc Fatima、7月16日16:18 PDT。Intake talk - conditions before the score.pptx、R184 on the same July set.png、July 16 R184 frozen results.pdf、R184 run comparison.tsv を添付。PDF と TSV は社内の判断資料。
- 2通目 “Re: R184 figure and wording for the talk” は Michael → Vanessa、Cc Fatima、7月16日17:11 PDT。1通目を引用し、返信自体の添付なし。Michael が許可したのは Applied Document Research Forum の口頭発表での R02 と 0〜1軸の指定図、同じ1,200文書の評価条件・差・分類によるばらつきの説明。
- 本文例・固定報告・集計表の外部配布、会場外での再公開、用途採用・運用効果の主張は許可の範囲外。6月に保留した用途判断はこの評価で進めず、誤振分けの復旧時間は未計測。

## 7月21日の発表練習

- 7月21日14:00 PDTから研究会室で15分間、Vanessa・Michael・Fatima・Emily が通し練習。R02 と指定図を保持し、差の読み方を “a difference of 0.021 in macro-F1 on this evaluation” にそろえた。
- 前年値を尋ねられたときは別の評価集合と説明し、前年の数値は口頭発表に加えない。ノート付きの会場配布資料は作らない。

## 8月の長文点検記録の二度読み比較

- 2026年8月4日に比較条件を固定。2025年の学習・開発・評価と系列を共有しない評価4,000原記録を同じ参照ラベル・モデル・トークナイザー・制御トークンで採点し、入力は本文のみ。2025年10月の追加の同予算条件は今回の結果に含まれない。
- 冒頭のみは全記録に先頭最大2,048本文トークンを1回。二度読み案は同じ初回のあと、切詰め前の本文が3,200トークンを超える記録だけを先頭1,536＋末尾1,024で再推論し、2回目の予測で置き換える。3,200ちょうどは1回、再推論は評価で1,000／4,000件。
- 正解が要処置4分類で予測が非要処置なら見落とし、逆方向は誤警報。要処置の支持と見落とし数（冒頭のみ／二度読み案）は Repair required 350（11／7）、Repeat inspection 280（9／6）、Remove from service 210（7／6）、Engineering review 160（5／5）。
- 8月6日の要処置1,000件：冒頭のみ見落とし14、二度読み案のみ6、両方18、いずれもなし962。合計は32／1,000（3.2%）と24／1,000（2.4%）、低下0.8ポイント。救済／新規の分類別件数は上の順で6・4・2・2／2・1・1・2。
- 非要処置3,000件：誤警報は冒頭のみ90、二度読み案103（冒頭のみ4、二度読み案のみ17、両方86、なし2,893）。Fatima と Vanessa の翌日の再読では新規見落とし6件のうち、中段根拠の欠落4件、根拠が残っても旧処置を優先2件。

## 8月の原記録あたり計測と月額上限

- Brandon が8月3日に用途側の通常月100,000件、多い月120,000件、月額上限USD 4,500を確認。仕事量は受領した原記録数で、二度目の推論は追加費用に入れ、受領件数には足さない。上限は用途側の処理費に対するもの。
- Brandon の8月7日の計測は、採点集合とは別の同じ10,000原記録を両案で3反復、同一機器・並列度・測定境界。二度読み案は反復ごとに2,500件を再推論し、ホストとアクセラレータは両回の実使用時間を含む。
- 10,000原記録当たりの共通入力準備USD 40、ホストは平均30／40時間×USD 2、アクセラレータは平均16／24時間×USD 12.50（冒頭のみ／二度読み案）。1万件当たり全変動費USD 300／420。待機監視USD 600は変動費と別に月1回だけ加える。
- 8月7日初版 `MDR-LDI-20260807-AW-01-R01` は別用途80,000件と昼間の仮枠60,000件を混在させ、USD 3,000／3,120とした。8月10日に指摘し、8月11日発行の同系列 `MDR-LDI-20260807-AW-01-R02` では両案に同じ月間処理量を適用する。
- 同じ月間80,000／100,000／120,000原記録で冒頭のみUSD 3,000／3,600／4,200、二度読み案USD 3,960／4,800／5,640。通常月の増分USD 1,200はアクセラレータ1,000・ホスト200。アクセラレータ費だけ置き換えた月額4,600でも上限を100超える。
- 費用曲線は二度読み率25%と固定費600を保持して月の量で比例換算し、二度読み案は約92,857原記録で上限に達する。8月18日の別の感度計算は月100,000件固定で二度読み率を動かし、増分の比例を仮定すると18.75%で4,500。18.75%での費用も精度も未測定。

## 8月11日の用途保留と後続の確認

- 2026年8月11日14:00 PDT、Vanessa Nguyen と Brandon Romero が同じ月100,000原記録で費目を再計算し、二度読み案は上限をUSD 300超過と確認。研究結果は保持し、用途採用は保留して現行の担当者確認を続ける。判断文書は8月12日発行 `MDR-LDI-20260812-RM-01-R01`。
- 独立した事前連絡 “For two - August inspection costs” は8月11日13:06 PDT、Vanessa → Brandon。“Can the second read fit in August.pptx” を添付。
- 1通目 “Inspection routing stays on hold” は8月12日11:18 PDT、Vanessa → Michael、Cc Brandon。“Inspection use decision - August 11.pdf” と “August costs at the same monthly volume.xlsx” を添付。
- 2通目 “Re: Inspection routing stays on hold” は8月13日09:42 PDT、Michael → Vanessa、Cc Brandon。本文の引用・添付はなし。両通の Message-ID は順に `<b38e1b4d-f8c4-41dc-9ee4-840cf70d5327@mercer-document.example>` と `<8bfd5d87-d24d-4a6d-9dda-12e2d7cec825@mercer-document.example>`。2通目の In-Reply-To と References に1通目のIDを入れる。
- Brandon はピーク時の到着と費用の計測、Vanessa は遅延・誤振分け時の人への引き継ぎを8月25日までに論点化する。8月21日の指摘で初回予測を自動で返す案を外し、確認担当への引き継ぎは未実装の計測案に留める。
- 8月25日14:00 PDT、Vanessa、Michael、Brandon、Emily、Fatima が保留を維持。Brandon は時間帯別の到着・処理時間、Emily と Fatima は別系列での見落とし・誤警報、Vanessa は確認担当と復旧工数を追う。応答時間と許容負担の基準、再審議日は未設定。

## 編集者への短文受付論文の改訂稿返送

- Eric Sandoval は4月29日付の編集用 R02（`MDR-STI-20260420-RM-01-R02`）を9月8日に Word コメント2件（図だけで数値を確認できるか、要旨で160件が採点対象に見えないか）付きで返し、17日までに改訂稿と別紙回答を求めた。本文・発行情報は維持。元著者 Vanessa、最終更新者 Eric。
- Nicole Foster の指摘PDFは `HEI-20260909-MR-01-R01`（Issued September 9, 2026）。指摘 N1〜N9 の通し番号は PDF・回答書・箇所照合表で共通。9月14日14:00 PDT に Vanessa、Michael Serrano、Fatima Rahman が反映7件・対象外2件と送付範囲を決めた。
- N1 は R02 Methods / Partitions and reference → R03 Case Allocation。案件は割付単位で、学習2,804／開発935／評価935文書、区画3組の共有案件は各0。
- N2 は同じ R02 小節 → R03 Scoring Set。受信本文1通を採点単位とし、Hold 126文書は割付・採点外。両手法は同じ評価935文書・12分類の参照ラベルで採点。
- N3 は R02 Methods / Comparison and scoring → R03 Comparator。学習区画の最多分類 New work request を基準が全評価文書に予測することを明示する。
- N4 は同じ R02 小節 → R03 Macro-F1。各分類の未丸め F1 を等重みで平均して最後に丸める順を明示する。
- N5 は R02 Results の比較段落 → R03 Results / Figure 1。0〜1の共通軸・12分類 macro-F1、学習多数クラス基準0.021／単語頻度の線形分類器0.832、同じ評価935文書。注釈入り PNG はこの図の軸起点・値ラベル・母数を指し、掲載図は注釈なし。
- N6 は R02 Abstract と Methods / Reading selected errors → R03 Abstract と Methods / Diagnostic Reading。開発区画の保存済み予測から選んだ誤分類160件を診断に用いたことを先に示す。
- N7 は R02 Discussion → R03 Scope Examples。41件（18／14／9）は否定の作用域の読解上の観察で、残り119件や分類器の予測原因へ一般化しない。
- N8（R02 Results の比較段落 → R03 Figure 1）は対象外。7月 R184 は別評価1,200文書・別比較対象で、4月の比較図に追加すると同一集合の差と読める。N9（R02 Partitions and reference → R03 Case Allocation）は対象外。窓口横断キーの対応表は内部資料として保持し、文書には共有案件数の集計を示す。
- 送付用改訂稿は `MDR-STI-20260420-RM-01-R03`、回答書は `MDR-STI-20260917-RM-01-R01`、9件の箇所照合表は `MDR-STI-20260917-AW-01-R01`（いずれも Issued September 17, 2026）。ローカルの改訂稿PDFは同じ R03 の Word 出力。
- 独立した受領メール “Short intake copy back from Ellis” は9月8日09:21 PDT、Eric → Vanessa。“Short intake paper - Eric's comments.docx” を添付。
- 独立した受領メール “Nine final points on short intake” は9月9日10:18 PDT、Nicole → Vanessa。“Short intake final methods comments.pdf” を添付。
- 1通目 “Short intake revision for Ellis” は9月17日11:26 PDT、Vanessa → Eric。“Short intake paper - Sep 17.docx”、“Response to the final methods comments.docx”、“Locations of the September edits.xlsx” を添付。ページ確認用PDFと注釈画像は手元用。
- 2通目 “Re: Short intake revision for Ellis” は9月18日09:46 PDT、Eric → Vanessa。3添付の受領と編集工程への引き渡しを知らせる。本文の引用なし、新規添付なし、採否は未定。
- 改訂稿返送2通の Message-ID は順に `<b2e78d44-5287-4802-9e49-b311fb296fc3@mercer-document.example>`、`<6fabe157-f37d-4244-80f5-dd27a01b17b5@ellis-technical.example>`。返信の In-Reply-To・References は1通目のID。

## 点検記録論文と購買記録論文の査読打診

- 2026年9月21日14:00 PDT、Vanessa と Michael が二つの打診を確認。点検記録の共著者 Scott Marquez（Dawson Industrial Documents Inc）とは2025年11月に共同研究費申請を準備した関係があり辞退する。申請準備は採択・受給を意味しない。
- Fenwick Public Records Lab の購買記録論文は Nicole の示した著者・所属について、共著・共同作業・資金関係の重なりが確認されず、Vanessa が引き受ける。可否の返事は2件とも9月23日まで、引受分の査読期限は10月12日。
- 点検記録の件名は “Review request - inspection follow-up records”、論文名は “Finding follow-up actions in inspection records”。1通目は Nicole → Vanessa、9月21日09:07 PDT。2通目は同件名に “Re: ” を付けた Vanessa → Nicole、9月22日09:22 PDT の辞退。
- 点検記録の Message-ID は順に `<1981d797-66e8-4c0c-9a01-eace80fa4bb2@hartwell-evaluation.example>`、`<f16c7ea6-94b5-438d-93ee-75f078793fc8@mercer-document.example>`。2通目は1通目を引用し In-Reply-To・References に1通目のIDを使う。両通とも添付なし。
- 購買記録の件名は “Review request - purchase record line grouping”、論文名は “Line grouping in municipal purchase records”。1通目は Nicole → Vanessa、9月21日10:12 PDT。2通目は同件名に “Re: ” を付けた Vanessa → Nicole、9月22日10:08 PDT の受諾。
- 購買記録の Message-ID は順に `<783ceba6-96da-4a0c-a2f3-d54bf1765d95@hartwell-evaluation.example>`、`<c972656e-022e-4dda-9d36-e8c3576a2cf5@mercer-document.example>`。2通目は1通目を引用し In-Reply-To・References に1通目のIDを使う。両通とも添付なし。
- 担当登録の独立した連絡 “October 12 review assignment confirmed” は9月23日09:36 PDT、Nicole → Vanessa。Fenwick の締切は10月12日、論文へのアクセスは後続の個別案内。添付なし。
