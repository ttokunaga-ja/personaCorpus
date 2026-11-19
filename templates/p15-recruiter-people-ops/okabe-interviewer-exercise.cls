\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{okabe-interviewer-exercise}[2025/08/18 Interviewer training materials]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[left=23mm,right=23mm,top=37mm,bottom=23mm,headheight=30pt,headsep=8mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,tikz,fancyhdr,longtable,array,colortbl}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{OKolive}{HTML}{53632A}
\definecolor{OKink}{HTML}{272F25}
\definecolor{OKpale}{HTML}{EDF0E4}
\AtBeginDocument{\color{OKink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\setlength{\tabcolsep}{8pt}

\newcommand{\OKLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.9mm]
    \draw[OKolive,line width=.55pt] (0,0) rectangle (6,6);
    \draw[OKolive,line width=1pt] (1.1,3.1)--(2.5,1.6)--(4.9,4.7);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\OKLogo\enspace\textbf{岡部人材評価合同会社}}
\fancyhead[R]{\color{OKolive}\small 面接者研修／演習資料}
\fancyfoot[L]{\color{OKolive}\small 岡部人材評価合同会社\enspace 評価研修}
\fancyfoot[R]{\color{OKolive}\small\thepage}
\renewcommand{\headrulewidth}{.55pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{OKolive}\leaders\hrule height \headrulewidth\hfill}}
\renewcommand{\footrulewidth}{.3pt}

\newcommand{\OKHeading}[5]{%
  {\LARGE\bfseries #1\par}\medskip
  {\setlength{\fboxsep}{3mm}%
   \noindent\colorbox{OKpale}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
     \small\raggedright
     \begin{tabular}{@{}p{.53\linewidth}@{\hspace{4mm}}p{\dimexpr.47\linewidth-4mm\relax}@{}}
       \textbf{資料番号}\enspace #2 & \textbf{版}\enspace #3 \\
       \textbf{発行日}\enspace #4 & \textbf{担当講師}\enspace #5
     \end{tabular}}}\par}\medskip}
\newcommand{\OKSection}[1]{%
  \par\medskip\noindent{\color{OKolive}\rule[-.2ex]{2pt}{1.2em}\enspace\large\bfseries #1\par}%
  \nobreak\smallskip}
\newcommand{\OKNote}[1]{\par{\small\textbf{\color{OKolive}留意点}\enspace #1\par}}
\newenvironment{OKTable}[1]{%
  \renewcommand{\arraystretch}{1.35}\begin{longtable}{#1}%
}{\end{longtable}}
\newcommand{\OKTableHeader}{\rowcolor{OKpale}}
