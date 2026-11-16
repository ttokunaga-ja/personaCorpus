\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{ledgerline-report}[2026/03/01 Ledgerline financial report]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=18mm,headheight=28pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs,colortbl}
\setmainfont{Liberation Sans}\setmainjfont{Noto Sans CJK JP}
\definecolor{cedar}{HTML}{264D3C}\definecolor{ochre}{HTML}{AE8244}\definecolor{pale}{HTML}{E8EEE9}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{cedar}\begin{tikzpicture}[baseline=0mm]\fill(0,0)rectangle(.09,.19);\fill(.13,0)rectangle(.22,.27);\fill(.26,0)rectangle(.35,.35);\end{tikzpicture}\quad Cedar Works 株式会社}
\fancyhead[R]{\small Ledgerline | 経理財務部}
\fancyfoot[L]{\small Cedar Works • Finance}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.7pt}\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\newcommand{\ReportHeading}[4]{{\Large\bfseries\color{cedar}#1}\par{\small 帳票番号：#2\quad 基準日：#3\par 出力：#4}\par\vspace{3mm}}
\newcommand{\ReportNote}[1]{{\small\color{cedar}#1}\par}
\newenvironment{LedgerTable}[1]{\renewcommand{\arraystretch}{1.45}\begin{longtable}{#1}\toprule}{\\\bottomrule\end{longtable}}
