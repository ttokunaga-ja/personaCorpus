\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{merritt-support-request}[2025/12/01 Merritt support request]
\LoadClass[10pt,letterpaper]{article}
\RequirePackage[margin=.8in,top=1.28in,bottom=.82in,headheight=36pt,headsep=.2in]{geometry}
\RequirePackage{fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr,array,longtable}
\RequirePackage[unicode,hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\definecolor{MERInk}{HTML}{2B3135}
\definecolor{MERBlue}{HTML}{243B56}
\definecolor{MERGold}{HTML}{A97934}
\definecolor{MERPaper}{HTML}{F3EFE6}
\AtBeginDocument{\color{MERInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\LTleft}{0pt}
\setlength{\LTright}{\fill}
\renewcommand{\arraystretch}{1.35}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{MERBlue}{\large\bfseries Merritt Equipment Repair Inc}\\[3pt]
  \textcolor{MERBlue}{\small OPERATIONS PLANNING}}
\renewcommand{\headrule}{\hbox to\headwidth{\color{MERGold}\leaders\hrule height 1.2pt\hfill}}
\fancyfoot[L]{\small\textcolor{MERBlue}{Merritt / Operations Planning}}
\fancyfoot[R]{\small\textcolor{MERBlue}{Page \thepage}}
\newcommand{\MERSupportRequest}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\color{MERBlue}\LARGE\bfseries #1\par}%
  \textbf{Request:} #2\quad\textbf{Issued:} #3\par
  \textbf{Requested by:} #4\quad\textbf{To:} #5\par\medskip}
\newcolumntype{M}[1]{>{\raggedright\arraybackslash}p{#1}}
\newcolumntype{N}[1]{>{\raggedleft\arraybackslash}p{#1}}
\newcommand{\MERRequestHeader}{\rowcolor{MERPaper}\textbf{Service requested} &
  \textbf{Hours} & \textbf{Preferred window and purpose}\\}
\newenvironment{MERRequests}{%
  \begin{longtable}{@{}M{1.62in}N{.6in}M{4.38in}@{}}
  \MERRequestHeader\endfirsthead
  \MERRequestHeader\endhead
}{\end{longtable}}
\newcommand{\MERRequest}[3]{#1 & #2 & #3\\\arrayrulecolor{MERPaper}\hline}
\endinput
