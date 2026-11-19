\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{takase-notice-review}[2024/08/05 Employee notice review]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[top=33mm,bottom=24mm,left=22mm,right=22mm,headheight=31pt,headsep=9mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,array,longtable,fancyhdr,lastpage}

\definecolor{TakaseBlue}{HTML}{29586E}
\definecolor{TakasePale}{HTML}{E8F1F5}
\definecolor{TakaseInk}{HTML}{253239}
\AtBeginDocument{\color{TakaseInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\setlength{\tabcolsep}{8pt}
\renewcommand{\arraystretch}{1.45}

\newcommand{\TakaseMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
    \fill[TakasePale] (0,0) rectangle (9,8);
    \draw[TakaseBlue,line width=.45mm,line join=round]
      (1.6,2.5) -- (3,2.5) -- (3,1.3) -- (4.4,2.5) -- (7.4,2.5)
      -- (7.4,6.3) -- (1.6,6.3) -- cycle;
    \draw[TakaseBlue,line width=.32mm] (2.7,4.9) -- (6.3,4.9) (2.7,3.8) -- (5.2,3.8);
  \end{tikzpicture}}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\TakaseMark\enspace\textcolor{TakaseBlue}{\bfseries 高瀬労務相談室}}
\fancyfoot[L]{\small\textcolor{TakaseBlue}{高瀬労務相談室\enspace{}｜\enspace 送付先限り}}
\fancyfoot[R]{\small\textcolor{TakaseBlue}{\thepage\ / \pageref{LastPage}}}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.5pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{TakaseBlue}\leaders\hrule height\footrulewidth\hfill}\kern-\footrulewidth}

\newcommand{\TakaseReview}[5]{%
  {\small\raggedleft\textcolor{TakaseBlue}{所見番号}\enspace #2\par
   \textcolor{TakaseBlue}{発行日}\enspace #3\par}
  \medskip
  {\LARGE\bfseries\color{TakaseBlue}#1\par}
  {\small #5\enspace 御中\par
   担当相談員\enspace #4\par}
  \medskip}
\newcommand{\TakaseSection}[1]{%
  \par\addvspace{9pt}\noindent
  \textcolor{TakaseBlue}{\rule{2pt}{.85em}\enspace\large\bfseries #1}%
  \par\nobreak}

\newenvironment{TakasePoints}{%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{9mm}p{42mm}p{97mm}}
  \rowcolor{TakasePale}
  \textcolor{TakaseBlue}{\bfseries No.}&
  \textcolor{TakaseBlue}{\bfseries 照会箇所}&
  \textcolor{TakaseBlue}{\bfseries 所見・確認事項}\\\endhead
}{\hline\end{longtable}}
