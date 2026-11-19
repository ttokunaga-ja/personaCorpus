\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{tomorie-partner-brief}[2026/01/08 Partner briefing]
\LoadClass{tomorie-guide}

\fancyhead[R]{\small Support \& Success\\Partner briefing}
\fancyfoot[L]{\footnotesize Tomorie Service | Support \& Success}
\setlength{\tabcolsep}{6pt}

\newcommand{\TSPartnerBrief}[5]{%
  {\color{TSviolet}\LARGE\bfseries #1}\par\vspace{3mm}%
  {\small Document no.\ #2\quad Revision #3\par
  Issued #4\quad Prepared by #5}\par\vspace{3mm}%
}
\endinput
