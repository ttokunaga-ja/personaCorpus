\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{denton-close-calendar}[2026/01/22 Denton Finance close calendar]
\LoadClass[a4paper,10pt]{article}
\RequirePackage[top=32mm,bottom=22mm,left=20mm,right=20mm,headheight=26pt]{geometry}
\RequirePackage{fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,hyperref}
\setmainfont{Liberation Sans}
\definecolor{dentonink}{HTML}{243D36}
\definecolor{dentonaccent}{HTML}{53705E}
\definecolor{dentonpale}{HTML}{E8EFE9}
\AtBeginDocument{\color{dentonink}}
\hypersetup{hidelinks}

\newcommand{\dentonref}{}
\newcommand{\dentondated}{}
\newcommand{\dentonowner}{}
\newcommand{\dentoncontrol}[3]{%
  \renewcommand{\dentonref}{#1}%
  \renewcommand{\dentondated}{#2}%
  \renewcommand{\dentonowner}{#3}%
  \hypersetup{pdfauthor={#3}}}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\large\bfseries Denton Prepared Foods Ltd}
\fancyhead[R]{\small\color{dentonaccent}\bfseries FINANCE / MONTH-END}
\fancyfoot[L]{\small\dentonref\quad\dentondated}
\fancyfoot[R]{\small Page \thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{dentonaccent}\leaders\hrule height 1.2pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}

\newcommand{\dentontitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\LARGE\bfseries #1\par}\vspace{3mm}%
  \begingroup\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.3}%
  \begin{tabular}{p{49mm}p{31mm}p{79mm}}
    \rowcolor{dentonpale}\small\bfseries Reference & \small\bfseries Issued & \small\bfseries Issued by\\
    \rowcolor{dentonpale}\dentonref & \dentondated & \dentonowner
  \end{tabular}\endgroup\par\vspace{3mm}}
\newcommand{\dentonsection}[1]{%
  \par\addvspace{3mm}{\large\bfseries #1\par}\nopagebreak}

\newcommand{\dentonhead}[4]{%
  \rowcolor{dentonink}\color{white}\bfseries #1 &
  \color{white}\bfseries #2 & \color{white}\bfseries #3 & \color{white}\bfseries #4\\}
\newenvironment{dentonschedule}[4]{%
  \begingroup\setlength{\tabcolsep}{5pt}\setlength{\LTleft}{0pt}%
  \renewcommand{\arraystretch}{1.32}\arrayrulecolor{dentonpale}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{16mm}%
                    >{\raggedright\arraybackslash}p{31mm}%
                    >{\raggedright\arraybackslash}p{53mm}%
                    >{\raggedright\arraybackslash}p{55mm}}
    \dentonhead{#1}{#2}{#3}{#4}\endfirsthead
    \dentonhead{#1}{#2}{#3}{#4}\endhead
}{\end{longtable}\endgroup}
\newcommand{\dentonentry}[4]{#1 & #2 & #3 & #4\\\hline}
