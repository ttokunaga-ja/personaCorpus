\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{carver-pilot-questionnaire}[2024/07/10 Carver interview worksheet]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[letterpaper,left=.86in,right=.86in,top=1.27in,bottom=.8in,headheight=27pt,headsep=14pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{CarverInk}{HTML}{252A35}
\definecolor{CarverBrick}{HTML}{B3463C}
\definecolor{CarverLinen}{HTML}{ECE5DA}
\AtBeginDocument{\color{CarverInk}\raggedbottom}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\CarverLogo}{\begin{tikzpicture}[x=1pt,y=1pt,baseline=0pt]
  \fill[CarverBrick] (0,10) rectangle (13,13);
  \fill[CarverBrick] (4,5) rectangle (17,8);
  \fill[CarverBrick] (8,0) rectangle (21,3);
\end{tikzpicture}\hspace{7pt}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\CarverLogo\bfseries Carver Workplace Research LLC}
\fancyhead[R]{\small Pilot Questionnaire}
\fancyfoot[L]{\small Research Practice}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrule}{\color{CarverBrick}\hrule width\headwidth height .8pt\vskip-.8pt}
\newcommand{\CarverDocument}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\fontsize{26}{31}\selectfont\bfseries #1\par}\vspace{6pt}%
  \begingroup\setlength{\fboxsep}{8pt}\noindent\colorbox{CarverLinen}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\small #2\hfill #3\hfill #4}}\par\endgroup\vspace{9pt}}
\newcommand{\CarverHeading}[1]{\par\medskip{\color{CarverBrick}\bfseries\large #1\par}\nobreak\smallskip}
\newcommand{\CarverQuestion}[3]{%
  \par\noindent\begin{minipage}{\linewidth}%
    \hangindent=2em\hangafter=1\makebox[2em][l]{\bfseries #1.}#2\par
    \if\relax\detokenize{#3}\relax\else\noindent\hspace*{2em}#3\par\fi
  \end{minipage}\par\vspace{11pt}}
