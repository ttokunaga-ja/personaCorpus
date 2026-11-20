\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fuji-training}[2025/05/01 Dispatch training reports]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[left=24mm,right=24mm,top=30mm,bottom=22mm,headheight=18pt,headsep=8mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{booktabs,fancyhdr,longtable}
\definecolor{FujiViolet}{HTML}{564669}
\definecolor{FujiInk}{HTML}{312D37}
\definecolor{FujiTint}{HTML}{ECE7F1}
\color{FujiInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{FujiViolet}{\bfseries ふじ人材サポート合同会社}}
\fancyhead[R]{\small 教育担当}
\fancyfoot[L]{\small 派遣先連絡用}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{.6pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{FujiViolet}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\FujiHeading}[5]{%
  {\LARGE\bfseries\color{FujiViolet}#1\par}
  {\small #2\ 御中\par
  管理番号：#3\par
  発行日：#4\hfill 作成者：#5\par}\medskip}
\newcommand{\FujiSection}[1]{\par\medskip{\large\bfseries\color{FujiViolet}#1\par}}
\endinput
