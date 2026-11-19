\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{mapleton-falls-records}[2024/04/01 Borough records office]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,left=.9in,right=.9in,top=1.32in,bottom=.85in,headheight=30pt,headsep=17pt,footskip=24pt]{geometry}
\RequirePackage{fontspec,xcolor,fancyhdr,tikz}
\setmainfont{Liberation Sans}
\definecolor{BoroughBlue}{HTML}{234C69}
\definecolor{BoroughSlate}{HTML}{293942}
\AtBeginDocument{\color{BoroughSlate}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{8pt}
\newcommand{\BoroughMark}{%
  \begin{tikzpicture}[baseline=-.7ex,x=1pt,y=1pt]
    \draw[draw=BoroughBlue,line width=.8pt] (0,0) circle (9);
    \draw[draw=BoroughBlue,line width=.8pt] (-5,1)--(0,5)--(5,1)--cycle;
    \draw[draw=BoroughBlue,line width=.8pt] (-4,0)--(-4,-4)--(4,-4)--(4,0);
    \draw[draw=BoroughBlue,line width=.8pt] (0,0)--(0,-4);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[C]{\color{BoroughBlue}\BoroughMark\enspace\large\bfseries MAPLETON FALLS BOROUGH\\[-1pt]
  \small\mdseries Public Records Office}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{BoroughBlue}\leaders\hrule height .45pt\hfill}}
\fancyfoot[L]{\footnotesize\color{BoroughBlue}Public Records Office}
\fancyfoot[R]{\footnotesize\color{BoroughBlue}\thepage}
\newcommand{\BoroughReply}[5]{%
  \colorbox{BoroughBlue}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{\vspace{4pt}\color{white}\large\bfseries\hspace{6pt}#1\vspace{4pt}}}\par
  \renewcommand{\arraystretch}{1.28}%
  \begin{tabular}{@{}p{1.03in}p{5.35in}@{}}
    \textcolor{BoroughBlue}{\bfseries Reference}& #2\\
    \textcolor{BoroughBlue}{\bfseries Date}& #3\\
    \textcolor{BoroughBlue}{\bfseries To}& #4\\
    \textcolor{BoroughBlue}{\bfseries From}& #5
  \end{tabular}\par\medskip}
\newcommand{\BoroughSection}[1]{\par\medskip{\bfseries\color{BoroughBlue}#1\par}\nobreak}
