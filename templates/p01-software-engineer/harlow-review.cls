\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{harlow-review}[2026/05/01 Editorial review report]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[margin=24mm,top=30mm,bottom=25mm,headheight=30pt,headsep=12pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\definecolor{HarlowWine}{HTML}{743B51}
\definecolor{HarlowInk}{HTML}{343034}
\RequirePackage{tikz,fancyhdr,lastpage,longtable,array,titlesec}
\newcommand{\HarlowLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=0mm]
\draw[HarlowWine,line width=.6mm] (2,0)--(0,0)--(0,7)--(2,7);
\draw[HarlowWine,line width=.6mm] (8,0)--(10,0)--(10,7)--(8,7);
\draw[HarlowInk,line width=.3mm] (3,5)--(7,5) (3,3.5)--(7,3.5) (3,2)--(6,2);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\HarlowLogo\quad\small\textcolor{HarlowWine}{Harlow Editorial Services}}
\fancyhead[R]{\small Editorial review}
\fancyfoot[L]{\scriptsize Harlow Editorial Services / Language review}
\fancyfoot[R]{\scriptsize\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{.4pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{HarlowWine}\leaders\hrule height \headrulewidth\hfill}}
\titleformat{\section}{\large\bfseries\color{HarlowWine}}{\thesection}{.7em}{}
\titleformat{\subsection}{\normalsize\bfseries\color{HarlowWine}}{\thesubsection}{.7em}{}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\renewcommand{\arraystretch}{1.35}
\AtBeginDocument{\color{HarlowInk}}
\newcommand{\ReviewTitle}[5]{{\LARGE\bfseries\color{HarlowWine}#1\par}\vspace{5pt}{\small #2\quad #3\par Date: #4\quad Reviewer: #5\par}\vspace{6pt}}
\newcommand{\ReviewNote}[1]{{\small\textcolor{HarlowWine}{#1}\par}}
\newcommand{\ReviewHead}[1]{\rowcolor{HarlowWine}#1\\\hline}
\newcommand{\ReviewCell}[1]{\color{white}\bfseries #1}
\endinput
