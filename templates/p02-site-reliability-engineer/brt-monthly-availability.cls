\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-monthly-availability}[Monthly service availability report]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Reliability Engineering • Internal}

\newcommand{\BRTAvailabilityHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries Measure &
  \color{white}\bfseries Result &
  \color{white}\bfseries Evidence / treatment\\}
\newenvironment{BRTAvailability}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{42mm}>{\raggedleft\arraybackslash}p{27mm}>{\raggedright\arraybackslash}p{83mm}}
  \BRTAvailabilityHead\endfirsthead
  \BRTAvailabilityHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endfoot
}{%
  \end{longtable}}
