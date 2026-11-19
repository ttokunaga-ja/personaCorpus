\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{milton-training-proposal}[2025/04/01 Milton Learning Partners proposals]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[left=23mm,right=23mm,top=35mm,bottom=24mm,headheight=28pt,headsep=7mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,tikz,fancyhdr,longtable,array,colortbl,hyperref}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{MLPteal}{HTML}{007477}
\definecolor{MLPink}{HTML}{1F3042}
\definecolor{MLPpale}{HTML}{E4F2EF}
\AtBeginDocument{\color{MLPink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\tabcolsep}{7pt}

\newcommand{\MLPLogo}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.65mm]
    \fill[MLPteal] (0,0)--(2.2,0)--(5.2,5)--(3,5)--cycle;
    \fill[MLPink] (3.5,0)--(5.7,0)--(8.7,5)--(6.5,5)--cycle;
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\MLPLogo\enspace\sffamily\bfseries Milton Learning Partners}
\fancyhead[R]{\small\sffamily\color{MLPteal} Training proposal}
\fancyfoot[L]{\small\sffamily\color{MLPteal} Milton Learning Partners\enspace\textbar\enspace Client programmes}
\fancyfoot[R]{\small\sffamily\color{MLPteal}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}

\newcommand{\MLPHeading}[6]{%
  \hypersetup{pdftitle={#1},pdfauthor={#6}}%
  {\sffamily\small\bfseries\color{MLPteal} PREPARED FOR\enspace #2\par}%
  \smallskip
  {\raggedright\fontsize{28}{32}\selectfont\bfseries #1\par}%
  \medskip
  {\sffamily\small\raggedright
   \begin{tabular}{@{}p{.53\linewidth}@{\hspace{4mm}}p{\dimexpr.47\linewidth-4mm\relax}@{}}
     \textbf{Proposal}\enspace #3 & \textbf{Revision}\enspace #4 \\
     \textbf{Issued}\enspace #5 & \textbf{Prepared by}\enspace #6
   \end{tabular}\par}%
  \smallskip
  {\color{MLPteal}\rule{\linewidth}{.6pt}\par}}
\newcommand{\MLPSection}[1]{%
  \par\medskip\noindent{\sffamily\large\bfseries\color{MLPteal}#1\par}%
  \nobreak\smallskip}
\newcommand{\MLPNote}[1]{\par{\small\sffamily\textbf{Note}\enspace #1\par}}
\newenvironment{MLPTable}[1]{%
  \small\sffamily\renewcommand{\arraystretch}{1.42}\begin{longtable}{#1}%
}{\end{longtable}}
\newcommand{\MLPTableHeader}{\rowcolor{MLPpale}}
