\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{tomorie-export}[2025/07/10 Support records]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[margin=22mm,top=29mm,bottom=23mm,headheight=35pt]{geometry}
\RequirePackage[table]{xcolor}
\definecolor{TSink}{HTML}{33313A}
\definecolor{TSviolet}{HTML}{514369}
\definecolor{TSamber}{HTML}{CC963F}
\definecolor{TSpale}{HTML}{F0EDF4}
\RequirePackage{tikz,fancyhdr,longtable,array}
\newcommand{\TSLogo}{\begin{tikzpicture}[baseline=-1mm,x=1mm,y=1mm]\draw[TSviolet,line width=.75mm] (0,0)--(3,3)--(3,6);\draw[TSviolet,line width=.75mm] (3,3)--(6,5);\fill[TSamber] (6,5) circle (1mm);\end{tikzpicture}\hspace{3mm}{\color{TSviolet}\sffamily 灯枝サービス株式会社}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\TSLogo}
\fancyhead[R]{\small サポート・サクセス\\問い合わせ記録}
\fancyfoot[L]{\footnotesize 灯枝サービス株式会社 | サポート・サクセス}
\fancyfoot[R]{\footnotesize\thepage}
\renewcommand{\headrulewidth}{.4pt}
\renewcommand{\footrulewidth}{0pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\renewcommand{\arraystretch}{1.35}
\AtBeginDocument{\color{TSink}}
\newcommand{\TSRecord}[4]{{\color{TSviolet}\LARGE\bfseries #1}\par\vspace{3mm}{\small 文書番号：#2\quad 基準日：#3}\par{\small 出力日時：#4}\par\vspace{3mm}}
\newcommand{\TSSection}[1]{\par\vspace{3mm}{\color{TSviolet}\large\bfseries #1}\par}
\newcommand{\TSColumns}[1]{\rowcolor{TSviolet}\color{white}#1}
\endinput
