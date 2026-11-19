\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hidaka-storage}[2024/08/01 Storage management documents]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[top=32mm,bottom=24mm,left=24mm,right=24mm,headheight=24pt,headsep=8mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Serif CJK JP}
\setmainfont{Liberation Serif}
\RequirePackage[table]{xcolor}
\definecolor{HDTeal}{HTML}{195B5B}
\definecolor{HDInk}{HTML}{242E30}
\RequirePackage{tikz,fancyhdr,lastpage,array,longtable,graphicx}
\color{HDInk}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\newcommand{\HidakaMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.8mm]
    \draw[HDTeal,line width=.5mm] (0,0) rectangle (9,5.6);
    \draw[HDTeal,line width=.5mm] (0,3.6)--(9,3.6) (2.8,3.6)--(2.8,0) (6,3.6)--(6,0);
  \end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{HDTeal}{\HidakaMark\quad\bfseries 日高文書保管株式会社}}
\fancyhead[R]{\small 保管管理}
\fancyfoot[L]{\small 日高文書保管株式会社}
\fancyfoot[R]{\small\thepage\ /\ \pageref{LastPage}}
\renewcommand{\headrulewidth}{.6pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{HDTeal}\leaders\hrule height\headrulewidth\hfill}}
\newcommand{\HidakaHeading}[5]{%
  {\centering\LARGE\bfseries\color{HDTeal}#1\par}
  \par\medskip
  {\small 管理番号：#2\hfill 版：#3\par
  発行日：#4\hfill 作成者：#5\par}\medskip}
\newcommand{\HidakaSection}[1]{\par\medskip{\large\bfseries\color{HDTeal}#1\par}}
\newcolumntype{H}[1]{>{\raggedright\arraybackslash}p{#1}}
\newenvironment{HidakaInspection}[1][]{%
  \begingroup
  \def\HidakaClosingMaterial{#1}%
  \setlength{\tabcolsep}{4pt}
  \renewcommand{\arraystretch}{1.5}
  \arrayrulecolor{HDTeal}
  \begin{longtable}{|H{32mm}|H{49mm}|H{45mm}|H{22mm}|}\hline
  \rowcolor{HDTeal}\textcolor{white}{確認箇所}&\textcolor{white}{点検項目}&
  \textcolor{white}{原記録・所見}&\textcolor{white}{判定}\\\hline\endhead
}{%
  % The final detail ends with \\* so the signature row stays on the same page.
  \ifx\HidakaClosingMaterial\empty\else
    % Span the table columns and keep the rule with the closing material.
    \multispan{4}{\color{HDTeal}\leaders\hrule height\arrayrulewidth\hfill}\cr
    \noalign{\nobreak}%
    \multicolumn{4}{@{}l@{}}{%
      \begin{minipage}[t]{\dimexpr148mm+8\tabcolsep+5\arrayrulewidth\relax}
        \vspace{4pt}%
        \HidakaClosingMaterial
      \end{minipage}}\\
  \fi
  \end{longtable}\endgroup}
\newcommand{\HidakaSignature}[2]{%
  \par\medskip\noindent\begin{minipage}{.96\linewidth}
  \strut 点検者署名\quad\makebox[45mm][l]{#1}\hfill
  立会者署名\quad\makebox[45mm][l]{#2}\par
  {\color{HDTeal}\rule{\linewidth}{.4pt}}
  \end{minipage}\par}
\endinput
