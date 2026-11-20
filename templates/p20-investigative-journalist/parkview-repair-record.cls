\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{parkview-repair-record}[2026/02/11 Parkview repair records]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,margin=.8in,top=1.25in,headheight=36pt,headsep=14pt,footskip=23pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fontspec,fancyhdr,array,longtable,tabularx,tikz}
\setmainfont{Liberation Sans}
\setmonofont{Liberation Mono}
\definecolor{ParkviewGreen}{HTML}{315642}
\definecolor{ParkviewInk}{HTML}{28332E}
\definecolor{ParkviewTint}{HTML}{E7EDE4}
\AtBeginDocument{\color{ParkviewInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}

\newcommand{\ParkviewMark}{%
  \begin{tikzpicture}[baseline=-.45ex,x=1pt,y=1pt]
    \draw[ParkviewGreen,line width=1.5pt] (0,7)--(9,15)--(18,7) (2,8)--(2,0)--(16,0)--(16,8);
    \fill[ParkviewGreen] (7,0) rectangle (11,7);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{ParkviewGreen}\ParkviewMark\quad\bfseries Parkview Court Tenants Association}
\fancyhead[R]{\color{ParkviewGreen}\small Repairs records}
\renewcommand{\headrulewidth}{0pt}
\fancyfoot[L]{\color{ParkviewGreen}\footnotesize Association file copy}
\fancyfoot[R]{\color{ParkviewGreen}\footnotesize Page \thepage}

\newcommand{\ParkviewRecord}[4]{%
  {\LARGE\bfseries\color{ParkviewGreen}#1\par}
  \smallskip
  {\renewcommand{\arraystretch}{1.35}%
  \begin{tabularx}{\linewidth}{@{}p{.9in}Xp{.9in}p{1.65in}@{}}
    \rowcolor{ParkviewTint} \textcolor{ParkviewGreen}{\textbf{Reference}} & {\ttfamily #2}
      & \textcolor{ParkviewGreen}{\textbf{Date}} & #3\\
    \rowcolor{ParkviewTint} \textcolor{ParkviewGreen}{\textbf{Prepared by}} & \multicolumn{3}{l@{}}{#4}\\
  \end{tabularx}}\par\medskip}

\newenvironment{ParkviewEntries}{%
  \renewcommand{\arraystretch}{1.35}%
  \setlength{\LTleft}{0pt}%
  \setlength{\LTright}{\fill}%
  \arrayrulecolor{ParkviewGreen}%
  \begin{longtable}{@{\hspace{.06in}}>{\raggedleft\arraybackslash}p{.35in}@{\hspace{.18in}}%
    >{\raggedright\arraybackslash}p{1.35in}%
    >{\raggedright\arraybackslash}p{1.2in}%
    >{\raggedright\arraybackslash}p{3.12in}@{\hspace{.06in}}}
    \rowcolor{ParkviewTint}\textbf{No.}&\textbf{Entry date}&\textbf{Entry type}&\textbf{Repair record}\\
    \hline\endhead
}{\hline\end{longtable}}
