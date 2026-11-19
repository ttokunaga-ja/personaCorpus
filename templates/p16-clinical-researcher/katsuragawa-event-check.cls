\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-event-check}[2025/10/09 Event reconciliation checklist]
\LoadClass{katsuragawa-submission-check}
\fancyhead[R]{\small\color{KRCWine}事象報告　確認票}
\newcommand{\KRCEventCheckField}[2]{%
  \par\noindent\begin{minipage}{\linewidth}
    \textcolor{KRCWine}{\ensuremath{\square}}\enspace\textbf{#1}\par
    {\small #2}\par
    \vspace{7mm}{\color{KRCRose}\hrule height .45pt}
    \vspace{8mm}{\color{KRCRose}\hrule height .45pt}
  \end{minipage}\par\medskip}
