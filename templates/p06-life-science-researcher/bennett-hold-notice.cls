\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-hold-notice}[2024/10/01 Bennett laboratory quality notice]
\LoadClass{bennett-bench-record}
\fancyfoot[L]{\small QUALITY NOTICE}
\newcommand{\BennettHoldNotice}[5]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Effective}\enspace #3\par
    \textbf{Issued by}\enspace #4\par
    \textbf{Distribution}\enspace #5}}\par\vspace{5pt}}
