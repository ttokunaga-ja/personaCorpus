\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-handover-retraining}[Handover retraining record]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Reliability Engineering • Internal}
\renewenvironment{BRTEvents}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{35mm}>{\raggedright\arraybackslash}p{40mm}>{\raggedright\arraybackslash}p{78mm}}
  \BRTEventHead\endfirsthead
  \BRTEventHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endlastfoot
}{\end{longtable}}
\renewcommand{\BRTEventHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries Stage & \color{white}\bfseries Check & \color{white}\bfseries Evidence or outcome\\}
