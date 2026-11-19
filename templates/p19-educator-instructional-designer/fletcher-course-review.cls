\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fletcher-course-review}[2025/05/13 Fletcher course review memorandum]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[top=1.20in,bottom=.91in,left=1.07in,right=1.05in,headheight=34pt,headsep=15pt]{geometry}
\RequirePackage{xcolor,tikz,fancyhdr,hyperref}
\definecolor{FletcherTeal}{HTML}{20565E}
\definecolor{FletcherInk}{HTML}{26333A}
\definecolor{FletcherMist}{HTML}{E7EFF0}
\newcommand{\FletcherLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.7mm]
    \draw[FletcherTeal,line width=.85pt] (0,0)--(0,6)--(5,6) (8,6)--(8,0)--(3,0);
    \draw[FletcherTeal,line width=.85pt] (2,3)--(3.6,1.6)--(7,5.1);
  \end{tikzpicture}%
}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\color{FletcherTeal}\FletcherLogo\hspace{.62em}\sffamily\small\bfseries Fletcher Course Review Ltd}
\fancyfoot[R]{\small\sffamily\color{FletcherTeal} Course review memorandum\enspace / \enspace\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}
\AtBeginDocument{\color{FletcherInk}\raggedright}
\newcommand{\FletcherReview}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\small\sffamily\bfseries\color{FletcherTeal} REVIEW MEMORANDUM\par}%
  {\huge\bfseries #1\par}%
  \begingroup
  \setlength{\fboxsep}{8pt}%
  \noindent\colorbox{FletcherMist}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{\small\sffamily\raggedright
      \textbf{Reference}\quad #2\qquad \textbf{Issued}\quad #3\par
      \textbf{Reviewer}\quad #4\par
      \textbf{To}\quad #5}}%
  \par\endgroup
  \medskip
}
\newcommand{\FletcherFinding}[2]{%
  \par\medskip
  {\large\sffamily\bfseries\color{FletcherTeal}#1\par}%
  \nobreak #2\par
}
