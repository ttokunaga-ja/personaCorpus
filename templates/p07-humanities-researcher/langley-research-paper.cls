\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{langley-research-paper}[2024/04/25 Langley Historical Trust research papers]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[left=26mm,right=24mm,top=29mm,bottom=23mm,headheight=16pt,headsep=8mm,footskip=11mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,graphicx,fancyhdr,titlesec,tabularx}
% Keep Latin curly quotes in the proportional Latin font.
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\definecolor{LangleyPlum}{HTML}{603346}
\definecolor{LangleyBronze}{HTML}{9A6C39}
\definecolor{LangleyInk}{HTML}{292526}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt plus 2pt}
\titleformat{\section}{\Large\bfseries\color{LangleyPlum}}{\thesection.}{0.65em}{}
\titleformat{\subsection}{\large\bfseries\color{LangleyPlum}}{}{0pt}{}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{LangleyPlum}\bfseries
  \raisebox{1pt}{\rotatebox{45}{\rule{5pt}{5pt}}}\hspace{0.65em}Langley Historical Trust}
\fancyfoot[L]{\small\color{LangleyPlum}York\hspace{0.6em}/\hspace{0.6em}Research paper}
\fancyfoot[R]{\small\color{LangleyPlum}\thepage}
\renewcommand{\headrule}{\color{LangleyBronze}\hrule width\headwidth height 0.5pt\relax}
\renewcommand{\footrule}{\color{LangleyBronze}\hrule width\headwidth height 0.5pt\relax}
\AtBeginDocument{\color{LangleyInk}\raggedright}
\newcommand{\ResearchHeading}[5]{%
  \par\noindent{\small\color{LangleyPlum}\itshape Research paper\par}%
  \vspace{4pt}{\Huge\color{LangleyPlum}#1\par}%
  \vspace{6pt}%
  {\small\setlength{\tabcolsep}{0pt}%
    \begin{tabularx}{\linewidth}{@{}>{\raggedright\arraybackslash}p{0.47\linewidth}@{\hspace{0.05\linewidth}}>{\raggedright\arraybackslash}X@{}}
      Ref.\ #2 & #3\\
      #4 & #5
    \end{tabularx}\par}%
  \vspace{8pt}}
\newcommand{\SourceNote}[1]{\par{\small\itshape #1\par}}
\newcommand{\ParallelPassage}[4]{%
  \par\medskip\noindent
  \begin{minipage}{\linewidth}
    {\large\bfseries\color{LangleyPlum}#1\par}
    \smallskip
    \begingroup
      \setlength{\tabcolsep}{0pt}%
      \begin{tabularx}{\linewidth}{@{}>{\raggedright\arraybackslash}X%
        @{\hspace{8mm}}>{\raggedright\arraybackslash}X@{}}
        \textcolor{LangleyPlum}{\bfseries Texte français} &
        \textcolor{LangleyPlum}{\bfseries English translation} \\[3pt]
        #2 & #3
      \end{tabularx}\par
    \endgroup
    \smallskip\SourceNote{#4}%
  \end{minipage}\par}
\endinput
