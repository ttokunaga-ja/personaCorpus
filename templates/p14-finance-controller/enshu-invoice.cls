\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{enshu-invoice}[2025/01/01 Enshu hardware invoice]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs,colortbl}
\setmainfont{Liberation Sans}\setmainjfont{Noto Sans CJK JP}
\definecolor{brand}{HTML}{384D6A}\definecolor{ink}{HTML}{30302C}
\AtBeginDocument{\color{ink}}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{brand}\begin{tikzpicture}[baseline=-1mm]\draw[line width=1pt](0,.14)--(.12,.28)--(.38,.28)--(.50,.14)--(.38,0)--(.12,0)--cycle;\draw(.25,.14)circle(.065);\end{tikzpicture}\quad 遠州金物}
\fancyhead[R]{\small 営業管理}
\fancyfoot[L]{\small 遠州金物 | 営業管理}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.6pt}
\newcommand{\InvoiceHeading}[5]{{\Huge\bfseries\color{brand}請求書}\par\vspace{3mm}\color{brand}\rule{\linewidth}{2pt}\color{ink}\par 番号：#1\hfill 発行日：#2\par{\large #3\quad 御中}\par 請求対象：#4\hfill 支払期限：#5\par\medskip}
\newcommand{\InvoiceTotal}[1]{\noindent\colorbox{brand}{\parbox{.94\linewidth}{\color{white}\large\strut 請求合計（税込）\hfill #1 円\strut}}\par}
\newenvironment{InvoiceTable}{\renewcommand{\arraystretch}{1.7}\begin{longtable}{p{77mm}r r r}\toprule 品名・摘要 & 数量 & 単価（円） & 金額（円）\\\midrule\endhead}{\\\bottomrule\end{longtable}}
\newcommand{\InvoiceSummary}[4]{\begin{flushright}\begin{tabular}{lr}税抜小計 & #1 円\\消費税（#2） & #3 円\\\midrule 税込合計 & #4 円\end{tabular}\end{flushright}}
\newcommand{\InvoiceNote}[1]{{\small #1}\par}
