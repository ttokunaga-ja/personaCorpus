\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{shinogaki-roof-report}[2024/08/08 Roof work submittal]
\LoadClass[a4paper,10pt]{ltjsarticle}
\RequirePackage[left=19mm,right=19mm,top=32mm,bottom=21mm,headheight=24pt,headsep=7mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,array,longtable,fancyhdr,lastpage}
\definecolor{SGGreen}{HTML}{24574C}
\definecolor{SGPale}{HTML}{E5EFEB}
\definecolor{SGInk}{HTML}{252F2D}
\AtBeginDocument{\color{SGInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyhead[L]{\color{SGGreen}\bfseries
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-0.6mm]
    \fill[SGGreen] (0,0)--(2.8,5.6)--(4.9,5.6)--(2.1,0)--cycle;
    \fill[SGGreen] (3.4,0)--(6.2,5.6)--(8.3,5.6)--(5.5,0)--cycle;
  \end{tikzpicture}\enspace 篠垣防水株式会社}
\fancyhead[R]{\small\color{SGGreen}施工部}
\fancyfoot[L]{\small\color{SGGreen}篠垣防水株式会社}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}

\newcommand{\RoofHeading}[6]{%
  {\LARGE\bfseries\color{SGGreen}#1\par}%
  \begingroup\small\renewcommand{\arraystretch}{1.65}%
  \begin{tabular}{@{}>{\columncolor{SGPale}}p{21mm}p{61mm}>{\columncolor{SGPale}}p{17mm}p{45mm}@{}}
    工事名 & #2 & 発行日 & #4 \\
    文書番号 & #3 & 作成者 & #5 \\
    版 & #6 & 提出先 & 湊梁建設株式会社
  \end{tabular}\par\endgroup\medskip
}
\newcommand{\RoofSection}[1]{\par\medskip{\large\bfseries\color{SGGreen}#1\par}}
\newenvironment{RoofWorkTable}{%
  \setlength{\LTleft}{0pt}\setlength{\tabcolsep}{4pt}%
  \renewcommand{\arraystretch}{1.45}%
  \begin{longtable}{@{}r@{\hspace{1.6em}}p{32mm}p{57mm}p{56mm}@{}}
  \rowcolor{SGPale}\bfseries 番号 & \bfseries 区画・位置 &
    \bfseries 作業・確認内容 & \bfseries 処置・記録 \\\endhead
}{\end{longtable}}
