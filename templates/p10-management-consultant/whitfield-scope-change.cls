\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{whitfield-scope-change}[2025/10/09 Whitfield finance scope change]
\LoadClass{whitfield-payment-procedure}

\newcommand{\wcsbase}{}
\newcommand{\wcscontrol}[4]{%
  \wccontrol{#1}{#2}{#3}%
  \renewcommand{\wcsbase}{#4}}

\fancyhead[R]{\textcolor{wcgreen}{\small JAPAN FINANCE / CHANGE CONTROL}}
\fancyfoot[L]{\small\textcolor{wcgreen}{\wcnumber\enspace\textbar\enspace\wcdate}}

\newcommand{\wcsfield}[2]{%
  \parbox[t]{\linewidth}{\textcolor{wcgreen}{\small\bfseries #1}\par #2}}
\newcommand{\wcstitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\color{wcgreen}\LARGE\bfseries #1\par}\vspace{4mm}%
  \begingroup\setlength{\tabcolsep}{7pt}\renewcommand{\arraystretch}{1.3}%
  \noindent\begin{tabular}{p{\dimexpr.53\linewidth-2\tabcolsep\relax}%
                           p{\dimexpr.47\linewidth-2\tabcolsep\relax}}
    \rowcolor{wcpale}\wcsfield{Change reference}{\wcnumber}&
       \wcsfield{Effective date}{\wcdate}\\[3pt]
    \rowcolor{wcpale}\wcsfield{Agreement / baseline}{\wcsbase}&
       \wcsfield{Prepared by}{\wcowner}\\[3pt]
  \end{tabular}\endgroup\par\vspace{4mm}}

\newcommand{\wcssigner}[3]{%
  {\color{wcgreen}\small\bfseries #1\par}%
  \vspace{8mm}%
  \textcolor{wcgreen}{\rule{\linewidth}{0.5pt}}\par
  {\small #2\par Signed on: #3\par}}
\newcommand{\wcsapprovals}[4]{%
  \par\noindent\begin{minipage}{\linewidth}
    \wcsection{Sign-off}
    \begin{minipage}[t]{.47\linewidth}
      \wcssigner{Whitfield Components Ltd}{#1}{#2}
    \end{minipage}\hfill
    \begin{minipage}[t]{.47\linewidth}
      \wcssigner{Holt \& Reeves Advisory Ltd}{#3}{#4}
    \end{minipage}
  \end{minipage}\par}
