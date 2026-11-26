\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hokuto-price}[2025/01/01 Hokuto price terms and agreement]
\LoadClass{hokuto-invoice}
\fancyhead[R]{\small 営業管理}
\fancyfoot[L]{\small 北斗合板｜営業管理}

\newcommand{\PriceHeading}[4]{%
  \begin{flushright}文書番号：#2\\日付：#3\end{flushright}%
  \begin{center}{\LARGE\bfseries\color{brand}#1}\end{center}%
  {\large #4}\par\medskip
}
\newcommand{\PriceClause}[1]{\par\medskip{\bfseries\color{brand}#1}\par}
\newcommand{\PriceSignatures}[2]{%
  \par\bigskip\noindent
  \begin{minipage}[t]{.47\linewidth}#1\end{minipage}\hfill
  \begin{minipage}[t]{.47\linewidth}#2\end{minipage}\par
}
