\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{holt-client-note}[2024/10/15 Holt and Reeves client note]
\LoadClass[a4paper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\RequirePackage[top=31mm,bottom=24mm,left=24mm,right=24mm,headheight=32pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,pict2e,hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{holtink}{HTML}{302435}
\definecolor{holtrule}{HTML}{964B37}
\definecolor{holtpale}{HTML}{F4ECE8}
\AtBeginDocument{\color{holtink}}
\hypersetup{hidelinks}

\newcommand{\holtmark}{%
  \begingroup
  % Proportions from the shared Word and Excel mark.
  \setlength{\unitlength}{0.125mm}%
  \begin{picture}(84,43)
    {\color{holtink}\polygon*(0,0)(54,0)(72,21)(18,21)}%
    {\color{holtrule}\polygon*(12,22)(66,22)(84,43)(30,43)}%
  \end{picture}%
  \endgroup}
\newcommand{\holtnumber}{}
\newcommand{\holtdate}{}
\newcommand{\holtowner}{}
\newcommand{\holtrecipient}{}
\newcommand{\holtcontrol}[4]{%
  \renewcommand{\holtnumber}{#1}\renewcommand{\holtdate}{#2}%
  \renewcommand{\holtowner}{#3}\renewcommand{\holtrecipient}{#4}%
  \hypersetup{pdfauthor={#3}}}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{holtink}{\holtmark\enspace\large\bfseries Holt \& Reeves Advisory Ltd}}
\fancyhead[R]{\textcolor{holtrule}{\small\bfseries CLIENT NOTE}}
\fancyfoot[L]{\textcolor{holtink}{\small\holtnumber\enspace\textbar\enspace\holtdate}}
\fancyfoot[R]{\textcolor{holtink}{\small Page \thepage}}
\renewcommand{\headrule}{\hbox to\headwidth{\color{holtrule}\leaders\hrule height 1.1pt\hfill}}
\renewcommand{\footrulewidth}{0.4pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{holtrule}\leaders\hrule height\footrulewidth\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}

\newcommand{\holttitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\LARGE\bfseries #1\par}\vspace{3mm}%
  \begingroup\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.45}%
  \begin{tabular}{p{64mm}p{29mm}p{58mm}}
    \rowcolor{holtpale}\small\bfseries Document no.&\small\bfseries Date&\small\bfseries Prepared by\relax\\
    \rowcolor{holtpale}\holtnumber&\holtdate&\holtowner\\
    \multicolumn{3}{@{}p{\linewidth}@{}}{\textcolor{holtrule}{\bfseries Prepared for}\enspace\holtrecipient}
  \end{tabular}\endgroup\par\vspace{3mm}}
\newcommand{\holtsection}[1]{%
  \par\addvspace{4mm}{\color{holtrule}\large\bfseries #1\par}\nopagebreak}
\newcommand{\holtitem}[3]{#1 & #2 & #3\\\hline}
\newcommand{\holtcolumns}[3]{%
  \rowcolor{holtink}\textcolor{white}{\bfseries #1}&
  \textcolor{white}{\bfseries #2}&\textcolor{white}{\bfseries #3}\\}
\newenvironment{holttable}[3]{%
  \begingroup\setlength{\tabcolsep}{5pt}\setlength{\LTleft}{0pt}%
  \setlength{\LTright}{0pt plus 1fill}\renewcommand{\arraystretch}{1.40}%
  \arrayrulecolor{holtpale}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{44mm}%
                    >{\raggedleft\arraybackslash}p{25mm}%
                    >{\raggedright\arraybackslash}p{82mm}}
  \holtcolumns{#1}{#2}{#3}\endfirsthead
  \holtcolumns{#1}{#2}{#3}\endhead
}{\end{longtable}\endgroup}
