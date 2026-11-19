\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-submission-check}[2024/10/15 Submission checklist]
\LoadClass{katsuragawa-summary}
\RequirePackage{amssymb}
\fancyhead[R]{\small\color{KRCWine}提出前確認票}
\newcommand{\KRCCheckHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\LARGE\bfseries\color{KRCWine}#1\par}
  {\small 文書番号\quad #2\qquad 版\quad #3\par
   作成日\quad #4\qquad 作成者\quad #5\par}\medskip}
\newcommand{\KRCCheckItem}[3]{%
  \par\noindent\begin{minipage}{\linewidth}
    \textcolor{KRCWine}{\ensuremath{\square}}\enspace\textbf{#1}\par
    \vspace{2mm}確認\quad #2\par
    \vspace{2mm}備考\quad #3\par
    \vspace{2mm}{\color{KRCRose}\hrule height .45pt}
  \end{minipage}\par\medskip}
