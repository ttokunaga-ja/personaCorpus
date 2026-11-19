\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{saint-aubin-inventory}[2024/07/09 Archives de Saint-Aubin-sur-Rive]
\LoadClass[10pt,a4paper]{article}
\RequirePackage[left=20mm,right=20mm,top=24mm,bottom=22mm,headheight=16pt,headsep=6mm,footskip=11mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,fancyhdr,longtable,array,booktabs}
% Keep Latin curly quotes in the proportional Latin font.
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{ArchiveGreen}{HTML}{28483C}
\definecolor{ArchiveInk}{HTML}{202923}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt plus 1pt}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{ArchiveGreen}\bfseries
  \rule{2pt}{7pt}\hspace{1pt}\rule{2pt}{10pt}\hspace{1pt}\rule{2pt}{13pt}\hspace{0.6em}%
  Archives de Saint-Aubin-sur-Rive}
\fancyfoot[L]{\small\color{ArchiveGreen}Salle de lecture}
\fancyfoot[R]{\small\color{ArchiveGreen}\thepage}
\renewcommand{\headrulewidth}{0pt}
\AtBeginDocument{\color{ArchiveInk}\raggedright}
\newcommand{\ArchiveHeading}[5]{%
  {\setlength{\fboxsep}{9pt}%
  \noindent\colorbox{ArchiveGreen!8}{%
    \parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      {\small\bfseries\color{ArchiveGreen}INSTRUMENT DE RECHERCHE\par}%
      \smallskip{\LARGE\bfseries\color{ArchiveGreen}#1\par}%
      \medskip {\small Cote du document : #2\hspace{1em}/\hspace{1em}#3\par}%
      {\small Établi par #4\hspace{1em}/\hspace{1em}#5\par}}}\par}}
\newcommand{\ArchiveColumns}{%
  \textbf{N\textsuperscript{o}} & \textbf{Cote} & \textbf{Description} &
  \textbf{Feuillets} & \textbf{Communication} \\ \midrule}
\newenvironment{ArchiveTable}{%
  \begingroup
  \setlength{\tabcolsep}{5pt}%
  \setlength{\extrarowheight}{3pt}%
  \small
  \begin{longtable}{@{}>{\raggedleft\arraybackslash}p{9mm}@{\hspace{4mm}}%
    >{\raggedright\arraybackslash}p{25mm}>{\raggedright\arraybackslash}p{59mm}%
    >{\raggedleft\arraybackslash}p{17mm}@{\hspace{4mm}}%
    >{\raggedright\arraybackslash}p{38mm}@{}}%
    \arrayrulecolor{ArchiveGreen}%
    \toprule\ArchiveColumns\endfirsthead
    \toprule\ArchiveColumns\endhead
    \midrule\endfoot
    \bottomrule\endlastfoot
  }{\end{longtable}\endgroup}
\newcommand{\ArchiveNote}[1]{\par{\small\itshape #1\par}}
\RequirePackage{colortbl}
\endinput
