\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-travel-close}[BRT Finance travel expense close]
\LoadClass{brt-customer-update}
\fancyfoot[L]{\color{BRTInk}\footnotesize Finance \textbullet{} Internal}
\newcommand{\BRTTravelClose}[6]{%
  \BRTUpdate{#1}{#2}{#3}{#4}%
  \textbf{Claimant} #5\hfill\textbf{Claim} #6\par}
\newenvironment{BRTTravelAmounts}{%
  \renewcommand{\arraystretch}{1.35}%
  \begin{longtable}{@{}>{\raggedright\arraybackslash}p{43mm}>{\raggedright\arraybackslash}p{76mm}>{\raggedleft\arraybackslash}p{29mm}@{}}%
  \rowcolor{BRTBlue}\color{white}\bfseries Line or stage &
  \color{white}\bfseries Treatment & \color{white}\bfseries GBP\\
  \endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endfoot
}{\end{longtable}}
