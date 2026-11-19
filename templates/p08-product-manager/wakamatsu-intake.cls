\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{wakamatsu-intake}[2024/04/01 Reception procedure]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[margin=23mm,top=30mm,bottom=24mm,headheight=23pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\RequirePackage[table]{xcolor}
\definecolor{WSKgreen}{HTML}{52644C}
\definecolor{WSKink}{HTML}{283C40}
\RequirePackage{tikz,fancyhdr,lastpage,longtable,array}
\newcommand{\WSKLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-1mm]
  \draw[WSKgreen,line width=.55mm] (0,2.3)--(2.7,5)--(5.4,2.3) (2.7,4.8)--(2.7,.5) (0,.5)--(5.4,.5);
\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\WSKLogo\quad\small\color{WSKgreen}若松設備管理株式会社}
\fancyhead[R]{\small\color{WSKgreen}依頼受付}
\fancyfoot[L]{\footnotesize\color{WSKgreen}管理部\enspace |\enspace 若松設備管理}
\fancyfoot[R]{\footnotesize\color{WSKgreen}\thepage\ / \pageref{LastPage}}
\renewcommand{\headrule}{{\color{WSKgreen}\hrule width\headwidth height .45pt}\vskip-.45pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\AtBeginDocument{\color{WSKink}}
\newcommand{\WSKRecord}[5]{%
  {\LARGE\bfseries\color{WSKgreen}#1\par}
  {\small 文書番号：#2\qquad 版：#3\par 発行日：#4\qquad 作成者：#5\par}\medskip}
\newcommand{\WSKStep}[1]{\par\medskip{\large\bfseries\color{WSKgreen}#1\par}\nobreak}
\newcommand{\WSKHead}[1]{\textcolor{white}{\bfseries #1}}
