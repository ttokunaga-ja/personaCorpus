\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{olta-extract}[2026/07/15 Usage extract]
\LoadClass[a4paper,10pt]{article}
\RequirePackage[margin=23mm,top=29mm,bottom=24mm,headheight=22pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\definecolor{OFteal}{HTML}{006D77}
\definecolor{OFamber}{HTML}{D69E36}
\definecolor{OFink}{HTML}{283C40}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage}
\newcommand{\OFLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-1mm]\draw[OFteal,line width=0.8mm] (0,4)--(0,0)--(4,0)--(4,3.5);\fill[OFamber] (2.9,3.3)--(4,5)--(5.1,3.3)--cycle;\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\OFLogo\quad\small\color{OFteal}オルタ現場システムズ株式会社}
\fancyhead[R]{\small\color{OFteal}利用状況の出力}
\fancyfoot[L]{\scriptsize\color{OFink}プロダクト企画 | オルタ現場システムズ}
\fancyfoot[R]{\small\color{OFteal}\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{0.4pt}
\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\AtBeginDocument{\color{OFink}\sffamily}
\newcommand{\OFRecord}[6][]{%
  {\LARGE\bfseries\color{OFteal}#2\par}
  {\small 文書番号：#3\quad
    \if\relax\detokenize{#1}\relax\else 版：#1\quad\fi
    作成者：#6\par 基準日：#4\quad 出力日時：#5\par}\medskip}
\newcommand{\OFMeetingRecord}[6]{%
  \fancyhead[R]{\small\color{OFteal}経営会議資料}%
  {\LARGE\bfseries\color{OFteal}#1\par}
  {\small 文書番号：#2\quad 版：#3\quad 作成者：#5\par
    発行日：#4\quad 対象期間：#6\par}\medskip}
\newcommand{\OFSection}[1]{\par\medskip{\large\bfseries\color{OFteal}#1}\par}
\newcommand{\OFHead}[1]{\textcolor{white}{\bfseries #1}}
