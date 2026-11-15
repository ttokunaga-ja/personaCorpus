# 制作と受入の手順

内容の基準は `SPEC.md`、規則は `AGENTS.md`、persona の設定は `personas/<persona>.md`、レビューの基準は `runbooks/REVIEW.md` にある。

制作は batch の単位で進める。統括（Opus）は「何を作るか」を依頼し、`bin/batch` が Codex に計画・制作・検査・レビュー・作り直しを最後までやらせる。統括は結果の報告を読み、抜き取りで確かめて、受入か破棄を決める。

```
統括：依頼を書く → bin/batch start
  └ bin/batch（Codex のジョブを順に動かす。統括は呼ばれない）
      1. テンプレート（その persona で初回だけ）   Sol
      2. ファイル計画                              Sol
      3. 制作（10 件ずつ）                         Terra
      4. 自動検査                                  bin/check
      5. レビュー（10 件ずつ、作った本人とは別）    Sol
      6. 不合格を破棄して作り直し（2 回まで）       Terra
      7. 結果の報告
統括：通知を受ける → 報告を読む → 抜き取り → bin/batch accept または discard → push
```

## 0. 環境の準備（初回だけ）

- `uv sync` で Python 環境を作る（`pyproject.toml`）。
- `bin/check --self-test` が通ることを確かめる。LibreOffice（`soffice`）、`pdftotext`、`pdftoppm`、`tesseract`、`ffprobe`、`lualatex`、日本語のフォントが要る。足りないものは名前を挙げて失敗する。
- `tests/run.sh` と `ops/tests/run.sh` が通ることを確かめる。

## 1. 依頼を書く（統括）

`progress/<persona>/<batch>.request.md` に、その batch で作る「仕事のまとまり」を書く。batch 名は `b001` からの連番、1 batch は 50 ファイル程度までとする。

```markdown
# p14 b003 の依頼

- 範囲：2026 年 3 月の月次締め一式と、その前後のやりとり
- 件数の目安：40
- 重点：銀行残高との差異 1,240 万円の調査の経過が、メール、調整表、締め報告でたどれること
- 含めたいもの：堀口さんの下書きに千田さんが赤字を入れた版、監査法人からの問い合わせ
- 避けるもの：2026-01 の二重計上（b002 で作成済み）
```

依頼を書くときに確かめること：

- `personas/<persona>.md` の「仕事の流れ」に、その範囲の出来事があるか。足りなければ、出来事（時期、相手、数値、結論）を書き足してから依頼する。既にある設定と受入済みのファイルに矛盾する変更はしない。
- これまでの batch（`progress/<persona>/*.md`）と範囲が重なっていないか。
- 形式の割合が `personas/<persona>.md` の目安から外れてきていないか（`bin/batch status <persona>` が現在の割合を出す）。

## 2. 起動する

```sh
bin/batch start <persona> <batch>          # 依頼を読み、最後まで自動で進める
bin/batch start <persona> <batch> --plan-only   # 計画を書いたところで止める（統括が計画を見てから続けたいとき）
bin/batch resume <persona> <batch>         # 止まったところから続ける
agent-alert wait --repo personaCorpus      # Bash の background で実行し、通知を待つ
```

- `bin/batch start` は、空き容量が 30 GB 以上あること、同じ persona の batch が動いていないことを確かめてから、`codex-job` のジョブ `<persona>-<batch>` として自分を起動する。同じ persona の 2 つ目は起動できない。違う persona は並行してよい。
- 途中の段階ごとの通知は出ない。通知が来るのは、batch が終わったとき（`done`）か、進められなくなったとき（`failed`）だけである。
- 進み具合は `bin/batch status <persona> <batch>` で見る。各段階の Codex の出力は `scratch/<persona>/<batch>/logs/` にある。

## 3. `bin/batch` がすること

| 段階 | 担当 | 内容 | 出力 |
|---|---|---|---|
| テンプレート | `gpt-6.1-sol` | `templates/<persona>/` が無いときだけ、その persona の文書の型を作る（下の「テンプレート」） | `templates/<persona>/` |
| 計画 | `gpt-6.1-sol` | 依頼、persona の定義、これまでの計画、テンプレートの一覧を読み、ファイル計画を書く。複数のファイルにまたがる事実（文書番号、金額の内訳、会議の日時、メールのスレッド）を事実の台帳に書き足す | `progress/<persona>/<batch>.md`、`progress/<persona>/facts.md` |
| 計画の検査 | スクリプト | 列、拡張子、パスの重複、日付が期間内か、制作区分の名前がないか、件数。不合格なら理由を付けて計画をやり直させる（1 回） | |
| 制作 | `gpt-6.1-terra` | 計画の行を 10 件ずつ、別々の Codex の実行で作る。既にあって検査に通っている行は飛ばす | `people/<persona>/…` |
| 日付 | スクリプト | 各ファイルの更新時刻を計画の「日付」にする | |
| 自動検査 | `bin/check` | `bin/check <persona> --batch <batch>` | `scratch/…/check-r<N>.tsv` |
| レビュー | `gpt-6.1-sol` | `runbooks/REVIEW.md` の観点で 1 件ずつ開いて読み、合格か破棄かと理由を書く。10 件ずつ、制作とは別の実行で行う | `scratch/…/review-r<N>.tsv` |
| 作り直し | `gpt-6.1-terra` | 自動検査かレビューで不合格のファイルを消し、理由を渡して同じ行を作り直す。2 回まで | |
| 報告 | スクリプト | まだ不合格のファイルを消し、計画のその行に「破棄：理由」と書く。結果を報告にまとめる | `progress/<persona>/<batch>.result.md` |

ファイル計画（`progress/<persona>/<batch>.md`）は Markdown の表で、列は次のとおり。

| 列 | 内容 |
|---|---|
| パス | `people/<persona>/` からの相対パス。`personas/<persona>.md` のフォルダ構成に従い、人が付ける名前にする |
| 日付 | そのファイルを最後に更新した日時（persona の世界の中の日時、`2026-03-06 18:42` の形） |
| 作り方 | `docx`・`xlsx`・`pptx` はテンプレート名、PDF は元にするもの（`office:<テンプレート名>`、`tex:<テンプレート名>`、`scan:<元のファイルまたは tex/office>`）、画像は `photo`・`screenshot`・`chart`・`diagram`、その他は `text` |
| 内容 | そのファイルにしかない中身。誰が、何について、どの案件・数値・結論で書いたか。2〜4 文 |
| 関連 | 参照する、または参照される既存ファイル（あれば） |

報告（`progress/<persona>/<batch>.result.md`）には、依頼との対応、作った件数と形式の内訳、破棄した行と理由、作り直しの回数、レビュー役の所見、抜き取りに使うレンダリングの場所（`scratch/<persona>/<batch>/render/`）を書く。

## 4. テンプレート

Office の文書と PDF は、その persona の世界の「型」を使って作る。同じ会社の文書は同じ見た目になり、社外から届いた文書は相手ごとに違う見た目になる。

- 置き場所は `templates/<persona>/`。配布物には含めない。
- 中身：
  - **自社の型**：Word（報告書、議事録、社内通知、社外あての文書など、その職種で使う 3〜5 種）、PowerPoint（スライドマスター 1〜2 種）、Excel（見出し、数値、合計の書式を決めたブック 1〜2 種）。ロゴ、配色、フォント、ヘッダとフッタ、文書番号の付け方をそろえる。
  - **社外の型**：よく文書が届く相手（取引先、監査法人、役所など）ごとに、その相手の見た目の型を 1 つずつ。
  - **TeX の型**：Office を経ない PDF（規程集、論文、公的な通知、請求書、機器や業務システムが出力する帳票など）のための `.cls` か `.tex` のひな形。`lualatex` で組む。
  - `README.md`：型の一覧。型ごとに、名前、どの組織の誰が何に使うか、ファイル名、見た目の説明。
- 型は persona ごとに別のデザインにする。20 persona が同じ見た目にならないようにする。
- 型を足すのは、依頼の範囲に今の型で作れない文書があるときだけで、計画の段階で行う。
- 個人のメモ、作業中の表、古い資料など、型を使わない文書も実際にはある。全部を型どおりにしない。

## 5. 制作の道具

| 道具 | 使い方 |
|---|---|
| `bin/render <file> <outdir>` | DOCX・XLSX・PPTX・PDF をページごとの PNG にする。制作とレビューで、見た目を確かめるのに使う |
| `bin/tex <file.tex> <out.pdf>` | `lualatex` で PDF にする。一時物は `scratch/` に置く |
| `bin/scan <in.pdf または画像> <out.pdf>` | 紙をスキャンしたような PDF にする（傾き、濃淡、ノイズ、JPEG 圧縮。テキスト層なし） |
| `bin/office-pdf <file> <out.pdf>` | DOCX・XLSX・PPTX を PDF にする（「PDF で保存した」文書用） |
| `uv run python` | `python-docx`、`openpyxl`、`python-pptx`、`matplotlib`、`pillow` で、テンプレートを開いて中身を入れる |
| Codex の `imagegen` skill | 写真、手書きのメモ、ホワイトボードなどの画像 |

- スキャン PDF は、中身のある文書（TeX か Office で組んだページ、または手書き風の画像）を `bin/scan` に通して作る。
- グラフと図は、その文書の数値から描く。文書の中の数値と一致させる。
- 文書を量産するスクリプトは作らない（`AGENTS.md` の「作り方」）。

## 6. 抜き取りと受入（統括）

通知が来たら、`progress/<persona>/<batch>.result.md` を読み、自分で開いて確かめる。

- 形式ごとに 1 件以上、合計で batch の 2 割以上（最低 8 件）。Office と PDF は `scratch/<persona>/<batch>/render/` の画像を見る。画像は画像そのものを見る。
- 見る点は `runbooks/REVIEW.md` と同じ。レビュー役が合格にしたものを、同じ基準で確かめる。
- 自分の抜き取りで不合格が 2 件以上出たら、レビューが甘いとみて batch 全体を破棄するか、`bin/batch resume <persona> <batch> --review-again` でレビューからやり直す。

```sh
bin/batch accept <persona> <batch>            # 検査をもう一度通し、manifest を更新し、その batch のファイル・計画・報告だけを commit する
bin/batch accept <persona> <batch> --drop <パス>...   # 指定のファイルを破棄して、残りを受け入れる
bin/batch discard <persona> <batch> --reason "<理由>" # batch のファイルをすべて消し、理由を計画に書いて commit する
git push origin main
```

- 受入または破棄が済むと、`scratch/<persona>/<batch>/` は消える。
- 同じ batch で不合格が 2 回続いたら、`bin/batch start ... --producer gpt-6.1-sol` で制作を Sol に切り替え、切替えと理由を依頼の末尾に書く。
- 形式ごとに、特に出来のよい受入済みファイルを `templates/<persona>/README.md` の「見本」に書き足す。制作とレビューは見本を水準として使う。

## 7. 配布物を組み立てる

```sh
bin/pack <出力先のフォルダ>
```

`bin/check --all` と `manifests/` の照合に合格したときだけ、persona ごとに `<persona>.tar.gz` を作る。中身は `people/<persona>/` の下だけで、更新時刻は `manifests/<persona>.tsv` の値にする。あわせて `SHA256SUMS` と配布用の `README.md` を出力する。出力先は repo の外か `scratch/` にし、Git に入れない。

## 8. 止まったとき

- `failed` の通知が来たら、`bin/batch status <persona> <batch>` で止まった段階と理由を見る。Codex の利用上限なら、上限が戻ってから `bin/batch resume` する。それ以外は、`scratch/<persona>/<batch>/logs/` の該当の出力を読んで原因を直してから `resume` する。
- `resume` は、済んだ段階と、検査に通っているファイルをやり直さない。
- ファイルの場所を移す commit を push するときは `git -c pack.useSparse=false push origin main` を使う。既定の設定では、移しただけのファイルを全部送り直して上限を超える。
- 規則を増やして対処しない。規則の変更が要ると思ったら、ユーザーに伝えて Fable のセッションで行う。
