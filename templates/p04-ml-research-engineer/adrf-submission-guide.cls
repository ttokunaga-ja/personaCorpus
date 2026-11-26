\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{adrf-submission-guide}[2025/02/03 Applied Document Research Forum submission circular]
\LoadClass[11pt,letterpaper]{article}
\RequirePackage[letterpaper,margin=.88in,top=1.06in,bottom=.86in,headheight=16pt,headsep=19pt]{geometry}
\RequirePackage{fontspec}
\RequirePackage{xcolor,fancyhdr,titlesec}
\setmainfont{Liberation Serif}
\setsansfont{Liberation Sans}
\definecolor{ForumBlue}{HTML}{213D56}
\definecolor{ForumInk}{HTML}{27313B}
\definecolor{ForumRule}{HTML}{C88B47}
\pagestyle{fancy}
\fancyhf{}
\fancyhead[L]{\small\sffamily\bfseries\color{ForumBlue}Applied Document Research Forum}
\fancyhead[R]{\small\sffamily\color{ForumBlue}Program Committee}
\renewcommand{\headrule}{\hbox to\headwidth{\color{ForumRule}\leaders\hrule height .7pt\hfill}}
\fancyfoot[R]{\small\sffamily\color{ForumBlue}\thepage}
\setlength{\parindent}{0pt}
\setlength{\parskip}{7pt}
\AtBeginDocument{\color{ForumInk}}
\titleformat{\section}{\color{ForumBlue}\sffamily\bfseries\large}{}{0pt}{}
\titlespacing*{\section}{0pt}{14pt}{4pt}
\newcommand{\ForumGuide}[3]{%
  {\raggedright\color{ForumBlue}\sffamily\bfseries\fontsize{24}{28}\selectfont #1\par}%
  \smallskip
  {\color{ForumBlue}\small\sffamily #2\quad\textperiodcentered\quad #3\par}%
  \medskip
}
