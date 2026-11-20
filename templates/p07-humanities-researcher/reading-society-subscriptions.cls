\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{reading-society-subscriptions}[2026/01/14 Printed subscription accounts]
\LoadClass[11pt,a5paper]{article}
\RequirePackage[margin=15mm]{geometry}
\RequirePackage{fontspec,luatexja-fontspec,xcolor,longtable,array,booktabs}
\ltjdefcharrange{10}{"2018-"2019,"201C-"201D}
\ltjsetparameter{jacharrange={-10}}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\definecolor{AccountInk}{HTML}{2D2823}
\pagestyle{empty}
\AtBeginDocument{\color{AccountInk}}
\newcommand{\AccountHeading}[3]{%
  \begin{center}
    \large\scshape #1\par\vspace{3pt}%
    \rule{25mm}{0.45pt}\par\smallskip
    \normalsize\upshape\bfseries #2\par
    \small\mdseries #3
  \end{center}}
\newcommand{\AccountColumns}{%
  \textbf{N\textsuperscript{o}} & \textbf{Reçu le} & \textbf{Nom inscrit} &
  \textbf{Francs} & \textbf{Objet du versement} \\ \midrule}
\newenvironment{SubscriptionEntries}{%
  \begingroup\setlength{\extrarowheight}{4pt}\small
  \begin{longtable}{@{}r@{\hspace{4mm}}p{21mm}p{28mm}r@{\hspace{4mm}}%
    >{\raggedright\arraybackslash}p{36mm}@{}}%
    \toprule\AccountColumns\endfirsthead
    \toprule\AccountColumns\endhead
    \bottomrule\endfoot
}{\end{longtable}\endgroup}
\newcommand{\AccountClosing}[1]{\begin{flushright}\small #1\end{flushright}}
\endinput
