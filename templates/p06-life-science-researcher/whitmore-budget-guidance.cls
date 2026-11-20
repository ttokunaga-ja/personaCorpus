\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{whitmore-budget-guidance}[2025/11/07 Whitmore applicant guidance]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[top=1in,bottom=.86in,left=1.04in,right=1.04in,includefoot]{geometry}
\RequirePackage{fontspec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\RequirePackage{xcolor,tikz,fancyhdr,needspace}
\definecolor{WhitmoreInk}{HTML}{293A31}
\definecolor{WhitmoreGreen}{HTML}{1D5748}
\definecolor{WhitmoreGold}{HTML}{AD843F}
\AtBeginDocument{\color{WhitmoreInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyfoot[L]{\small\sffamily\color{WhitmoreGreen}WHITMORE SCIENCE FUND / GRANTS OFFICE}
\fancyfoot[R]{\small\sffamily\thepage}
\newcommand{\WhitmoreMark}{\tikz[baseline=-.18ex,x=1em,y=1em]{%
  \draw[WhitmoreGreen,line width=.85pt] (0,.04) -- (0,.45) arc[start angle=180,end angle=0,radius=.45] -- (.9,.04);
  \draw[WhitmoreGreen,line width=.85pt] (.24,.04) -- (.24,.45) arc[start angle=180,end angle=0,radius=.21] -- (.66,.04);
  \draw[WhitmoreGold,line width=1.2pt] (-.12,0) -- (1.02,0);}}
\newcommand{\WhitmoreGuidance}[4]{%
  {\raggedleft\large\sffamily\bfseries\color{WhitmoreGreen}%
    \WhitmoreMark\hspace{.6em}Whitmore Science Fund\par}%
  \vspace{9pt}{\color{WhitmoreGold}\rule{\linewidth}{.9pt}\par}%
  \vspace{12pt}{\fontsize{27}{31}\selectfont\bfseries\raggedright #1\par}%
  {\small\sffamily
    \textbf{Guidance ref.}\enspace #2\hfill\textbf{Issued}\enspace #3\par
    \textbf{Grants officer}\enspace #4\par}%
  \medskip}
\newcommand{\WhitmoreSection}[1]{%
  \par\Needspace{4\baselineskip}\medskip
  {\large\bfseries\color{WhitmoreGreen}#1\par}\smallskip}
