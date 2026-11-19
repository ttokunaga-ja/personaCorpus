\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{myojo-ad-proof}[2025/02/24 Recruitment ad proofs]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[left=21mm,right=21mm,top=33mm,bottom=23mm,headheight=24pt,headsep=9mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,tikz,fancyhdr,longtable,array,colortbl}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{MAink}{HTML}{273449}
\definecolor{MAorange}{HTML}{A84816}
\definecolor{MApale}{HTML}{F8EFE7}
\AtBeginDocument{\color{MAink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{3pt}
\setlength{\tabcolsep}{7pt}
\newcommand{\MALogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
  \fill[MAorange] (0,3)--(2.1,3.9)--(3,6)--(3.9,3.9)--
    (6,3)--(3.9,2.1)--(3,0)--(2.1,2.1)--cycle;
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\MALogo\enspace\bfseries 明星採用広告株式会社}
\fancyhead[R]{\small\bfseries\color{MAorange}掲載原稿のご確認}
\fancyfoot[L]{\small 明星採用広告株式会社\enspace 営業担当}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{1.2pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{MAorange}\leaders\hrule height \headrulewidth\hfill}}
\renewcommand{\footrulewidth}{.3pt}

\newcommand{\MAHeading}[6]{%
  {\small #1\par}%
  \vspace{3mm}%
  {\LARGE\bfseries #2\par}%
  \vspace{3mm}%
  {\setlength{\fboxsep}{3mm}%
   \noindent\colorbox{MApale}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
     \small\raggedright
     \begin{tabular}{@{}p{.55\linewidth}@{\hspace{4mm}}p{\dimexpr.45\linewidth-4mm\relax}@{}}
       \textbf{校正番号}\enspace #3 & \textbf{版}\enspace #4 \\
       \textbf{発行日}\enspace #5 & \textbf{発行担当}\enspace #6
     \end{tabular}}}\par}%
  \medskip}
\newcommand{\MASection}[1]{%
  \par\medskip\noindent{\large\bfseries\color{MAorange}#1\par}\nobreak\smallskip}
\newcommand{\MANote}[1]{\par\smallskip{\small\textbf{\color{MAorange}確認事項}\enspace #1\par}}
\newenvironment{MATable}[1]{%
  \renewcommand{\arraystretch}{1.3}\begin{longtable}{#1}%
}{\end{longtable}}
\newcommand{\MATableHeader}{\rowcolor{MApale}}
