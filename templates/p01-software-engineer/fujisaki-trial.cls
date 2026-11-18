\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fujisaki-trial}[2025/12/01 Purchasing trial report]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[margin=24mm,top=30mm,bottom=24mm,headheight=26pt,headsep=12pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\definecolor{FujisakiTeal}{HTML}{17696D}
\definecolor{FujisakiInk}{HTML}{2E3535}
\definecolor{FujisakiTint}{HTML}{E9F3F1}
\RequirePackage{tikz,fancyhdr,lastpage,longtable,array,titlesec}
\newcommand{\FujisakiLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
\fill[FujisakiTeal] (0,0) rectangle (2,6);
\fill[FujisakiTeal] (2.8,4.4) rectangle (8,6);
\fill[FujisakiTeal] (2.8,2.2) rectangle (6.7,3.8);
\fill[FujisakiTeal] (2.8,0) rectangle (5.4,1.6);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\FujisakiLogo\quad\textcolor{FujisakiTeal}{\sffamily 藤崎医療資材株式会社}}
\fancyhead[R]{\small 購買部門 / 試行報告}
\fancyfoot[L]{\small 藤崎医療資材株式会社 / 購買部門}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{.4pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{FujisakiTeal}\leaders\hrule height \headrulewidth\hfill}}
\titleformat{\section}{\large\bfseries\color{FujisakiTeal}}{\thesection}{.7em}{}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\renewcommand{\arraystretch}{1.35}
\AtBeginDocument{\color{FujisakiInk}}
\newcommand{\TrialTitle}[4]{%
  {\LARGE\bfseries\color{FujisakiTeal}#1\par}\vspace{5pt}%
  \begingroup
  \setlength{\fboxsep}{6pt}%
  \noindent\colorbox{FujisakiTint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \small 文書番号\enspace #2\quad 発行日\enspace #3\par
    作成者\enspace #4}}\par
  \endgroup\vspace{6pt}}
\newcommand{\TrialNote}[1]{{\small\textcolor{FujisakiTeal}{#1}\par}}
\newcommand{\TrialHead}[1]{\rowcolor{FujisakiTeal}#1\\\hline}
\newcommand{\TrialCell}[1]{\color{white}\bfseries #1}
\endinput
