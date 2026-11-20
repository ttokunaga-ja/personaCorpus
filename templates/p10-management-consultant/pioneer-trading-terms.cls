\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{pioneer-trading-terms}[2025/10/10 Pioneer Buying trading terms]
\LoadClass{pioneer-exception-guidance}

\fancyhead[R]{\small\bfseries\textcolor{ptdroute}{BUYING / COMMERCIAL TERMS}}

\newcommand{\ptdtermstitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\color{ptdroute}\rule[-1.3mm]{2mm}{10mm}}\hspace{3mm}{\LARGE\bfseries #1\par}%
  \vspace{3mm}%
  \begingroup\setlength{\tabcolsep}{5pt}\renewcommand{\arraystretch}{1.5}%
  \begin{tabular}{@{}>{\raggedright\arraybackslash}p{55mm}%
    >{\raggedright\arraybackslash}p{20mm}%
    >{\raggedright\arraybackslash}p{34mm}%
    >{\raggedright\arraybackslash}p{55mm}@{}}
  \textcolor{ptdroute}{\small\bfseries Terms reference}&
    \textcolor{ptdroute}{\small\bfseries Revision}&
    \textcolor{ptdroute}{\small\bfseries Issue date}&
    \textcolor{ptdroute}{\small\bfseries Issued by}\\
  \texttt{\ptdnumber}&\ptdrevision&\ptddate&\ptdowner
  \end{tabular}\endgroup\par\vspace{2mm}}

\newcommand{\ptdscope}[3]{%
  \textbf{Supplier / account} #1\par
  \textbf{Effective window} #2\par
  \textbf{Currency and tax basis} #3\par}

\newcommand{\ptdterm}[5]{#1 & #2 & #3 & #4 & #5\\\hline}
\newcommand{\ptdtermscolumns}{%
  \rowcolor{ptdice}\bfseries Term ref.&\bfseries Item / unit&
  \bfseries Min. qty&\bfseries Unit price&\bfseries Application / settlement\\}
\newenvironment{ptdterms}{%
  \begingroup\setlength{\tabcolsep}{5pt}\setlength{\LTleft}{0pt}%
  \setlength{\LTright}{0pt plus 1fill}\renewcommand{\arraystretch}{1.45}%
  \arrayrulecolor{ptdice}%
  \begin{longtable}{@{}>{\raggedright\arraybackslash}p{24mm}%
    >{\raggedright\arraybackslash}p{36mm}%
    >{\raggedleft\arraybackslash}p{20mm}%
    >{\raggedleft\arraybackslash}p{22mm}%
    >{\raggedright\arraybackslash}p{59mm}@{}}
  \ptdtermscolumns\endfirsthead
  \ptdtermscolumns\endhead
}{\end{longtable}\endgroup}
