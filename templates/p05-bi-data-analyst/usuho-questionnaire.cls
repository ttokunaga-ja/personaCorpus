\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{usuho-questionnaire}[2026/08/26 Usuho Research questionnaire]
\LoadClass[a4paper,11pt]{ltjsarticle}
\RequirePackage[left=25mm,right=25mm,top=30mm,bottom=25mm,headheight=22pt,headsep=10pt]{geometry}
\RequirePackage{luatexja-fontspec,xcolor,tikz,fancyhdr,tabularx}
\RequirePackage[hidelinks]{hyperref}
\setmainfont{Liberation Serif}
\setmainjfont{Noto Serif CJK JP}

\definecolor{UsuhoBrown}{HTML}{49392E}
\definecolor{UsuhoOchre}{HTML}{976015}
\definecolor{UsuhoSand}{HTML}{F5EDDF}
\AtBeginDocument{\color{UsuhoBrown}\raggedbottom}
\setlength{\parindent}{0pt}
\setlength{\parskip}{5pt}

\newcommand{\UsuhoMark}{%
  \begin{tikzpicture}[x=1mm,y=1mm,baseline=-.75mm]
    \fill[UsuhoOchre] (0,1.25)--(3,5.5)--(3,1.25)--cycle;
    \fill[UsuhoBrown] (3.6,1.25)--(3.6,4.25)--(5.9,1.25)--cycle;
    \draw[UsuhoBrown,line width=.6pt] (0,0)--(6.5,0);
  \end{tikzpicture}%
}
\newcommand{\UsuhoDocumentNumber}{}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\color{UsuhoBrown}\UsuhoMark\enspace\bfseries 薄帆リサーチ合同会社}
\fancyhead[R]{\small\color{UsuhoBrown}設問票}
\fancyfoot[L]{\small\color{UsuhoBrown}\UsuhoDocumentNumber}
\fancyfoot[R]{\small\color{UsuhoBrown}\thepage}
\renewcommand{\headrulewidth}{0pt}
\renewcommand{\footrule}{\hbox to \headwidth{\color{UsuhoOchre}\leaders\hrule height .5pt\hfill}\vskip 3pt}

\newcommand{\UsuhoHeading}[4]{%
  \gdef\UsuhoDocumentNumber{#2}%
  \hypersetup{pdftitle={#1},pdfauthor={#4}}%
  \par\noindent{\LARGE\bfseries #1\par}\nobreak\medskip
  \begingroup
    \setlength{\fboxsep}{3mm}%
    \noindent\colorbox{UsuhoSand}{%
      \parbox{\dimexpr\linewidth-2\fboxsep\relax}{%
        \small\renewcommand{\arraystretch}{1.2}%
        \begin{tabularx}{\linewidth}{@{}p{22mm}X@{}}
          文書番号 & #2\\
          発行日 & #3\\
          作成者 & #4
        \end{tabularx}%
      }%
    }\par
  \endgroup\medskip
}

\newenvironment{UsuhoQuestion}[2]{%
  \par\addvspace{9pt}\noindent
  \begin{minipage}{\linewidth}
    \begin{list}{}{%
      \setlength{\labelwidth}{13mm}%
      \setlength{\labelsep}{4mm}%
      \setlength{\leftmargin}{17mm}%
      \setlength{\topsep}{0pt}%
    }%
      \item[{\color{UsuhoOchre}\bfseries #1}] \textbf{#2}\par
}{%
    \end{list}%
  \end{minipage}\par
}

\newenvironment{UsuhoOptions}{%
  \begin{list}{}{%
    \setlength{\labelwidth}{8mm}%
    \setlength{\labelsep}{4mm}%
    \setlength{\leftmargin}{12mm}%
    \setlength{\topsep}{3pt}%
    \setlength{\itemsep}{1pt}%
    \setlength{\parsep}{0pt}%
  }%
}{\end{list}}
\newcommand{\UsuhoOption}[2]{\item[\textbf{#1}] #2}
