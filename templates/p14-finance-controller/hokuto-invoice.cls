\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hokuto-invoice}[2025/01/01 Hokuto plywood invoice]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs,colortbl}
\setmainfont{Liberation Serif}\setmainjfont{Noto Serif CJK JP}
\definecolor{brand}{HTML}{654B35}\definecolor{ink}{HTML}{30302C}
\AtBeginDocument{\color{ink}}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{brand}\begin{tikzpicture}[baseline=-1mm]\draw[line width=.7pt](0,0)rectangle(.55,.28);\draw(0,.09)--(.55,.09)(0,.18)--(.55,.18);\end{tikzpicture}\quad 北斗合板}
\fancyhead[R]{\small 請求管理}
\fancyfoot[L]{\small 北斗合板 | 請求管理}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.6pt}
\newcommand{\InvoiceHeading}[5]{\begin{flushright}請求書番号：#1\\発行日：#2\end{flushright}\begin{center}{\LARGE\bfseries\color{brand}請 求 書}\end{center}{\large #3\quad 御中}\par 対象：#4\hfill お支払期日：#5\par\medskip}
\newcommand{\InvoiceTotal}[1]{\begin{flushright}\fbox{\rule[-3mm]{0pt}{10mm}\large ご請求金額（税込）\quad #1 円}\end{flushright}}
\newenvironment{InvoiceTable}{\renewcommand{\arraystretch}{1.7}\begin{longtable}{p{77mm}r r r}\toprule 品名・摘要 & 数量 & 単価（円） & 金額（円）\\\midrule\endhead}{\\\bottomrule\end{longtable}}
\newcommand{\InvoiceSummary}[4]{\begin{flushright}\begin{tabular}{lr}税抜小計 & #1 円\\消費税（#2） & #3 円\\\midrule 税込合計 & #4 円\end{tabular}\end{flushright}}
\newcommand{\InvoiceNote}[1]{{\small #1}\par}
