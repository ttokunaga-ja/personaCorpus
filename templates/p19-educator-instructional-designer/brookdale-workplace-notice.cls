\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brookdale-workplace-notice}[2024/04/18 Brookdale operations notice]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\RequirePackage[top=1.12in,bottom=.84in,left=.94in,right=.94in,headheight=25pt,headsep=16pt]{geometry}
\RequirePackage{xcolor,tikz,fancyhdr,hyperref}
\definecolor{BrookGreen}{HTML}{245747}
\definecolor{BrookInk}{HTML}{28322E}
\newcommand{\BrookLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
    \draw[BrookGreen,line width=.8pt] (3.2,3.2) circle (3mm);
    \draw[BrookGreen,line width=.8pt] (1.4,2.8)--(3.2,4.5)--(5,2.8);
    \draw[BrookGreen,line width=.8pt] (1.4,1.8)--(3.2,3.5)--(5,1.8);
  \end{tikzpicture}%
}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\color{BrookGreen}\BrookLogo\hspace{.7em}\bfseries Brookdale Service Cooperative}
\fancyfoot[L]{\small\itshape\color{BrookGreen}Service operations}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.4pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}
\AtBeginDocument{\color{BrookInk}}
\newcommand{\BrookNotice}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \begin{center}
    {\small\scshape\color{BrookGreen}Operations notice\par}
    \smallskip
    {\LARGE\bfseries #1\par}
    \smallskip
    {\small #2\quad /\quad #3\quad /\quad #4\par}
  \end{center}
  \medskip
}
\newcommand{\BrookClause}[2]{\par\medskip{\bfseries\color{BrookGreen}#1\par}\nobreak#2\par}
