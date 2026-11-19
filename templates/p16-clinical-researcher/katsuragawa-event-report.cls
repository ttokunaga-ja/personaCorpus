\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-event-report}[2025/10/02 Event report reconciliation]
\LoadClass{katsuragawa-summary}
\fancyhead[R]{\small\color{KRCWine}事象報告の照合}
\newcommand{\KRCEventHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\LARGE\bfseries\color{KRCWine}#1\par}
  {\small 文書番号\quad #2\qquad 版\quad #3\par
   作成日\quad #4\qquad 作成者\quad #5\par}\medskip}
