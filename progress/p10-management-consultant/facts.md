# Nicholas Hargreaves — 事実の台帳

## Whitfield 日本事業の予備調査と現行支払手順

- 2024-04-08、Kenneth Townsend と Nicholas Hargreaves が日本事業の仕入先支払14工程の予備調査を開始。Kenneth は入力の遅さを疑い、Nicholas はシステム更新を先送りし、承認待ちを先に確かめる方針を取った。
- 顧客の現行手順は Whitfield Components Ltd Japan Finance 発行、Kenneth 承認の WCL-JP-FIN-PAY-2024-001（v1.0、2024-04-08）。国内仕入先の通常支払が対象。給与・税金・緊急手動送金は対象外。
- 01 Receive invoice → 02 Check completeness → 03 Enter invoice → 04 Match PO and receipt → 05 Resolve exceptions → 06 Route for cost-centre approval → 07 Approve cost centre。
- 08 Review for Finance approval → 09 Approve in Finance → 10 Build payment proposal → 11 Check payment batch → 12 Authorise bank release → 13 Release payment → 14 Reconcile settlement。05 は該当時のみ通り、例外解消後は再照合してから回付する。
- 役割は AP が01〜06、費用責任者が07、Japan Finance が08・09・14、Treasury が10〜13。
- 07 は請求書・照合根拠・費用負担先が揃ってから、09 は費用責任者承認と例外解消の確認後、12 は支払バッチの照合と二重確認を終えてから承認者へ渡す。差戻し中は承認可能な状態から外す。13 の実行と14 の消込は12 の承認と別工程。
- 診断メールのアドレスは Nicholas が `nicholas@holt-reeves.example`、Kenneth が `kenneth@whitfield-components.example`、Neha が `neha@holt-reeves.example`、Connor が `connor@holt-reeves.example`。
- メール件名「Japan supplier payments — current procedure」。2024-04-08 09:25 BST Kenneth → Nicholas（Cc Neha）、現行手順PDFを添付。10:16 BST Nicholas → Kenneth（Cc Neha、Connor）、同件名に Re: を付けて先信を引用し分析の切り分けを回答（添付なし）。

## Whitfield 支払の面談と引渡条件の確認

- Neha Patel が 2024-04-15〜05-24 に18人を面談。重複なしで、AP 6人、事業部の費用責任者4人、Japan Finance 5人、Treasury 3人。Oliver Hewitt は聞き方と記録の補助。
- 2024-05-29 09:00 BST、Nicholas・Neha・Connor・Oliver がマンチェスターの Holt & Reeves 会議室で工程を検討（Nicholas が09:37に撮影）。14工程と3つの承認待ちの境目を板書し、受領箱の時刻と承認可能時刻を分けた。
- 6月3日の Neha の Evidence note は HRA-WHI-DIAG-EV-2024-001 v1.0。07・09・12 の引渡条件を記録し、面談証言と時刻で検証できる事実を区別。7月17日の結論も18人の証言を母集団の発生率には換算しない。
- 6月3日から6月14日まで、Nicholas と Paul Entwistle は Oliver の水曜の面談記録指導枠を守り、Practice の金曜の内部会は最後の20分だけ Oliver が出ることで調整した。

## Whitfield 支払記録の時刻照合と工程別集計

- Connor が許可済みの日本側支払履歴から、2024-04-08〜05-31 JST に受領し同期間に消込済みの40件を抽出。承認開始の対が欠ける2件を集計から外し、完全な38件（差戻しなし28件、あり10件）を採用。両群とも14工程の標準支払の範囲内。
- 期間は受領受付から消込完了まで。組にできる実作業区間を先に計上し、未作業の区間を滞留として重複なしで集計する。承認待ちは07・09・12の直前、資料が揃い当該承認者に回った時刻から、承認者がレビューを開始した時刻まで。承認そのものの作業、差戻し中の未作業、支払日までの待ちは区別する。
- 時刻は元の JST（`+09:00`）。集計は夜間・休日込みの経過時間（案件別の時間を加算、並行案件を暦の稼働時間に換算しない）。判定不能をゼロで補完しない。報告・会議・メールの日時は BST。
- 案件符号は WB-0401〜WB-0440。差戻しなしは0401〜0428、ありは0429〜0438、着手欠損で除外するのは0439・0440。抜粋3件は0417・0432・0439。
- 照合例の WB-0417 は2024-05-14 10:10–10:28 JSTに入力（18分）、同日10:35に費用責任者に回付、05-15 11:05にレビュー開始（待ち24時間30分）。入力終了から回付までの7分は承認待ちに含めない。
- 6月26日確定の HRA-WHI-DIAG-TIM-2024-001 v1.0 は38件合計で実作業76.0時間（うち03入力19.0時間）、滞留1,000.0時間（うち承認待ち610.0、その他390.0時間）。610.0 ÷ 1,000.0 = 61.0%で、受領から消込までの1,076.0時間を分母にしない。
- 承認待ちの内訳は07費用責任者340.0時間、09 Finance 220.0時間、12 銀行送金承認50.0時間。工程別時間表と整理後の工程図は、各待ち時間を対応する承認工程の直前に置く。表ではその承認工程の行に帰属させる。
- メール件名「Japan payment timestamps — cleared extract」。2024-06-05 10:46 BST Connor Atkinson → Nicholas（Cc Neha）。時刻抜粋 `Japan payment events - cleared extract.csv` を添付。抜粋は照合例3件のイベント列で、全38件の集計元は権限内の明細。

## Whitfield 入力速度仮説の改訂と診断報告

- Nicholas の説明案 HRA-WHI-DIAG-BRF-2024-002 は7月3日 v0.2。7月8日 11:10 BST の v0.3 は Elizabeth Hargreaves の赤字入り。入力主因という文を仮説に戻し、滞留を分母と明示し、現状担当者の責任と断定しないよう指摘。
- 7月17日 09:00–09:45 BST、Kenneth、Nicholas、Neha、Connor が診断会（Teams、英国・日本間）。Kenneth は07・09の引渡しから待ちを扱う方針に同意。12 の二重統制を維持し、Nicholas は Japan Finance と現行の引渡条件の担当を確認する。システム更新、金額試算、試行の設計と成果は今回の決定に含めない。
- Kenneth 向け投影資料は Nicholas の HRA-WHI-DIAG-BRF-2024-004 v1.0（7月17日）。同日の送付PDFはこの資料のスライドのみをそのまま出力し、診断会の合意は議事録と送信メールに残す。
- メール件名「Japan payments — diagnostic readout」。2024-07-17 11:26 BST Nicholas → Kenneth（Cc Neha、Connor）、`17 Jul - Japan payments diagnosis.pdf` を添付。14:08 BST Kenneth → Nicholas（同Cc）、同件名に Re: を付けて先信の対応箇所を引用し、次の段取りを確認（添付なし）。

## Marston の対象工場決定と顧客資料

- 2024-08-05 10:00 BST、Caroline Wood、Nicholas Hargreaves、Connor Atkinson が4候補から Trafford T2 と Warrington W1 を選定。同じ板紙 B420・箱仕様 S3 を扱い、投入と良品の重量を突合できることが条件。
- Rochdale は別仕様が中心、Preston は提出見本にシフト別投入重量がないため対象外。両工場とも根拠は選定時の資料目録と口頭説明で、現地訪問・実測はない。8週の当初報告日は9月30日。
- Marston 生産部門の現行標準は Caroline 発行 MCP-OPS-STD-2024-014 Rev 3（8月1日発効、8月6日に提供用出力）。切替・詰まり復旧の双方で初品3枚を照合し、班長承認後に良品に繰り入れ、切替と再始動のロスは別欄に記録。
- Caroline の6〜7月の材料費 CSV は購買重量 kg と純材料費 GBP。B420 は両工場とも6月 £0.62/kg、7月 £0.71/kg。B510 は別銘柄で、B420・S3 の歩留まり比較の対象外。
- 診断のメールは Caroline が `caroline@marston-carton.example`、Nicholas が `nicholas@holt-reeves.example`、Connor・Neha・Siobhan はそれぞれ `connor@holt-reeves.example`、`neha@holt-reeves.example`、`siobhan@holt-reeves.example`。
- 件名「Marston board costs and current work standard」、2024-08-06 10:16 BST Caroline → Nicholas（Cc Connor、Neha）。材料費 CSV と現行標準 PDF を実添付。

## Marston の面談延期と二工場での観察

- 当初の訪問は9月10日 Trafford、11日 Warrington。Trafford の引継ぎ担当が欠員を補い、Warrington は当番交代前に朝夕の班長を揃えられず、両班を見るため翌週17日 Trafford・18日 Warrington の後半枠へ変更。
- 件名「Marston shift interviews — change of dates」、2024-09-09 09:04 BST Caroline → Nicholas（Cc Neha）が変更を依頼。10:02 BST Nicholas → Caroline（Cc Neha）が Re: を付け、日勤だけなら可能との記述を引用して新しい訪問枠と報告日を返信。両通とも添付なし。
- 面談後の記録照合・経済性レビューと配布前確認の時間を確保するため、9月30日の枠は Caroline との途中確認に転用。最終報告を10月7日へ1週移し、診断は8週予定に対して9週。対象は同じ2工場。
- Neha と Oliver が9月17・18日に両班長と面談。Trafford T2 は初品3枚の照合をしているとの説明だったが、17日14:26 BST の一例で復旧直後に良品へ流れ、再始動分が先のセットアップ欄へ合算された。
- Warrington W1 は18日15:04 BST の一例で保留・照合・班長承認・ロス別記を確認。訪問時の各1例は9月の定性証拠であり、7月の集計にも順守率の推定にも使わない。Neha の照合記録は HRA-MAR-COST-EV-2024-001 v1.0。

## Marston の価格差と歩留まり比較

- Connor の HRA-MAR-COST-YLD-2024-001 v1.0 は7月の B420・S3 の比較可能な各12ラン。両工場の投入各30,000 kgに対し、良品は Trafford 26,400 kg（88.0%）、Warrington 27,600 kg（92.0%）。重複投入票や別銘柄は除外。
- 各工場のロスは切替／再始動／その他の順に Trafford 1,020／780／1,800 kg、Warrington 600／480／1,320 kg。現場記録で分類を照合し、同一の重量は一つの区分だけに入れる。
- 標準統一で確かめる機会量は切替差420 kg＋再始動差300 kg＝720 kg、対 Trafford 投入30,000 kgで2.4ポイント（88.0%から推定90.4%）。その他差480 kg＝1.6ポイントは推定に含めない。
- 購買単価差 £0.09/kg の影響例は固定30,000 kg当たり £2,700。重量比の改善余地から価格の影響を分離し、720 kgを月間・年間の削減実績や購入減とみなさない。

## Marston の給紙機更新の審査と標準統一の提言

- Deepa の HRA-MAR-COST-CAP-2024-001 v1.0 は Trafford T2 給紙機のみ。初期費用は本体 £326,000＋据付 £38,000＋立上げ検証 £20,000＝£384,000。
- 8月6日、Carolineが材料費抜粋を送ったメール本文に、設備対象の通過量を生産記録から併記した。6〜7月のCSVは同じ価格契約で抽出した2工場の購入行だけを含む。T2の全契約の購入量や、給紙機を通った全仕様のB420投入量を示すものではない。Trafford T2 の生産払出・戻入台帳から、給紙機を通ったB420の正味投入（戻入・再投入の重複を除く）を抽出する。2023年8–10月952,000、11月–2024年1月1,067,000、2–4月987,000、5–7月1,012,000 kg、計4,018,000 kg。翌年も同程度に稼働する場合の試算として十万kg単位に丸めて年4,000,000 kgを使う。
- 年間総便益は、標準統一と重複する分を外した材料 £49,600（年間対象4,000,000 kg × 設備で追加回収2.0% × 固定単価 £0.62/kg）＋外注・時間外の回避 £23,400＝£73,000。7月の720 kgの年換算は足さない。
- 年維持費 £13,000 を控除し年純便益 £60,000、単純回収6.4年。6年に必要な純便益は年 £64,000。総便益のみ年 £12,000 下振れした試算では純便益 £48,000、8.0年。
- HRA-MAR-COST-BRF-2024-003 は9月25日 Siobhan の v0.2（総便益で5.3年を提示）、翌26日10:23 BST の v0.3 は Nicholas の赤字。元を残し純便益と下振れの試算を確認して設備判断を保留。
- 10月2日 Siobhan・Neha と Caroline の班長候補が、両工場で共通の初品承認時刻・投入重量・良品重量・切替／再始動ロスを各シフトで記録する4週間の案を机上確認。10月4日 Elizabeth はこの条件付き提言の客先説明を了承。
- 10月7日10:00〜10:45 BST、Caroline、Nicholas、Siobhan、Connor が最終報告会。投影版は Siobhan の HRA-MAR-COST-BRF-2024-007 v1.0。Caroline が班長を指名してから4週間の開始日を決め、設備見積りの次段階には進まない。
- 件名「Marston cost diagnosis — readout and next steps」、2024-10-07 12:07 BST Nicholas → Caroline（Cc Siobhan、Connor）。回覧PDFを実添付し、10月11日までに両工場の班長名を求める。報告会当日は班長指名と試行開始日が未確定。

## Whitfield 日本事業の年換算見込の照合と訂正

- 積算の数量は Japan Finance の2023年7月〜2024年6月の締め済み四半期集計。通常確認 AP-CHECK 年16,000件、独立した確認連絡 AP-CALL 年3,000件、例外書類取りまとめ EXC-PACK 年2,000件、承認済みの残業割増 OT-PREM 年1,000時間。
- 旧試算 HRA-WHI-DIAG-EST-2024-001 v0.3 の元行は AP model!D12/D13/D14、Overtime!D8、Finance model!D11 の順（保管ブックの Original estimate 7〜11行）。D12・D13 は各7.5分・30分、D14 と Finance model!D11 は各30分の削減仮定に作業価値 JPY 4,000/時を掛ける。Overtime!D8 は労務の基礎単価を含めず、独立した割増部分 JPY 4,000/時の全1,000時間を回避可能と仮定。
- 2024-10-15 09:00〜09:45 BST、Nicholas・Connor が旧年額 JPY 26m を照合。部署・名称で突合した初回は漏れたが、AP model!D14 と Finance model!D11 は同じ EXC-PACK の対象期間・2,000件・30分で、AP がまとめた同じ束を Finance が受けるだけと確認した。AP側を残し Finance側の JPY 4m を控除する。
- 10月15日に AP の準備記録と Finance 側の受領控えから照合した例は EP-184・EP-219・EP-267 の3束。準備作業と受領の区別を確かめるための例で、年2,000件は四半期集計の値。
- 訂正内訳は HRA-WHI-DIAG-REC-2024-001 v1.0（10月15日）。年 JPY 8m＋6m＋4m＝18m は再配分可能な作業余力の評価、4m は独立した残業割増回避の見込み、計 JPY 22m は安定運用時の年換算。実績、人員削減額、初年度の金額として使わない。
- 件名「Basis for the Japan savings figure」は10月8日09:12 BST Kenneth → Nicholas（Cc Connor、添付なし）。Japan Finance の部内説明に載せる前に旧年 JPY 26m の前提を尋ねた。
- 件名「Correction to the Japan savings estimate」は10月15日13:42 BST Nicholas → Kenneth（Cc Connor、Neha）。訂正説明 HRA-WHI-DIAG-NTE-2024-001 v1.0 の PDF を実添付し、部内説明の差し替えを依頼した。
- 10月16日09:07 BST Kenneth → Nicholas（同Cc）が「Re: Correction to the Japan savings estimate」で先信を引用し、部内説明と自身の説明メモの差し替え、一時費用と時期の説明を依頼（添付なし）。10月21日09:00 BST、Kenneth・Nicholas・Connor が両資料の差し替えを確認。

## Whitfield の段階導入と費用の見積り

- 2024-11-13、Neha が Nicholas・Siobhan・Connor と同時導入案を2例で机上検討。07の費用負担情報欠落は AP、09の例外解消証跡欠落は Japan Finance に戻す。07を先行し、09への引渡しには07の承認と例外解消の記録を要する。12の Treasury の二重確認は維持する。
- 初期費用の見積り HRA-WHI-DIAG-CST-2024-001 v0.2（Connor、12月4日）。着手承認を起点とした M1＝最初の月。設計・統制確認3m（M1）、設定・権限検証7m（M2）、教育・代替要員5m（M4）、安定化支援3m（M7）、計 JPY 18m。
- 2025-01-06 09:00 GMT、Kenneth・Nicholas・Neha・Siobhan が着手条件を確認。Kenneth は07→09の証跡と現行経路へ戻す条件の提示を求め、顧客側の担当指名と着手日は持ち帰った。

## Whitfield 実行案の初年度表示と Elizabeth の審査

- 内部実行案 HRA-WHI-DIAG-BRF-2025-001 は Nicholas の1月10日 v0.3、Elizabeth の1月13日11:26 GMT 赤字入り v0.4、Nicholas の1月20日 v1.0。回答 HRA-WHI-DIAG-REV-2025-001 v1.0 と書き込み用 PDF は v1.0 のスライド番号で参照する。
- v0.3 の3枚目は「First 12 months — JPY 22m savings」。Elizabeth の指摘で v1.0 は初年度額を未算定とし、3枚目で年換算 JPY 22m の内訳、4枚目で条件付き一時費用 JPY 18m、5枚目で承認された着手から効果実現まで9か月の計画値、6枚目で顧客説明の審査と着手判断を分けた。
- 初年度実績の判定には着手日、着手前4週の基準候補、比較対象件数、導入後の観測期間・作業時間・承認済み割増支出が要る。承認日と実績は未確定で、9か月から初年度額を機械的に按分しない。
- 件名「Papers for the implementation review」、2025-01-20 09:16 GMT Nicholas → Elizabeth（Cc Connor、Neha、Siobhan）。指摘への回答 DOCX を実添付し、書き込み用 PDF は会議室で手渡す。
- 2025-01-20 10:00〜10:45 GMT、Elizabeth・Nicholas・Connor・Neha・Siobhan の審査で、Elizabeth が修正後の顧客説明方針を了承。顧客側の着手承認・担当指名・基準期間が揃うまで開始日は保留。
- 件名「Before we date the Japan changes」、1月20日14:12 GMT Nicholas → Kenneth（Cc Neha、Siobhan、添付なし）。顧客側の担当指名と着手前4週の記録を揃えられる時期を照会。

## Pioneer の仕入先選定と全社再交渉の旧案

- 2025-04-07 14:00 BST、Meera Kulkarni・Nicholas Hargreaves・Connor Atkinson が開始会議。Meera は診断を取引上位12社に限定し、42社の一斉再交渉案（HRA-PIO-BUY-BRF-2025-001 v0.2）を見送った。
- 2025-01-01〜03-31に計上した税抜 USD の請求と貸方を仕入先IDで合算する。純仕入額の降順、同額はID昇順。母集団42社のうち1〜12位が診断対象、13〜42位は母集団に残す。
- S042 の別表示名2件の請求 USD 121,000（AP-48106）と USD 78,000（AP-49271）に貸方 USD 9,000（AP-49308）を反映すると11位 USD 190,000。12位 S018 は AP-48962 の USD 186,400、13位 S021 は AP-49115 の USD 182,900。
- 4月7日は仕入先IDによる暫定順位を使用し、4月9日の名寄せと4月11日の確定表 HRA-PIO-BUY-SEL-2025-001 v1.0 でこの順位を確認した。取引明細CSVは境界と別名義の抜粋であり、42社を数える元は Q1 全明細。

## Pioneer の現行例外要領と緊急申請の照会

- 現行要領は Meera 発行の PTD-BUY-EXC-2025-001 Rev 2、2025-01-06 発効。理由・必要日・数量・依頼元・根拠を記入し、緊急も Buying の判断と時刻を残してから発注する。需要初認時刻の照合や7月の先行共有票は現行の必須手順に含まれない。
- Meera は `meera@pioneer-trade.example`、Nicholas、Neha、Connor、Paul、Oliver はそれぞれ `nicholas@holt-reeves.example`、`neha@holt-reeves.example`、`connor@holt-reeves.example`、`paul@holt-reeves.example`、`oliver@holt-reeves.example`。
- 「The procedure Buying uses today」、2025-04-09 13:40 BST、Meera → Nicholas（Cc Neha）。現行要領 `Buying exception requests - evidence and approval.pdf` を実添付。
- 例外は2025-04-01〜06-30に受付、全仕入先横断で一意の申請IDを数える。93件のうち申請時に Urgent が37件、非緊急が56件。再送は同一IDに束ね、時刻は Pioneer の現地表示 EDT を使用。
- 需要初認は数量と必要日が確認できる最初の証跡。通常手配の基準は必要日の前営業日15:00 EDT。締切前に需要が判明し、Buying への初回共有が締切後だと確認できたものを計画遅れ、締切後の新規・増量を突発需要、発注済み分の供給変更を供給側変更、根拠が足りないものを証拠不足とする。
- 7月8日の暫定内訳は計画遅れ22、突発需要8、供給側変更4、保留3。7月10日の照会回答を反映した確定内訳は23、8、4、証拠不足2（合計37）、明細は HRA-PIO-BUY-EXC-2025-001 v1.0。非緊急56件の原因内訳は集計していない。
- EX-052 は申請理由「stock needed today」、6月11日08:42 EDTの申請が Buying への初回共有。6月10日14:20 EDTに補充記録 RP-6118 で240ケース・必要日6月11日が揃い、前営業日15:00の締切より40分前と確認できたので、保留から計画遅れに移す。
- EX-077 は申請理由「same-day top-up」、6月24日08:31 EDTに申請。08:13 EDTの転送より前の数量確定時刻が取れず、証拠不足。もう1件の EX-088 は必要日が口頭で変わり原記録から確定時刻を復元できず、証拠不足。
- 「Two requests with dates I cannot place」、2025-07-09 13:23 BST Neha → Meera（Cc Nicholas）。2025-07-10 14:18 BST Meera → Neha（Cc Nicholas）が「Re: Two requests with dates I cannot place」で両件の問いを引用し、EX-052 の数量・必要日と EX-077 の転送元の欠落を回答。両通とも添付なし。

## Pioneer の需要共有案と Meera の判断

- HRA-PIO-BUY-BRF-2025-005 は Nicholas の7月11日 v0.4、同日13:18 BSTの Elizabeth 赤字入り v0.5、Nicholas の7月16日 v1.0。曖昧な「late demand」を v1.0 で「Known before cut-off, shared late」に替え、23・8・4・2の分母37を示す。7月16日配布PDFは v1.0 と同じ版。
- 7月15日11:00 BST、Nicholas・Neha・Siobhan が EX-052 と EX-077 で机上確認。提案は依頼元の数量・必要日・需要初認時刻を前営業日15:00 EDTまでに Buying へ渡すこと。締切後の突発需要と供給側変更には現行例外経路を使い、時刻不明は Buying に確認する。
- 「Papers for the buying discussion」、2025-07-16 13:29 BST、Nicholas → Meera（Cc Neha、Connor）。v1.0 の `16 Jul - Demand sharing first.pdf` と別添集計 `15 Jul - Coverage and urgent breakdown.xlsx` を実添付。
- 2025-07-16 14:00〜14:40 BST、Meera・Nicholas・Neha・Connor。Meera は4月の全社再交渉見送りと12社の診断範囲を維持し、需要共有の詰めを先行することに同意。Meera が Buying の窓口を指名し、Neha が共有票を詰める。開始日と窓口名は未確定。
- 23件は当時の証跡で需要の共有が遅かった件数。先行共有で回避できる件数、削減額、運用後の実績はこの診断では測っていない。

## Whitfield の2部門試行の着手と引渡し

- 2025-07-17 09:00 BST、Kenneth、Nicholas、Neha、Siobhan。通常国内仕入先支払の Components・Packaging の2費用負担部門を選び、8月15日に正副担当と権限を確認できれば18日から07側を開始と決定。給与・税金・緊急手動送金は対象外。
- 8月6日 Manchester の説明会は Nicholas・Neha・Siobhan と日本側の AP・費用責任者・Japan Finance。Nicholas は10:34 BSTに白板を撮影。8月15日に Packaging の07代理を含め、両部門の07側が準備済みと確認。
- Japan Finance 発行・Kenneth 承認の試行手順 WCL-JP-FIN-PAY-2025-001 v1.0 は8月11日付。8月18日は07側を開始し、8月22日09:00 BSTの Kenneth・Nicholas・Neha・Siobhan の証跡確認を経て、25日以降に初回06を通る試行案件から09側を適用。進行中は初回の経路で完結させる。
- AP は請求書・照合根拠・費用負担先を揃え06で07へ回し、不備を引き取る。08の Japan Finance は07承認と該当する例外解消証跡を確認して09に渡す。09の不足は08が引き取り AP などに訂正を求める。再回付は別に記録し、12の Treasury 二重確認は WCL-JP-FIN-PAY-2024-001 v1.0 どおり。
- 件名「Desk instructions for 18 August」、2025-08-11 10:26 BST Kenneth → Nicholas（Cc Neha、Siobhan）。8月11日の試行手順PDFを実添付。

## Whitfield の試行前後の観測窓と保留案件

- 初回適格回付06の日付（JST）で区分。基準は2025-07-21〜08-17、試行は08-18〜09-14、両端の日を含む。試行には07側のみの立上げ週も含む。両群の追跡を10月1日17:00 JSTで凍結し、組入れ終了後の09も締めまでなら使う。共通締めのため組入れ日ごとの追跡可能期間は異なる。
- 起点は資料が揃った時点の最初の06、終点は承認済みを記録した09。経過は時刻差÷24時間（休日・夜間・差戻し中も含む）。再回付、請求書受信、承認着手、12・13・14の日時は両端の代用にしない。
- 67候補は基準33、試行34。締めまでに未承認の基準2／試行3、両端の時刻が揃わない基準1／試行1は平均から除外。閉じた保留は基準4／試行3で算入し、合計60件の内数。保留は処理手順上は承認待ち行列から外しても、算入する案件の経過時計は止めない。
- Connor の HRA-WHI-DIAG-PLT-2025-001 v1.0（10月3日）。基準は Components 15件96日、Packaging 15件90日。試行は Components 15件60日、Packaging 15件54日。合計186日÷30件＝6.2日、114日÷30件＝3.8日（小数1桁）。完了案件の記述比較であり、未承認は別数、追加3部門の計測は未実施。
- 基準の Components は WP-B01〜B15 と B31・B33、Packaging は B16〜B30 と B32。試行の Components は WP-P01〜P15 と P31・P34、Packaging は P16〜P30 と P32・P33。各群01〜30が算入、閉じた保留は基準 B01・B04・B18・B23、試行 P01・P06・P24。
- 抜粋の完了例（以下 JST）は B01 07-23 09:00→07-29 15:00、B16 07-29 09:00→08-04 15:00、P01 08-18 09:00→08-22 15:00、P16 08-27 09:00→09-01 09:00。B16・P16 は保留なし。
- B01 は07から AP への差戻しで、費用負担先の訂正控えを照合。P01 のログは「Coding query」、8月20日に AP へ戻し、費用負担先根拠の訂正を確認して21日に再回付。P01 の4.25日には差戻し中も含め、08-22の09承認で終える。
- 締め時未承認は B31（AP、費用負担先の再確認）、B32（08、例外証跡待ち）、P31（AP、費用負担先の確定）、P32（08、例外解消証跡）、P33（08、07主担当不在時に当該案件を処理できる代理を確認中）。
- B33 は AP の日報で8月12日の初回06を確認できるが時刻がない。P34 は Finance の別控えで09承認済みと分かるが時刻がない。どちらも候補群への所属は確認済みで、日付や更新時刻から時刻を補わない。
- 許可済み抜粋は B01・B16・B31〜B33、P01・P16・P31〜P34 の11案件。列は Case ID、Cohort、Department、Event、Event time、Hold reason。Cohort は Baseline／Pilot、JST は +09:00、時刻欠損と未発生を区別し、未発生の承認イベントは作らない。
- 件名「Cleared pilot event extract」、2025-10-01 11:37 BST Kenneth → Connor（Cc Nicholas、Neha）。11案件の CSV を実添付。元の記録は権限付きの VDI 内。許可済み抜粋と分析用に匿名化した時刻明細は顧客・案件チーム限り。

## Whitfield の5部門への試行範囲変更

- 決裁スライド HRA-WHI-DIAG-BRF-2025-007 は10月3日 Nicholas の v0.3、10月6日 Elizabeth の赤字 v0.4、10月8日 Nicholas の顧客向け v1.0。赤字は2部門の観測、完了件数と保留・欠測の扱い、追加3部門で必要な準備を分けるよう求めた。配布PDFは v1.0 の表示面そのまま。
- 2025-10-09 09:00〜09:40 BST、Kenneth・Nicholas・Neha・Connor・Siobhan。既存 Components・Packaging を維持し、Tooling・Maintenance・Facilities を追加する承認。Siobhan と Japan Finance が10月17日までに部門別の正副担当・権限・証跡の引取を確認し、確認済みの追加部門のみ20日から適用する。未達の部門は従来経路を続ける。
- WCL-JP-FIN-SCA-2025-001 の基準は WCL-JP-FIN-PAY-2025-001 v1.0。Kenneth Townsend（CFO）と Elizabeth Hargreaves（Partner）が10月9日に署名。条件付き発効は10月20日。手順の配布版は WCL-JP-FIN-PAY-2025-002 v1.0（10月8日付、同じ発効条件）。次契約や追加費用は今回の承認対象外。
- 件名「Papers for the payment pilot decision」、2025-10-08 15:18 BST Nicholas → Kenneth（Cc Neha、Siobhan）。配布PDFと Connor の集計 XLSX を別々に実添付。
- 件名「Re: Papers for the payment pilot decision」、2025-10-09 10:28 BST Kenneth → Nicholas（Cc Elizabeth、Neha、Siobhan）。前日の依頼に返信し、決裁依頼を引用して対象と条件を明記。添付なし。
- 件名「Signed change and desk copy」、2025-10-09 14:24 BST Nicholas → Kenneth（Cc Elizabeth、Neha、Siobhan）。署名済みスキャンと担当者向け手順PDFを実添付。

## Pioneer の中間確認の範囲と取引条件

- 2025-10-13 14:00 BST、Meera・Nicholas・Deepa が中間確認の対象を確定。4月の診断対象12社のうち S018・S042・S036・S011・S008 の各1条件・各1品目。Q4（2025-10-01〜12-31）の受領と紐づく返品を取り、2026-01-12までの計上を締めとする。
- 率の分母は5条件の Q4 実購入純数量を10月の税抜 USD 基準単価で評価した USD 1,000,000。10月の条件・移行時期の見込を同じ数量に当てた比較で、全社、通年、別品目、運賃、税、需要共有案の効果は含めない。
- Buying 保管の PTD-BUY-TRM-2025-001 Rev. 1 は Meera 発行、2025-10-30。条件行 T1〜T5 は順に S018（shipping labels）、S042（sealing tape）、S036（protective sleeves）、S011（bin liners）、S008（packing dispensers）。S036 の切替前を含め各行の根拠は10〜12月の取引に対応する。
- T1 / S018：基準 USD 20.00／case。Q4に12,000 net cases以上なら対象全量 USD 18.40、未達なら USD 19.60。6件各1,700 casesを受領し、元行に紐づけた返品200 casesを USD 19.60 で控除。10,000 net casesに USD 19.60 が適用され、追加の精算なし。
- T2 / S042：基準 USD 20.00／case。7,000 net cases以上なら対象全量 USD 18.00、未達なら USD 20.00。10 cases入り pack を5件各150 packs、USD 180／packで仮請求し、Q4の7,500 casesで達成したため期末調整なし。
- T3 / S036：基準 USD 20.00／case。最低数量なし。12月1日以降の受領分に USD 18.00、以前は USD 20.00。各1,250 cases×5件を境界の両側に受領。11月28日受領・12月2日計上の行は USD 20.00、12月1日受領の行は USD 18.00。
- T4 / S011：基準 USD 20.00／case、書面価格 USD 19.50。5件各1,000 casesを USD 19.50 で計上。数量しきい値なし。10月試算で見込んだ追加 USD 1.10／case 相当の商業貸方は、別承認が必要で、締め時点で計上なし。
- T5 / S008：基準 USD 30.00／unit、固定 USD 27.80。最低数量なし。5件各2,000 unitsに同単価を適用した。

## Pioneer の購入抜粋と差額の照会

- 1月13日に受領した Q4 抜粋は5条件の対象全32行（T1〜T5順に7・5・10・5・5）。T1 貸方行は1件、符号付き数量と明細額を使う。他の行は請求行で、数量・単価を相殺せず保持する。
- S018 返品の原伝票は AP-62114、貸方は AP-62851（−200 cases、−USD 3,920）。S036 の境界例は AP-62418（11月28日受領・12月2日計上）と AP-62431（12月1日受領）。S042 の換算例は AP-61972（150 packs、USD 27,000）。
- 「The terms Buying has on file」（10月30日14:26 GMT、Meera → Nicholas、Cc Deepa）は条件書PDFを実添付。「The cleared Q4 purchase lines」（1月13日13:24 GMT、Meera → Deepa、Cc Nicholas）は Q4 CSV を実添付。いずれも独立した送付。
- 「Three checks before Wednesday」は1月14日14:14 GMT、Nicholas → Meera（Cc Deepa）。返品・受領日・追加貸方の照会表を実添付し、1月16日までの回答を依頼。
- 「Re: Three checks before Wednesday」は1月15日14:34 GMT、Meera → Nicholas（Cc Deepa）。14日の3問を引用して T1 の返品と T3 の受領日を確認。T4 の承認も計上も証拠なしと回答。返信に新規添付なし。
- Deepa は `deepa@holt-reeves.example`。Meera と Nicholas のアドレスは「Pioneer の現行例外要領と緊急申請の照会」の記録を使う。

## Pioneer の差額表示の改訂と1月中間確認

- 10月モデルの計画数量を使った基準額 USD 1,060,000、見込 USD 86,000は10月の作業表の数値。1月は条件達成時単価と計画の切替割合を Q4 の実購入純数量に当て直して比較する。
- 1月の比較では S018 の条件達成単価を実購入10,000 casesに当てる。S036 は実購入12,500 casesの80%（10,000）を切替後、2,500を従前価格と置く。S011 は書面単価に加えて USD 5,500の追加貸方を見込む。
- 基準額／見込削減／実現削減（税抜 USD）は T1 が200,000／16,000／4,000、T2 が150,000／15,000／15,000、T3 が250,000／20,000／12,500、T4 が100,000／8,000／2,500、T5 が300,000／22,000／22,000。
- HRA-PIO-BUY-SAV-2026-001 v1.0（1月16日、Deepa）の合計は見込 USD 81,000（8.1%）、実績 USD 56,000（5.6%）。差 USD 25,000（2.50ポイント）は数量未達 USD 12,000（1.20）、受領時期 USD 7,500（0.75）、要確認 USD 5,500（0.55）。
- T1 の未達分は実購入10,000 cases × (USD 19.60 − USD 18.40)。T3 の時期差は (計画10,000 − 実績6,250) cases × USD 2。T4 の USD 5,500は見込と計上実績の既知残差で、追加貸方そのものの確定額は空欄。
- HRA-PIO-BUY-BRF-2026-001 は Nicholas の1月16日 v0.3 と1月20日 v1.0。Elizabeth は1月16日に「Discounts lost」を指摘。v1.0 では取引条件どおりの価格適用と、しきい値を満たさなかったことによる見込との差を区別する。
- 「Papers for the savings checkpoint」、1月21日10:14 GMT、Nicholas → Meera（Cc Deepa）。v1.0 配布PDF `21 Jan - Savings at the January checkpoint.pdf` と別添集計表 `20 Jan - Q4 bridge for Buying.xlsx` の2件を実添付。
- 1月21日14:00〜14:45 GMT、Meera・Nicholas・Deepa。中間値5.6%と差の3区分を確認。Meera は T4 の追加貸方の承認経路と伝票を1月28日までに確認し、Deepa は証拠が揃うまで中間値を据え置く。

## Whitfield 日本事業の次契約の依頼と Finance への絞り込み

- WCL-JP-FIN-REQ-2026-001 v1.0（2026-04-13、Kenneth）の候補は Japan Finance の支払統制と Sales の受注例外。決裁希望日は7月20日、依頼時点の金額・期間は未定。
- 4月13日10:00〜10:40 BST、Elizabeth・Nicholas。既存の引渡し証跡と担当を確認できる Finance だけを提案し、Sales は対象外。候補併記案 HRA-WHI-DIAG-BRF-2026-001 v0.1（4月13日）は社内旧案、v0.2（4月17日）は Finance のみ。
- スレッド「Two areas for the next engagement」1. 4月13日08:46 BST Kenneth → Nicholas（Cc Elizabeth、依頼PDF実添付）。2. 同日12:08 BST Nicholas → Kenneth（Cc Elizabeth）「Re: Two areas for the next engagement」で1を引用し、Finance に絞ると回答。返信の添付なし。

## Whitfield Japan Finance Phase 1 の納入条件、工程と決裁

- 検討・契約範囲は Components・Packaging・Tooling・Maintenance・Facilities の通常国内仕入先支払。2025年の条件付き拡大承認は追加3部門の移行完了を意味しない。Treasury の12の二重確認を維持し、給与、税金、緊急手動送金、Sales を除外。
- 6月8日09:00〜09:35 BST、Kenneth・Nicholas・Siobhan・Neha。Kenneth が7月24日までに Japan Finance から各部門の正副担当、両者の07承認処理権限と証跡受領の確認、許可済み週次CSVを手配。確認を満たさない部門は経路を変更せず従来運用を続ける。
- WCL-JP-FIN-AGR-2026-001 v1.0 第2条の納入物は (a) 5部門の07・08・09の担当・権限・証跡・差戻しを記した引渡しカード、(b) 許可済み週次CSVを手動で取り込む滞留表（既存5部門、初回適格06を起点、再回付は別記）、(c) Japan Finance への移管パック。ライブ連携、過年度の再処理、新規部門と新規自動化は含まない。
- 契約コメント HRA-WHI-DIAG-REV-2026-001 は6月17日 Nicholas v0.3、6月19日 Joanne の赤字 v0.4。第4条は新規作業の範囲・費用・日程への影響を両社権限者が書面承認してから着手。第5条は W12 に5枚の引渡しカード、週次CSVから照合できる滞留表、移管パックの3納入物を Kenneth が確認して書面受入。
- HRA-WHI-DIAG-PLN-2026-001 v1.0（7月10日、Connor）の人日×円/人日は Nicholas 18×400,000、Siobhan 48×350,000、Neha 30×300,000、Connor 28×250,000、Elizabeth 7×400,000。計131人日、JPY 42,800,000 税別。通常の訂正用の別料金枠は置かない。
- 12週は W1開始 7月27日、W12終了 10月16日。確認点は W2 8月7日（記録源）、W6 9月4日（カード案と滞留表初版）、W10 10月2日（確認事項の整理）、W12 10月16日（移管と受入）。8月25日時点で W6 以降は予定。
- HRA-WHI-DIAG-BRF-2026-005 v1.0（7月13日、Nicholas）は決裁用の PPTX と同一表示面の配布PDF。7月13日11:04 BST「Papers for the 20 July decision」は Nicholas → Kenneth（Cc Elizabeth、Siobhan）。配布PDFと PLN v1.0 XLSX の2点を実添付。
- 7月20日09:30〜10:15 BST、Kenneth・Elizabeth・Nicholas・Siobhan・Joanne。Finance 限定の12週、JPY 42.8m 税別を承認。着手・W6設計確認・W12書面受入で30%・40%・30%（JPY 12.84m・17.12m・12.84m）。
- 7月20日発効の上記合意書に Kenneth Townsend（Whitfield CFO）と Elizabeth Hargreaves（Holt Partner）が同日署名。7月20日14:51 BST「Signed Japan Finance agreement」は Nicholas → Kenneth（Cc Elizabeth、Joanne、Siobhan）、同日付の署名済みスキャンPDFを実添付。

## Whitfield Japan Finance Phase 1 の8月追加要求と保留

- 2026-08-12 に顧客側作業会で7件受領。Connor の HRA-WHI-DIAG-CHG-2026-001 は8月19日 v0.2（01・02の条項照合待ち）、8月24日 v0.3（Joanne の8月21日の照合を反映）。基準は WCL-JP-FIN-AGR-2026-001 v1.0。
- CH-01「Return reason on the hand-off card」は既存差戻し理由欄への追記、2(a)、Siobhan。CH-02「Department filter in the weekly sheet」は既存5部門の手動滞留表に部門フィルターを付ける、2(b)、Connor。両方とも既存範囲で追加料金 JPY 0。
- CH-03「Daily dashboard feed」JPY 1,400,000（日次自動接続）。CH-04「2025 queue backfill」JPY 1,250,000（前年分の滞留再構成）。CH-05「Add Warehouse」JPY 950,000（6番目の部門）。
- CH-06「Read invoice references by OCR」JPY 1,100,000（請求書参照の自動読取）。CH-07「Sales credit holds」JPY 1,500,000（販売の与信保留）。03〜07はすべて追加要求で第4条の書面承認待ち、税別小計 JPY 6,200,000。
- 8月25日09:00〜09:40 BST、Nicholas・Connor・Joanne・Elizabeth。01〜02を既存作業へ、03〜07は着手せず保留。契約は JPY 42.8m・12週のまま、Joanne が書面変更の窓口、Nicholas が顧客への回答担当。
- 8月25日13:18 BST「Five requests held for written approval」は Nicholas → Kenneth（Cc Joanne、Elizabeth、Siobhan）。顧客用判定 HRA-WHI-DIAG-NTE-2026-002 v1.0 のPDFを実添付し、進めたい項目があれば優先順位を求める。
- Holt 側メールは `elizabeth@holt-reeves.example`、`siobhan@holt-reeves.example`、`joanne@holt-reeves.example`、`paul@holt-reeves.example` を使用。

## Pioneer の6月・8月の発注遅延と8月の削減実績

- 発注の対象は前年に選んだ12社への2拠点の発注。Pioneer の発注登録の対象月（承認期限日 EDT が属する2026年6月1〜30日／8月1〜31日）で切り、9月4日に比較を確定。2025年の例外93件や需要初認・共有時刻とは別の観測。
- 発注は PO ごとの対仕入先初回送信。期限はその送信前に有効だった最後の承認済み期限。EDT（UTC−04:00）で送信＞期限を遅延、同時刻は期限内。日付だけで同日の前後が確かめられない行は Review とし、時刻を補わない。
- 6月は原行416、重複再送5と初回送信前取消5を Exclude、時刻要照会6を Review。判定済み400＝期限内364＋遅延36で9%。8月は原行438、再送7・取消5を Exclude、時刻要照会6を Review。判定済み420＝期限内399＋遅延21で5%（4ポイント低下）。
- 8月の削減は対象12社のうち条件を確認済みの5購買群。8月1〜31日の受領と紐づく返品を9月4日計上締めで照合。実購入純数量を各群の承認済み比較基準単価で評価し、税・運賃を除いた USD 800,000 に対し、確認済み純仕入 USD 752,000、実績差 USD 48,000（6.0%）。
- 5購買群の基準／確認済み純仕入（税抜 USD）は S018 180,000／168,000、S042 160,000／152,000、S008 140,000／133,000、S036の照合済み範囲 200,000／187,000、S011の照合済み範囲 120,000／112,000。要照会行との明細重複なし。
- S036 の別受領群は8月に適用する承認済み条件書版が未確認。S011 の別受領群は8月分貸方の承認と Finance の転記を要確認。両群は差額空欄で確定集計の分子・分母から外す。1月の中間5.6%と S011 の USD 5,500 の既知残差は当時の集計のまま。
- HRA-PIO-BUY-BRF-2026-003 は9月7日 v0.2（Nicholas）、9日 v0.3（Elizabeth の赤字）、18日 v1.0（Nicholas、報告会用）。6.0%と9%→5%を分けて示し、指標間の因果は認定しない。顧客報告 HRA-PIO-BUY-NTE-2026-001 v1.0 は同日付の別文書。

## Pioneer の発注明細による通し確認と Buying への引継ぎ

- 8月登録の抜粋14行は判定済み10（期限内8、遅延2）、除外2（送信前取消1、再送1）、要照会2。HRA-PIO-BUY-MCK-2026-001 は9月8日 v0.3（PO-84255 を誤って期限内として2/11）、9月17日 v1.0（Review に訂正し2/10）。
- 注記図と手順に使う抜粋冒頭は PO-84217（8月18日15:08送信／同日15:00期限）、PO-84241（8月19日15:00／同日15:00）、PO-84255（8月20日の日付のみ／同日15:00）。時刻はすべて EDT。PO-84263 は送信前取消、PO-84279 は初回のみ算入（期限内）、再送行は除外。
- 9月14日 14:00〜14:30 BST、Nicholas・Neha・Deepa が8月データで通し確認。Neha が日付だけの PO-84255 の期限内扱いを指摘、Deepa が未確認の価格差は空欄と確認。手順 HRA-PIO-BUY-EV-2026-002 は9月10日 v0.4 から16日 v1.0 へ改訂。
- 9月18日 14:00〜14:45 BST、Meera・Nicholas・Neha・Deepa。Meera が Buying の点検責任を持ち、9月分の初回確認を10月7日に設定。送信履歴と S036 の条件書は Buying、S011 の承認は Buying・転記は Finance に照会する。
- 「Leave Buying something we can rerun」8月26日13:22 BST、Meera → Nicholas（Cc Neha、Deepa）、添付なし。独立した依頼。
- 「August dispatch rows for the handover」9月3日13:22 BST、Meera → Connor（Cc Nicholas、Neha）、August - Order dispatch extract.csv を実添付。独立した送付。
- 「Can Buying trace these two August lines」9月11日13:11 BST、Deepa → Meera（Cc Nicholas）、10 Sep - Price proof for the two held lines.xlsx を実添付。9月15日14:18 BST「Re: Can Buying trace these two August lines」、Meera → Deepa（Cc Nicholas）が元の問いを引用、新規添付なし。
- 「Final results and the Buying checks」9月18日15:48 BST、Nicholas → Meera（Cc Neha、Deepa）。独立した送付で、確定報告PDF、発注明細CSV、確定の点検XLSX・手順DOCX・画面注記PNGの5点を実添付。
