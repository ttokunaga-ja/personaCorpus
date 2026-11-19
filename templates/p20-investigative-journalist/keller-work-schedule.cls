\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{keller-work-schedule}[2025/06/01 Keller work schedule]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,left=.78in,right=.78in,top=1.22in,bottom=.76in,headheight=26pt,headsep=16pt,footskip=24pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fontspec,fancyhdr,array,longtable,tabularx,tikz}
\setmainfont{Liberation Sans}
\definecolor{KellerAsphalt}{HTML}{252C30}
\definecolor{KellerOchre}{HTML}{AE741C}
\definecolor{KellerSand}{HTML}{F3EAD8}
\AtBeginDocument{\color{KellerAsphalt}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\newcommand{\KellerMark}{%
  \begin{tikzpicture}[baseline=-.6ex,x=1pt,y=1pt]
    \fill[KellerAsphalt] (0,-2)--(9,-2)--(17,12)--(8,12)--cycle;
    \draw[KellerOchre,line width=1pt] (5,0)--(7,3.5) (9,7)--(11,10.5);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\KellerMark\quad\large\bfseries KELLER MUNICIPAL PAVING INC}
\renewcommand{\headrule}{\hbox to\headwidth{\color{KellerOchre}\leaders\hrule height 1.2pt\hfill}}
\renewcommand{\footrulewidth}{.4pt}
\renewcommand{\footrule}{{\color{KellerOchre}\hrule width \headwidth height \footrulewidth}}
\fancyfoot[L]{\footnotesize Project records}
\fancyfoot[R]{\footnotesize Page \thepage}
\newcommand{\KellerIssue}[1]{\newline\mbox{#1}}
\newcommand{\KellerSchedule}[5]{%
  {\LARGE\bfseries #1\par}
  \smallskip
  {\renewcommand{\arraystretch}{1.27}%
  \begin{tabularx}{\linewidth}{@{}>{\bfseries}p{1in}>{\raggedright\arraybackslash}X>{\bfseries}p{.85in}>{\raggedright\arraybackslash}X@{}}
    Reference & #2 & Date & #3\\
    Location & #4 & Issued by & #5
  \end{tabularx}}\par\medskip}
\newenvironment{KellerItems}{%
  \renewcommand{\arraystretch}{1.16}%
  \setlength{\LTleft}{0pt}%
  \setlength{\LTright}{\fill}%
  \arrayrulecolor{KellerOchre}%
  \begin{longtable}{@{\hspace{.08in}}>{\raggedleft\arraybackslash}p{.39in}@{\hspace{.19in}}%
    >{\raggedright\arraybackslash}p{2.79in}%
    >{\raggedleft\arraybackslash}p{.84in}@{\hspace{.19in}}%
    >{\raggedright\arraybackslash}p{.61in}%
    >{\raggedright\arraybackslash}p{1.45in}@{}}
    \hline\rowcolor{KellerSand}\textbf{No.}&\textbf{Work item}&\textbf{Qty.}&
    \textbf{Unit}&\textbf{Record reference}\\\hline\endhead
}{\hline\end{longtable}}
