\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{yoshimura-statistical-note}[2024/07/18 Statistical correspondence]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=30mm,bottom=25mm,left=28mm,right=23mm,headheight=26pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage{xcolor,tikz,fancyhdr,hyperref}
\definecolor{YSTBlue}{HTML}{214D81}
\definecolor{YSTInk}{HTML}{222D3C}
\newcommand{\YSTMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.9mm]
  \draw[YSTBlue,line width=.40mm] (0,4.5)--(0,0)--(6,0);
  \fill[YSTBlue] (1.35,1.3) rectangle (2.30,2.25);
  \fill[YSTBlue] (3.25,2.0) rectangle (4.20,2.95);
  \fill[YSTBlue] (5.15,3.15) rectangle (6.10,4.10);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\small\color{YSTBlue}\YSTMark\enspace 吉村統計研究合同会社}
\fancyfoot[L]{\small\color{YSTInk}\thepage}
\fancyfoot[R]{\small\color{YSTBlue}YOSHIMURA STATISTICS}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{YSTBlue}\leaders\hrule height .55pt\hfill}\vskip 4pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}
\AtBeginDocument{\color{YSTInk}}
\newcommand{\YSTHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\small\color{YSTBlue}STATISTICAL CONSULTATION\par}
  {\LARGE\bfseries\color{YSTBlue}#1\par}
  \begin{tabular}{@{}ll@{}}
  Reference & #2\\
  Date & #3\\
  Prepared by & #4\\
  To & #5
  \end{tabular}\par\medskip}
\newcommand{\YSTSection}[1]{\par\medskip{\large\bfseries\color{YSTBlue}#1\par}\nobreak}
