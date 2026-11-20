\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{futaba-process}[2025/04/07 Process explanation and record forms]
\LoadClass{futaba-verification}
\fancyhead[L]{\small 工程・記録}

\newcommand{\FutabaProcessHeading}[6]{%
  {\raggedleft\small\color{FBurgundy}#2\par #3\par}%
  \vspace{5mm}%
  {\large #4\par}%
  {\centering\LARGE\bfseries\color{FBurgundy}#1\par}%
  \vspace{3mm}%
  {\small 対象 #5\par #6\par}%
  \medskip
}

\newcommand{\FutabaRecordHeading}[3]{%
  {\raggedleft\small\color{FBurgundy}#2\par}%
  {\centering\Large\bfseries\color{FBurgundy}#1\par}%
  {\small #3\par}%
  \medskip
}
