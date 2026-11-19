\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{caldwell-comparison-note}[2024/04/01 Caldwell technical correspondence]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[margin=1.02in, top=1.17in]{geometry}
\RequirePackage{fontspec,xcolor,fancyhdr,tikz}
\setmainfont{Liberation Serif}
\definecolor{CalInk}{HTML}{18354B}
\definecolor{CalBlue}{HTML}{1B6473}
\definecolor{CalMist}{HTML}{E7F1F2}
\AtBeginDocument{\color{CalInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}
\setlength{\headheight}{28pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyhead[C]{\CaldwellMark\quad{\large\bfseries Caldwell Assay Services}}
\fancyfoot[C]{\small\itshape Technical correspondence \enspace / \enspace \thepage}
\newcommand{\CaldwellMark}{\tikz[baseline=-.3ex,x=.76em,y=.76em]{
  \draw[CalBlue,line width=1pt] (-.52,0)--(0,.65)--(.52,0)--(0,-.65)--cycle;
  \fill[CalBlue] (0,.08)--(.30,-.31)--(-.30,-.31)--cycle;}}
\newcommand{\CaldwellRecord}[4]{%
  \begin{center}
    {\color{CalBlue}\rule{.82\linewidth}{.55pt}}\par\medskip
    {\LARGE\bfseries #1}\par\smallskip
    \colorbox{CalMist}{\strut\enspace #2\quad /\quad #3\enspace}\par
  \end{center}
  \textbf{From}\enspace #4\par\medskip}
\newcommand{\CaldwellSection}[1]{\par\medskip{\large\bfseries\itshape\color{CalBlue}#1}\par}
