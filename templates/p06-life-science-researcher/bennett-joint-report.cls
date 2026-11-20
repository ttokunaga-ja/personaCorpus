\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{bennett-joint-report}[2026/09/09 Bennett joint report]
\LoadClass{bennett-bench-record}
\RequirePackage{graphicx,caption,titlesec}
\titleformat{\section}{\large\bfseries\color{BCEIris}}{\thesection}{.65em}{}
\titleformat{\subsection}{\normalsize\bfseries\color{BCEIris}}{\thesubsection}{.65em}{}
\DeclareCaptionFont{bceink}{\color{BCEInk}}
\DeclareCaptionFont{bceiris}{\color{BCEIris}}
\captionsetup{font={small,bceink},labelfont={bf,bceiris},
  justification=raggedright,singlelinecheck=false,skip=6pt}
\fancyfoot[L]{\small JOINT REPORT}
\newcommand{\BennettJointReport}[5]{%
  {\fontsize{24}{29}\selectfont\bfseries #1\par}\vspace{4pt}%
  \setlength{\fboxsep}{8pt}%
  \colorbox{BCETint}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Document}\enspace #2\hfill \textbf{Revision date}\enspace #3\par
    \textbf{Prepared by}\enspace #4\hfill \textbf{For review by}\enspace #5}}%
  \par\vspace{5pt}}
