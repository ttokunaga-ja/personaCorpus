\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{cedar-expense-guide}[Cedar Works expense guidance]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs,colortbl}

\setmainfont{Noto Sans CJK JP}
\setmainjfont{Noto Sans CJK JP}
\definecolor{cedargreen}{HTML}{264D3C}
\definecolor{cedarochre}{HTML}{AE8244}
\definecolor{cedarink}{HTML}{202924}
\AtBeginDocument{\color{cedarink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{cedargreen}\begin{tikzpicture}[baseline=0mm]
  \fill (0,0) rectangle (.09,.19);
  \fill (.13,0) rectangle (.22,.27);
  \fill (.26,0) rectangle (.35,.35);
\end{tikzpicture}\quad Cedar Works 株式会社}
\fancyhead[R]{\small 経理財務部｜経費精算}
\fancyfoot[L]{\small Cedar Works｜経理財務部}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{cedargreen}\leaders\hrule height .65pt\hfill}}

\newcommand{\ExpenseHeading}[5]{%
  {\LARGE\bfseries\color{cedargreen}#1\par}\medskip
  \begin{tabular}{@{}p{103mm}p{59mm}@{}}
    文書番号：#2 & 発行日：#3\\
    作成：#4 & 版：#5
  \end{tabular}\par\medskip
}
\newcommand{\ExpenseSection}[1]{\par\medskip{\large\bfseries\color{cedargreen}#1\par}}
\newcommand{\ExpenseNote}[1]{\par\noindent{\color{cedarochre}\rule{1.3mm}{2mm}}\hspace{2mm}{\small #1}\par}
\newenvironment{ExpenseTable}[1]
  {\setlength{\tabcolsep}{7pt}\renewcommand{\arraystretch}{1.5}\begin{longtable}{#1}\toprule}
  {\\\bottomrule\end{longtable}}
