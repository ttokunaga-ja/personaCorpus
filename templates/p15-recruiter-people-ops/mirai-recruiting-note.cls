\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{mirai-recruiting-note}[2026/08/10 Recruitment notes]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[margin=24mm,top=29mm,bottom=24mm,headheight=25pt]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,tikz,fancyhdr,longtable,array,colortbl,booktabs}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{MWmain}{HTML}{963A65}
\definecolor{MWtext}{HTML}{34383C}
\definecolor{MWaccent}{HTML}{7B8270}
\definecolor{MWpale}{HTML}{F5EDF1}
\AtBeginDocument{\color{MWtext}}
\newcommand{\MWLogo}{\begin{tikzpicture}[x=.09cm,y=.09cm,baseline=-1mm]
\fill[MWmain] (0,6)--(5,4.8)--(5,0)--(0,1.2)--cycle;
\fill[MWaccent] (6,4.8)--(11,6)--(11,1.2)--(6,0)--cycle;
\end{tikzpicture}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\MWLogo\quad{\small\color{MWmain}みらい職場研修株式会社}}
\newcommand{\MWDocumentUse}{採用記録}
\fancyhead[R]{\small 人事／\MWDocumentUse}
\fancyfoot[L]{\footnotesize みらい職場研修株式会社 | 人事}
\fancyfoot[R]{\footnotesize\thepage}
\renewcommand{\headrulewidth}{.3pt}
\newcommand{\MWHeading}[5]{\noindent{\LARGE\bfseries\color{MWmain}#1}\par\medskip
{\small 文書番号：#2\quad 版：#3\par 作成日：#4\quad 作成者：#5}\par\medskip}
\newcommand{\MWSection}[1]{\par\bigskip\noindent{\large\bfseries\color{MWmain}#1}\par\smallskip}
\newcommand{\MWNote}[1]{\par\smallskip{\footnotesize\color{MWtext}#1}\par}
\newenvironment{MWTable}[1]{\renewcommand{\arraystretch}{1.25}\begin{longtable}{#1}}{\end{longtable}}
