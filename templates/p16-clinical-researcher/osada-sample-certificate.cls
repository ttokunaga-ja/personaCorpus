\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{osada-sample-certificate}[2025/02/06 Sample certificate]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=32mm,bottom=27mm,left=23mm,right=23mm,headheight=25pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,fancyhdr,tabularx,hhline,hyperref}
\definecolor{OSAOchre}{HTML}{8A521C}
\definecolor{OSAInk}{HTML}{292725}
\definecolor{OSAEcru}{HTML}{F2E8D9}
\newcommand{\OSAMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-.7mm]
  \fill[OSAOchre] (0,0) rectangle (1.7,5.5);
  \fill[OSAOchre] (2.6,0) rectangle (4.3,3.7);
  \fill[OSAOchre] (5.2,0) rectangle (6.9,1.9);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[R]{\OSAMark\enspace\color{OSAOchre}\bfseries 長田研究機材株式会社}
\fancyfoot[L]{\small\color{OSAOchre}品質管理}
\fancyfoot[R]{\small\color{OSAInk}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{OSAOchre}\leaders\hrule height .5pt\hfill}\vskip 4pt}
\AtBeginDocument{\color{OSAInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\newcommand{\OSAHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \begin{flushright}\small 証明書番号\enspace #2\par
    発行日\enspace #3\par 発行担当\enspace #4
  \end{flushright}
  {\large #5\par}\vspace{3mm}
  \begin{center}\LARGE\bfseries\color{OSAOchre}#1\end{center}\medskip}
\newenvironment{OSADetails}{%
  \par\noindent
  \renewcommand{\arraystretch}{1.65}%
  \arrayrulecolor{OSAOchre}%
  \tabularx{\linewidth}{>{\raggedright\arraybackslash}p{34mm}%
    >{\raggedright\arraybackslash}X}\hhline{--}
}{\endtabularx\par\medskip}
\newcommand{\OSARow}[2]{%
  \cellcolor{OSAEcru}\leavevmode\textcolor{OSAOchre}{\textbf{#1}} & #2\\\hhline{--}}
