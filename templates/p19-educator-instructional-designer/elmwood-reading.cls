\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{elmwood-reading}[2024/04/18 Elmwood classroom handout]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Lato}
\setmainjfont{Noto Sans CJK JP}
\RequirePackage[top=1.03in,bottom=.77in,left=.82in,right=.82in,headheight=24pt,headsep=16pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,fancyhdr,hyperref}
\definecolor{ElmIndigo}{HTML}{393967}
\definecolor{ElmGold}{HTML}{A87323}
\definecolor{ElmInk}{HTML}{29283A}
\colorlet{ElmPaper}{ElmGold!7!white}
\newcommand{\ElmLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=0pt]
    \fill[ElmIndigo] (0,0) rectangle (1.65,3.15);
    \fill[ElmIndigo] (2.35,0) rectangle (4,4.35);
    \fill[ElmGold] (4.70,0) rectangle (6.35,5.55);
  \end{tikzpicture}%
}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{ElmIndigo}\ElmLogo\hspace{.8em}\small\bfseries Elmwood Adult Learning Cooperative}
\fancyfoot[L]{\small\color{ElmInk}Adult learning / Course handout}
\fancyfoot[R]{\small\color{ElmInk}Page \thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{ElmGold}\leaders\hrule height .7pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\AtBeginDocument{\color{ElmInk}}
\newcommand{\ElmHandout}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\color{ElmIndigo}\Huge\bfseries #1\par}%
  {\large #2\par}%
  {\small #3\quad |\quad #4\quad |\quad #5\par}%
  \medskip
}
\newcommand{\ElmHeading}[1]{\par\medskip{\large\bfseries\color{ElmIndigo}#1}\par\nobreak}
\newenvironment{ElmExtract}
  {\par\begingroup\setlength{\fboxsep}{10pt}\noindent\begin{lrbox}{\ElmExtractBox}%
   \begin{minipage}{\dimexpr\linewidth-2\fboxsep\relax}\color{ElmInk}}
  {\end{minipage}\end{lrbox}\colorbox{ElmPaper}{\usebox{\ElmExtractBox}}\par\endgroup}
\newsavebox{\ElmExtractBox}
\newcommand{\ElmSource}[1]{{\small\itshape #1\par}}
