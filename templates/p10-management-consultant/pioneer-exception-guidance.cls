\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{pioneer-exception-guidance}[2025/04/07 Pioneer purchasing exception guidance]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\RequirePackage[top=33mm,bottom=23mm,left=19mm,right=19mm,headheight=38pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,pict2e,hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmonofont{Liberation Mono}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{ptdink}{HTML}{20262A}
\definecolor{ptdroute}{HTML}{11677C}
\definecolor{ptdice}{HTML}{E5F3F5}
\AtBeginDocument{\color{ptdink}}
\hypersetup{hidelinks}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}

% Two lanes merging into the outbound route.
\newcommand{\ptdmark}{\raisebox{-1mm}{\begingroup
  \setlength{\unitlength}{1mm}\color{ptdroute}\linethickness{0.9mm}%
  \begin{picture}(12,7)
    \put(0,6){\line(1,0){4}}\put(0,1){\line(1,0){5}}%
    \put(4,6){\line(2,-1){4}}\put(5,1){\line(2,1){6}}%
    \put(8,4){\line(1,0){4}}
  \end{picture}\endgroup}}

\newcommand{\ptdnumber}{}
\newcommand{\ptdrevision}{}
\newcommand{\ptddate}{}
\newcommand{\ptdowner}{}
\newcommand{\ptdcontrol}[4]{%
  \renewcommand{\ptdnumber}{#1}\renewcommand{\ptdrevision}{#2}%
  \renewcommand{\ptddate}{#3}\renewcommand{\ptdowner}{#4}%
  \hypersetup{pdfauthor={#4}}}

\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}
\fancyhead[L]{\ptdmark\hspace{2.5mm}%
  {\color{ptdink}\bfseries\large Pioneer Trade Distribution Inc}}
\fancyhead[R]{\small\bfseries\textcolor{ptdroute}{BUYING OPERATIONS}}
\fancyfoot[L]{\small\texttt{\ptdnumber}\enspace/\enspace Rev.\ \ptdrevision\enspace/\enspace\ptddate}
\fancyfoot[R]{\small\textcolor{ptdroute}{\bfseries\thepage}}

\newcommand{\ptdtitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\color{ptdroute}\rule[-1.3mm]{2mm}{10mm}}\hspace{3mm}{\LARGE\bfseries #1\par}%
  \vspace{3mm}%
  \begingroup\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.45}%
  \begin{tabular}{@{}>{\raggedright\arraybackslash}p{55mm}%
    >{\raggedright\arraybackslash}p{20mm}%
    >{\raggedright\arraybackslash}p{34mm}%
    >{\raggedright\arraybackslash}p{55mm}@{}}
  \textcolor{ptdroute}{\small\bfseries Procedure ID}&
    \textcolor{ptdroute}{\small\bfseries Revision}&
    \textcolor{ptdroute}{\small\bfseries Effective date}&
    \textcolor{ptdroute}{\small\bfseries Issued by}\\
  \texttt{\ptdnumber}&\ptdrevision&\ptddate&\ptdowner
  \end{tabular}\endgroup\par\vspace{2mm}}
\newcommand{\ptdsection}[1]{%
  \par\addvspace{3mm}{\large\bfseries\textcolor{ptdroute}{#1}\par}\nopagebreak}

\newcommand{\ptdrow}[3]{#1 & #2 & #3\\\hline}
\newcommand{\ptdcolumns}{%
  \rowcolor{ptdice}\bfseries Ref.&\bfseries Request or evidence&\bfseries Routing instruction\\}
\newenvironment{ptdroute}{%
  \begingroup\setlength{\tabcolsep}{6pt}\setlength{\LTleft}{0pt}%
  \setlength{\LTright}{0pt plus 1fill}\renewcommand{\arraystretch}{1.45}%
  \arrayrulecolor{ptdice}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{10mm}%
    >{\raggedright\arraybackslash}p{51mm}%
    >{\raggedright\arraybackslash}p{104mm}}
  \ptdcolumns\endfirsthead
  \ptdcolumns\endhead
}{\end{longtable}\endgroup}
