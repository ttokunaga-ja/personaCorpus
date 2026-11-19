\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{katsuragawa-summary}[2024/04/26 Research summary]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=31mm,bottom=25mm,left=25mm,right=25mm,headheight=21pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\RequirePackage{xcolor,tikz,fancyhdr,hyperref}
\definecolor{KRCWine}{HTML}{742747}
\definecolor{KRCRose}{HTML}{AB586C}
\definecolor{KRCInk}{HTML}{302830}
\newcommand{\KRCMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.8mm]
  \draw[KRCWine,line width=.33mm] (2.4,2.4) circle (2.15);
  \fill[KRCWine] (2.4,2.4) circle (.57);
  \fill[KRCRose] (3.52,3.06) circle (.40);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\KRCMark\enspace\small\bfseries\color{KRCWine}かつら川臨床研究センター}
\fancyhead[R]{\small\color{KRCWine}研究概要}
\fancyfoot[L]{\small\color{KRCInk}研究事務局}
\fancyfoot[R]{\small\color{KRCInk}\thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{KRCRose}\leaders\hrule height .55pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\AtBeginDocument{\color{KRCInk}}
\newcommand{\KRCHeading}[6]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\LARGE\bfseries\color{KRCWine}#1\par}
  {\small 文書番号\quad #2\qquad 版\quad #3\par
   作成日\quad #4\qquad 作成者\quad #5\par
   確認者\quad #6\par}\medskip}
\newcommand{\KRCSection}[1]{\par\medskip{\large\bfseries\color{KRCWine}#1\par}\nobreak}
\newcommand{\KRCNote}[1]{\par{\small\color{KRCWine}#1\par}}
