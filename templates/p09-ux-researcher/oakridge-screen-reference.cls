\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{oakridge-screen-reference}[2024/04/16 Oakridge dispatch screen reference]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[letterpaper,left=.82in,right=.82in,top=1.31in,bottom=.82in,headheight=31pt,headsep=17pt]{geometry}
\RequirePackage[match]{luatexja-fontspec}
\RequirePackage{xcolor,tikz,fancyhdr,graphicx,booktabs,tabularx,colortbl}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{OakInk}{HTML}{183D52}
\definecolor{OakIce}{HTML}{D9E9EE}
\AtBeginDocument{\color{OakInk}\raggedbottom}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\OakLogo}{\begin{tikzpicture}[x=1pt,y=1pt,baseline=-1pt]
  \fill[OakIce] (0,0) rectangle (20,17);
  \draw[OakInk,line width=.9pt] (3,4)--(9,12)--(17,12) (9,12)--(16,5);
  \fill[OakInk] (3,4) circle (1.4) (17,12) circle (1.4) (16,5) circle (1.4);
\end{tikzpicture}\hspace{7pt}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\OakLogo\sffamily\bfseries Oakridge Dispatch Cooperative}
\fancyhead[R]{\sffamily\small Dispatch Systems}
\fancyfoot[L]{\sffamily\small Screen Reference}
\fancyfoot[R]{\sffamily\small\thepage}
\renewcommand{\headrule}{{\color{OakInk}\hrule width\headwidth height 1pt}\vskip-1pt}
\renewcommand{\footrule}{{\color{OakIce}\hrule width\headwidth height 1pt}}
\newcommand{\OakDocument}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\fontsize{30}{34}\selectfont\bfseries #1\par}\vspace{6pt}%
  \begingroup\setlength{\fboxsep}{9pt}\noindent\colorbox{OakIce}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\sffamily\small #2\hfill #3\hfill #4}}\par\endgroup\vspace{8pt}}
\newcommand{\OakHeading}[1]{\par\medskip{\sffamily\bfseries\large #1\par}\nobreak\smallskip}
\newcommand{\OakTableHeader}{\rowcolor{OakIce}}
