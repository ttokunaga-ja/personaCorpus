\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{whitfield-engagement}[2026/04/13 Whitfield Japan engagement papers]
\LoadClass{whitfield-payment-procedure}

\newcommand{\wcecontrol}[5]{%
  \wccontrol{#1}{#2}{#3}%
  \fancyhead[R]{\textcolor{wcgreen}{\small JAPAN FINANCE / #4}}%
  \def\wceaudience{#5}}
\newcommand{\wcetitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\color{wcgreen}\LARGE\bfseries #1\par}\vspace{4mm}%
  \begingroup
  \setlength{\tabcolsep}{6pt}\renewcommand{\arraystretch}{1.3}%
  \noindent\begin{tabular}{p{24mm}p{119mm}}
  \rowcolor{wcpale}\textbf{Reference} & \wcnumber\\
  \rowcolor{wcpale}\textbf{Date} & \wcdate\\
  \rowcolor{wcpale}\textbf{Issued by} & \wcowner\\
  \rowcolor{wcpale}\textbf{For} & \wceaudience
  \end{tabular}\par
  \endgroup\vspace{4mm}}
\newcommand{\wcesignatures}[6]{%
  \par\noindent\begin{minipage}{\linewidth}
    \wcsection{Signed for and on behalf of}
    \begin{minipage}[t]{.47\linewidth}
      \textbf{Whitfield Components Ltd}\par
      \rule{0pt}{9mm}#3\par
      \rule{\linewidth}{0.5pt}\par
      #1\par Signed on: #2
    \end{minipage}\hfill
    \begin{minipage}[t]{.47\linewidth}
      \textbf{Holt \& Reeves Advisory Ltd}\par
      \rule{0pt}{9mm}#6\par
      \rule{\linewidth}{0.5pt}\par
      #4\par Signed on: #5
    \end{minipage}
  \end{minipage}\par}
