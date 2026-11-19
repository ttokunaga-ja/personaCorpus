\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{reading-society-letterprint}[2024/07/09 Printed correspondence]
\LoadClass[12pt,a5paper]{article}
\RequirePackage[left=17mm,right=16mm,top=19mm,bottom=17mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor}
% Keep Latin curly quotes in the proportional Latin font.
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\definecolor{LetterInk}{HTML}{2D2823}
\pagestyle{empty}
\setlength{\parindent}{1.2em}
\setlength{\parskip}{4pt plus 1pt}
\AtBeginDocument{\color{LetterInk}}
\newcommand{\LetterHeading}[3]{%
  \begin{center}\large\MakeUppercase{#1}\par\vspace{3pt}%
  \rule{24mm}{0.45pt}\end{center}%
  \begin{flushright}\small #2\end{flushright}%
  \noindent #3\par}
\newcommand{\LetterClosing}[2]{%
  \par\medskip\begin{flushright}#1\\[3pt]\MakeUppercase{#2}\end{flushright}}
\endinput
