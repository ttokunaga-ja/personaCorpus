\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{brt-customer-update}[Customer service update]
\LoadClass[11pt,a4paper]{article}
\RequirePackage[margin=23mm,headheight=25pt,headsep=17pt,footskip=28pt,includeheadfoot]{geometry}
\RequirePackage{luatexja-fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,booktabs,fancyhdr,tikz}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{BRTBlue}{HTML}{233E8B}
\definecolor{BRTOrange}{HTML}{DB703A}
\definecolor{BRTInk}{HTML}{242D3B}
\AtBeginDocument{\color{BRTInk}}
\newcommand{\BRTMark}{%
  \begin{tikzpicture}[x=.45mm,y=.45mm,baseline=-.25ex]
    \fill[BRTBlue] (0,0) rectangle (4,3);
    \fill[BRTBlue] (5.5,0) rectangle (9.5,6);
    \fill[BRTBlue] (11,0) rectangle (15,9);
    \fill[BRTOrange] (11,10) rectangle (15,11.3);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{BRTBlue}\BRTMark\hspace{2.7mm}\small\bfseries BRT Booking Systems Ltd}
\fancyfoot[L]{\color{BRTInk}\footnotesize Customer Support}
\fancyfoot[R]{\color{BRTInk}\footnotesize Page \thepage}
\renewcommand{\headrule}{\color{BRTOrange}\hrule width\headwidth height.65pt\vskip-.65pt}
\renewcommand{\footrulewidth}{.5pt}
\renewcommand{\footrule}{\color{BRTOrange}\hrule width\headwidth height.5pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\raggedbottom
\newcommand{\BRTUpdate}[4]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\color{BRTBlue}\bfseries\fontsize{25}{31}\selectfont #1\par}%
  \vspace{4pt}%
  {\small\textbf{Reference}\enspace #2\hspace{8mm}\textbf{Issued}\enspace #3\par
   \textbf{Prepared by}\enspace #4\par}%
  \vspace{8pt}}
\newcommand{\BRTSection}[1]{\section*{\color{BRTBlue}\large #1}}
\newcommand{\BRTNote}[1]{{\small\itshape #1\par}}
\newcommand{\BRTEventHead}{%
  \rowcolor{BRTBlue}%
  \color{white}\bfseries No. & \color{white}\bfseries Service checkpoint & \color{white}\bfseries Customer information\\}
\newenvironment{BRTEvents}{%
  \small\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{11mm}@{\hspace{5mm}}>{\raggedright\arraybackslash}p{48mm}>{\raggedright\arraybackslash}p{87mm}}
  \BRTEventHead\endfirsthead
  \BRTEventHead\endhead
  \arrayrulecolor{BRTBlue}\bottomrule\endlastfoot
}{\end{longtable}}
