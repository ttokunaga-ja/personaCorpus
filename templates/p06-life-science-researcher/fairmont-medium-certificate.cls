\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{fairmont-medium-certificate}[2025/01/01 Fairmont medium certificate]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[margin=.89in,top=.88in,includefoot]{geometry}
\RequirePackage{fontspec}
\setmainfont{Liberation Sans}
\RequirePackage{xcolor,fancyhdr,tikz}
\definecolor{FCMInk}{HTML}{282D29}
\definecolor{FCMRust}{HTML}{923C24}
\definecolor{FCMSand}{HTML}{F5E7D7}
\AtBeginDocument{\color{FCMInk}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\pagestyle{fancy}
\fancyhf{}
\renewcommand{\headrulewidth}{0pt}
\fancyfoot[L]{\small\color{FCMRust}FAIRMONT / QUALITY DOCUMENTATION}
\fancyfoot[R]{\small Page \thepage}
\newcommand{\FairmontMark}{\tikz[baseline=-.22ex,x=1em,y=1em]{
  \fill[FCMRust] (0,0) rectangle (.24,1);
  \fill[FCMRust] (.34,0) rectangle (.58,.73);
  \fill[FCMRust] (.68,0) rectangle (.92,.46);}}
\newcommand{\FairmontCertificate}[3]{%
  {\Large\bfseries\FairmontMark\hspace{.5em}Fairmont Culture Materials Inc\par}%
  {\small\bfseries\color{FCMRust}QUALITY DOCUMENTATION\par}%
  \vspace{13pt}{\color{FCMRust}\rule{\linewidth}{1.2pt}}\par
  {\LARGE\bfseries Medium Preparation Certificate\par}%
  \medskip
  \begingroup\setlength{\fboxsep}{9pt}%
  \colorbox{FCMSand}{\parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
    \textbf{Certificate no.}\enspace #1\hfill\textbf{Issue date}\enspace #2\par
    \textbf{Issued by}\enspace #3}}\par\endgroup\medskip}
\newcommand{\FairmontSection}[1]{\par\medskip{\large\bfseries\color{FCMRust}#1\par}\smallskip}
\newcommand{\FairmontField}[2]{\par\textbf{#1}\enspace #2\par}
