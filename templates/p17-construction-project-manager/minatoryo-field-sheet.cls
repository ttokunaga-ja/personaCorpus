\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{minatoryo-field-sheet}[2025/09/08 Site inspection sheet]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[margin=21mm,top=29mm,bottom=22mm,headheight=26pt,headsep=9mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\definecolor{MBNavy}{HTML}{263D4A}
\definecolor{MBOrange}{HTML}{CE702B}
\definecolor{MBInk}{HTML}{30383C}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage}
\color{MBInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\tabcolsep}{2.5pt}
\renewcommand{\arraystretch}{1.5}
\newcommand{\SiteLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=0mm]
\fill[MBNavy] (1.3,0) rectangle (3.1,5.1);
\fill[MBNavy] (10.5,0) rectangle (12.3,5.1);
\fill[MBNavy] (0,5) rectangle (13.6,6.8);
\fill[MBOrange] (0,7.1) rectangle (13.6,7.8);
\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\SiteLogo\quad\textcolor{MBNavy}{\bfseries 湊梁建設株式会社}}
\fancyfoot[L]{\small 湊梁建設株式会社｜工事部}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{1pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{MBOrange}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\SiteHeading}[5]{%
{\LARGE\bfseries\color{MBNavy}#1\par}
{\small 文書番号：#2\hfill 実施日：#4\par}
工事名：#3\par 記録者：#5\par\medskip}
\newcommand{\SiteSection}[1]{\par\medskip{\large\bfseries\color{MBNavy}#1}\par}
\newcommand{\SiteNote}[1]{\par{\small #1}\par}
\newenvironment{FieldTable}{%
\begin{longtable}{|p{8mm}|p{26mm}|p{48mm}|p{48mm}|p{19mm}|}\hline
\rowcolor{MBNavy}\color{white}番号&\color{white}位置（通り芯）&\color{white}点検・指摘内容&\color{white}対応・確認根拠&\color{white}確認者\\\hline\endhead
}{\end{longtable}}
