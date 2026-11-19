\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{rosedale-meeting-agenda}[2024/08/22 Rosedale meeting agenda extract]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\RequirePackage[top=1.03in,bottom=.88in,left=.88in,right=.88in,headheight=30pt,headsep=15pt]{geometry}
\RequirePackage{xcolor,tikz,fancyhdr,hyperref}
\definecolor{RoseBrick}{HTML}{873F38}
\definecolor{RoseSlate}{HTML}{2D3439}
\definecolor{RoseLinen}{HTML}{F3ECE1}
\newcommand{\RoseLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.7mm]
    \fill[RoseBrick] (0,3.2)--(3.6,6.5)--(7.2,3.2)--(7.2,0)--(0,0)--cycle;
    \fill[RoseLinen] (2.9,0) rectangle (4.3,2.7);
  \end{tikzpicture}%
}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{RoseSlate}\RoseLogo\hspace{.65em}\small\bfseries Rosedale Workers Housing Cooperative}
\fancyfoot[L]{\small\color{RoseBrick}\bfseries Meeting papers}
\fancyfoot[R]{\small\color{RoseSlate}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{RoseBrick}\leaders\hrule height .55pt\hfill}}
\setlength{\parskip}{8pt}
\setlength{\parindent}{0pt}
\AtBeginDocument{\color{RoseSlate}}
\newcommand{\RoseAgenda}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\small\bfseries\color{RoseBrick} MEETING AGENDA EXTRACT\par}
  {\huge\bfseries #1\par}
  \noindent\colorbox{RoseLinen}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\small\strut #2\quad /\quad #3\quad /\quad #4\strut}}\par\medskip
}
\newcommand{\RoseItem}[2]{\par\medskip{\large\bfseries\color{RoseBrick} #1\par}\nobreak #2\par}
