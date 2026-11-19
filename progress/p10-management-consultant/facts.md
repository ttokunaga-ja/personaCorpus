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
