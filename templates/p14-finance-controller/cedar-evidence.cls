\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{cedar-evidence}[2026/01/01 Cedar Works receiving and disposal records]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs}
\setmainfont{Noto Sans CJK JP}\setmainjfont{Noto Sans CJK JP}
\definecolor{cedar}{HTML}{264D3C}\definecolor{evidenceink}{HTML}{30302C}
\AtBeginDocument{\color{evidenceink}}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{cedar}\begin{tikzpicture}[baseline=0mm]\fill(0,0)rectangle(.09,.19);\fill(.13,0)rectangle(.22,.27);\fill(.26,0)rectangle(.35,.35);\end{tikzpicture}\quad Cedar Works 株式会社}
\fancyhead[R]{\small 社内保管控え}
\fancyfoot[L]{\small Cedar Works | 社内記録}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.6pt}
\newcommand{\EvidenceHeading}[4]{{\LARGE\bfseries\color{cedar}#1}\par\medskip 管理番号：#2\hfill 記録日：#3\par 記録部署：#4\par}
\newcommand{\EvidenceSection}[1]{\par\medskip{\large\bfseries\color{cedar}#1}\par}
\newenvironment{EvidenceTable}[1]{\renewcommand{\arraystretch}{1.65}\begin{longtable}{#1}\toprule}{\\\bottomrule\end{longtable}}
\newcommand{\EvidenceField}[2]{\textbf{#1}：#2\par}
\newcommand{\EvidenceNote}[1]{{\small #1}\par}
