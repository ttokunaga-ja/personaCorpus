\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-weekend-duty-sheet}[Weekend duty distribution sheet]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Reliability Engineering \textbullet{} Internal}

\newcommand{\BRTRotaTime}[2]{#1\newline #2}
\newcommand{\BRTRotaHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries No. &
  \color{white}\bfseries Starts &
  \color{white}\bfseries Ends &
  \color{white}\bfseries On duty &
  \color{white}\bfseries Agreed relief &
  \color{white}\bfseries Handover note\\}
\newenvironment{BRTRotaWindows}{%
  \small\setlength{\tabcolsep}{4pt}\renewcommand{\arraystretch}{1.25}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{8mm}@{\hspace{5mm}}%
    >{\raggedright\arraybackslash}p{24mm}%
    >{\raggedright\arraybackslash}p{24mm}%
    >{\raggedright\arraybackslash}p{26mm}%
    >{\raggedright\arraybackslash}p{26mm}%
    >{\raggedright\arraybackslash}p{35mm}}
  \BRTRotaHead\endfirsthead
  \BRTRotaHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endfoot
}{%
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
