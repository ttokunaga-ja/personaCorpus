\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{merecairn-equipment-sheet}[Equipment general arrangement and interface notes]
\LoadClass[a4paper,landscape,10pt]{article}
\RequirePackage[margin=18mm,headheight=21pt,headsep=6mm]{geometry}
\RequirePackage{fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{array,fancyhdr,tabularx,tikz}
\definecolor{MEInk}{HTML}{232945}
\definecolor{MEAccent}{HTML}{574681}
\definecolor{MEPanel}{HTML}{EEEBF5}
\AtBeginDocument{\color{MEInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\newcommand{\MerecairnMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-0.5mm]
    \fill[MEAccent] (0,0)--(2.3,5.5)--(4.6,0)--cycle;
    \fill[MEInk] (3.2,0)--(5.5,5.5)--(7.8,0)--cycle;
  \end{tikzpicture}%
}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\MerecairnMark\enspace\textbf{Merecairn Equipment Ltd}}
\fancyhead[R]{\small Technical documentation}
\fancyfoot[L]{\small Merecairn Equipment Ltd}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.4pt}
\newcommand{\EquipmentHeading}[5]{%
  {\LARGE\bfseries #1\par}
  \begingroup
  \small\renewcommand{\arraystretch}{1.35}%
  \begin{tabularx}{\linewidth}{@{}XXX@{}}
    \rowcolor{MEPanel}
    \textbf{Project}\quad #2 & \textbf{Revision}\quad #3 & \textbf{Issue date}\quad #4 \\
    \multicolumn{3}{@{}l@{}}{\textbf{Issued by}\quad #5}
  \end{tabularx}%
  \endgroup\par\medskip
}
\newcommand{\EquipmentSection}[1]{%
  \par\medskip{\large\bfseries\color{MEAccent}#1}\par
}
