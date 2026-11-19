\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{whitfield-payment-procedure}[2024/04/08 Whitfield Components finance procedure]
\LoadClass[a4paper,11pt]{article}
\RequirePackage{luatexja-fontspec}
\RequirePackage[top=29mm,bottom=23mm,left=20mm,right=20mm,headheight=36pt]{geometry}
\RequirePackage[table]{xcolor}
\RequirePackage{array,longtable,fancyhdr,hyperref}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}
\setsansjfont{Noto Serif CJK JP}
\definecolor{wcgreen}{HTML}{214738}
\definecolor{wcink}{HTML}{27322E}
\definecolor{wcpale}{HTML}{E9EFEA}
\AtBeginDocument{\color{wcink}}
\hypersetup{hidelinks}
\newcommand{\wcmark}{\textcolor{wcgreen}{%
  \rule{3mm}{4mm}\hspace{0.6mm}\rule{3mm}{4mm}\hspace{0.6mm}\raisebox{2mm}{\rule{3mm}{2mm}}}}
\newcommand{\wcnumber}{}
\newcommand{\wcdate}{}
\newcommand{\wcowner}{}
\newcommand{\wccontrol}[3]{%
  \renewcommand{\wcnumber}{#1}\renewcommand{\wcdate}{#2}\renewcommand{\wcowner}{#3}%
  \hypersetup{pdfauthor={#3}}}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\textcolor{wcgreen}{\wcmark\quad{\large\bfseries Whitfield Components Ltd}}}
\fancyhead[R]{\textcolor{wcgreen}{\small JAPAN FINANCE}}
\fancyfoot[L]{\small\wcnumber\enspace\textbar\enspace\wcdate}
\fancyfoot[R]{\small Page \thepage}
\renewcommand{\headrule}{\hbox to\headwidth{\color{wcgreen}\leaders\hrule height 1.2pt\hfill}}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}
\newcommand{\wctitle}[1]{%
  \hypersetup{pdftitle={#1}}%
  {\color{wcgreen}\Huge\bfseries #1\par}\vspace{5mm}%
  \begingroup\setlength{\tabcolsep}{6pt}\renewcommand{\arraystretch}{1.4}%
  \begin{tabular}{@{}p{59mm}p{39mm}p{59mm}@{}}
  \rowcolor{wcpale}
  \textcolor{wcgreen}{\small\bfseries Procedure no.}&
  \textcolor{wcgreen}{\small\bfseries Issue date}&
  \textcolor{wcgreen}{\small\bfseries Approved by}\\
  \rowcolor{wcpale}
  \wcnumber&\wcdate&\wcowner
  \end{tabular}\endgroup\par\vspace{5mm}}
\newcommand{\wcsection}[1]{%
  \par\addvspace{4mm}{\color{wcgreen}\large\bfseries #1\par}\nopagebreak}
\newcommand{\wcstep}[3]{#1 & #2 & #3\\\hline}
\newenvironment{wcsteps}{%
  \begingroup\setlength{\tabcolsep}{7pt}\setlength{\LTleft}{0pt}%
  \setlength{\LTright}{0pt plus 1fill}\renewcommand{\arraystretch}{1.35}%
  \arrayrulecolor{wcgreen}%
  \begin{longtable}{>{\raggedleft\arraybackslash}p{9mm}>{\raggedright\arraybackslash}p{46mm}>{\raggedright\arraybackslash}p{100mm}}
  \rowcolor{wcgreen}\textcolor{white}{\bfseries No.}&\textcolor{white}{\bfseries Process step}&\textcolor{white}{\bfseries Control and handoff}\\\endfirsthead
  \rowcolor{wcgreen}\textcolor{white}{\bfseries No.}&\textcolor{white}{\bfseries Process step}&\textcolor{white}{\bfseries Control and handoff}\\\endhead
}{\end{longtable}\endgroup}
