\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{marston-work-standard}[2024/08/05 Marston production work standard]
\LoadClass[a4paper,10pt]{article}
\RequirePackage{luatexja-fontspec}
\RequirePackage[top=34mm,bottom=24mm,left=18mm,right=18mm,headheight=41pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,graphicx,hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{mcblue}{HTML}{15374C}
\definecolor{mcamber}{HTML}{D89C36}
\definecolor{mcpale}{HTML}{EDF2F4}
\AtBeginDocument{\color{mcblue}}
\hypersetup{hidelinks}

% Offset flaps above the base of a folding carton.
\newcommand{\mcmark}{\raisebox{-1mm}{\colorbox{mcblue}{%
  \setlength{\unitlength}{1mm}\begin{picture}(9,6)
    \color{mcamber}\linethickness{0.65mm}
    \put(1,1){\line(1,0){7}}\put(1,1){\line(0,1){2}}\put(8,1){\line(0,1){2}}
    \put(1,3.4){\rotatebox{27}{\rule{3.7mm}{0.65mm}}}
    \put(4.7,5.0){\rotatebox{-27}{\rule{3.7mm}{0.65mm}}}
  \end{picture}}}}
\newcommand{\mcnumber}{}
\newcommand{\mcrevision}{}
\newcommand{\mcdate}{}
\newcommand{\mcowner}{}
\newcommand{\mccontrol}[4]{%
  \renewcommand{\mcnumber}{#1}\renewcommand{\mcrevision}{#2}%
  \renewcommand{\mcdate}{#3}\renewcommand{\mcowner}{#4}%
  \hypersetup{pdfauthor={#4}}}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\mcmark\hspace{3mm}\textcolor{mcblue}{\large\bfseries Marston Carton Packaging Ltd}}
\fancyhead[R]{\textcolor{mcblue}{\footnotesize\bfseries PRODUCTION / WORK STANDARD}}
\fancyfoot[L]{\textcolor{mcblue}{\small\mcnumber\enspace\textbar\enspace\mcrevision\enspace\textbar\enspace\mcdate}}
\fancyfoot[R]{\textcolor{mcblue}{\small Page \thepage}}
\renewcommand{\headrule}{\hbox to\headwidth{\color{mcamber}\leaders\hrule height 2.2pt\hfill}}
\renewcommand{\footrulewidth}{0.4pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{mcblue}\leaders\hrule height\footrulewidth\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\newcommand{\mctitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\LARGE\bfseries #1\par}\vspace{3mm}%
  \begingroup\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.45}%
  \begin{tabular}{>{\raggedright\arraybackslash}p{50mm}%
    >{\raggedright\arraybackslash}p{24mm}%
    >{\raggedright\arraybackslash}p{44mm}%
    >{\raggedright\arraybackslash}p{40mm}}
    \rowcolor{mcpale}\small\bfseries Standard no.&\small\bfseries Revision&
      \small\bfseries Effective date&\small\bfseries Issued by\\
    \rowcolor{mcpale}\mcnumber&\mcrevision&\mcdate&\mcowner
  \end{tabular}\endgroup\par\vspace{4mm}}
\newcommand{\mcsection}[1]{%
  \par\addvspace{3mm}{\large\bfseries #1\par}\nopagebreak}
\newcommand{\mcstep}[4]{#1 & #2 & #3 & #4\\\hline}
\newcommand{\mccolumns}{%
  \rowcolor{mcblue}\textcolor{white}{\bfseries No.}&
  \textcolor{white}{\bfseries Operation}&\textcolor{white}{\bfseries Control / acceptance}&
  \textcolor{white}{\bfseries Record / action}\\}
\newenvironment{mcsteps}{%
  \begingroup\setlength{\tabcolsep}{5pt}\setlength{\LTleft}{0pt}%
  \setlength{\LTright}{0pt plus 1fill}\renewcommand{\arraystretch}{1.45}%
  \arrayrulecolor{mcblue}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{8mm}%
    >{\raggedright\arraybackslash}p{38mm}%
    >{\raggedright\arraybackslash}p{65mm}%
    >{\raggedright\arraybackslash}p{48mm}}
  \mccolumns\endfirsthead
  \mccolumns\endhead
}{\end{longtable}\endgroup}
