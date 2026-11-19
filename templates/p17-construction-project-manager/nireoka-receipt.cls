\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{nireoka-receipt}[2024/08/08 Facility receipt]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[left=24mm,right=24mm,top=32mm,bottom=24mm,headheight=25pt,headsep=8mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Serif}
\RequirePackage[table]{xcolor}
\RequirePackage{tikz,array,longtable,fancyhdr,lastpage}
\definecolor{NWBrown}{HTML}{544437}
\definecolor{NWRule}{HTML}{8E7969}
\definecolor{NWInk}{HTML}{282725}
\AtBeginDocument{\color{NWInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyhead[C]{\color{NWBrown}\bfseries
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-0.5mm]
    \draw[NWBrown,line width=0.65pt] (0,0) rectangle (6,5);
    \draw[NWRule,line width=0.65pt] (1.1,1.7)--(4.9,1.7) (1.1,3.3)--(4.9,3.3);
  \end{tikzpicture}\enspace 楡岡倉庫株式会社}
\fancyfoot[L]{\small\color{NWBrown}施設課}
\fancyfoot[R]{\small\thepage\ / \pageref{LastPage}}

\newcommand{\ReceiptHeading}[6]{%
  {\centering\LARGE\bfseries\color{NWBrown}#1\par}\medskip
  {\small
    \begin{tabular}{@{}p{20mm}p{64mm}p{19mm}p{42mm}@{}}
      受領番号 & #2 & 受領日 & #4 \\
      工事名 & #3 & 受領担当 & #5 \\
      提出者 & #6 & 所管 & 施設課
    \end{tabular}\par}%
  {\color{NWRule}\hrule height 0.55pt}\medskip
}
\newcommand{\ReceiptSection}[1]{\par\medskip{\large\bfseries\color{NWBrown}#1\par}}
\newenvironment{ReceiptList}{%
  \setlength{\LTleft}{0pt}\arrayrulecolor{NWRule}%
  \renewcommand{\arraystretch}{1.45}%
  \begin{longtable}{@{}r@{\hspace{1.6em}}p{67mm}p{56mm}@{}}\hline
  \textcolor{NWBrown}{\bfseries 番号} & \textcolor{NWBrown}{\bfseries 受領品目} &
    \textcolor{NWBrown}{\bfseries 文書番号・版} \\\hline\endhead
}{\end{longtable}}
