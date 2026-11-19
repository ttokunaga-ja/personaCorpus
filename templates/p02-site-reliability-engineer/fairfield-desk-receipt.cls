\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fairfield-desk-receipt}[Staffed desk exercise receipt record]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[margin=22mm,headheight=26pt,headsep=15pt,footskip=25pt,includeheadfoot]{geometry}
\RequirePackage{luatexja-fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,booktabs,fancyhdr,tikz}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{FairfieldWine}{HTML}{762E39}
\definecolor{FairfieldPaper}{HTML}{F4EAE5}
\definecolor{FairfieldInk}{HTML}{362E2C}
\AtBeginDocument{\color{FairfieldInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\raggedbottom

\newcommand{\FairfieldMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.35ex]
    \fill[FairfieldWine,rounded corners=.5mm] (0,0) rectangle (12,6);
    \fill[white] (0,3) circle (.8mm) (12,3) circle (.8mm);
    \draw[white,densely dotted,line width=.65pt] (8,1) -- (8,5);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{FairfieldWine}\large\bfseries Fairfield Ticket Services Ltd}
\fancyhead[R]{\FairfieldMark}
\fancyfoot[L]{\color{FairfieldInk}\footnotesize Ticket Office Operations}
\fancyfoot[R]{\color{FairfieldWine}\footnotesize\bfseries Page \thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.45pt}
\renewcommand{\footrule}{\color{FairfieldWine}\hrule width\headwidth height.45pt}

\newcommand{\FairfieldReceipt}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\raggedright\hyphenpenalty=10000\relax
   \color{FairfieldWine}\bfseries\fontsize{23}{28}\selectfont #1\par}%
  \vspace{5pt}%
  \begingroup
    \setlength{\fboxsep}{8pt}%
    \noindent\colorbox{FairfieldPaper}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
      \raggedright\small
      \textbf{Record}\enspace #2\hspace{10mm}\textbf{Issued}\enspace #3\par
      \textbf{Recorded by}\enspace #4\hspace{10mm}\textbf{Exercise}\enspace #5}}\par
  \endgroup}
\newcommand{\FairfieldSection}[1]{\section*{\color{FairfieldWine}\large #1}}
\newcommand{\FairfieldNote}[1]{{\small\itshape #1\par}}
\newcommand{\FairfieldReceiptHeading}{%
  \rowcolor{FairfieldPaper}%
  \bfseries No. & \bfseries Staffed desk & \bfseries Receipt (UTC) & \bfseries Record or exception\\}
\newenvironment{FairfieldReceipts}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.42}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{9mm}@{\hspace{6mm}}%
    >{\raggedright\arraybackslash}p{37mm}>{\raggedright\arraybackslash}p{27mm}%
    >{\raggedright\arraybackslash}p{72mm}}
    \FairfieldReceiptHeading\endfirsthead
    \FairfieldReceiptHeading\endhead
    \arrayrulecolor{FairfieldWine}\bottomrule\endfoot
}{%
  \noalign{\nobreak\vskip0pt\penalty0}%
  \end{longtable}}
\newcommand{\FairfieldReceiptNote}[1]{%
  \multicolumn{4}{>{\raggedright\arraybackslash}p{\dimexpr151mm+4\tabcolsep\relax}}{\itshape #1}\\}
