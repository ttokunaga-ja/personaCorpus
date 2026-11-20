\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{montclair-ledger-copy}[2026/06/25 Montclair Reading Archive ledger copies]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[landscape,margin=19mm,headheight=21pt,headsep=5mm,footskip=10mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,graphicx,fancyhdr,longtable,array,colortbl,tabularx}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\definecolor{MontclairOchre}{HTML}{825019}
\definecolor{MontclairInk}{HTML}{292B2E}
\definecolor{MontclairPaper}{HTML}{F1E8D7}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{MontclairOchre}{%
  \rlap{\rule[4.5pt]{11pt}{2pt}}\makebox[11pt][c]{\rule{2pt}{11pt}}}\hspace{8pt}%
  {\bfseries Montclair Reading Archive}}
\fancyhead[R]{\small Service des reproductions}
\fancyfoot[L]{\small Copie de consultation}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0pt}
\AtBeginDocument{\color{MontclairInk}\raggedright}
\newcommand{\MontclairHeading}[5]{%
  \noindent\colorbox{MontclairPaper}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      {\Large\bfseries\color{MontclairOchre}#1\par}%
      \smallskip
      \begin{tabularx}{\linewidth}{@{}p{0.38\linewidth}XX@{}}
        \textbf{Copie n\textsuperscript{o}} #2 & \textbf{Émise le} #3 & \textbf{Préparée par} #4 \\
        \multicolumn{3}{@{}l@{}}{\textbf{Cote et feuillets} #5}
      \end{tabularx}}}\par\medskip}
\newcommand{\MontclairFacsimile}[1]{%
  \par\noindent\includegraphics[width=\linewidth,height=0.67\textheight,keepaspectratio]{#1}\par}
\newcommand{\MontclairColumns}{%
  \rowcolor{MontclairPaper}
  \textbf{Ligne} & \textbf{Date inscrite} & \textbf{Ouvrage} &
  \textbf{Destination inscrite} & \textbf{Mention au registre} \\ \hline}
\newenvironment{MontclairPrintedLedger}{%
  \begingroup\small\setlength{\tabcolsep}{4pt}\setlength{\extrarowheight}{4pt}%
  \arrayrulecolor{MontclairOchre!50}%
  \begin{longtable}{@{}>{\raggedleft\arraybackslash}p{12mm}@{\hspace{5mm}}%
    >{\raggedright\arraybackslash}p{32mm}%
    >{\raggedright\arraybackslash}p{55mm}%
    >{\raggedright\arraybackslash}p{61mm}%
    >{\raggedright\arraybackslash}p{72mm}@{}}%
  \MontclairColumns\endfirsthead
  \MontclairColumns\endhead
  \hline\endlastfoot
}{\end{longtable}\endgroup}
\endinput
