\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{carver-venue-statement}[2026/01/30 Carver venue charge statement]
\LoadClass{carver-field-report}
\RequirePackage{array}
\fancyhead[R]{\small Venue Charge Statement}
\fancyfoot[L]{\small Research Operations}

\newcommand{\CarverVenueStatement}[4]{%
  \CarverReport{Venue charge statement}{#1}{#2}{#3}{#4}}

\newenvironment{CarverVenueLines}
  {\tabularx{\linewidth}{@{}l l X r@{}}
    \rowcolor{CarverInk}
    \textcolor{white}{\textbf{Line}} &
    \textcolor{white}{\textbf{Session date}} &
    \textcolor{white}{\textbf{Room and charge basis}} &
    \textcolor{white}{\textbf{USD}} \\}
  {\endtabularx}

\newcommand{\CarverVenueTotal}[1]{%
  \rowcolor{CarverLinen} \multicolumn{3}{@{}l}{\textbf{Statement total}} &
  \textbf{#1} \\}
