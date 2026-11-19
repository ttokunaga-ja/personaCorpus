\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{halstead-rfp-response}[2024/10/23 Halstead conditional proposal response]
\LoadClass{halstead-renewal}
\fancyhead[L]{\HBCorporateMark\\[3pt]{\footnotesize\color{HBTeal}\bfseries PROPOSAL RESPONSE}}
\let\HBResponse\HBRenewal

\newcommand{\HBResponseHead}{%
  \rowcolor{HBTeal}\HBHeadingCell{Requirement} &
  \HBHeadingCell{Disposition} & \HBHeadingCell{Qualification or response reference}\\}
\newenvironment{HBExceptions}{%
  \begin{longtable}{H{1.58in}H{1.32in}H{3.58in}}
    \HBResponseHead\endfirsthead
    \HBResponseHead\endhead
}{\end{longtable}}
\newcommand{\HBException}[3]{#1 & #2 & #3\\\arrayrulecolor{HBMist}\hline}
\endinput
