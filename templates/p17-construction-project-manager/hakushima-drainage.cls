\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hakushima-drainage}[2026/07/01 Drawing and design reply]
\LoadClass[a4paper,landscape,10pt]{ltjsarticle}
\RequirePackage[margin=15mm,headheight=18pt,headsep=6mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage}
\definecolor{HDInk}{HTML}{303A43}
\definecolor{HDLine}{HTML}{637786}
\AtBeginDocument{\color{HDInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\newcommand{\DrawingLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-0.6mm]
    \draw[HDLine,line width=.65pt] (0,0)--(0,4.8)--(5.8,0)--cycle;
    \draw[HDLine,line width=.5pt] (1.5,0)--(1.5,1.5)--(0,1.5);
  \end{tikzpicture}%
}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\DrawingLogo\enspace\textsf{\bfseries 白島設計室}\quad 設計図・質疑回答}
\fancyfoot[L]{白島設計室}
\fancyfoot[R]{\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{0.4pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{HDLine}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\DrawingHeading}[6]{%
{\Large\bfseries #1\par}
\begingroup
\arrayrulecolor{HDLine}%
\begin{tabular}{|p{95mm}|p{70mm}|p{80mm}|}\hline
工事名：#2&図番・版：#3&作成日：#4\\\hline
作成者：#5&\multicolumn{2}{l|}{状態・縮尺：#6}\\\hline
\end{tabular}%
\endgroup\par\medskip}
\newcommand{\DrawingSection}[1]{\par\medskip{\large\bfseries #1}\par}
