\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{meeting-registration-detail}[2026/09/22 Registration fee statement]
\LoadClass[a4paper,11pt]{article}
\RequirePackage[top=34mm,bottom=25mm,left=23mm,right=23mm,headheight=36pt]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setsansfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\RequirePackage[table]{xcolor}
\RequirePackage{array,fancyhdr,hyperref,tikz}

\definecolor{MRTeal}{HTML}{146065}
\definecolor{MRPale}{HTML}{E6F1EF}
\definecolor{MRInk}{HTML}{263137}
\newcommand{\MRIssuer}{臨床研究運営実務会合 参加登録事務局}
\newcommand{\MRMark}{\begin{tikzpicture}[x=1mm,y=1mm,baseline=-1mm]
  \fill[MRTeal] (0,0) rectangle (1.5,6);
  \fill[MRTeal] (1.5,0) rectangle (5,1.5);
  \fill[MRTeal] (3.5,4.5) rectangle (8.5,6);
  \fill[MRTeal] (7,0) rectangle (8.5,6);
\end{tikzpicture}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\MRMark\hspace{3mm}\small\bfseries\color{MRTeal}\MRIssuer}
\fancyfoot[L]{\small\color{MRInk}参加登録・会計}
\fancyfoot[R]{\small\color{MRInk}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to\headwidth{\color{MRTeal}\leaders\hrule height .45pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\AtBeginDocument{\color{MRInk}\arrayrulecolor{MRTeal}}

\newcommand{\MRHeading}[4]{%
  \hypersetup{pdftitle={参加費明細},pdfauthor={\MRIssuer}}%
  {\setlength{\fboxsep}{0pt}%
    \colorbox{MRTeal}{\parbox{\linewidth}{\vspace{2mm}%
      \hspace*{4mm}{\color{white}\Large\bfseries 参加費明細}\par\vspace{2mm}}}\par}%
  \vspace{3mm}%
  {\small\begin{tabular}{@{}p{21mm}p{64mm}p{19mm}p{40mm}@{}}
    \textcolor{MRTeal}{明細番号} & #1 & \textcolor{MRTeal}{発行日} & #2 \\[2mm]
    \textcolor{MRTeal}{登録名} & #3 & \textcolor{MRTeal}{登録番号} & #4 \\
  \end{tabular}\par}%
  \vspace{4mm}}

\newenvironment{MRCharges}{%
  \par\noindent\renewcommand{\arraystretch}{1.6}%
  \begin{tabular}{>{\raggedright\arraybackslash}p{93mm}
    >{\centering\arraybackslash}p{24mm}>{\raggedleft\arraybackslash}p{34mm}}
  \rowcolor{MRPale}\textcolor{MRTeal}{\bfseries 費目} &
    \textcolor{MRTeal}{\bfseries 数量} & \textcolor{MRTeal}{\bfseries 金額（円）}\\\hline
}{\end{tabular}\par}
\newcommand{\MRCharge}[3]{#1 & #2 & #3\\}
\newcommand{\MRTotal}[1]{\hline\textcolor{MRTeal}{\bfseries 合計} & & \bfseries #1\\}
\newcommand{\MRNote}[2]{\par{\small\textcolor{MRTeal}{\bfseries #1}\enspace #2\par}}
