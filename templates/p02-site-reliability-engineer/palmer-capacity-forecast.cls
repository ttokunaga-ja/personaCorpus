\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{palmer-capacity-forecast}[Demand forecast and capacity assessment]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[margin=22mm,headheight=28pt,headsep=14pt,footskip=26pt,includeheadfoot]{geometry}
\RequirePackage{luatexja-fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,booktabs,fancyhdr,tikz}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\setsansfont{Liberation Sans}
\setsansjfont{Noto Sans CJK JP}
\definecolor{PalmerPlum}{HTML}{532C55}
\definecolor{PalmerLilac}{HTML}{E9DFEB}
\definecolor{PalmerInk}{HTML}{302A34}
\AtBeginDocument{\color{PalmerInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\raggedbottom

\newcommand{\PalmerMark}{%
  \begin{tikzpicture}[x=.65mm,y=.65mm,baseline=-.45ex]
    \fill[PalmerPlum] (0,4) -- (4,8) -- (8,4) -- (4,0) -- cycle;
    \draw[PalmerPlum,line width=.65pt] (5,4) -- (9,8) -- (13,4) -- (9,0) -- cycle;
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\color{PalmerPlum}\PalmerMark\hspace{2.5mm}\sffamily\small\bfseries Palmer Capacity Services}
\fancyfoot[L]{\color{PalmerPlum}\sffamily\footnotesize Capacity advisory}
\fancyfoot[R]{\color{PalmerPlum}\sffamily\footnotesize Page \thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.45pt}
\renewcommand{\footrule}{\color{PalmerPlum}\hrule width\headwidth height.45pt}

\newcommand{\PalmerReport}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\raggedright\color{PalmerPlum}\bfseries\fontsize{26}{31}\selectfont #1\par}%
  \vspace{5pt}%
  \begingroup
    \setlength{\fboxsep}{8pt}%
    \noindent\colorbox{PalmerLilac}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      \sffamily\small
      \textbf{Reference}\enspace #2\hspace{10mm}\textbf{Issued}\enspace #3\par
      \textbf{Prepared by}\enspace #4\hspace{10mm}\textbf{Prepared for}\enspace #5}}\par
  \endgroup
  \vspace{7pt}}
\newcommand{\PalmerSection}[1]{\section*{\color{PalmerPlum}\large #1}}
\newcommand{\PalmerNote}[1]{{\small\itshape #1\par}}
\newcommand{\PalmerCheckHead}{%
  \rowcolor{PalmerPlum}%
  \color{white}\sffamily\bfseries No. & \color{white}\sffamily\bfseries Forecast basis & \color{white}\sffamily\bfseries Treatment or limitation\\}
\newenvironment{PalmerChecks}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.35}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{10mm}@{\hspace{6mm}}>{\raggedright\arraybackslash}p{50mm}>{\raggedright\arraybackslash}p{84mm}}
    \PalmerCheckHead\endfirsthead
    \PalmerCheckHead\endhead
    \arrayrulecolor{PalmerPlum}\bottomrule\endfoot
}{%
  % Test the page break while longtable can still repeat the heading.
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
\newcommand{\PalmerCheckNote}[1]{%
  \multicolumn{3}{>{\raggedright\arraybackslash}p{\dimexpr150mm+2\tabcolsep\relax}}{\itshape #1}\\}
