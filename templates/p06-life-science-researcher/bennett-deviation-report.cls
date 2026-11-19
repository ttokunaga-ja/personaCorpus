\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-deviation-report}[2025/07/01 Bennett quality deviation report]
\LoadClass{bennett-bench-record}
\fancyfoot[L]{\small QUALITY DEVIATION REPORT}
\newcommand{\BennettDeviationReport}[5]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}%
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Decision date}\enspace #3\par
    \textbf{Issued by}\enspace #4\hfill \textbf{Status}\enspace #5}}\par\vspace{5pt}}
