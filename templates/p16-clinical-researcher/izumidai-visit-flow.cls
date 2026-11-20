\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{izumidai-visit-flow}[2024/08/21 Izumidai operations sheet]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=28mm,bottom=23mm,left=24mm,right=24mm,headheight=21pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr,tabularx,hyperref}
\definecolor{IZMBlue}{HTML}{23526B}
\definecolor{IZMPale}{HTML}{E6F0F4}
\definecolor{IZMInk}{HTML}{242E34}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\large\bfseries\color{IZMBlue}泉台研究病院}
\fancyhead[R]{\small\color{IZMBlue}事務局}
\fancyfoot[R]{\small\thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{IZMBlue}\leaders\hrule height .55pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\AtBeginDocument{\color{IZMInk}}
\newcommand{\IZMHeading}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\LARGE\bfseries\color{IZMBlue}#1\par}\medskip
  {\small 文書番号\quad #2\qquad 発行日\quad #3\par
   作成\quad #4\qquad 宛先\quad #5\par}\medskip}
\newcommand{\IZMSection}[1]{\par\medskip{\large\bfseries\color{IZMBlue}#1\par}\nobreak}
\newenvironment{IZMFlow}{%
  \par\renewcommand{\arraystretch}{1.45}%
  \tabularx{\linewidth}{@{}p{23mm}p{35mm}p{26mm}X@{}}%
  \rowcolor{IZMPale}\textbf{場所}&\textbf{段階}&\textbf{担当}&\textbf{その場で行うこと}\\
}{\endtabularx\par}
\newcommand{\IZMFlowRow}[4]{#1&#2&#3&#4\\}
