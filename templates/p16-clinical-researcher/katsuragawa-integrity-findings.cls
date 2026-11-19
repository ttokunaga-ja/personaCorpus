\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-integrity-findings}[2025/04/15 Record integrity findings]
\LoadClass{katsuragawa-summary}
\fancyhead[R]{\small\color{KRCWine}完全性点検}
\fancyfoot[L]{\small\color{KRCInk}品質確認}
\newcommand{\KRCFindingsHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\LARGE\bfseries\color{KRCWine}#1\par}
  {\small 文書番号\quad #2\qquad 版\quad #3\par
   作成日\quad #4\qquad 作成者\quad #5\par}\medskip}
