\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{kenton-count-instructions}[2024/08/21 Kenton inventory instructions]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[letterpaper,left=.72in,right=.72in,top=1.3in,bottom=.85in,headheight=29pt,headsep=14pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{DejaVu Sans}
\setsansfont{DejaVu Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{KentonInk}{HTML}{202D3A}
\definecolor{KentonAmber}{HTML}{F4C85B}
\definecolor{KentonFog}{HTML}{EDF0F2}
\AtBeginDocument{\color{KentonInk}\raggedbottom}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\KentonMark}{\begin{tikzpicture}[x=1pt,y=1pt,baseline=-2pt]
  \fill[KentonAmber] (0,0) rectangle (22,18);
  \draw[KentonInk,line width=1.2pt] (4,4)--(4,13)--(18,13)--(18,4) (4,8)--(18,8) (11,8)--(11,13);
\end{tikzpicture}\hspace{7pt}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\KentonMark\bfseries Kenton Contract Warehousing Inc}
\fancyhead[R]{\small Inventory Control}
\fancyfoot[L]{\small Warehouse Operations}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrule}{{\color{KentonAmber}\hrule width\headwidth height 3pt\vskip-3pt}}
\newcommand{\KentonInstruction}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}%
  \begingroup\setlength{\fboxsep}{8pt}\noindent\colorbox{KentonFog}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\small #2\hfill #3\hfill #4}}\par\endgroup\vspace{8pt}}
\newcommand{\KentonHeading}[1]{\par\medskip{\textbf{\MakeUppercase{#1}}\par}\nobreak\smallskip}
