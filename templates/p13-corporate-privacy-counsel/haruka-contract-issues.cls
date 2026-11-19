\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{haruka-contract-issues}[2024/10/18 Contract issues note]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[top=30mm,bottom=25mm,left=28mm,right=28mm,headheight=20pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,lastpage}
\setmainjfont{Noto Serif CJK JP}
\setmainfont{Liberation Serif}
\definecolor{HCWine}{HTML}{713539}
\definecolor{HCSand}{HTML}{AE7849}
\definecolor{HCInk}{HTML}{342D29}
\color{HCInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\HarukaMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.6mm]
    \draw[HCWine,line width=.55mm] (2.5,4) -- (0,4) -- (0,0) -- (2.5,0);
    \draw[HCSand,line width=.55mm] (4.5,4) -- (7,4) -- (7,0) -- (4.5,0);
    \draw[HCWine,line width=.45mm] (2,2) -- (5,2);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\small\textcolor{HCWine}{はるか契約コンサルティング合同会社\enspace\HarukaMark}}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.35pt}
\fancyfoot[L]{\small\textcolor{HCWine}{契約検討メモ}}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}
\newcommand{\HarukaIssues}[5]{%
  {\small\raggedleft 管理番号 #2\par #3\par}
  \medskip
  {\LARGE\bfseries\color{HCWine}#1\par}
  {\small #5 様\quad 作成 #4\par}
  \medskip}
\newcommand{\HarukaSection}[1]{%
  \par\medskip{\large\bfseries\color{HCWine}\textcolor{HCSand}{\rule{1mm}{2.8mm}}\enspace#1\par}}
