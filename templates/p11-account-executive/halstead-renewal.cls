\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{halstead-renewal}[2024/04/22 Halstead Branch Connect renewal terms]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage{fontspec}
\RequirePackage{luatexja-fontspec}
\RequirePackage[letterpaper,margin=.76in,top=1.28in,bottom=.82in,headheight=42pt,headsep=.22in]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr}
\RequirePackage{titlesec}
\RequirePackage{array,longtable}
\RequirePackage[unicode,hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{HBTeal}{HTML}{133D45}
\definecolor{HBRust}{HTML}{AF5636}
\definecolor{HBInk}{HTML}{26353B}
\definecolor{HBMist}{HTML}{EBF2F1}
\AtBeginDocument{\color{HBInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\LTleft}{0pt}
\setlength{\LTright}{\fill}
\renewcommand{\arraystretch}{1.36}
\titleformat{\section}{\color{HBTeal}\Large\bfseries}{}{0pt}{}
\titleformat{\subsection}{\color{HBTeal}\large\bfseries}{}{0pt}{}
\titlespacing*{\section}{0pt}{15pt}{4pt}
\titlespacing*{\subsection}{0pt}{10pt}{3pt}
\pagestyle{fancy}
\fancyhf{}
\newcommand{\HBCorporateMark}{%
  \raisebox{-.25em}[1em][.2em]{\color{HBTeal}\rule{.32em}{1.2em}%
    \raisebox{.48em}{\rule{.50em}{.20em}}%
    \raisebox{.14em}{\color{HBRust}\rule{.25em}{.94em}}}%
  \hspace{.45em}{\color{HBTeal}\bfseries Halstead Branch Communications Inc}}
\newcommand{\HBDocumentType}{RENEWAL TERMS}
\fancyhead[L]{\HBCorporateMark\\[3pt]{\footnotesize\color{HBTeal}\bfseries \HBDocumentType}}
\renewcommand{\headrule}{\hbox to\headwidth{\color{HBRust}\leaders\hrule height 1.4pt\hfill}}
\fancyfoot[L]{\footnotesize\color{HBTeal}Halstead Branch Connect / Account Services}
\fancyfoot[R]{\footnotesize\color{HBTeal}Page \thepage}
\newcommand{\HBRenewal}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\color{HBTeal}\huge\bfseries #1\par}
  \medskip
  \begingroup\setlength{\tabcolsep}{0pt}%
  \begin{tabular}{@{}p{.53\linewidth}p{.45\linewidth}@{}}
    \textbf{Document no.}\enspace #2 & \textbf{Issued}\enspace #3 \\
    \textbf{Prepared by}\enspace #4 & \textbf{Account}\enspace #5
  \end{tabular}\endgroup
  \medskip\par}
\newcolumntype{H}[1]{>{\raggedright\arraybackslash}p{#1}}
\newcolumntype{Q}[1]{>{\raggedleft\arraybackslash}p{#1}}
\newcommand{\HBHeadingCell}[1]{\textcolor{white}{\bfseries #1}}
\newcommand{\HBHeadRow}{\rowcolor{HBTeal}\HBHeadingCell{Term} & \HBHeadingCell{Value} & \HBHeadingCell{Basis or treatment}\\}
\newenvironment{HBTerms}{%
  \begin{longtable}{H{1.6in}Q{1.35in}H{3.53in}}
  \HBHeadRow\endfirsthead
  \HBHeadRow\endhead
}{\end{longtable}}
\newcommand{\HBTerm}[3]{#1 & #2 & #3\\\arrayrulecolor{HBMist}\hline}
\newcommand{\HBNoteText}[1]{{\small\textcolor{HBRust}{\textbf{Note.}}\enspace #1}}
\newcommand{\HBTotalRow}[3]{\rowcolor{HBMist}\textbf{#1} & \textbf{#2} & #3}
\newcommand{\HBTotal}[3]{\HBTotalRow{#1}{#2}{#3}\\}
% Include the two internal column gaps in the full-width note.
\newcommand{\HBTotalWithNote}[4]{%
  \HBTotalRow{#1}{#2}{#3}\\*
  \multicolumn{3}{H{\dimexpr6.48in+4\tabcolsep\relax}}{\HBNoteText{#4}}\\}
\newcommand{\HBNote}[1]{\par\smallskip\HBNoteText{#1}\par}
\endinput
