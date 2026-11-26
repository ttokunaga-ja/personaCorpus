\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fuller-accessibility-observations}[2025/05/12 Fuller observation record]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[letterpaper,left=.78in,right=.78in,top=1.30in,bottom=.8in,headheight=32pt,headsep=16pt]{geometry}
\RequirePackage[match]{luatexja-fontspec}
\RequirePackage{xcolor,tikz,fancyhdr,tabularx,colortbl,booktabs}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{DejaVu Sans}
\setsansfont{DejaVu Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{FullerViolet}{HTML}{513E75}
\definecolor{FullerWash}{HTML}{EEE9F4}
\definecolor{FullerInk}{HTML}{282339}
\AtBeginDocument{\color{FullerInk}\raggedbottom}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\FullerMark}{%
  \begin{tikzpicture}[x=1pt,y=1pt,baseline=-2pt]
    \draw[FullerViolet,line width=1.8pt,line join=round,line cap=round]
      (0,7)--(5,7)--(8,13)--(12,1)--(16,10)--(20,7)--(25,7);
  \end{tikzpicture}\hspace{8pt}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\FullerMark\color{FullerViolet}\bfseries Fuller Accessibility Lab LLC}
\fancyhead[R]{\begingroup\setlength{\fboxsep}{6pt}%
  \colorbox{FullerWash}{\color{FullerViolet}\footnotesize FIELD OBSERVATION}\endgroup}
\fancyfoot[L]{\color{FullerViolet}\small Accessibility observations}
\fancyfoot[R]{\color{FullerViolet}\small\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{{\color{FullerWash}\hrule width\headwidth height 1pt}}
\newcommand{\FullerObservation}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\raggedright\color{FullerViolet}\fontsize{27}{32}\selectfont\bfseries #1\par}\vspace{6pt}%
  \begingroup\setlength{\fboxsep}{9pt}\noindent\colorbox{FullerWash}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\small
    \textbf{Record} #2\hfill\textbf{Issued} #3\hfill\textbf{Observer} #4}}\par\endgroup\vspace{8pt}}
\newcommand{\FullerHeading}[1]{\par\medskip{\color{FullerViolet}\large\bfseries #1\par}\nobreak\smallskip}
\newcommand{\FullerTableHeader}{\rowcolor{FullerWash}}
\NewDocumentEnvironment{FullerTable}{m +b}{%
  \par\noindent\begin{tabularx}{\linewidth}{#1}#2\end{tabularx}\par}{}
