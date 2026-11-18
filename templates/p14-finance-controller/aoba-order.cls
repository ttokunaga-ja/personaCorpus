\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{aoba-order}[Aoba Stay purchasing documents]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=22mm,headheight=28pt,headsep=9mm]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs,colortbl}

\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\definecolor{aobaplum}{HTML}{53394F}
\definecolor{aobarose}{HTML}{A65149}
\definecolor{aobaink}{HTML}{322B30}
\AtBeginDocument{\color{aobaink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\small\color{aobaplum}購買・開業準備}
\fancyhead[R]{%
  \color{aobaplum}\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.4mm]
    \draw[line width=.55pt] (0,0) -- (0,3.1) arc (180:0:2.45) -- (4.9,0) -- cycle;
    \draw[line width=.45pt] (0,2.35) -- (4.9,2.35) (2.45,0) -- (2.45,5.55);
    \fill[aobarose] (3.66,3.25) circle (.43);
  \end{tikzpicture}\enspace{\large\bfseries Aoba Stay}}
\fancyfoot[L]{\small\color{aobaplum}Aoba Stay\enspace 購買}
\fancyfoot[R]{\small\color{aobaplum}\thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{aobarose}\leaders\hrule height .7pt\hfill}}
\renewcommand{\footrulewidth}{0pt}

\newcommand{\AobaOrderHeading}[5]{%
  {\small\color{aobaplum}#4\par}
  {\LARGE\bfseries\color{aobaplum}#1\par}\medskip
  \begin{tabular}{@{}p{91mm}p{65mm}@{}}
    照会番号\enspace #2 & 発行日\enspace #3\\
    発行担当\enspace #5 &
  \end{tabular}\par\medskip
}
\newcommand{\AobaSection}[1]{\par\medskip{\large\bfseries\color{aobaplum}#1\par}}
\newcommand{\AobaNote}[1]{\par\smallskip{\small\textcolor{aobarose}{連絡事項}\enspace #1\par}}
\newcommand{\AobaColumn}[1]{\textcolor{white}{\bfseries #1}}
\newenvironment{AobaOrderTable}[5]
  {\small\setlength{\tabcolsep}{6pt}\renewcommand{\arraystretch}{1.55}%
   \begin{longtable}{>{\raggedleft\arraybackslash}p{11mm}p{42mm}p{26mm}%
                     >{\raggedleft\arraybackslash}p{29mm}p{34mm}}%
   \rowcolor{aobaplum}\AobaColumn{#1}&\AobaColumn{#2}&\AobaColumn{#3}&\AobaColumn{#4}&\AobaColumn{#5}\\
   \midrule\endhead}
  {\\\bottomrule\end{longtable}}
