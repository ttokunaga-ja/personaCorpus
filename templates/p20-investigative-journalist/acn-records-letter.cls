\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{acn-records-letter}[2024/04/01 Records correspondence]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,left=.83in,right=.83in,top=1.18in,bottom=.78in,headheight=21pt,headsep=14pt,footskip=25pt]{geometry}
\RequirePackage{fontspec,fancyhdr,array,longtable}
\RequirePackage[table]{xcolor}
\setmainfont{Liberation Serif}
\definecolor{DeskInk}{HTML}{232826}
\definecolor{DeskWine}{HTML}{792C3D}
\definecolor{DeskPaper}{HTML}{F6EFEB}
\AtBeginDocument{\color{DeskInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\newcommand{\DeskMark}{%
  \textcolor{DeskWine}{\raisebox{0pt}{\rule{3.5pt}{3.5pt}}\hspace{1.4pt}%
  \raisebox{2.75pt}{\rule{3.5pt}{3.5pt}}\hspace{1.4pt}%
  \raisebox{5.5pt}{\rule{3.5pt}{3.5pt}}}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{DeskWine}\large\bfseries\DeskMark\quad Allegheny Civic News Cooperative}
\renewcommand{\headrule}{\hbox to\headwidth{\color{DeskWine}\leaders\hrule height .75pt\hfill}}
\fancyfoot[L]{\footnotesize\color{DeskWine}Investigations desk}
\fancyfoot[R]{\footnotesize\color{DeskWine}\thepage}
\newcommand{\DeskRequest}[6]{%
  {\small\bfseries\color{DeskWine}PUBLIC RECORDS CORRESPONDENCE\par}
  {\LARGE\bfseries #1\par}
  \smallskip
  {\small\textcolor{DeskWine}{Document no.}\quad #2\quad
  \textcolor{DeskWine}{Date}\quad #3\par}
  {\small\textcolor{DeskWine}{From}\quad #4\par
  \textcolor{DeskWine}{To}\quad #5\par}
  \medskip #6\par}
\newcommand{\DeskHeading}[1]{\par\medskip{\large\bfseries\color{DeskWine}#1\par}\nobreak}
\newcommand{\DeskSign}[1]{%
  \par\medskip\noindent\parbox[t]{\linewidth}{\raggedright #1\par}\par}
\newenvironment{DeskSchedule}{%
  \renewcommand{\arraystretch}{1.08}%
  \setlength{\LTpost}{5pt}%
  \arrayrulecolor{DeskWine}%
  \begin{longtable}{@{}>{\raggedleft\arraybackslash}p{.35in}@{\hspace{.17in}}%
    >{\raggedright\arraybackslash}p{2.35in}%
    >{\raggedright\arraybackslash}p{3.6in}@{}}
  \hline\rowcolor{DeskPaper}\textcolor{DeskWine}{\bfseries No.}&\textcolor{DeskWine}{\bfseries Record or reference}&
  \textcolor{DeskWine}{\bfseries Scope and limits}\\\hline\endhead
}{\hline\end{longtable}}
% Use \\* on the final detail row to keep it with the signature row.
\newcommand{\DeskScheduleSign}[1]{%
  \multicolumn{3}{@{}l@{}}{%
    \parbox[t]{\dimexpr 6.47in+2\tabcolsep\relax}{%
      \vspace{6pt}\raggedright #1\par}}\\}
