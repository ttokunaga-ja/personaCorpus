\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{winslow-work-scope}[2024/11/01 Processing work scope]
\LoadClass{winslow-subprocessor-notice}
\RequirePackage{array,longtable,colortbl}
\setlength{\tabcolsep}{6pt}
\renewcommand{\arraystretch}{1.3}

\newcommand{\WinslowScope}[6]{%
  {\large\bfseries\color{WRBlue}\WinslowMark\enspace Winslow Records Services\par}
  {\color{WRGreen}\rule{\linewidth}{1.2pt}\par}
  \medskip
  {\huge\bfseries\color{WRBlue}#1\par}
  {\small Reference: #2\quad Revision: #3\par
   Issued: #4\quad Prepared by: #5\par
   Prepared for: #6\par}
  \medskip}

\newenvironment{WinslowSchedule}{%
  \begin{longtable}{%
    >{\raggedleft\arraybackslash}p{9mm}%
    >{\raggedright\arraybackslash}p{37mm}%
    >{\raggedright\arraybackslash}p{46mm}%
    >{\raggedright\arraybackslash}p{52mm}}
  \rowcolor{WRBlue}
  \color{white}\bfseries No.&
  \color{white}\bfseries Activity&
  \color{white}\bfseries Handoff or return&
  \color{white}\bfseries Schedule treatment\\\endhead
}{\end{longtable}}
