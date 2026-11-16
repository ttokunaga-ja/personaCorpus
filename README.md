# personaCorpus

20 職種の人が日常使っている PC のフォルダ環境を、合成データで再現した corpus。文書、表、スライド、PDF、メール、画像、テキストなど、Git では管理されない仕事のファイルを対象にする。文書検索・文書管理の製品の評価に使うが、どの製品にも依存しない。

| 場所 | 中身 |
|---|---|
| `SPEC.md` | corpus の内容の正本（目的、独立、構成、ファイル形式、品質、受入と破棄、規模） |
| `AGENTS.md` | 制作の規則と、Claude・Codex の役割 |
| `personas/` | persona ごとの定義（人物、組織、仕事の流れ、フォルダ構成、形式の目安） |
| `people/<persona>/` | 配布物。その人の PC のユーザーフォルダそのもの |
| `sources/<persona>/` | 成果物ごとの原稿（中身を書いた Markdown） |
| `templates/<persona>/` | その persona の世界の文書の型 |
| `progress/<persona>/` | batch ごとの依頼、ファイル計画、結果の報告 |
| `manifests/` | 受入済みファイルの一覧（パス、sha256、大きさ、更新時刻） |
| `runbooks/PRODUCTION.md`、`runbooks/REVIEW.md` | 制作と受入の手順、レビューの基準 |
| `bin/` | batch の進行（`batch`）、自動検査（`check`）、制作の道具（`render`・`tex`・`scan`・`office-pdf`）、一覧と梱包（`manifest`・`pack`） |
| `config/limits.json` | 同時に動く Codex の数と、persona あたりの同時 batch 数の上限 |
| `ops/` | Codex のジョブの起動と通知 |

すべて合成データで、実在の人物、組織、顧客、文書を含まない。

ライセンスは CC0 1.0（`LICENSE`）。
