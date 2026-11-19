\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{toto-review-sheet}[2026/06/15 Legal review sheet]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[top=29mm,bottom=23mm,left=23mm,right=23mm,headheight=27pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\definecolor{TDMain}{HTML}{563F58}
\definecolor{TDAccent}{HTML}{B28B65}
\definecolor{TDInk}{HTML}{302F33}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage}
\color{TDInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\tabcolsep}{3pt}
\renewcommand{\arraystretch}{1.5}
\newcommand{\TDLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=0mm]
\draw[TDMain,line width=.65mm] (0,1) rectangle (4.7,6);
\draw[TDAccent,line width=.65mm] (2.5,0) rectangle (7.2,5);
\draw[TDMain,line width=.5mm] (3.3,3) -- (4.3,2.2) -- (6.2,4.2);
\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\TDLogo\quad\textcolor{TDMain}{東都文書データサービス株式会社}}
\newcommand{\TDReviewMatter}{}
\newcommand{\ReviewMatter}[1]{\renewcommand{\TDReviewMatter}{｜#1}}
\fancyfoot[L]{\small 法務部\TDReviewMatter}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{.6pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{TDAccent}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\ReviewHeading}[5]{%
{\LARGE\bfseries\color{TDMain}#1\par}
{\small 文書番号：#2\par 版：#3\quad 作成日：#4\quad 作成者：#5\par}}
\newcommand{\ReviewSection}[1]{\par\medskip{\large\bfseries\color{TDMain}#1}\par}
\newcommand{\ReviewNote}[1]{\par{\small #1}\par}
\newcommand{\ReviewDecision}[2]{\par\textcolor{TDMain}{\bfseries 判断者：#1}\quad #2\par}
\newenvironment{ReviewTable}{%
\begin{longtable}{p{17mm}p{40mm}p{42mm}p{35mm}p{15mm}}
\hline\rowcolor{TDMain}\color{white}区分&\color{white}判断の前提&\color{white}根拠・変更点&\color{white}判断・期限&\color{white}担当\\\hline\endhead
}{\hline\end{longtable}}
