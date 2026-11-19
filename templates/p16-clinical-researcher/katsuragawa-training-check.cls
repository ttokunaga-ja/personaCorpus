\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-training-check}[2025/08/25 Training debrief form]
\LoadClass{katsuragawa-submission-check}
\definecolor{KRCPaleRose}{HTML}{F7EDF1}
\fancyhead[R]{\small\color{KRCWine}手続研修 確認票}
\fancyfoot[L]{\small\color{KRCInk}研究事務局・教育担当}
\newcommand{\KRCTrainingField}[2]{%
  \par\medskip\noindent\begin{minipage}{\linewidth}
    \setlength{\fboxsep}{0pt}%
    \colorbox{KRCPaleRose}{\parbox{\linewidth}{%
      \strut\hspace{2mm}\textcolor{KRCWine}{\bfseries #1}\strut}}\par
    \vspace{2mm}#2\par
    \vspace{8mm}{\color{KRCRose}\hrule height .45pt}%
    \vspace{9mm}{\color{KRCRose}\hrule height .45pt}%
  \end{minipage}\par}
