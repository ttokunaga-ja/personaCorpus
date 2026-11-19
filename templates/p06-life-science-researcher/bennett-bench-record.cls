\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-bench-record}[2024/04/01 Bennett laboratory work sheet]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[margin=.8in, top=1.06in, includefoot]{geometry}
\RequirePackage{fontspec}
\setmainfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,tikz}
\definecolor{BCEInk}{HTML}{362742}
\definecolor{BCEIris}{HTML}{765387}
\definecolor{BCETint}{HTML}{F2EAF3}
\AtBeginDocument{\color{BCEInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\headheight}{21pt}
\setlength{\tabcolsep}{7pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrule}{\color{BCEIris}\hrule height .65pt width\headwidth}
\fancyhead[L]{\BennettMark\enspace\textbf{Bennett Cell Evaluation Inc} / Cell Evaluation Laboratory}
\fancyfoot[L]{\small LABORATORY RECORD}
\fancyfoot[R]{\small Page \thepage}
\newcommand{\BennettMark}{\tikz[baseline=-.28ex,x=1em,y=1em]{
  \draw[BCEInk,line width=.95pt] (0,0) circle[radius=.42];
  \fill[BCEIris] (.11,.08) circle[radius=.13];
  \fill[BCEIris] (.48,-.17) circle[radius=.14];}}
\newcommand{\BennettRecord}[4]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Measurement date}\enspace #3\par
    \textbf{Recorded by}\enspace #4}}\par\vspace{5pt}}
\newcommand{\BennettSection}[1]{\par\medskip{\large\bfseries\color{BCEIris}#1}\par\smallskip}
\newenvironment{BennettBenchTable}{%
  \renewcommand{\arraystretch}{1.5}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{.34in}
    >{\raggedright\arraybackslash\setlength{\leftskip}{.10in}}p{1.40in}
    >{\raggedright\arraybackslash}p{1.15in}
    >{\raggedright\arraybackslash}p{3.03in}}
  \rowcolor{BCEIris}
  \color{white}\bfseries No. & \color{white}\bfseries Reference &
  \color{white}\bfseries Review status & \color{white}\bfseries Observation \\
  \endhead
}{\arrayrulecolor{BCEIris}\hline\end{longtable}}
