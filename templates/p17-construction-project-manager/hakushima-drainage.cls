\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hakushima-drainage}[2026/07/01 Drawing and design reply]
\LoadClass[a4paper,landscape,10pt]{ltjsarticle}
\RequirePackage[margin=15mm,headheight=18pt,headsep=6mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage,xcolor}
\definecolor{HDInk}{HTML}{303A43}
\definecolor{HDLine}{HTML}{637786}
\color{HDInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\textsf{\bfseries 白島設計室}\quad 設計図・質疑回答}
\fancyfoot[L]{白島設計室}
\fancyfoot[R]{\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{0.4pt}
\newcommand{\DrawingHeading}[6]{%
{\Large\bfseries #1\par}
\begin{tabular}{|p{95mm}|p{70mm}|p{80mm}|}\hline
工事名：#2&図番・版：#3&作成日：#4\\\hline
作成者：#5&\multicolumn{2}{l|}{状態・縮尺：#6}\\\hline
\end{tabular}\par\medskip}
\newcommand{\DrawingSection}[1]{\par\medskip{\large\bfseries #1}\par}
