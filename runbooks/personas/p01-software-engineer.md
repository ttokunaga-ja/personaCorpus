# p01 ソフトウェアエンジニア

## 役割と作業世界

macOS 開発ラップトップ（ja-JP、ja/en）。架空組織 **Aster Loop 株式会社** の製品開発者として、2026-07-13 の Product Alpha/Beta リリース周期と非同期開発を扱う。Git/Drive の合成スナップショットのみを題材にし、S1/S2 相当の架空業務情報に留める。

## 正式配分（W0 12,000 物理ファイル）

`md 22% / txt_log 8% / code 28% / structured_text 12% / csv_tsv 3% / html_eml 5% / ipynb 1% / pdf_text 7% / pdf_scan 1% / docx 3% / xlsx 2% / pptx 2% / image 3% / media 0% / domain_binary 3%`（計100%）。

## 正式 primary paths（12）

`documents/work/product-alpha/architecture`; `documents/work/product-alpha/api-contracts`; `documents/work/product-alpha/release-notes`; `documents/work/product-beta/architecture`; `documents/work/product-beta/api-contracts`; `repos/product-alpha/docs`; `repos/product-beta/docs`; `work-items/decision-records`; `work-items/code-reviews`; `meetings/engineering`; `vendor-docs/platforms`; `operations/migration-notes`。

secondary は accepted Rust plan の共通 secondary paths だけを参照し、ここで別の正式パスを創設しない。

## Full 現行パス（structure v2）

Full 完了後の現行パスは
`progress/p01/full/revisions/v2/path-revision.jsonl` の append-only overlay を
authority とする。凍結済み Full assignment ledger と batch checkpoint は、
初期の割当・受入を示す historical authority として保持し、現行の物理パスを
再定義しない。M1 の200件と受入済み Full pilot の12件は既存パス・bytes を
固定し、残る11,788件だけを同一 canonical scope 内で byte-preserving に移す。

各 scope 内では、意味別の業務領域、形式／技術別の下位領域、必要に応じて
`2026-q3/<kind>/series-00` または `series-01` で整理する。Full-to-Full 参照の
対象だけは basename 互換性を保つため307件を例外とし、対応する参照群を
`evidence/linked/cNNN` に共置する。v2 後は12,000 regular files、756 leaf
parents、任意一ディレクトリ直下の最大131 filesであり、symlink・nonregular・
empty artifact は0である。

この再編は形式配分を変えない。PDF は960件（text layerあり840、scan120）、
DOCX 360、XLSX 240、PPTX 240、PNG 360を含み、上記の全 family 比率を維持する。
詳細な移行根拠と形式検査は
`progress/p01/full/revisions/v2/structure-policy.md`、
`migration-report.md`、`format-audit.json` を参照する。

## 初回制作バッチ（比率達成そのものではない）

次の各 primary path に高忠実度の1件を置く：architecture=ADR Markdown、api-contracts=OpenAPI YAML、release-notes=release Markdown、beta architecture=Rust設計コード、beta api-contracts=TypeScript契約、alpha docs=README、beta docs=migration guide、decision-records=意思決定 DOCX、code-reviews=JSONL review export、meetings=設計会議 PPTX、platforms=テキスト層 PDF、migration-notes=移行図 PNG。これは全配分を満たすと主張せず、上記比率へ継続制作する種である。

## 生成・受け渡し

DOCX は Documents、PDF は PDF（text/scan を明示）、XLSX は Spreadsheets、PPTX は Presentations、PNG/スキャン原稿は ImageGen、その他は通常生成で作る。アンカーは `Aster Loop / Product Alpha R7 / 2026-07-13 / p95 184 ms / ADR-042`。実在の氏名・メール・顧客名・認証情報・トークン・秘密鍵は禁じる。

受け渡し前に、拡張子と実体の一致、開封可能性、Office/PDF のレンダリング、画像の可読性、表・コード・数値アンカーの視認性だけを確認する（Kio 検索評価は対象外）。
