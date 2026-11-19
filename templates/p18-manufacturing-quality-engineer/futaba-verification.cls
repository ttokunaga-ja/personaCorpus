\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{futaba-verification}[2026/09/01 Process verification report]
\LoadClass[a4paper,10pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\RequirePackage[margin=22mm,top=30mm,bottom=24mm,headheight=24pt]{geometry}
\RequirePackage[table]{xcolor}
\definecolor{FBurgundy}{HTML}{702C46}
\definecolor{FInk}{HTML}{342C32}
\definecolor{FBlush}{HTML}{F4E8ED}
\RequirePackage{tikz,fancyhdr,longtable,array}
\AtBeginDocument{\color{FInk}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\small 報告書}
\fancyhead[R]{\color{FBurgundy}\begin{tikzpicture}[baseline=-1pt,x=1mm,y=1mm]
  \fill (0,0) rectangle (2.6,3.0);
  \fill (3.7,0) rectangle (6.3,4.8);
\end{tikzpicture}\enspace 双葉表面処理工業株式会社}
\fancyfoot[L]{\small 双葉表面処理工業株式会社}
\fancyfoot[R]{\small\color{FBurgundy}\thepage}
\renewcommand{\headrulewidth}{.4pt}
\renewcommand{\footrulewidth}{.4pt}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\newcommand{\FutabaHeading}[5]{%
  {\raggedleft\small\color{FBurgundy}報告番号 #1\par 発行日 #2\par}%
  \vspace{5mm}%
  {\large #3\par}%
  {\centering\LARGE\bfseries\color{FBurgundy}是正後の工程・寸法確認報告\par}%
  \vspace{3mm}%
  {\small 対象 #4\par 作成 #5\par}%
  \medskip
}
\newcommand{\FutabaSection}[1]{\par\medskip{\large\bfseries\color{FBurgundy}#1}\par}
\newcommand{\FutabaNote}[1]{\par{\small #1}\par}
\newcommand{\FutabaColumns}{%
  \rowcolor{FBlush}\color{FBurgundy}製造ロット&
  \color{FBurgundy}処理記録&
  \color{FBurgundy}測定条件・確認数&
  \color{FBurgundy}結果・添付\\\hline
}
\newenvironment{FutabaLots}{%
  \renewcommand{\arraystretch}{1.55}%
  \arrayrulecolor{FBurgundy}%
  \begin{longtable}{p{25mm}p{32mm}p{42mm}p{47mm}}%
  \hline\FutabaColumns\endfirsthead
  \hline\FutabaColumns\endhead
}{\end{longtable}}
