\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{nansei-requirement}[Nansei design requirement sheet]
\LoadClass[a4paper,10pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\RequirePackage[margin=23mm,top=30mm,bottom=24mm,headheight=19pt]{geometry}
\RequirePackage{xcolor,tikz,fancyhdr}
\definecolor{NSIndigo}{HTML}{303F73}
\definecolor{NSInk}{HTML}{292D38}
\AtBeginDocument{\color{NSInk}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{NSIndigo}\begin{tikzpicture}[baseline=-.5ex,x=1mm,y=1mm]
  \fill (0,0)--(2,0)--(4,4)--(2,4)--cycle;
  \fill (3,0)--(5,0)--(7,4)--(5,4)--cycle;
\end{tikzpicture}\enspace\bfseries 南星搬送機器株式会社}
\fancyhead[R]{\small 設計部}
\fancyfoot[L]{\small 南星搬送機器株式会社}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{.6pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\NanseiRequirementHeading}[6]{%
  {\raggedleft\small 文書番号\enspace #2\quad #3\par 発行日\enspace #4\par}%
  \medskip
  {\small #5\par}%
  \medskip
  {\LARGE\bfseries\color{NSIndigo}#1\par}%
  \smallskip
  {\small 発行者\enspace #6\par}%
  \medskip
}
\newcommand{\NanseiRequirementSection}[1]{%
  \par\medskip{\large\bfseries\color{NSIndigo}#1\par}%
}
