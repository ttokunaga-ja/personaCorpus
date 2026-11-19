\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{wallace-calibration}[2024/10/01 Calibration certificate]
\LoadClass[a4paper,11pt]{article}
\RequirePackage{fontspec}
\setmainfont{Liberation Serif}
\setmonofont{Liberation Mono}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\definecolor{WallaceNavy}{HTML}{28354D}
\definecolor{WallaceOchre}{HTML}{956018}
\definecolor{WallaceInk}{HTML}{292A2E}
\RequirePackage[margin=22mm,top=33mm,bottom=25mm,headheight=28pt]{geometry}
\RequirePackage{tikz,fancyhdr,longtable,array,booktabs,lastpage}
\AtBeginDocument{\color{WallaceInk}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{%
  \color{WallaceOchre}%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-1pt]
    \draw[line width=.7pt] (3.2,3.2) circle (3.2);
    \draw[line width=.7pt] (1.1,2.9)--(2.4,4.2)--(3.8,2.5)--(5.2,4.3);
  \end{tikzpicture}\enspace
  {\sffamily\bfseries\color{WallaceNavy}Wallace Gauge Services}%
}
\fancyhead[R]{\sffamily\small\color{WallaceNavy}Dimensional Calibration}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{0pt}
\fancyfoot[L]{\small\sffamily\color{WallaceNavy}\WallaceNumber}
\fancyfoot[R]{\small\sffamily\color{WallaceNavy}Page \thepage\ of \pageref{LastPage}}
\newcommand{\WallaceNumber}{}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\newcommand{\WallaceLabel}[1]{{\sffamily\small\bfseries\color{WallaceNavy}#1}}
\newcommand{\WallaceHeading}[5]{%
  \gdef\WallaceNumber{#1}%
  {\raggedleft\LARGE\bfseries\color{WallaceNavy}Calibration Certificate\par}%
  {\color{WallaceOchre}\rule{\linewidth}{1.25pt}\par}%
  \begin{tabular}{@{}p{.50\linewidth}p{.44\linewidth}@{}}
    \WallaceLabel{Certificate number} & \WallaceLabel{Date of issue}\\
    #1 & #2 \\[6pt]
    \WallaceLabel{Customer} & \WallaceLabel{Date of calibration}\\
    #3 & #4 \\[6pt]
    \multicolumn{2}{@{}l}{\WallaceLabel{Issued by}\quad #5}
  \end{tabular}\par\medskip
}
\newcommand{\WallaceSection}[1]{\par\medskip{\sffamily\large\bfseries\color{WallaceNavy}#1\par}}
\newcommand{\WallaceResultsHead}[1]{%
  \toprule
  \multicolumn{1}{@{}l}{\WallaceLabel{Condition}} &
  \WallaceLabel{Reference (#1)} &
  \WallaceLabel{Indicated (#1)} &
  \WallaceLabel{Error (#1)} &
  \WallaceLabel{Expanded uncertainty (#1)}\\\midrule
}
\newenvironment{WallaceResults}[1]{%
  \renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{@{}p{27mm}rrrr@{}}%
  \WallaceResultsHead{#1}\endfirsthead
  \WallaceResultsHead{#1}\endhead
  \bottomrule\endfoot
}{\end{longtable}}
