\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{owada-return-record}[2025/08/04 Records storage return form]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[top=27mm,bottom=22mm,left=19mm,right=19mm,headheight=20pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,tikz,fancyhdr,lastpage}

\definecolor{OWGreen}{HTML}{284E3C}
\definecolor{OWPale}{HTML}{E5ECE5}
\definecolor{OWInk}{HTML}{282D2A}
\color{OWInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}

\newcommand{\OwadaMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-1mm]
    \draw[OWGreen,line width=.45mm] (0,3) rectangle (10,6);
    \draw[OWGreen,line width=.45mm] (0,0) rectangle (10,3);
    \draw[OWGreen,line width=.45mm] (4,4.5) -- (6,4.5) (4,1.5) -- (6,1.5);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\OwadaMark\enspace\textcolor{OWGreen}{\bfseries 大和田文書保管株式会社}}
\fancyhead[R]{\small 保管業務}
\renewcommand{\headrulewidth}{.55pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{OWGreen}\leaders\hrule height\headrulewidth\hfill}}
\fancyfoot[L]{\small 大和田文書保管株式会社｜返却記録}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}

% Title, reference, issue date, preparer, consignee
\newcommand{\OwadaReturnHeading}[5]{%
  \par\vspace*{4mm}%
  {\LARGE\bfseries\color{OWGreen}#1\par}
  {\small 発行番号：#2\quad 発行日：#3\par
  担当：#4\quad お渡し先：#5\par}\medskip}
\newcommand{\OwadaNote}[1]{\par\smallskip{\small #1}\par}

\newcolumntype{R}[1]{>{\raggedright\arraybackslash}p{#1}}
\newenvironment{OwadaReturns}{%
  \setlength{\tabcolsep}{4pt}%
  \renewcommand{\arraystretch}{1.4}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{13mm}R{27mm}R{31mm}R{52mm}R{34mm}}
  \hline\rowcolor{OWPale}\color{OWGreen}\bfseries No. &
  \color{OWGreen}\bfseries 保管箱番号 &
  \color{OWGreen}\bfseries 明細上の取扱い &
  \color{OWGreen}\bfseries 受渡し記録・照合資料 &
  \color{OWGreen}\bfseries 連絡事項 \\\hline\endhead
}{\end{longtable}}
