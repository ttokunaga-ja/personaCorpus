\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{shinonome-audit}[2026/01/01 Shinonome audit correspondence]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs}
\setmainfont{Liberation Serif}\setmainjfont{Noto Serif CJK JP}
\definecolor{shinonome}{HTML}{51465F}\definecolor{auditink}{HTML}{303038}
\AtBeginDocument{\color{auditink}}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{shinonome}\begin{tikzpicture}[baseline=-1mm]\draw[line width=.9pt](0,0)--(.38,0);\draw[line width=.7pt](0,.10)--(.38,.10);\draw[line width=.7pt](.19,.17)arc(0:180:.12);\end{tikzpicture}\quad 東雲監査法人}
\fancyhead[R]{\small 監査資料連絡}
\fancyfoot[L]{\small 東雲監査法人}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.5pt}
\newcommand{\AuditHeading}[4]{{\LARGE\bfseries\color{shinonome}#1}\par\medskip 文書番号：#2\hfill 発行日：#3\par 宛先：#4\par}
\newcommand{\AuditContact}[2]{担当：#1\hfill 確認：#2\par}
\newcommand{\AuditSection}[1]{\par\medskip{\large\bfseries\color{shinonome}#1}\par}
\newenvironment{AuditTable}[1]{\renewcommand{\arraystretch}{1.5}\begin{longtable}{#1}\toprule}{\\\bottomrule\end{longtable}}
\newcommand{\AuditNote}[1]{{\small #1}\par}
