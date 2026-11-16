\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{chukyo-paper-check}[2026/07/01 Access approval paper inspection]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[left=22mm,right=22mm,top=30mm,bottom=23mm,headheight=27pt,headsep=9mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\definecolor{CDRust}{HTML}{8C3B2F}
\definecolor{CDInk}{HTML}{333C43}
\definecolor{CDSand}{HTML}{A99474}
\RequirePackage{tikz,longtable,array,fancyhdr,lastpage}
\color{CDInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\tabcolsep}{2.5pt}
\renewcommand{\arraystretch}{1.5}
\newcommand{\ChukyoMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=0mm]
\draw[CDRust,line width=.6mm] (0,0) rectangle (7,5.5);
\draw[CDRust,line width=.5mm] (0,5.5)--(3.5,2.8)--(7,5.5);
\fill[CDSand] (4.2,.2)--(7.6,.2)--(7.6,-.8)--(9.7,1)--(7.6,2.8)--(7.6,1.8)--(4.2,1.8)--cycle;
\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\ChukyoMark\quad\textcolor{CDRust}{\bfseries 中京文書配送サービス株式会社}}
\fancyfoot[L]{\small 中京文書配送サービス｜情報管理統制}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}
\renewcommand{\headrulewidth}{.8pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{CDSand}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\CheckHeading}[5]{%
{\LARGE\bfseries\color{CDRust}#1\par}
{\small 文書番号：#2\hfill 版：#3\par}
点検日：#4\hfill 記録者：#5\par\medskip}
\newcommand{\CheckSection}[1]{\par\medskip{\large\bfseries\color{CDRust}#1}\par}
\newcommand{\CheckNote}[1]{\par{\small #1}\par}
\newenvironment{ApprovalTable}{%
\begin{longtable}{|p{15mm}|p{35mm}|p{49mm}|p{26mm}|p{22mm}|}\hline
\rowcolor{CDRust}\color{white}証跡番号&\color{white}点検項目&\color{white}照合した記録&\color{white}結果&\color{white}確認者\\\hline\endhead
}{\end{longtable}}
\newcommand{\CheckSignatures}[2]{\par\medskip 点検者：#1\hfill 確認者：#2\par}
