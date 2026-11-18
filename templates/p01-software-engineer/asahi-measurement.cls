\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{asahi-measurement}[2026/06/15 Measurement brief]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[margin=22mm,top=29mm,bottom=23mm,headheight=35pt,headsep=10pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\definecolor{AsahiBlue}{HTML}{2548A6}
\definecolor{AsahiGold}{HTML}{D8B83C}
\definecolor{AsahiInk}{HTML}{29313F}
\definecolor{AsahiPale}{HTML}{EEF1FA}
\RequirePackage{tikz,fancyhdr,lastpage,longtable,array,titlesec}
\newcommand{\AsahiLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=0mm]
\fill[AsahiGold] (2,1) arc[start angle=180,end angle=0,radius=3mm] -- cycle;
\fill[AsahiBlue] (0,0) rectangle (10,.5);
\fill[AsahiBlue] (1.5,-1.1) rectangle (8.5,-.7);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\newcommand{\AsahiReportKind}{性能測定・評価記録}
\fancyhead[L]{\AsahiLogo\quad\small\color{AsahiBlue}あさひ業務システムズ株式会社\\\scriptsize \AsahiReportKind}
\fancyfoot[L]{\scriptsize あさひ業務システムズ株式会社 / 開発部}
\fancyfoot[R]{\scriptsize\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{.7pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{AsahiBlue}\leaders\hrule height \headrulewidth\hfill}}
\titleformat{\section}{\large\bfseries\color{AsahiBlue}}{\thesection}{.7em}{}
\titleformat{\subsection}{\normalsize\bfseries\color{AsahiBlue}}{\thesubsection}{.7em}{}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\renewcommand{\arraystretch}{1.3}
\AtBeginDocument{\color{AsahiInk}}
\newcommand{\ReportTitle}[4]{{\LARGE\bfseries\color{AsahiBlue}#1\par}\vspace{5pt}{\small #2\quad #3\quad #4\par}\vspace{6pt}}
\newcommand{\ReportNote}[1]{{\small #1\par}}
\newcommand{\ReportHead}[1]{\rowcolor{AsahiBlue}#1\\\hline}
\newcommand{\HeadCell}[1]{\color{white}\bfseries #1}
\endinput
