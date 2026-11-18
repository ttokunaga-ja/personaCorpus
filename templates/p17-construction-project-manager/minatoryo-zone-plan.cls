\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{minatoryo-zone-plan}[2025/04/07 Staged work area drawing sheet]
\LoadClass{minatoryo-field-sheet}
\geometry{landscape,left=18mm,right=18mm,top=29mm,bottom=22mm}
\AtBeginDocument{\setlength{\headwidth}{\textwidth}}
\usetikzlibrary{arrows.meta,patterns}
\definecolor{MBRoom}{HTML}{DBEAE8}
\definecolor{MBBuffer}{HTML}{EEEEED}
\newcommand{\ZoneHeading}[6]{%
  {\LARGE\bfseries\color{MBNavy}#1\par}%
  {\small 工事名：#2\quad 図番・版：#3\hfill 作成日：#4\par
    作成者：#5\hfill 状態・縮尺：#6\par}\medskip
}
\tikzset{
  zone grid/.style={MBInk,thin,densely dashed},
  zone wall/.style={MBInk,thick},
  zone work/.style={pattern=north east lines,pattern color=MBOrange},
  zone open/.style={fill=MBRoom},
  zone buffer/.style={fill=MBBuffer},
  zone public route/.style={MBNavy,thick,-{Stealth[length=2mm]}},
  zone work route/.style={MBOrange,thick,dashed,-{Stealth[length=2mm]}}
}
