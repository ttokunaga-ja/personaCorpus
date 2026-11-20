\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-safety-record}[2026/02/01 Bennett laboratory safety records]
\LoadClass{bennett-bench-record}
\fancyfoot[L]{\small LABORATORY SAFETY RECORD}
\newenvironment{BennettWalkTable}{%
  \renewcommand{\arraystretch}{1.45}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{.34in}
    >{\raggedright\arraybackslash\setlength{\leftskip}{.10in}}p{1.38in}
    >{\raggedright\arraybackslash}p{1.05in}
    >{\raggedright\arraybackslash}p{3.15in}}
  \rowcolor{BCEIris}
  \color{white}\bfseries No. & \color{white}\bfseries Location / ID &
  \color{white}\bfseries Opened on & \color{white}\bfseries Finding / action \\
  \endhead
}{\arrayrulecolor{BCEIris}\hline\end{longtable}}
\newenvironment{BennettAttendanceTable}{%
  \renewcommand{\arraystretch}{2.10}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{.34in}
    >{\raggedright\arraybackslash\setlength{\leftskip}{.10in}}p{2.03in}
    >{\raggedright\arraybackslash}p{2.16in}
    >{\raggedright\arraybackslash}p{1.39in}}
  \rowcolor{BCEIris}
  \color{white}\bfseries No. & \color{white}\bfseries Attendee &
  \color{white}\bfseries Signature & \color{white}\bfseries Date \\
  \endhead
}{\arrayrulecolor{BCEIris}\hline\end{longtable}}
