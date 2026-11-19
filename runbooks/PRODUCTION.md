# 制作と受入の手順

内容の基準は `SPEC.md`、規則は `AGENTS.md`、persona の設定は `personas/<persona>.md`、レビューの基準は `runbooks/REVIEW.md` にある。

制作は batch の単位で進める。「何を作るか」の依頼から、`bin/batch` が Codex に計画・制作・検査・レビュー・作り直しを最後までやらせる。終わった batch は、独立した監査を経て受入か破棄になる。ふだんは自動運転（`bin/autopilot`。10 節）が、依頼・起動・やり直し・監査・受入を回す。統括（Claude Opus か Codex `gpt-6.2-astra` のセッション）は、自動運転が保留にした batch の処理、抜き取りの確認、仕組みの修正を行う。1〜6 節は、batch を手で動かすときの手順であり、自動運転が内部で行っていることでもある。

```
統括：依頼を書く → bin/batch start
  └ bin/batch（Codex のジョブを順に動かす。統括は呼ばれない）
      1. テンプレート（その persona で初回だけ）と、その検査  Sol
      2. ファイル計画と事実の台帳                       Sol
      3. 原稿：中身を Markdown で書く（10 件ずつ）       Terra
      4. 原稿のレビュー（不合格は書き直し、2 回まで）    Sol
      5. 成形：合格した原稿を型に入れて形にする          Terra
      6. 自動検査                                       bin/check
      7. 見た目のレビュー（不合格は成形し直し、2 回まで） Sol
      8. 結果の報告
統括：通知を受ける → 報告を読む → 抜き取り → bin/batch accept または discard → push
```

中身（何が書いてあるか）と形（どう見えるか）は、別の段階で作り、別の観点でレビューする。中身の不合格は、形にする前に原稿の段階で落とす。

## 0. 環境の準備（初回だけ）

- `cargo build --release --manifest-path tools/Cargo.toml` で道具をビルドする。`bin/` と `ops/bin/` のコマンドは、どれも `tools/target/release/corpus` を呼ぶ薄い shell スクリプトで、ビルドしていないと、ビルドを促して失敗する。道具を直したあとも、ビルドし直す。
- `uv sync` で Python 環境を作る（`pyproject.toml`）。Python は、制作の Codex が文書を組む（`python-docx`、`openpyxl` など）のと、テストの fixture を作るのに使う。
- clone した直後は、受入済みファイルの更新時刻が clone した時刻になっている（Git は更新時刻を保存しない）。persona ごとに `bin/manifest <persona> --restore` を実行して、`manifests/` に記録した時刻に戻す。戻さないと `bin/check` の `mtime` が不合格になる。
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
bin/batch resume <persona> <batch> --rebuild     # 「成形待ち」の行を、残っている原稿から成形し直す
bin/batch resume <persona> <batch> --redraft     # 原稿の段階で破棄になった行の印を外し、原稿からやり直す
agent-alert wait --repo personaCorpus      # Bash の background で実行し、通知を待つ
```

- `bin/batch start` は、空き容量が 30 GB 以上あることと、下の「並列」の条件を確かめてから、`codex-job` のジョブ `<persona>-<batch>` として自分を起動する。
- 途中の段階ごとの通知は出ない。通知が来るのは、batch が終わったとき（`done`）か、進められなくなったとき（`failed`）だけである。
- 進み具合は `bin/batch status <persona> <batch>` で見る。各段階の Codex の出力は `scratch/<persona>/<batch>/logs/` にある。

### 並列

並列には 3 つの層がある。上の層ほど互いに関係がなく、安全に広げられる。統括は、この順に広げる。

| 層 | 何を並べるか | 条件 |
|---|---|---|
| 1. persona の間 | 違う persona の batch | 条件なし。persona どうしは、ファイルも設定も共有しない |
| 2. batch の中 | 同じ batch の中の組（10 件ずつ） | `bin/batch` が自動で行う。レビューはすべての組を同時に動かす。原稿と成形は、計画の「先に要るもの」と、派生・添付の関係で順序を決め、同じ段に並んだ組だけを同時に動かす |
| 3. 同じ persona の batch | 同じ persona の 2 つ目、3 つ目の batch | 下の条件をすべて満たすときだけ |

同じ persona で複数の batch を同時に動かす条件：

- その persona に受入済みの batch が 1 つ以上ある（型と事実の台帳ができている）。
- 依頼の範囲が、動いている batch と重ならない。違う時期か、違う案件にする。互いの文書を引用・添付・参照する必要がある範囲は、同時に動かさない（後の batch を、前の batch の受入のあとに起動する）。
- 同時に動かすのは persona あたり 3 つまで（`config/limits.json` の `max_batches_per_persona`）。

`bin/batch` は、同じ persona の batch について次を守る。統括が気にする必要はない。

- 計画の段階（型の追加、ファイル計画、事実の台帳と persona の定義への追記）は、persona ごとに 1 つずつ行う。後から計画する batch は、先に計画した batch の依頼・計画・台帳を読み、パスと内容が重ならないようにする。
- 計画が済んだあとの原稿・レビュー・成形は、batch どうしで同時に進む。型・台帳・persona の定義は、計画の段階のあとは変更しない。
- 自動検査は、ほかの batch が作りかけているファイルを「計画にないファイル」と見なさない。受入のときの一覧（`manifests/`）には、受入済みのファイルと、その batch のファイルだけを入れる。
- 受入と破棄（commit）、指摘の記録への追記は、repo 全体で 1 つずつ行う。

全体の上限：

- 同時に動く Codex の実行は、repo 全体で `config/limits.json` の `max_codex`（32）までである。上限に達している間、後の組は空くのを待つ。`bin/batch status` が使用中の数を出す。
- repo の直下に、Codex の実行が置いたファイルが見つかったら、`bin/batch` がその実行の作業フォルダ（`work/<段階>-<連番>/stray/`）へ移し、報告に載せる。くり返すものは、改善のジョブで指示に足す。
- 上限は統括が変えてよい。`config/limits.json` を書き換えて、資料・スクリプトの変更として commit する。次に起動する batch から効く。メモリは上限の理由にならない（Codex の実行 1 つは約 240 MB）。目安：Codex の利用上限のエラーが続くなら下げる。利用上限に当たらず、負荷平均（`uptime`）がコア数（`nproc`）を下回っているなら上げてよい。
- Codex の利用上限に当たった batch は、`failed`（利用上限）で止まる。上限が戻ってから `bin/batch resume` する。
- 並列を広げるほど、統括の抜き取りと受入が詰まる。結果待ちの batch が 5 つを超えたら、新しい batch を起動する前に受入を進める。

## 3. `bin/batch` がすること

| 段階 | 担当 | 内容 | 出力 |
|---|---|---|---|
| テンプレート | `gpt-6.2-sol` | `templates/<persona>/` が無いときだけ、その persona の文書の型を作る（下の「テンプレート」） | `templates/<persona>/` |
| テンプレートのレビュー | `gpt-6.2-sol` | 型を作ったのとは別の実行で、型を開いて画像にし、`README.md` の説明と実物が合っているか、崩れがないか、余計な空の行や仮の欄がないかを確かめる。不合格は理由を渡して作り直させる（2 回まで） | |
| 計画 | `gpt-6.2-sol` | 依頼、persona の定義、これまでの計画、テンプレートの一覧を読み、ファイル計画を書く。複数のファイルにまたがる事実（文書番号、金額の内訳、会議の日時、メールのスレッド）を事実の台帳に書き足す | `progress/<persona>/<batch>.md`、`progress/<persona>/facts.md` |
| 計画の検査 | スクリプト | 列、拡張子、パスの重複、日付が期間内か、制作区分の名前がないか、件数。不合格なら理由を付けて計画をやり直させる（1 回） | |
| 原稿 | `gpt-6.1-terra` | 計画の行を 10 件ずつ、別々の Codex の実行で、中身を Markdown の原稿に書く（下の「原稿」）。「先に要るもの」が済んだ行から書き、同じ段の組は同時に動かす。先に要る原稿が不合格のまま破棄されても、後の行は計画と台帳をもとに書き、報告に一覧で出す | `sources/<persona>/…​.md` |
| 原稿の検査 | スクリプト | 全行に原稿があるか、制作の痕跡、同じ batch と既存の原稿との近似重複。計画の「関連」「先に要るもの」か原稿の `derived_from` で関係が書かれた版どうしは、近似重複にしない（中身が同じかどうかはレビューが見る） | |
| 原稿のレビュー | `gpt-6.2-sol` | `runbooks/REVIEW.md` の「原稿のレビュー」の観点で 1 件ずつ読み、合格か破棄かと理由を書く。不合格の原稿は消して書き直させる。2 回まで。まだ不合格の行は破棄し、成形しない | `scratch/…/draft-review-r<N>.tsv` |
| 成形 | `gpt-6.1-terra` | 合格した原稿を、形式の重さで分けた組ごとに別々の実行で、型に入れて DOCX・XLSX・PPTX・PDF・`eml`・画像にする。原稿の中身を変えない。1 組は重さの合計 6 まで（PPTX 3、XLSX・画像・スキャン 2、DOCX と TeX の PDF 1.5、そのほか 1。`config/limits.json` の `build_weights`・`max_build_weight` で変えられる）。実行が正常に終わったのに作られていない行は、不合格の回に数えず、同じ回の中でその行だけをもう一度渡す（2 回まで） | `people/<persona>/…` |
| 数式の計算 | `bin/recalc` | 数式を含む XLSX を再計算し、計算結果を各セルに保存する（数式と書式は変えない） | |
| 日付と内部の情報 | `bin/stamp` | 各ファイルの更新時刻を計画の「日付」にし、ファイルの内部の作成者・作成日時・更新日時・アプリ名を、原稿の `author` と計画の日付とその人が使うアプリにそろえる。制作の道具が残した情報を消す | |
| 自動検査 | `bin/check` | `bin/check <persona> --batch <batch>`。成果物の本文が原稿と一致するか（`source`）、数式の計算結果が保存されているか（`formula`）、内部に制作時の情報が残っていないか（`metadata`）も見る | `scratch/…/check-r<N>.tsv` |
| 見た目のレビュー | `gpt-6.2-sol` | `runbooks/REVIEW.md` の「見た目のレビュー」の観点で 1 件ずつ画像にして見る。不合格の成果物は消して成形し直させる（原稿は変えない）。2 回まで | `scratch/…/review-r<N>.tsv` |
| 報告 | スクリプト | 原稿が不合格のままの行は原稿を消し、計画のその行に「破棄：理由」と書く。原稿は合格したのに成形が不合格のままの行は、成果物だけを消して原稿を残し、計画に「成形待ち：理由」と書く。結果と指摘の記録をまとめる | `progress/<persona>/<batch>.result.md`、`progress/findings.tsv`、`progress/batches.tsv` |

ファイル計画（`progress/<persona>/<batch>.md`）は Markdown の表で、列は次のとおり。

| 列 | 内容 |
|---|---|
| パス | `people/<persona>/` からの相対パス。`personas/<persona>.md` のフォルダ構成に従い、人が付ける名前にする |
| 日付 | そのファイルを最後に更新した日時（persona の世界の中の日時、`2026-03-06 18:42` の形） |
| 作り方 | `docx`・`xlsx`・`pptx` はテンプレート名、PDF は元にするもの（`office:<テンプレート名>`、`tex:<テンプレート名>`、`scan:<元のファイルまたは tex/office>`）、画像は `photo`・`screenshot`・`chart`・`diagram`、その他は `text` |
| 内容 | そのファイルにしかない中身。誰が、何について、どの案件・数値・結論で書いたか。2〜4 文 |
| 関連 | 参照する、または参照される既存ファイル（あれば） |
| 先に要るもの | この行の原稿を書く前に、原稿ができていなければならない同じ batch の行のパス。後の文書が前の文書の文面そのものを使うときだけ書く（引用して返信するメールの元のメール、直す前の版、要約する元の文書、添付する文書）。出来事の前後関係だけなら書かない。無ければ空。循環させない |

報告（`progress/<persona>/<batch>.result.md`）には、依頼との対応、作った件数と形式の内訳、原稿の段階と成形の段階それぞれの不合格の件数・理由・やり直しの回数、破棄した行と理由、レビュー役の所見、抜き取りに使うレンダリングの場所（`scratch/<persona>/<batch>/work/` の下の各 `render/`）を書く。

### 原稿

原稿は、1 つの成果物の中身を全部書いた Markdown である。成形は、原稿に書いてあることだけを形にする。

- 置き場所は `sources/<persona>/<成果物と同じ相対パス>.md`（例：`sources/p14-finance-controller/Documents/月次/2026/2026-03/2026-03_現預金調整.xlsx.md`）。Git で管理し、配布物には含めない。あとの batch の計画と原稿は、過去の原稿を読んで事実を合わせる。
- 先頭に設定を置く。

  ```
  ---
  path: Documents/月次/2026/2026-03/2026-03_現預金調整.xlsx
  kind: cedar-reconciliation.xlsx
  author: 千田 真理子
  derived_from:
  ---
  ```

  - `kind`：計画の「作り方」。
  - `author`：その文書を作った人物（ファイルの作成者として入る）。社外から届いた文書は、その相手。
  - `modified_by`・`created`（任意）：別の人が直した版や、前の版を上書き保存した文書に書く。最終更新者と、最初に作った日時。書かなければ、最終更新者は `author`、作成日時は計画の日付になる。
  - `device`（任意）：写真を撮った機器や、スキャンした複合機の名前。写真とスキャンの内部の情報に入る。
  - `derived_from`：**元が成果物として `people/<persona>/` にある**ものだけに、その相対パスを書く（Word から出した PDF、手元の文書を印刷してスキャンした PDF、メールに添付した控え）。その場合、本文には元との違い（押印、手書きの書き込み、スキャンの傾きなど。違いが無ければ「違いなし」）だけを書く。元が成果物でないもの（社外から紙で届いた文書のスキャンなど）は空にして、本文に全文を書き、紙やスキャンの様子を末尾に書く。
- 本文に書くもの：
  - **文書**（DOCX、PDF、テキスト）：見出しと本文の全文。表は Markdown の表。変更履歴やコメントがあるなら、誰がいつ何をどう直したか。
  - **表**（XLSX、CSV）：シートごとに、見出し、全部の行、数式（合計や差異は値ではなく `=SUM(E7:E11)` の形）、注記。行が多いデータ（数十行以上の明細）は、列の定義、行の作り方、合計が合う条件、代表の 10 行を書く。
  - **スライド**（PPTX）：1 枚ごとに、レイアウト、題、本文、表や図の中身、発表者のメモ。
  - **メール**（`eml`）：From、To、Cc、Date、Subject、Message-ID、In-Reply-To、本文、引用、添付の名前。
  - **グラフと図**：元の数値、軸、単位、凡例、題。
  - **写真・手書き・スクリーンショット**：何がどう写っているか、読める文字は全部。
- 書かないもの：書式の指定（フォント、色、余白）。それは型が決める。

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
- 型には、見出しと書式の決まりだけを持たせる。中身の入っていない書式つきの行、仮の合計欄を残さない。行数は文書ごとに違うので、明細の行と合計の行は成形のときに書式ごと作る。作り方は `README.md` に書く。
- 見た目のレビューで型そのものの欠陥（説明と実物の食い違い、型の崩れ）が見つかったら、統括が `bin/batch templates <persona> --fix "<欠陥の内容>"` で型を直させる（型の作り直しとレビューが動く）。直した型は、まだ受入していない batch から使う。
- 個人のメモ、作業中の表、古い資料など、型を使わない文書も実際にはある。全部を型どおりにしない。

## 5. 制作の道具

| 道具 | 使い方 |
|---|---|
| `bin/render <file> <outdir>` | DOCX・XLSX・PPTX・PDF をページごとの PNG にする。制作とレビューで、見た目を確かめるのに使う |
| `bin/tex <file.tex> <out.pdf>` | `lualatex` で PDF にする。一時物は `scratch/` に置く |
| `bin/scan <in.pdf または画像> <out.pdf>` | 紙をスキャンしたような PDF にする（傾き、濃淡、ノイズ、JPEG 圧縮。テキスト層なし） |
| `bin/stamp <file> --author <名前> --date <日時> [--timezone <tz>]` | ファイルの内部の作成者・日時・アプリ名をそろえ、制作の道具の情報を消す。進行役が成形のあとに全ファイルにかけるので、成形の中で呼ぶ必要はない |
| `bin/recalc <file.xlsx>` | 数式を再計算して計算結果を保存する。進行役が成形のあとにかける。成形の中で結果を確かめるのに使ってもよい |
| `bin/office-pdf <file> <out.pdf>` | DOCX・XLSX・PPTX を PDF にする（「PDF で保存した」文書用） |
| `uv run python` | `python-docx`、`openpyxl`、`python-pptx`、`matplotlib`、`pillow` で、テンプレートを開いて中身を入れる |
| Codex の `imagegen` skill | 写真、手書きのメモ、ホワイトボードなどの画像 |

- スキャン PDF は、中身のある文書（TeX か Office で組んだページ、または手書き風の画像）を `bin/scan` に通して作る。
- グラフと図は、その文書の数値から描く。文書の中の数値と一致させる。
- 文書を量産するスクリプトは作らない（`AGENTS.md` の「作り方」）。

## 6. 抜き取りと受入（統括）

通知が来たら、`progress/<persona>/<batch>.result.md` を読み、自分で開いて確かめる。

- 形式ごとに 1 件以上、合計で batch の 2 割以上（最低 8 件）。Office と PDF は `scratch/<persona>/<batch>/work/` の下の各 `render/` の画像を見る（報告に場所が書いてある）。画像は画像そのものを見る。
- 見る点は `runbooks/REVIEW.md` と同じ。成果物を開いて、中身（原稿のレビューの観点）と見た目（見た目のレビューの観点）の両方を、同じ基準で確かめる。中身は `sources/<persona>/` の原稿でも読める。
- 自分の抜き取りで不合格が 2 件以上出たら、レビューが甘いとみて batch 全体を破棄するか、`bin/batch resume <persona> <batch> --draft-review-again`（中身）または `--review-again`（見た目）でレビューからやり直す。

```sh
bin/batch accept <persona> <batch>            # 検査をもう一度通し、manifest を更新し、その batch の成果物・原稿・計画・報告だけを commit する
bin/batch accept <persona> <batch> --drop <パス>...   # 指定のファイルを破棄して、残りを受け入れる
bin/batch discard <persona> <batch> --reason "<理由>" # batch のファイルをすべて消し、理由を計画に書いて commit する
git push origin main
```

- 「成形待ち」の行が残っていても受入できる（原稿だけが commit される）。あとで `bin/batch resume <persona> <batch> --rebuild` で成形し直し、抜き取りで確かめてから、もう一度 `bin/batch accept` する。原稿に原因があって成形できない行は、`--drop` で破棄する。原稿の段階で破棄になった行が、仕組みや指示の不備によるものだったときは、`bin/batch resume <persona> <batch> --redraft` で原稿からやり直せる（統括が `--drop` で破棄した行と、合格済みの行には触れない）。
- 受入または破棄が済むと、`scratch/<persona>/<batch>/` は消える。ただし「成形待ち」の行が残っている間は、原因を調べられるように残す。
- 同じ batch で不合格が 2 回続いたら、`bin/batch start ... --producer gpt-6.2-sol` で原稿と成形を Sol に切り替え、切替えと理由を依頼の末尾に書く。 自動運転の統括は、対象の依頼に独立した行で `- 再制作の担当：gpt-6.2-sol` と理由を記す。`--rebuild` と `--redraft` がこの指定を読み、明示した `--producer` があればそちらを使う。
- 形式ごとに、特に出来のよい受入済みファイルを `templates/<persona>/README.md` の「見本」に書き足す。制作とレビューは見本を水準として使う。

## 7. 配布物を組み立てる

```sh
bin/pack <出力先のフォルダ>
```

`bin/check --all` と `manifests/` の照合に合格したときだけ、persona ごとに `<persona>.tar.gz` を作る。中身は `people/<persona>/` の下だけで、更新時刻は `manifests/<persona>.tsv` の値にする。あわせて `SHA256SUMS` と配布用の `README.md` を出力する。出力先は repo の外か `scratch/` にし、Git に入れない。

## 8. 指摘から仕組みを直す

レビューや検査でくり返し見つかる問題は、検査や指示に取り込んで、作る前に防ぐか機械的に落とす。

- 記録の対象外：作る順序の都合で成形を見送った行（元のファイルが未完成）は、報告には載せるが、指摘の記録には入れない。
- 記録：`bin/batch` が batch のたびに、不合格を `progress/findings.tsv` に、batch ごとの件数と率を `progress/batches.tsv` に追記する。対象は、原稿の検査、原稿のレビュー、自動検査、見た目のレビュー、統括の抜き取りでの破棄。レビューの指摘には観点の番号（原稿は `D1`〜`D10`、見た目は `V1`〜`V6`、当てはまらなければ `other`）が付く。
- 集計：`bin/batch findings` が、段階 × 観点の件数、多い順の一覧、batch ごとの初回の不合格率の推移、改善の前後の件数を出す。
- 改善の時期：受入済みの batch が 10 たまるか、同じ観点（または同じ検査名）の指摘が 5 件を超えたら、統括が改善のジョブを起動する。batch が動いていても起動してよい。
- 改善のジョブは、本体の作業フォルダを直接変えず、git の worktree の中で作業する。動いている batch は、指示のひな形と検査を段階ごとに読むので、作りかけの変更を読ませないためである。

  ```sh
  git worktree add -B improve scratch/wt-improve main     # 既にあれば: git -C scratch/wt-improve merge --ff-only main
  bin/batch findings > scratch/wt-improve/scratch/findings.txt
  codex-job start --repo personaCorpus --name improve-<日付> --brief "$PWD/prompts/improve.md" --model gpt-6.2-sol --effort medium
  ```

- 取り込み：統括が、下の「統括の確認」を worktree の中で済ませてから、worktree で commit し、`git -C scratch/wt-improve rebase main`、`git merge --ff-only improve`、`git push origin main` の順に取り込む。取り込んだ指示のひな形と `bin/check` は、動いている batch にも次の段階から効く。`bin/batch` の変更は、取り込みのあとに起動した実行（`resume` を含む）から効く。
- 計画・原稿・台帳・状態の書式を変える変更と、段階の順序を変える変更は、動いている batch と食い違うので、改善のジョブには任せない。統括が、どの batch も動いていないとき（`bin/autopilot stop` のあと、動いている batch が終わってから）に、自分で、または Codex（`gpt-6.2-sol`）に指示書を書いて行う。旧い計画と状態が読めることを、テストで確かめる。
- 改善のジョブがしてよいこと：
  1. 機械的に判定できる指摘を、`bin/check` か原稿の検査に、テストつきで足す。
  2. 作る前に防げる指摘を、`prompts/` の指示に書き足す。
  3. 型の作り方に原因がある指摘を、`prompts/templates.md` に書き足す。
  4. 何を足したか、対象の観点、日付を `progress/improvements.md` に書く。
- 改善のジョブがしてはいけないこと：既存の検査の条件を緩める、レビューの観点を削る・弱める、`SPEC.md`・`AGENTS.md`・`runbooks/` を変える、成果物と原稿を変える。必要だと判断したら、変更せずに理由を報告に書く。その変更を行うかどうかは、統括が決めて、自分で行う（検査を緩める変更は、原因が検査の誤りであることを実際のファイルで確かめてから。理由を commit に書き、ユーザーへの報告に含める）。
- 統括の確認：テスト（`tests/run.sh`）が通ること、既存の検査が緩んでいないこと（差分を読む）、新しい検査を受入済みの全ファイルに当てて（`bin/check --all --accepted`）、落ちたものが見逃していた不良か検査の誤りかを確かめること。検査の誤りなら、その検査を入れない。確認できたら、資料・スクリプトの変更として、成果物とは別に commit する。
- 目標は、不合格の件数を減らすことではない。同じ種類の不合格がくり返されないことと、統括の抜き取りでの不合格がゼロに近いことである。

## 9. 止まったとき

- `failed` の通知が来たら、`bin/batch status <persona> <batch>` で止まった段階と理由を見る。Codex の利用上限なら、上限が戻ってから `bin/batch resume` する。それ以外は、`scratch/<persona>/<batch>/logs/` の該当の出力を読んで原因を直してから `resume` する。
- `resume` は、済んだ段階と、検査に通っているファイルをやり直さない。
- ファイルの場所を移す commit を push するときは `git -c pack.useSparse=false push origin main` を使う。既定の設定では、移しただけのファイルを全部送り直して上限を超える。
- 止まるたびに規則を増やして対処しない。原因が仕組みにあれば仕組みを直し、規則の変更が要るなら、統括が自分で直す（`AGENTS.md` の「規則の変更」）。

## 10. 自動運転（`bin/autopilot`）

Claude の利用上限で制作が止まらないように、依頼の作成から受入までを、LLM ではないスクリプトが回す。Codex のジョブ（制作、レビュー、監査）を起動し、結果のファイルを読んで次の操作を決める。統括のセッションが無くても進む。

```sh
bin/autopilot start [--personas p01,p05,...] [--target 300]   # tmux の中で動き続ける（codex-job のジョブ autopilot）
bin/autopilot status                                           # persona ごとの受入済み・進行中・保留、Codex の使用中の数
bin/autopilot stop                                             # 新しい batch と roadmap の起動をやめる。動いている batch は最後まで進み、やり直し・監査・受入まで行う
bin/autopilot once                                             # 1 回りだけ実行して終わる（手で確かめるとき）
bin/autopilot stop --now                                       # 自動運転のループだけをすぐ止める（動いている batch は止めない）
```

### 制作の道筋（roadmap）

- `progress/<persona>/roadmap.md` に、その persona で作る batch を順に並べる。列は、batch、範囲、件数の目安、重点、含めたいもの、避けるもの、状態。
- roadmap が無い persona は、自動運転が最初に Codex（`gpt-6.2-astra`、`prompts/roadmap.md`）に作らせる。persona の定義の「仕事の流れ」「フォルダ構成」「形式の目安」「規模の目安」と、これまでの計画を読み、最初の区切り（300 件）までの batch を、範囲が重ならないように並べる。最初の batch は、その persona の中心になる仕事にする。
- 統括は roadmap を直してよい（順番、範囲、重点）。自動運転は、状態が空の行を上から順に使う。除外したファイルだけを再制作するときは、統括が範囲の先頭に「再制作：」を書き、元のbatchと対象を特定する。この行は30件未満でも実際の対象件数を書く。

### 自動運転が 1 回りで行うこと（60 秒ごと。`AUTOPILOT_INTERVAL_SECONDS` で変えられる）

1. **止める条件を見る**：`bin/autopilot stop` が呼ばれている、空き容量が 30 GB 未満、保留の batch が 5 つ以上、のどれかなら、新しい起動をしない。
2. **終わった batch を進める**（persona ごとに）：
   - 失敗で止まった batch：理由が利用上限なら、30 分おいてから `bin/batch resume`。それ以外は 1 回だけ `resume` し、また失敗したら保留にする。
   - 終わった batch に、原稿の段階の破棄が 3 行以上あれば `--redraft` を 1 回、成形待ちがあれば `--rebuild` を 2 回まで行う。
   - やり直しが済んだ batch に、受入の監査（下）を起動する。
   - 監査が合格なら、監査が挙げた行を `--drop` して `bin/batch accept`、`git push origin main`、roadmap の状態を「受入済み」にする。監査が保留なら、何もせず保留にする。
3. **新しい batch を起動する**：進行中の batch が無い persona について、roadmap の次の行から依頼（`progress/<persona>/<batch>.request.md`）を書き、`bin/batch start` する。受入済みの件数が目標（`--target`、既定 300）に達した persona は起動しない。同じ persona の batch を並べるのは、roadmap の行に「並行可」と書いてあるときだけ（「並列」の条件を満たす範囲であることは、roadmap を書いた者が確かめる）。
4. **記録する**：行ったことを `scratch/autopilot/log.tsv` に 1 行ずつ書く。保留にした batch は `progress/holds.md` に、persona、batch、理由、日時を書く（Git で管理する。統括が処理したら行を消す）。

### 受入の監査

- 担当は Codex `gpt-6.2-astra`（`prompts/audit.md`）。制作（Terra）ともレビュー（Sol）とも違うモデルが、できあがった成果物だけを見る。統括の抜き取りの代わりである。
- 見る範囲：形式ごとに 1 件以上、合計で batch の 2 割以上（最低 8 件）。報告に「先に要るものが欠けたまま書いた行」「続きを使った行」があれば、それを優先する。成果物を開き、Office と PDF は `bin/render` で画像にして見る。観点は `runbooks/REVIEW.md` の両方（中身と見た目）。原稿・計画・台帳・persona の定義と照らす。
- 出力：`progress/<persona>/<batch>.audit.md`。先頭の 2 行が `verdict: accept`（または `verdict: hold`）と `drop: ["<パス>", …]`（JSON の配列。無ければ `[]`）。続けて、見たファイルの表（パス、判定、理由）と、batch の全体の所見。成果物が 8 件未満の batch は、全件を見る。
- 判定の決まり：見たうちの不合格が 1 件以下なら `accept`（その 1 件は `drop` に入れる）。2 件以上、または同じ原因が batch の全体に及ぶと見たら `hold`。迷ったら `hold`。
- 監査はファイルを変更しない。自動運転が、監査の前後で成果物と原稿の sha256 が変わっていないことを確かめる。

### 初めて動かす前に

- `codex-job status --repo personaCorpus` でジョブが起動できること、`git push origin main` ができること、空き容量を確かめる。
- 最初の数 batch は、自動運転が受入したものを統括が全部開いて、監査の判定が妥当かを確かめる。妥当なら、10 batch に 1 つの確認に移る。

### 統括がすること

統括は、Claude Opus のセッションでも、Codex `gpt-6.2-astra` のセッションでもよい（`prompts/coordinator.md` を最初に渡す）。自動運転が動いていれば、統括がいない間も制作は進む。

自動運転は、次のときに Codex `gpt-6.2-astra` の統括（`prompts/coordinator.md`）を 1 つだけ起動する：`progress/holds.md` に保留がある、確認が要る受入済みの batch がある、同じ batch の処理が 3 回以上失敗した。統括は sandbox の中で `.git` に書けないので、自動運転が main から作った worktree（`scratch/autopilot/wt`）で編集し、操作（`merge`・`accept`・`discard`・`resume`・`unhold`・`ask`・`start`）を `actions.json` に書く。自動運転は、統括のジョブが終わったら操作を順に実行し、失敗したら後続の操作を中止する。`merge` は、`SPEC.md`・`AGENTS.md` の変更と `people/`・`sources/` の追加・変更を拒み、道具の変更ならテストと受入済みの全ファイルの検査に通ったときだけ取り込んで、ビルドし直し、自動運転を新しい道具で起動し直す。ユーザーの確認が要る項目は、統括が `ask` で問いを残し、自動運転はその項目で統括を起動し直さず、`bin/autopilot status` の「ユーザーの判断待ち」に出す。ユーザーが `bin/autopilot answer <項目> <答え>` で答えると、次の統括の起動にその答えが渡る。受入済みファイルの大量の削除を認めるときは `--allow-delete <数>` を付け、次に成功する `merge` 1 回だけ、削除の上限（ふだんは 3 件）がその数になる。Codex の利用上限で終わったジョブ（記録の末尾に usage limit など）は、失敗にも保留にも数えず、自動運転は 30 分ごとに試すまで新しい Codex のジョブ（batch の開始、監査、統括）を起動しない。同じ項目が 3 回の統括で片付かなければ、自動運転は統括の起動をやめ、`bin/autopilot status` の「手が要るもの」に出す（ユーザーか対話の統括が処理する）。

- `bin/autopilot status` と `progress/holds.md` を見て、保留の batch を処理する：監査の報告を読み、自分で開いて確かめ、受入（`--drop` つき）か、やり直しか、破棄かを決める。原因が仕組みにあれば、仕組みを直す。
- 確認の連続一致（`progress/checks.tsv`）が 10 に達するまでは、受入済みの batch をすべて、達したあとは 10 につき 1 つを自分で開いて確かめる（監査の甘さの確認）。結果は `progress/checks.tsv` に `一致`・`不一致` で記録する。不合格が見つかったら、自動運転を止め、監査の指示を直し、同じ時期に受入された batch を見直す。
- `bin/batch findings` を見て、8 節の改善のジョブを回す。
- roadmap を見直す。形式の割合が目安から外れてきたら、次の行の「含めたいもの」で調整する。
- ユーザーへの報告：persona ごとの受入済みの件数、保留と破棄、直した仕組みと緩めた検査、判断を仰ぐこと（選択肢と自分の案）。
