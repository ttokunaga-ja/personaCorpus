# 指摘を取り込んだ改善

## 2026-11-16（UTC）

| 対象の段階・観点／検査名 | 変更箇所 | 変更の理由と参照先 |
|---|---|---|
| draft-review D4、build-review V1 → `email-headers` | `bin/check` の `email_source_reason`、`tests/check_regression_test.py` の `EmailHeaders` | 台帳の固定メール識別子が再生成される指摘が複数回発生した。既存の `source` の単語照合だけでは、引用に識別子が残ったヘッダ変更も通ることをテストで確認した。原稿から成果物への識別子の保持を直接検査する。台帳と原稿の一致はこの検査の対象外で、D4 のレビューを維持する。 |
| draft-review D4 | `prompts/draft.md`「版とメール参照の照合」 | p14 b006 の工程名の時点混同、p14 b006・p05 b001 の固定 Message-ID 不一致と再試行での再生成を防ぐため、執筆後の照合手順を具体化した。 |
| templates-review（記録上の観点は other） | `prompts/templates.md`「型の作り方」の追加2項目 | p14・p05・p17 の継承フォント／テーマ色と説明の不一致、p18・p12・p17 の規定文書番号の切れを受け、既定スタイルと実際の入力値を使う見本の確認を具体化した。 |
| build-review V2、V3、other（図の表現） | `prompts/build.md`「表の表示と印刷」「スライド」 | p14 b005 の追加シートの型欠落・ページ番号と数値の配置・橋渡し図、p14 b006 の印刷範囲外の基準日、p12 b001 の折返しラベルの切れを防ぐ。 |
| check `opens`、`email-headers` | `prompts/build.md`「メールの符号化」 | p17 b001 等で繰り返された invalid email headers を受け、アドレスの符号化と保存後のヘッダ確認を具体化した。既存の `opens` 検査を維持する。 |

根拠は `progress/findings.tsv`・`progress/batches.tsv` と現行コード。集計は再試行を含む延べ件数であり、過去の指摘が現行実装でも再現するとは限らない。検証結果、対象ファイル、保留事項は作業報告 `scratch/result.md` に記載する。検査の条件、レビュー基準、制作段階や各種書式は変更していない。
