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
