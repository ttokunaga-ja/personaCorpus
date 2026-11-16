# persona の文書の型を作る

指示の「読むファイル」に示されたパスを用途に従って開いてください。資料の本文は指示に埋め込まれていません。事実の台帳は、対象の行（計画前は依頼）に関係する案件・出来事の見出しを検索し、該当する部分を読んでください。
AGENTS.md、SPEC.md、runbooks/PRODUCTION.md のテンプレート節、personas/README.md を読み、「読むファイル」に指定された persona 定義と依頼に必要な型を作ってください。
テンプレート・計画のモデルは Sol です。型は templates/<persona>/ に置き、README.md に名前・組織と用途・実際のファイル名・見た目を列挙してください。
自社・社外の型を分け、Office の型と必要な TeX の型を作り、既存の型があれば開いて維持してください。
書いてよい場所は templates/<persona>/、progress/<persona>/、指定された scratch のみです。personas/<persona>.md は「仕事の流れ」への追記だけが可能です。
一時物は指定された scratch 以下。people/ に制作記録を置かず、他 persona と資料を変更せず、stage・commit・push はしないでください。
下の BATCH_CONTEXT の persona・batch・scratch が実際の出力先です。

型の作り方：
- その会社の規模と業種に合った、実在しそうな見た目にする。ロゴ（図形と社名の文字で作る）、配色（2〜3 色）、フォント（日本語は Noto Sans CJK JP か Noto Serif CJK JP、英語は環境にあるもの）、ヘッダとフッタ、文書番号の付け方を決め、Word・PowerPoint・Excel・TeX でそろえる。
- Word は段落と見出しのスタイル、表のスタイル、表紙の有無を型に持たせる。PowerPoint はスライドマスターとレイアウト（表紙、見出しと本文、2 段、表、図）を持たせる。Excel は見出し行、数値、合計行、注記の書式の決まりと、印刷の設定を持たせる。中身の入っていない書式つきの行や仮の合計欄は残さない。明細と合計の行は成形のときに書式ごと作るので、その作り方を README.md に書く。
- 社外の型は、相手ごとに自社とはっきり違う見た目にする。
- 型ごとに、中身を仮に入れた見本を bin/render で画像にして自分で見て、崩れを直す。見本と画像は scratch に置き、templates/<persona>/ には型と README.md だけを置く。
- 型の中に、仮の文、合成であることの注記、制作用の ID を残さない。
- 他の persona の templates/ を見て、似た見た目にならないようにする。

BATCH_CONTEXT の reasons に欠陥の指摘があれば、実際の型と README.md を開き、指摘された欠陥を解消してください。型を直しても受入済み成果物は変更しません。修正後は別の実行の templates-review が検査します。

実行専用の作業フォルダは BATCH_CONTEXT の work、レンダリング先は render です。他の実行の作業フォルダへ一時物を書きません。指定された review_output だけは指定先へ書きます。
