\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{dalton-request-brief}[2026/09/01 Partner request brief]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[left=22mm,right=22mm,top=35mm,bottom=24mm,headheight=32pt]{geometry}
\RequirePackage{fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,array,longtable,booktabs,fancyhdr,lastpage}

\definecolor{DDPnavy}{HTML}{20314C}
\definecolor{DDPcopper}{HTML}{9B472F}
\definecolor{DDPstone}{HTML}{F0EBE5}

\newcommand{\DDPLogo}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-1.1mm]
  \fill[DDPnavy] (0,0)--(2.2,0)--(5.4,4.7)--(3.2,4.7)--cycle;
  \fill[DDPcopper] (3.5,0)--(5.7,0)--(8.9,4.7)--(6.7,4.7)--cycle;
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\DDPLogo\enspace\sffamily\bfseries\color{DDPnavy}Dalton Distribution Partners}
\fancyhead[R]{\footnotesize\sffamily\bfseries\color{DDPcopper}PARTNER REQUEST BRIEF}
\fancyfoot[L]{\footnotesize\sffamily\color{DDPnavy}Partner Sales}
\fancyfoot[R]{\footnotesize\sffamily\color{DDPnavy}\thepage\ of \pageref{LastPage}}
\renewcommand{\headrule}{{\color{DDPcopper}\hrule width\headwidth height 1.2pt}\vskip-1.2pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\AtBeginDocument{\color{DDPnavy}}

% Start the label paragraph before setting its color to keep the p-cell baseline.
\newcommand{\DDPLabel}[1]{\textcolor{DDPcopper}{\bfseries #1}}
\newcommand{\DDPRecord}[6]{%
  {\fontsize{25}{28}\selectfont\bfseries #1\par}
  \smallskip
  {\sffamily\small\renewcommand{\arraystretch}{1.3}%
    \begin{tabular}{@{}>{\columncolor{DDPstone}}p{32mm}p{52mm}
                       >{\columncolor{DDPstone}}p{23mm}p{42mm}@{}}
      \DDPLabel{Reference} & #2 & \DDPLabel{Revision} & #3 \\
      \DDPLabel{Issued} & #4 & \DDPLabel{Prepared by} & #5 \\
      \DDPLabel{Sales window} & \multicolumn{3}{p{\dimexpr117mm+4\tabcolsep\relax}@{}}{#6} \\
    \end{tabular}\par}%
  \medskip}
\newcommand{\DDPSection}[1]{\par\medskip{\large\sffamily\bfseries\color{DDPcopper}#1\par}\nobreak}
\newcommand{\DDPHead}[1]{\textcolor{white}{\sffamily\bfseries #1}}

% The request title starts well beyond the right-aligned index.
\newenvironment{DDPRequests}{%
  \renewcommand{\arraystretch}{1.18}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{12mm}
                     @{\hspace{6mm}}>{\raggedright\arraybackslash}p{42mm}
                     >{\raggedright\arraybackslash}p{102mm}}
    \rowcolor{DDPnavy}\DDPHead{No.} & \DDPHead{Request} & \DDPHead{Use and question} \\
    \endfirsthead
    \rowcolor{DDPnavy}\DDPHead{No.} & \DDPHead{Request} & \DDPHead{Use and question} \\
    \endhead
    \bottomrule
    \endlastfoot
}{\end{longtable}}
