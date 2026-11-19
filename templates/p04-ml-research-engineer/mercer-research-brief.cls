\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{mercer-research-brief}[2024/04/15 Mercer research brief]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,top=1.15in,bottom=.87in,hmargin=.86in,headheight=18pt,headsep=23pt]{geometry}
\RequirePackage{fontspec}
\RequirePackage{luatexja-fontspec}
% Keep English smart quotes in the Latin font and line-breaking rules.
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr,titlesec,array,longtable,booktabs}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{MercerInk}{HTML}{242331}
\definecolor{MercerPlum}{HTML}{593875}
\definecolor{MercerOchre}{HTML}{A56720}
\definecolor{MercerTint}{HTML}{EFE9F4}
\newcommand{\MercerMark}{\makebox[16pt][l]{%
  \textcolor{MercerPlum}{\rule{4pt}{4pt}}\hspace{1pt}%
  \raisebox{3pt}{\textcolor{MercerPlum}{\rule{4pt}{4pt}}}\hspace{1pt}%
  \raisebox{6pt}{\textcolor{MercerOchre}{\rule{4pt}{4pt}}}}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{MercerPlum}\small\bfseries\MercerMark\hspace{.5em}Mercer Document Research Inc}
\fancyhead[R]{\color{MercerPlum}\small\bfseries SACRAMENTO / RESEARCH}
\fancyfoot[L]{\color{MercerPlum}\small Document Classification Research}
\fancyfoot[R]{\color{MercerPlum}\small\thepage}
\renewcommand{\headrule}{\color{MercerPlum}\hrule width\headwidth height .55pt
  \vskip1.25pt\color{MercerOchre}\hrule width\headwidth height .35pt\vskip-2.15pt}
\titleformat{\section}{\color{MercerPlum}\fontsize{15}{18}\selectfont\bfseries}{}{0pt}{}
\titleformat{\subsection}{\color{MercerPlum}\fontsize{12}{15}\selectfont\bfseries}{}{0pt}{}
\titlespacing*{\section}{0pt}{14pt}{5pt}
\titlespacing*{\subsection}{0pt}{9pt}{4pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\AtBeginDocument{\color{MercerInk}}
\newcommand{\MercerBrief}[4]{%
  {\raggedright\color{MercerPlum}\hyphenpenalty=10000\exhyphenpenalty=10000\fontsize{27}{31}\selectfont\bfseries #1\par}%
  \vspace{3pt}%
  {\small\textbf{Document ID}\enspace #2\quad
     \textbf{Issued}\enspace #3\quad\textbf{Author}\enspace #4\par}%
  \vspace{5pt}}
\newcommand{\MercerTableHead}[1]{\rowcolor{MercerTint}#1}
\newcolumntype{L}[1]{>{\color{MercerInk}\raggedright\arraybackslash}p{#1}}
\newcolumntype{R}[1]{>{\color{MercerInk}\raggedleft\arraybackslash}p{#1}}
\setlength{\tabcolsep}{7pt}
\renewcommand{\arraystretch}{1.25}
\arrayrulecolor{MercerPlum}
\AddToHook{env/longtable/after}{\color{MercerInk}}
