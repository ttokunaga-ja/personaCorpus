\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{marston-bench-card}[2024/11/21 Marston production bench card]
\LoadClass{marston-work-standard}

\geometry{a5paper,top=21mm,bottom=15mm,left=12mm,right=12mm,headheight=22pt,headsep=5mm,footskip=8mm}
\setlength{\headwidth}{\textwidth}
\fancyhead[L]{\mcmark\hspace{2mm}\textcolor{mcblue}{\textbf{Marston Carton Packaging Ltd}}}
\fancyhead[R]{\footnotesize\textbf{PRODUCTION}}
\fancyfoot[R]{\footnotesize\thepage/2}

\newcommand{\mcbface}[2]{%
  {\small\bfseries #1\par}%
  {\LARGE\bfseries #2\par}\medskip
  {\small T2 / W1\enspace\textbar\enspace Issued by \mcowner\par}\medskip}

\newcommand{\mcbaction}[2]{\mcsection{#1}{\fontsize{12}{16}\selectfont #2\par}}
