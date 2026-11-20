\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{carver-field-report}[2024/09/26 Carver field research report]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[letterpaper,left=.86in,right=.86in,top=1.27in,bottom=.88in,headheight=27pt,headsep=14pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,tabularx,colortbl}
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
\renewcommand{\arraystretch}{1.25}
\newcommand{\CarverLogo}{\begin{tikzpicture}[x=1pt,y=1pt,baseline=0pt]
  \fill[CarverBrick] (0,10) rectangle (13,13);
  \fill[CarverBrick] (4,5) rectangle (17,8);
  \fill[CarverBrick] (8,0) rectangle (21,3);
\end{tikzpicture}\hspace{7pt}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\CarverLogo\bfseries Carver Workplace Research LLC}
\fancyhead[R]{\small Field Research Report}
\fancyfoot[L]{\small Research Practice}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrule}{{\color{CarverBrick}\hrule width\headwidth height .8pt\vskip-.8pt}}
\newcommand{\CarverReport}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\raggedright\fontsize{26}{31}\selectfont\bfseries #1\par}\vspace{6pt}%
  \begingroup\setlength{\fboxsep}{8pt}\noindent\colorbox{CarverLinen}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      \small #2\hfill #3\hfill #4\par
      \textbf{Prepared for}\enspace #5}}\par\endgroup\vspace{9pt}}
\newcommand{\CarverHeading}[1]{\par\medskip{\color{CarverBrick}\bfseries\large #1\par}\nobreak\smallskip}
\newcommand{\CarverEvidenceHeader}[2]{%
  \rowcolor{CarverInk}\textcolor{white}{\bfseries #1}&\textcolor{white}{\bfseries #2}\\}
