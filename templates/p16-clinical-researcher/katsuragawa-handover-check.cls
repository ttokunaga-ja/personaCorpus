\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-handover-check}[2025/02/06 Sample handover checklist]
\PassOptionsToPackage{table}{xcolor}
\LoadClass{katsuragawa-summary}
\RequirePackage{array,tabularx}
\definecolor{KRCBlush}{HTML}{F7EDF1}
\fancyhead[R]{\small\color{KRCWine}試料受渡し確認票}
\fancyfoot[L]{\small\color{KRCInk}薬剤管理}
\newcommand{\KRCHandoverHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\LARGE\bfseries\color{KRCWine}#1\par}
  {\small 文書番号\quad #2\qquad 版\quad #3\par
   確認日\quad #4\qquad 確認者\quad #5\par}\medskip}
\newenvironment{KRCHandoverItems}{%
  \renewcommand{\arraystretch}{1.55}%
  \arrayrulecolor{KRCRose}%
  \tabularx{\linewidth}{@{\hspace{2mm}}>{\raggedright\arraybackslash}p{34mm}%
    >{\raggedright\arraybackslash}X@{\hspace{2mm}}}%
  \hline
  \rowcolor{KRCBlush}
  {\color{KRCWine}\bfseries 確認項目} &
  {\color{KRCWine}\bfseries 確認結果・引継ぎ}\\\hline
}{\endtabularx}
\newcommand{\KRCHandoverItem}[2]{#1 & #2\\\hline}
