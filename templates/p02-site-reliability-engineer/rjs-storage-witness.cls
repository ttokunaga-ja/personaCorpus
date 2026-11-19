\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{rjs-storage-witness}[Storage recovery witness observations]
\LoadClass[10pt,a4paper]{article}
\RequirePackage[left=22mm,right=24mm,top=18mm,bottom=18mm,headheight=28pt,headsep=12pt,footskip=24pt,includeheadfoot]{geometry}
\RequirePackage{luatexja-fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,booktabs,fancyhdr,tikz}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\definecolor{RJSTeal}{HTML}{12665E}
\definecolor{RJSInk}{HTML}{253331}
\definecolor{RJSMist}{HTML}{E5EFED}
\AtBeginDocument{\color{RJSInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\raggedbottom

\newcommand{\RJSMark}{%
  \begin{tikzpicture}[x=.63mm,y=.63mm,baseline=-.55ex]
    \fill[RJSTeal] (1,4) -- (7,7) -- (13,4) -- (7,1) -- cycle;
    \draw[RJSTeal,line width=.65pt,line join=round]
      (1,2) -- (7,-1) -- (13,2) (1,0) -- (7,-3) -- (13,0);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\RJSMark\hspace{3mm}\color{RJSTeal}\bfseries RJS Data Storage Ltd}
\fancyhead[R]{\color{RJSInk}\small Storage Assurance}
\fancyfoot[C]{\color{RJSTeal}\small\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}

\newcommand{\RJSWitnessReport}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \noindent{\color{RJSTeal}\rule{20mm}{2pt}}\par
  {\raggedright\color{RJSInk}\bfseries\fontsize{24}{29}\selectfont #1\par}%
  \vspace{5pt}%
  \begingroup
  \setlength{\fboxsep}{8pt}%
  \noindent\colorbox{RJSMist}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \small
    \textbf{Witness record}\enspace #2\hfill\textbf{Issued}\enspace #3\par
    \textbf{Witness}\enspace #4\hfill\textbf{Prepared for}\enspace #5}}\par
  \endgroup
  \vspace{5pt}}
\newcommand{\RJSSection}[1]{\section*{\color{RJSTeal}\normalsize\MakeUppercase{#1}}}
\newcommand{\RJSNote}[1]{{\small\itshape #1\par}}
\newcommand{\RJSObservationHead}{%
  \rowcolor{RJSMist}\color{RJSTeal}\bfseries No. &
  \color{RJSTeal}\bfseries Evidence read with BRT &
  \color{RJSTeal}\bfseries Witness observation\\}
\newenvironment{RJSObservations}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.4}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{11mm}
    >{\raggedright\arraybackslash}p{47mm}>{\raggedright\arraybackslash}p{92mm}}
    \RJSObservationHead\endfirsthead
    \RJSObservationHead\endhead
    \arrayrulecolor{RJSTeal}\bottomrule\endfoot
}{%
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
\newcommand{\RJSObservationNote}[1]{%
  \multicolumn{3}{>{\raggedright\arraybackslash}p{\dimexpr150mm+4\tabcolsep\relax}}{\itshape #1}\\}
