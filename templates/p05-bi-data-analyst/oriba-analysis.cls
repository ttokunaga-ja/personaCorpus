\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{oriba-analysis}[2025/10/01 Oriba analysis brief]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=21mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,longtable,booktabs,colortbl}
\setmainfont{Liberation Sans}\setmainjfont{Noto Sans CJK JP}
\definecolor{oriba}{HTML}{304E67}\definecolor{accent}{HTML}{C5614F}\definecolor{pale}{HTML}{EAF0F3}
\newcommand{\OribaMark}{\begin{tikzpicture}[baseline=-1mm,scale=.55]\fill[oriba](.12,0)rectangle(.26,.72);\fill[oriba](.48,0)rectangle(.62,.72);\fill[accent](.04,.14)rectangle(.76,.28);\fill[accent](.04,.46)rectangle(.76,.60);\fill[oriba](.48,.14)rectangle(.62,.28);\fill[oriba](.12,.46)rectangle(.26,.60);\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}\fancyhead[L]{\OribaMark\quad\color{oriba}織庭生活雑貨株式会社}\fancyhead[R]{\small 分析報告}
\fancyfoot[L]{\small 営業企画部・分析室\quad |\quad 社内資料}\fancyfoot[R]{\thepage}
\renewcommand{\headrulewidth}{.5pt}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\newcommand{\AnalysisHeading}[5]{{\LARGE\bfseries\color{oriba}#1}\par{\small 文書番号：#2\quad 版：#3\par 作成・確認：#4\quad 日付：#5}\par\vspace{3mm}}
\newcommand{\AnalysisSection}[1]{\par\vspace{3mm}{\large\bfseries\color{oriba}#1}\par}
\newcommand{\AnalysisNote}[1]{\par{\small #1}\par}
\newenvironment{AnalysisTable}[1]{\renewcommand{\arraystretch}{1.4}\begin{longtable}{#1}\toprule}{\\\bottomrule\end{longtable}}
