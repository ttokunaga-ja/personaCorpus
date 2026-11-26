\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{sunline-invoice}[2025/01/01 Sunline freight invoice]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=27pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable,booktabs}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\definecolor{sunline}{HTML}{945219}
\definecolor{sunink}{HTML}{302C27}
\AtBeginDocument{\color{sunink}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{sunline}\begin{tikzpicture}[baseline=-1mm]
  \draw[line width=.7pt] (0,0) -- (.2,.24) -- (.4,0);
  \draw[line width=.7pt] (.3,0) -- (.5,.24) -- (.7,0);
\end{tikzpicture}\quad サンライン物流}
\fancyhead[R]{\small 請求担当}
\fancyfoot[L]{\small サンライン物流｜輸送業務}
\fancyfoot[R]{\small\thepage}
\newcommand{\FreightHeading}[5]{%
  \begin{flushright}請求書番号：#1\\発行日：#2\end{flushright}%
  {\LARGE\bfseries\color{sunline} 運賃請求書}\par
  {\large #3\quad 御中}\par
  運行対象期間：#4\hfill お支払期日：#5\par\medskip
}
\newenvironment{FreightTable}{%
  \renewcommand{\arraystretch}{1.65}%
  \begin{longtable}{@{}p{75mm}p{22mm}rr@{}}%
  \toprule 運行区分・摘要 & 数量 & 税抜単価（円） & 税抜運賃（円）\\\midrule\endhead
}{\\\bottomrule\end{longtable}}
\newcommand{\FreightSummary}[4]{%
  \begin{flushright}\begin{tabular}{lr}
    運賃小計 & #1 円\\
    消費税（#2） & #3 円\\\midrule
    \bfseries ご請求額（税込） & \bfseries #4 円
  \end{tabular}\end{flushright}%
}
