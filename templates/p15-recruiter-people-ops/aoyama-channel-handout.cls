\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{aoyama-channel-handout}[2024/10/25 Recruiting channel handouts]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[left=25mm,right=25mm,top=31mm,bottom=25mm,headheight=28pt]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,tikz,fancyhdr,longtable,array,colortbl,hyperref}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\definecolor{AYblue}{HTML}{486477}
\definecolor{AYink}{HTML}{34383C}
\definecolor{AYpale}{HTML}{EAF0F3}
\AtBeginDocument{\color{AYink}}
\newcommand{\AYLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
  \draw[AYblue,line width=.9pt] (0,0)--(3.4,3.2)--(6.8,0);
  \draw[AYblue,line width=.65pt] (1.35,0)--(3.4,1.9)--(5.45,0);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\color{AYblue}\AYLogo\enspace\small 青山キャリア支援合同会社}
\fancyfoot[L]{\color{AYblue}\footnotesize 青山キャリア支援合同会社　採用支援}
\fancyfoot[R]{\color{AYblue}\footnotesize\thepage}
\renewcommand{\headrulewidth}{.5pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{AYblue}\leaders\hrule height \headrulewidth\hfill}}

\newcommand{\AYHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#5}}%
  {\centering\LARGE\bfseries\color{AYblue}#1\par}\medskip
  {\small\noindent 資料番号：#2\quad 版：#3\par
   \noindent 発行日：#4\quad 作成者：#5\par}\medskip}
\newcommand{\AYSection}[1]{\par\bigskip\noindent{\large\bfseries\color{AYblue}#1}\par\smallskip}
\newcommand{\AYNote}[1]{\par\smallskip{\footnotesize\noindent #1\par}}
\newenvironment{AYTable}[1]{%
  \renewcommand{\arraystretch}{1.32}\begin{longtable}{#1}%
}{\end{longtable}}
\newcommand{\AYHeader}{\rowcolor{AYpale}}
