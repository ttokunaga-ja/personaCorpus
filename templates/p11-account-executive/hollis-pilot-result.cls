\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{hollis-pilot-result}[2025/03/18 Hollis IT pilot report]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage{fontspec}
\RequirePackage{luatexja-fontspec}
\RequirePackage[letterpaper,left=.88in,right=.88in,top=1.13in,bottom=.82in,headheight=25pt,headsep=.19in]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{fancyhdr}
\RequirePackage{titlesec}
\RequirePackage{array,longtable}
\RequirePackage[unicode,hidelinks]{hyperref}
\setmainfont{DejaVu Sans}
\setsansfont{DejaVu Sans}
\setmainjfont{Noto Sans CJK JP}
\setsansjfont{Noto Sans CJK JP}
\definecolor{HMSNavy}{HTML}{243752}
\definecolor{HMSBlue}{HTML}{3B6190}
\definecolor{HMSPale}{HTML}{E9EFF8}
\AtBeginDocument{\color{HMSNavy}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\setlength{\LTleft}{0pt}
\setlength{\LTright}{\fill}
\renewcommand{\arraystretch}{1.55}
\titleformat{\section}{\large\bfseries\color{HMSBlue}}{}{0pt}{}
\titlespacing*{\section}{0pt}{14pt}{4pt}
\pagestyle{fancy}
\fancyhf{}
\newcommand{\HMSMark}{%
  \textcolor{HMSNavy}{\rule{.64em}{.64em}}\kern.15em%
  \textcolor{HMSBlue}{\raisebox{.3em}{\rule{.39em}{.39em}}}%
  \enspace\textbf{Hollis Medical Supply Inc}}
\fancyhead[L]{\HMSMark}
\fancyhead[R]{\small\bfseries\color{HMSBlue} INFORMATION TECHNOLOGY}
\renewcommand{\headrule}{\hbox to\headwidth{\color{HMSBlue}\leaders\hrule height .6pt\hfill}}
\fancyfoot[L]{\footnotesize\color{HMSNavy} Hollis Medical Supply Inc / IT Operations}
\fancyfoot[R]{\footnotesize\color{HMSNavy} Page \thepage}
\newcommand{\HMSPilotResult}[5]{%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  {\huge\bfseries #1\par}
  \medskip
  \begingroup\setlength{\tabcolsep}{0pt}%
  \begin{tabular}{@{}p{.52\linewidth}p{.45\linewidth}@{}}
    \textbf{Report no.}\enspace #2 & \textbf{Issued}\enspace #3 \\
    \textbf{Prepared by}\enspace #4 & \textbf{To}\enspace #5
  \end{tabular}\endgroup
  \par\medskip}
\newcommand{\HMSResultHead}{%
  \rowcolor{HMSPale}\textbf{Measure} & \textbf{Seats} & \textbf{Reporting basis}\\}
\newenvironment{HMSResults}{%
  \begin{longtable}{@{}p{1.85in}>{\raggedleft\arraybackslash}p{.62in}
      @{\hspace{15pt}}>{\raggedright\arraybackslash}p{3.62in}@{}}
    \HMSResultHead\endfirsthead
    \HMSResultHead\endhead
}{\end{longtable}}
\newcommand{\HMSResult}[3]{#1 & #2 & #3\\\arrayrulecolor{HMSPale}\hline}
\endinput
