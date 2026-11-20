\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{henmere-house-folio}[Henmere House checkout documents]
\newif\ifHenmereReceipt
\DeclareOption{receipt}{\HenmereReceipttrue}
\ProcessOptions\relax
\LoadClass[10pt,a4paper]{article}
\RequirePackage[margin=24mm]{geometry}
\RequirePackage{fontspec}
\RequirePackage[table]{xcolor}
\RequirePackage{array,tabularx,booktabs}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Sans}
\definecolor{HenmereForest}{HTML}{294D3F}
\definecolor{HenmereTint}{HTML}{EAF1ED}
\setlength{\parindent}{0pt}
\setlength{\parskip}{6pt}
\pagestyle{empty}
\ifHenmereReceipt
  \geometry{paperwidth=88mm,paperheight=124mm,margin=8mm}
  \setmainfont{Liberation Mono}
\fi
\newcommand{\HenmereFolio}[5]{%
  \hypersetup{pdftitle={Guest folio #1},pdfauthor={Henmere House Accounts}}%
  {\color{HenmereForest}\fontspec{Liberation Serif}\fontsize{32}{37}\selectfont Henmere House\par}%
  {\color{HenmereForest}\rule{\linewidth}{1pt}}\par
  {\Large Guest folio\par}%
  \textbf{Folio} #1\hfill\textbf{Issued} #2\par
  \textbf{Guest} #3\par
  \textbf{Check-in} #4\hfill\textbf{Check-out} #5\par\medskip}
\newenvironment{HenmereEntries}{%
  \renewcommand{\arraystretch}{1.4}%
  \begin{tabular}{@{}l>{\raggedright\arraybackslash}p{61mm}rr@{}}%
  \rowcolor{HenmereTint}\textbf{Date} & \textbf{Description / reference}
  & \textbf{Charge (GBP)} & \textbf{Payment (GBP)}\\
}{\end{tabular}\par}
\newcommand{\HenmereCardReceipt}[5]{%
  \hypersetup{pdftitle={Merchant card receipt #1},pdfauthor={Henmere House}}%
  \begin{center}\bfseries Henmere House\\Merchant card receipt\end{center}%
  \rule{\linewidth}{.5pt}\par
  Receipt: #1\par
  Date: #2\par
  Folio: #3\par
  \noindent\begin{tabularx}{\linewidth}{@{}Xr@{}}
    Purchase & GBP #4\\
  \end{tabularx}\par
  #5\par
  \rule{\linewidth}{.5pt}\par}
