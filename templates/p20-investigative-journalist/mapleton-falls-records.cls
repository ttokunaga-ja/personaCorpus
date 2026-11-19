\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{mapleton-falls-records}[2024/04/01 Borough records office]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,left=.9in,right=.9in,top=1.32in,bottom=.85in,headheight=30pt,headsep=17pt,footskip=24pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fontspec,fancyhdr,tikz}
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
\newcommand{\BoroughUnit}{Public Records Office}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[C]{\color{BoroughBlue}\BoroughMark\enspace\large\bfseries MAPLETON FALLS BOROUGH\\[-1pt]
  \small\mdseries\BoroughUnit}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{BoroughBlue}\leaders\hrule height .45pt\hfill}}
\fancyfoot[L]{\footnotesize\color{BoroughBlue}\BoroughUnit}
\fancyfoot[R]{\footnotesize\color{BoroughBlue}\thepage}
\newcommand{\BoroughHeading}[7]{%
  \colorbox{BoroughBlue}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{\vspace{4pt}\color{white}\large\bfseries\hspace{6pt}#1\vspace{4pt}}}\par
  \renewcommand{\arraystretch}{1.28}%
  \begin{tabular}{@{}p{1.03in}p{5.35in}@{}}
    \textcolor{BoroughBlue}{\bfseries Reference}& #2\\
    \textcolor{BoroughBlue}{\bfseries Date}& #3\\
    \textcolor{BoroughBlue}{\bfseries #6}& #4\\
    \textcolor{BoroughBlue}{\bfseries #7}& #5
  \end{tabular}\par\medskip}
\newcommand{\BoroughReply}[5]{\BoroughHeading{#1}{#2}{#3}{#4}{#5}{To}{From}}
\newcommand{\BoroughSection}[1]{\par\medskip{\bfseries\color{BoroughBlue}#1\par}\nobreak}

% Public Works records in the borough's own contract file.
\RequirePackage{array,longtable}
\newcommand{\BoroughContract}[5]{%
  \renewcommand{\BoroughUnit}{Public Works}%
  \BoroughHeading{#1}{#2}{#3}{#4}{#5}{Party}{Office}}
\newcommand{\BoroughAmountHead}[1]{\textcolor{BoroughBlue}{\bfseries #1}}
\newenvironment{BoroughPriceSchedule}{%
  \renewcommand{\arraystretch}{1.28}%
  \setlength{\LTleft}{0pt}%
  \setlength{\LTright}{\fill}%
  \arrayrulecolor{BoroughBlue}%
  \begin{longtable}{@{}>{\raggedleft\arraybackslash}p{.37in}@{\hspace{.17in}}%
    >{\raggedright\arraybackslash}p{2.60in}%
    >{\raggedleft\arraybackslash}p{1.10in}@{\hspace{.17in}}%
    >{\raggedright\arraybackslash}p{2.08in}@{}}
  \BoroughAmountHead{No.}&\BoroughAmountHead{Item or price field}&
  \BoroughAmountHead{USD}&\BoroughAmountHead{Basis or reference}\\\hline\endhead
}{\hline\end{longtable}}
