\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{osada-packing-notice}[2025/03/25 Packing notice]
\LoadClass{osada-sample-certificate}

\newcommand{\OSANoticeHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \begin{flushright}\small 案内番号\enspace #2\par
    発行日\enspace #3\par 発行担当\enspace #4
  \end{flushright}
  {\large #5\par}\vspace{3mm}
  \begin{center}\LARGE\bfseries\color{OSAOchre}#1\end{center}\medskip}
