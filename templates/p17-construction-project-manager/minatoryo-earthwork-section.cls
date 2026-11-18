\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{minatoryo-earthwork-section}[2024/10/01 Earthwork section sheet]
\LoadClass{minatoryo-field-sheet}
\geometry{landscape,left=18mm,right=18mm,top=29mm,bottom=22mm}
\AtBeginDocument{\setlength{\headwidth}{\textwidth}}
\usetikzlibrary{arrows.meta}

\newcommand{\EarthworkHeading}[6]{%
  {\LARGE\bfseries\color{MBNavy}#1\par}%
  {\small
    文書番号：#2\quad 工事名：#3\hfill 実施日：#4\par
    作成者：#5\quad 版・資料区分・縮尺：#6\par}%
}
\tikzset{
  earthwork axis/.style={MBInk,thin,densely dashed},
  earthwork existing/.style={MBNavy,thick},
  earthwork revised/.style={MBOrange,very thick},
  earthwork dimension/.style={MBInk,{Stealth[length=2mm]}-{Stealth[length=2mm]}}
}

\newenvironment{EarthworkTable}{%
  \setlength{\parskip}{0pt}%
  \setlength{\tabcolsep}{4pt}%
  \setlength{\LTleft}{0pt}%
  \begin{longtable}{|r|m{43mm}|r|r|r|m{110mm}|}\hline
  \rowcolor{MBNavy}%
  \color{white}番号 & \color{white}断面・位置（通り芯） &
  \color{white}当初（m³） & \color{white}再計算（m³） &
  \color{white}増減（m³） & \color{white}根拠・再確認事項 \\\hline
  \endhead
}{\end{longtable}}
