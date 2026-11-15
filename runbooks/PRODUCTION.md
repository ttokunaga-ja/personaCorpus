# 制作と受入の手順

内容の基準は `SPEC.md`、規則は `AGENTS.md`、persona の設定は `personas/<persona>.md` にある。この文書は、統括（Opus）が 1 つの batch を回す手順である。

## 0. 環境の準備（初回だけ）

- `uv sync` で Python 環境を作る（`pyproject.toml`）。
- `bin/check --self-test` が通ることを確かめる。LibreOffice（`soffice`）、`pdftotext`、`pdftoppm`、`tesseract`、`ffprobe` が要る。足りないものは検査が名前を挙げて失敗する。

## 1. ファイル計画を書く（統括）

`progress/<persona>/<batch>.md` に、その batch で作るファイルを 1 行ずつ書く。batch 名は `b001` からの連番、1 batch は 50 ファイル程度までとする。

| 列 | 内容 |
|---|---|
| パス | `people/<persona>/` からの相対パス。`personas/<persona>.md` のフォルダ構成に従い、人が付ける名前にする |
| 日付 | そのファイルを最後に更新した日時（persona の世界の中の日時） |
| 内容 | そのファイルにしかない中身。誰が、何について、どの案件・数値・結論で書いたか。2〜4 文 |
| 関連 | 参照する、または参照される既存ファイル（あれば） |

計画を書くときに確かめること：

- 「内容」が、既存のファイルや同じ batch の他の行と、題名や日付の違いだけになっていないか。なっていれば、その行を削る。
- 実際の仕事の流れになっているか。たとえば、依頼のメール、下書き、レビューの指摘、改訂版、会議の議事録、決定の連絡。
- 形式の割合が `personas/<persona>.md` の目安から大きく外れていかないか。件数合わせの行を足さない。
- 人物、組織、案件、数値が `personas/<persona>.md` と既存ファイルに合っているか。

計画の下書きを Codex（`gpt-6.1-sol`）に任せてもよい。その場合も、上の点は統括が確かめる。

## 2. 制作を指示する

指示書を `scratch/<persona>/<batch>/brief.md` に書き、次で起動する。同じ persona のジョブが動いていないこと、空き容量が 30 GB 以上あることを先に確かめる。

```sh
codex-job status --repo personaCorpus
df -B1 --output=avail . | tail -1
codex-job start --repo personaCorpus --name <persona>-<batch> \
  --brief "$PWD/scratch/<persona>/<batch>/brief.md" --model gpt-6.1-terra --effort medium
agent-alert wait --repo personaCorpus   # Bash の background で実行する
```

指示書に書くこと：

```markdown
# <persona> <batch> の制作

読むもの：`SPEC.md`、`AGENTS.md` の「personaCorpus の制作規則」、`personas/<persona>.md`、`progress/<persona>/<batch>.md`。

作るもの：`progress/<persona>/<batch>.md` の各行のファイルを、`people/<persona>/` の下の指定のパスに作る。計画にないファイルは作らない。

- 1 件ずつ、「内容」の列に書かれた固有の中身で書く。文書を量産するスクリプトは使わない。
- 本文、ファイル名に、合成であることの注記、制作用の ID、batch の情報を書かない。
- 作ったファイルの更新時刻を、計画の「日付」に合わせる（`touch -d`）。
- 一時物は `scratch/<persona>/<batch>/` だけに置く。
- 作り終えたら `bin/check <persona> --batch <batch>` を実行し、不合格を直す。直せないファイルは消して、理由を報告する。
- stage・commit・push はしない。

返すもの：作ったファイルの件数、作らなかった行とその理由、`bin/check` の最後の結果。
```

## 3. 自動検査（`bin/check`）

```sh
bin/check <persona> --batch <batch>   # その batch の計画のファイルを、persona 全体と照らして検査する
bin/check <persona>                   # persona 全体を検査する
bin/check --all                       # 全 persona を検査する
```

終了コードは、合格が 0、不合格が 1 である。不合格のファイルは、パス、検査名、理由を 1 行ずつ出す。`--report <path>` で同じ内容を TSV に書く。

| 検査名 | 不合格の条件 |
|---|---|
| `format` | 拡張子が `SPEC.md` の「含める」にない。隠しファイル、空のファイル、0 バイト |
| `opens` | 拡張子どおりに開けない。DOCX・XLSX・PPTX は ZIP と必須の部品、PDF は `pdftotext` と `pdfinfo`、画像と音声はヘッダとデコード、`eml` はヘッダ、`json`・`xml` は構文、`csv`・`tsv` は列数の一致、テキストは UTF-8 |
| `path` | パスの成分が制作区分の名前（`foundation`、`visual`、`source-art`、`full`、`m1`、`primary`、`secondary`、`pdf_scan`、`pdf_text`、`txt_log`、`csv_tsv`、`html_eml`、`structured_text`、`domain_binary`、`imagegen` を含む名前）。ファイル名に persona ID（`p01` など）、`-s` と 6 桁の数字、5 桁以上の連番だけで区別される名前がある |
| `trace` | 抽出した本文に制作の痕跡がある：`Kio`、`kio-eval`、`personaCorpus`、`persona`、`corpus`、`fixture`、`content spine`、`synthetic`、`fictional`、`not a real`、`artifact id`、`source id`、`batch`、`scope` と ID の組、`pNN-` で始まる ID、`/Users/`、`scratch/`、`lorem ipsum`、`placeholder`。その職種の用語として正しい語（例：研究者の「synthetic route」、経理の「batch payment」）は、`personas/<persona>.md` の「許可する語」に書いてあれば通す |
| `duplicate` | 同じ persona の中に、バイトが同じファイルがある。または、数字と日付をならした本文の 5 語つづりの一致率が 0.8 以上のファイルがある（テキストを取り出せる形式が対象） |
| `empty-scan` | 画像だけの PDF と画像で、OCR で読める文字が 20 字未満（写真と明記された置き場所を除く。置き場所は `personas/<persona>.md` の「写真のフォルダ」） |
| `secret` | 秘密情報の形の文字列がある（AWS・GitHub・Slack・Vault のトークンの形、秘密鍵のヘッダ、`sk-` で始まる鍵の形、JWT の形）。バイナリも生のバイトで調べる。PDF に `/ASCII85Decode` がある |
| `mtime` | 更新時刻が、persona の定義の期間の外にある。`--batch` のときは、計画の「日付」と 1 日以上ずれている |
| `plan` | `--batch` のとき：計画にあるのに無いファイル、計画にないのに `people/<persona>/` に増えた未追跡のファイルがある |

検査は不合格の候補を機械的に挙げるものである。合格は、品質が十分であることの証明ではない。

## 4. 抜き取り確認（統括）

`bin/check` に合格した batch について、統括が自分で開いて読む。

- 形式ごとに 1 件以上、合計で batch の 2 割以上（最低 8 件）。Office と PDF は `scratch/<persona>/<batch>/` にレンダリングした画像を見る。画像は画像そのものを見る。
- 見る点：計画の「内容」どおりの固有の中身があるか。その職種の実務の文書として具体的か。persona の設定と矛盾しないか。同じ batch の隣のファイルと文面が同じでないか。表の数値と数式が合っているか。レイアウトが崩れていないか。
- 判断が難しいときは、Codex（`gpt-6.1-sol`、`--sandbox read-only`）に見る点を指定して監査させ、その指摘を自分で確かめる。

## 5. 受入または破棄

- **受入**：抜き取りで問題がなければ、その batch のファイルと計画だけを stage して commit・push する。

  ```sh
  bin/manifest <persona>                       # manifests/<persona>.tsv を更新する（パス、sha256、大きさ、更新時刻）
  git add progress/<persona>/<batch>.md manifests/<persona>.tsv
  git add --pathspec-from-file=<その batch のパスの一覧>
  git commit -m "Accept <persona> <batch>: <内容の要約>"
  git push origin main
  ```

- **一部だけ不合格**：不合格のファイルを消し、計画のその行に「破棄：理由」と書いて、残りを受け入れる。作り直す場合は次の batch の計画に入れる。
- **batch 全体が不合格**：その batch の未追跡ファイルをすべて消し、計画に理由を書いて commit する。同じ batch で不合格が 2 回続いたら、`AGENTS.md` のとおり `gpt-6.1-sol` に切り替える。
- 受入または破棄が済んだら、`scratch/<persona>/<batch>/` を消す。

## 6. 配布物を組み立てる

```sh
bin/pack <出力先のフォルダ>
```

`bin/check --all` と `manifests/` の照合に合格したときだけ、persona ごとに `<persona>.tar.gz` を作る。中身は `people/<persona>/` の下だけで、更新時刻は `manifests/<persona>.tsv` の値にする。あわせて `SHA256SUMS` と配布用の `README.md` を出力する。出力先は repo の外か `scratch/` にし、Git に入れない。

## 7. ジョブが止まったとき

- `agent-alert` の `limit` は、成果物の本文に「quota」「429」などの語があるだけでも出る。`codex-job status` と log の末尾で、実際に利用上限かどうかを確かめる。
- `failed`、`stalled` のときは、log の末尾で原因を見て、同じ batch を別のジョブ名（`<persona>-<batch>-r2` など）で起動し直す。作りかけのファイルは、`bin/check <persona> --batch <batch>` で不合格のものだけ消してから続ける。
- 規則を増やして対処しない。規則の変更が要ると思ったら、ユーザーに伝えて Fable のセッションで行う。
