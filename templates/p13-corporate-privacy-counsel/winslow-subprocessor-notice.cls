\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{winslow-subprocessor-notice}[2024/10/18 Subprocessor notice]
\LoadClass[letterpaper,11pt]{article}
\RequirePackage[top=27mm,bottom=23mm,left=25mm,right=25mm]{geometry}
\RequirePackage{fontspec,xcolor,tikz,fancyhdr,lastpage}
\setmainfont{Liberation Sans}
\definecolor{WRBlue}{HTML}{234B69}
\definecolor{WRGreen}{HTML}{16736C}
\definecolor{WRInk}{HTML}{252E35}
\color{WRInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\raggedright
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyfoot[L]{\small\textcolor{WRBlue}{Winslow Records Services}}
\fancyfoot[R]{\small\thepage\ of \pageref{LastPage}}
\newcommand{\WinslowMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.5mm]
    \fill[WRBlue] (0,1) rectangle (2,7);
    \fill[WRGreen] (3,1) rectangle (5,9);
    \fill[WRBlue] (6,1) rectangle (8,6);
  \end{tikzpicture}}
\newcommand{\WinslowNotice}[5]{%
  {\large\bfseries\color{WRBlue}\WinslowMark\enspace Winslow Records Services\par}
  {\color{WRGreen}\rule{\linewidth}{1.2pt}\par}
  \medskip
  {\huge\bfseries\color{WRBlue}#1\par}
  {\small
   Notice reference: #2\quad Date issued: #3\par
   Issued by: #4\par
   Prepared for: #5\par}
  \medskip}
\newcommand{\WinslowSection}[1]{\par\medskip{\large\bfseries\color{WRBlue}#1\par}}
