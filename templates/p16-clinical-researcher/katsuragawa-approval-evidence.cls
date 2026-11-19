\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-approval-evidence}[2025/04/15 Approval evidence check]
\LoadClass{katsuragawa-submission-check}
\fancyhead[R]{\small\color{KRCWine}承認日根拠確認票}
\definecolor{KRCPaleRose}{HTML}{F7EDF1}
\RequirePackage{colortbl}
\newcommand{\KRCEvidenceItem}[4]{%
  \par\medskip\noindent\begin{minipage}{\linewidth}
    \setlength{\tabcolsep}{3mm}%
    \renewcommand{\arraystretch}{1.35}%
    \arrayrulecolor{KRCRose}%
    \begin{tabular}{@{}p{29mm}p{119mm}@{}}
      \rowcolor{KRCPaleRose}\multicolumn{2}{l}{\textcolor{KRCWine}{\bfseries #1}}\\\hline
      照合資料 & #2\\
      確認結果 & #3\\
      次の確認先 & #4\\\hline
    \end{tabular}
  \end{minipage}\par}
