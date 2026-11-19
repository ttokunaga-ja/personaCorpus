\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-recovery-exercise}[Service recovery exercise report]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Reliability Engineering • Internal}

\newenvironment{BRTExerciseIntervals}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{42mm}>{\raggedleft\arraybackslash}p{23mm}@{\hspace{6mm}}>{\raggedright\arraybackslash}p{84mm}}
  \BRTExerciseIntervalHead\endfirsthead
  \BRTExerciseIntervalHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endfoot
}{%
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
\newcommand{\BRTExerciseIntervalHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries Interval & \color{white}\bfseries Minutes & \color{white}\bfseries Observation\\}
\newcommand{\BRTExerciseNote}[1]{%
  \multicolumn{3}{>{\raggedright\arraybackslash}p{\dimexpr155mm+2\tabcolsep\relax}}{\itshape #1}\\}
