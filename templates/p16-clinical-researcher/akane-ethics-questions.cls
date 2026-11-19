\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{akane-ethics-questions}[2024/09/30 Ethics review inquiries]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=32mm,bottom=25mm,left=23mm,right=23mm,headheight=23pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage{xcolor,tikz,fancyhdr,tabularx,enumitem,hyperref}
\definecolor{AKEGreen}{HTML}{345C47}
\definecolor{AKEPaper}{HTML}{E9EFE9}
\definecolor{AKEInk}{HTML}{29332D}
\newcommand{\AKEMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.7mm]
  \fill[AKEGreen] (0,0)--(0,4.4)--(3,2.2)--cycle;
  \fill[AKEGreen] (6.6,0)--(6.6,4.4)--(3.6,2.2)--cycle;
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{AKEGreen}\AKEMark\enspace\large\bfseries あかね研究倫理協議会}
\fancyfoot[L]{\small\color{AKEGreen}審査事務局}
\fancyfoot[R]{\small\color{AKEGreen}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{AKEGreen}\leaders\hrule height .5pt\hfill}\vskip 4pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\AtBeginDocument{\color{AKEInk}}
\newcommand{\AKEHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \noindent\colorbox{AKEPaper}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \vspace{6pt}\hspace{7pt}{\LARGE\bfseries\color{AKEGreen}#1}\par\vspace{6pt}}}\par
  \begin{tabularx}{\linewidth}{@{}lX@{}}
  照会番号 & #2\\
  発行日 & #3\\
  担当 & #4\\
  宛先 & #5
  \end{tabularx}\par\medskip}
\newlist{AKEQuestions}{enumerate}{1}
\setlist[AKEQuestions]{label=\textcolor{AKEGreen}{\bfseries\arabic*.},leftmargin=9mm,itemsep=8pt}
