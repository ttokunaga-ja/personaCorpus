\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{cedar-count-sheet}[2025/01/01 Cedar Works physical inventory sheet]
\LoadClass[10pt,a4paper]{ltjsarticle}
\RequirePackage[margin=20mm,headheight=30pt]{geometry}
\RequirePackage{luatexja-fontspec,fontspec,xcolor,tikz,fancyhdr,array,longtable}
\setmainfont{Noto Sans CJK JP}\setmainjfont{Noto Sans CJK JP}
\definecolor{brand}{HTML}{264D3C}\definecolor{ink}{HTML}{30302C}
\AtBeginDocument{\color{ink}}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{brand}\begin{tikzpicture}[baseline=0mm]\fill(0,0)rectangle(.09,.19);\fill(.13,0)rectangle(.22,.27);\fill(.26,0)rectangle(.35,.35);\end{tikzpicture}\quad Cedar Works 株式会社}
\fancyhead[R]{\small 静岡工場 管理課}
\fancyfoot[L]{\small Cedar Works | 静岡工場}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{0.6pt}
\newcommand{\CountHeading}[4]{{\LARGE\bfseries\color{brand}実地棚卸票}\par\medskip 管理番号：#1\hfill 実施日：#2\par 区画：#3\hfill 票番号：#4\par}
\newcommand{\CountTableWidth}{\dimexpr140mm+10\tabcolsep+6\arrayrulewidth\relax}
\NewDocumentEnvironment{CountTable}{mmm}{%
  \renewcommand{\arraystretch}{2.1}%
  \begin{longtable}{|p{27mm}|p{49mm}|>{\raggedleft\arraybackslash}p{20mm}|p{12mm}|p{32mm}|}
  \hline 品番 & 品名・仕掛工程 & 実数 & 単位 & 備考\\\hline\endhead
}{%
  % Longtable's \hline allows a break between its two copies of the rule.
  \\*\noalign{%
    \hbox to \linewidth{\hfil\vrule width \CountTableWidth height \arrayrulewidth depth 0pt\hfil}%
    \nobreak
  }%
  \multicolumn{5}{@{}p{\CountTableWidth}@{}}{%
    数えた人：#1\hfill 確認した人：#2\par
    \IfBlankF{#3}{{\small #3}\par}%
  }\\%
  \end{longtable}%
}
