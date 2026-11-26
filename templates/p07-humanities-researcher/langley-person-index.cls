\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{langley-person-index}[2026/04/16 Langley Historical Trust person index]
\LoadClass{langley-research-paper}
\RequirePackage{longtable,booktabs,colortbl}
\definecolor{LangleyIndexTint}{HTML}{F1E8EC}
\fancyfoot[L]{\small\color{LangleyPlum}York\hspace{0.6em}/\hspace{0.6em}Person index}
\newcommand{\IndexHeading}[5]{%
  {\small\color{LangleyPlum}\itshape Person index\par}%
  \vspace{4pt}{\Huge\color{LangleyPlum}#1\par}%
  \vspace{6pt}%
  {\small
  \begin{tabularx}{\linewidth}{@{}>{\raggedright\arraybackslash}p{0.47\linewidth}@{\hspace{0.05\linewidth}}>{\raggedright\arraybackslash}X@{}}
    Ref.\ #2 & #3\\
    #4 & #5
  \end{tabularx}\par}%
  \vspace{8pt}}
\newcommand{\IndexColumns}{%
  \rowcolor{LangleyIndexTint}%
  \textcolor{LangleyPlum}{\textbf{Index entry}} &
  \textcolor{LangleyPlum}{\textbf{Source and line}} &
  \textcolor{LangleyPlum}{\textbf{Identification note}} \\ \midrule}
\newenvironment{PersonIndex}{%
  \begingroup
  \small\setlength{\tabcolsep}{2mm}\setlength{\extrarowheight}{5pt}%
  \setlength{\LTleft}{0pt}\setlength{\LTright}{0pt}%
  \arrayrulecolor{LangleyBronze}%
  \begin{longtable}{>{\raggedright\arraybackslash}p{40mm}
    >{\raggedright\arraybackslash}p{39mm}
    >{\raggedright\arraybackslash}p{69mm}}%
    \toprule\IndexColumns\endfirsthead
    \toprule\IndexColumns\endhead
    \bottomrule\endfoot
}{\end{longtable}\endgroup}
\endinput
