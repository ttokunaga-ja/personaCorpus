\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{holdings-budget}[Cedar Works Holdings planning instructions]
\LoadClass[11pt,a4paper]{ltjsarticle}
\RequirePackage[top=26mm,bottom=23mm,left=24mm,right=24mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs}

\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\definecolor{holdingsnavy}{HTML}{233C60}
\definecolor{holdingsblue}{HTML}{5287B4}
\AtBeginDocument{\color{holdingsnavy}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{holdingsnavy}\begin{tikzpicture}[baseline=-.5mm]
  \fill (0,.15) -- (.15,.3) -- (.3,.15) -- (.15,0) -- cycle;
  \fill[holdingsblue] (.36,.11) rectangle (.6,.19);
\end{tikzpicture}\enspace\bfseries Cedar Works Holdings}
\fancyhead[R]{\small\bfseries\color{holdingsnavy}GROUP FINANCE}
\fancyfoot[L]{\small\color{holdingsnavy}FINANCE \textbullet\ INTERNAL}
\fancyfoot[R]{\small\color{holdingsnavy}\thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{holdingsblue}\leaders\hrule height 1.4pt\hfill}}

\newcommand{\DirectiveHeading}[4]{%
  \vspace*{3mm}{\LARGE\bfseries #1\par}\medskip
  \begin{tabular}{@{}p{87mm}p{68mm}@{}}
    Reference: #2 & Issued: #3\\
    Prepared by: #4 &
  \end{tabular}\par\medskip
}
\newcommand{\DirectiveSection}[1]{\par\medskip{\large\bfseries #1\par}}
\newcommand{\DirectiveNote}[1]{\par\noindent{\color{holdingsblue}\rule{1.3mm}{2mm}}\hspace{2mm}{\small #1}\par}
\newenvironment{DirectiveTable}[1]
  {\renewcommand{\arraystretch}{1.45}\begin{longtable}{#1}\toprule}
  {\\\bottomrule\end{longtable}}
