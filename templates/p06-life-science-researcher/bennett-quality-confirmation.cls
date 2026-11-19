\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-quality-confirmation}[2026/07/21 Bennett quality confirmation]
\LoadClass{bennett-bench-record}
\fancyfoot[L]{\small QUALITY CONFIRMATION}
\newcommand{\BennettQualityConfirmation}[5]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}%
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Review date}\enspace #3\par
    \textbf{Reviewed by}\enspace #4\hfill \textbf{Run}\enspace #5}}\par\vspace{5pt}}
