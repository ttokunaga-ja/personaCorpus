\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{mason-service-record}[2024/04/01 Mason field service record]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[margin=.9in, top=.8in]{geometry}
\RequirePackage{fontspec,xcolor,fancyhdr,tikz}
\setmainfont{Liberation Mono}
\definecolor{MasonInk}{HTML}{26291F}
\definecolor{MasonOlive}{HTML}{515A34}
\definecolor{MasonPaper}{HTML}{F0EDDF}
% Allow the unused space at the right of each monospaced line to stretch.
\AtBeginDocument{\color{MasonInk}\raggedright}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrulewidth}{.5pt}
\fancyfoot[L]{\small FIELD SERVICE OPERATIONS}
\fancyfoot[R]{\small \thepage}
\newcommand{\MasonMark}{\tikz[baseline=-.1ex,x=.82em,y=.82em]{
  \fill[white] (0,0) rectangle (.25,1);
  \fill[white] (.31,0) rectangle (.56,.76);
  \fill[white] (.62,0) rectangle (.87,.52);}}
\newcommand{\MasonRecord}[4]{%
  \setlength{\fboxsep}{10pt}%
  \colorbox{MasonOlive}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \color{white}\bfseries\MasonMark\enspace MASON LABORATORY MAINTENANCE}}\par
  \vspace{.20in}{\Large\bfseries #1}\par
  \colorbox{MasonPaper}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Service no.}\enspace #2\par
    \textbf{Service date}\enspace #3\par
    \textbf{Issued by}\enspace #4}}\par\vspace{.14in}}
\newcommand{\MasonSection}[1]{\par\medskip{\color{MasonOlive}\bfseries\MakeUppercase{#1}}\par}
