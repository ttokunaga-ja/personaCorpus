\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-handover-retraining}[Handover retraining record]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Reliability Engineering • Internal}
\renewenvironment{BRTEvents}{%
  \fontsize{10}{11}\selectfont
  \setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.2}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{35mm}>{\raggedright\arraybackslash}p{40mm}>{\raggedright\arraybackslash}p{78mm}}
  \BRTEventHead\endfirsthead
  \BRTEventHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endfoot
}{%
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
\renewcommand{\BRTEventHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries Stage & \color{white}\bfseries Check & \color{white}\bfseries Evidence or outcome\\}
\renewcommand{\BRTEventNote}[1]{%
  \multicolumn{3}{>{\raggedright\arraybackslash}p{\dimexpr153mm+4\tabcolsep\relax}}{\itshape #1}\\}
