\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{ainsworth-peer-review}[2025/02/12 Ainsworth Scholarly Press peer review]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[left=24mm,right=24mm,top=29mm,bottom=24mm,headheight=18pt,headsep=7mm,footskip=12mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,fancyhdr,tabularx,titlesec,needspace}
% Keep Latin curly quotes in the proportional Latin font.
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{PressBlue}{HTML}{18374D}
\definecolor{PressInk}{HTML}{29353C}
\definecolor{PressTint}{HTML}{E6EEF2}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt plus 1pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\newcommand{\PressMark}{%
  \raisebox{0.5pt}{\rule{2mm}{3mm}\kern0.6mm\rule{4mm}{3mm}}}
\fancyhead[L]{\color{PressBlue}\bfseries\PressMark\hspace{3mm}AINSWORTH SCHOLARLY PRESS}
\fancyfoot[L]{\small\color{PressBlue}Editorial Office\hspace{1em}|\hspace{1em}Confidential peer review}
\fancyfoot[R]{\small\color{PressBlue}\thepage}
\titleformat{\section}{\large\bfseries\color{PressBlue}}{\thesection.}{0.6em}{}
\AtBeginDocument{\color{PressInk}\raggedright}
\newcommand{\ReviewHeading}[5]{%
  {\setlength{\fboxsep}{9pt}%
    \noindent\colorbox{PressTint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      {\small\bfseries\color{PressBlue}PEER REVIEW REPORT\par}%
      \smallskip{\LARGE\bfseries\color{PressBlue}#1\par}%
      \medskip\small
      \begin{tabularx}{\linewidth}{@{}>{\raggedright\arraybackslash}p{0.44\linewidth}@{\hspace{6mm}}>{\raggedright\arraybackslash}X@{}}
        \textbf{Review ref.}\enspace #2 & \textbf{Issued}\enspace #3\\
        \textbf{Issued by}\enspace #4 & \textbf{Prepared by}\enspace #5
      \end{tabularx}}}\par}
  \medskip}
\newcommand{\ReviewPoint}[2]{%
  \Needspace{4\baselineskip}%
  \par\medskip{\bfseries\color{PressBlue}#1\par}\nopagebreak
  \noindent #2\par}
\endinput
