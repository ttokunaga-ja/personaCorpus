\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{aldridge-rfp}[2024/10/01 Aldridge Service Group proposal request]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage{fontspec}
\RequirePackage{luatexja-fontspec}
\RequirePackage[letterpaper,left=.92in,right=.92in,top=1.18in,bottom=.85in,headheight=24pt,headsep=.22in]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr}
\RequirePackage{titlesec}
\RequirePackage{array,longtable}
\RequirePackage[unicode,hidelinks]{hyperref}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\definecolor{ASGIndigo}{HTML}{343454}
\definecolor{ASGViolet}{HTML}{575483}
\definecolor{ASGInk}{HTML}{2C2B34}
\definecolor{ASGPale}{HTML}{EEEDF5}
\AtBeginDocument{\color{ASGInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\LTleft}{0pt}
\setlength{\LTright}{\fill}
\renewcommand{\arraystretch}{1.35}
\titleformat{\section}{\color{ASGIndigo}\Large\bfseries}{\thesection.}{.45em}{}
\titleformat{\subsection}{\color{ASGIndigo}\large\bfseries}{\thesubsection}{.45em}{}
\pagestyle{fancy}
\fancyhf{}
\newcommand{\ASGMark}{%
  \textcolor{ASGViolet}{\rule{.18em}{.45em}\kern.11em\rule{.18em}{.72em}\kern.11em\rule{.18em}{.99em}}}
\fancyhead[R]{\sffamily\bfseries\color{ASGIndigo} Aldridge Service Group\enspace\ASGMark}
\renewcommand{\headrule}{\hbox to\headwidth{\color{ASGViolet}\leaders\hrule height .55pt\hfill}}
\fancyfoot[L]{\sffamily\footnotesize\color{ASGIndigo} Aldridge Service Group / Business Planning}
\fancyfoot[R]{\sffamily\footnotesize\color{ASGIndigo} \thepage}
\newcommand{\ASGRequest}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\small\sffamily\bfseries\color{ASGViolet} BUSINESS PLANNING\par}
  {\huge\bfseries\color{ASGIndigo} #1\par}
  \medskip
  \begingroup\setlength{\tabcolsep}{0pt}%
  \begin{tabular}{@{}p{.49\linewidth}p{.49\linewidth}@{}}
    \textbf{Request no.}\enspace #2 & \textbf{Issued}\enspace #3 \\
    \textbf{Issued by}\enspace #4 & \textbf{To}\enspace #5
  \end{tabular}\endgroup
  \medskip\par}
\newcolumntype{G}[1]{>{\raggedright\arraybackslash}p{#1}}
\newcolumntype{J}[1]{>{\raggedleft\arraybackslash}p{#1}}
\newcommand{\ASGReqHead}{%
  \rowcolor{ASGPale}\textcolor{ASGIndigo}{\bfseries No.} &
  \textcolor{ASGIndigo}{\bfseries Requirement} &
  \textcolor{ASGIndigo}{\bfseries Priority}\\}
\newenvironment{ASGRequirements}{%
  \begin{longtable}{J{.42in}G{4.65in}G{1.00in}}
    \ASGReqHead\endfirsthead
    \ASGReqHead\endhead
}{\end{longtable}}
\newcommand{\ASGRequirement}[3]{#1 & #2 & #3\\\arrayrulecolor{ASGPale}\hline}
\endinput
