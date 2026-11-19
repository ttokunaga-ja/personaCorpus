\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-operating-decision}[2025/10/16 Bennett operating decision]
\LoadClass{bennett-bench-record}
\fancyfoot[L]{\small LABORATORY OPERATING DECISION}
\newcommand{\BennettOperatingDecision}[5]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}%
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Decision date}\enspace #3\par
    \textbf{Prepared by}\enspace #4\hfill \textbf{Approved by}\enspace #5}}\par\vspace{5pt}}
