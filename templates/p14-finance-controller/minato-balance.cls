\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{minato-balance}[2026/03/01 Minato balance certificate]
\LoadClass[11pt,a4paper]{ltjsarticle}
\RequirePackage[margin=25mm,headheight=25pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,booktabs}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\definecolor{bank}{HTML}{783B45}
\definecolor{ink}{HTML}{292929}
\AtBeginDocument{\color{ink}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{bank}\begin{tikzpicture}[baseline=-1mm]\draw[line width=1pt] (0,0) circle (.18);\draw[line width=1pt](-.13,-.1)--(0,.13)--(.13,-.1);\end{tikzpicture}\quad みなと中央銀行}
\fancyhead[R]{\color{ink}\small 横浜支店}
\fancyfoot[C]{\color{ink}\small みなと中央銀行 横浜支店\quad\thepage}
\renewcommand{\headrulewidth}{0.4pt}
\renewcommand{\headrule}{\hbox to\headwidth{\color{ink}\leaders\hrule height \headrulewidth\hfill}}
\setlength{\parindent}{0pt}\setlength{\parskip}{8pt}
\newcommand{\CertificateHeading}[4]{\begin{flushright}証明番号：#1\\発行日：#2\end{flushright}\vspace{5mm}{\large #3\quad 御中}\par\vspace{6mm}\begin{center}{\LARGE\bfseries 残高証明書}\end{center}\vspace{3mm}#4 現在の下記預金残高を証明いたします。\par}
\newenvironment{BalanceTable}{\begin{center}\renewcommand{\arraystretch}{1.8}\begin{tabular}{p{38mm}p{43mm}r}\toprule 預金種類 & 口座（下4桁） & 残高（円）\\\midrule}{\\\bottomrule\end{tabular}\end{center}}
\newcommand{\BankSignature}[1]{\vfill\begin{flushright}みなと中央銀行 横浜支店\\担当：#1\end{flushright}}
