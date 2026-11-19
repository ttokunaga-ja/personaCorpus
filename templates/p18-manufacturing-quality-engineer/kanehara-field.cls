\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{kanehara-field}[2026/09/01 Quality field record]
\LoadClass[a4paper,10pt]{article}
\RequirePackage{luatexja-fontspec}
\setmainfont{Liberation Sans}
\setmainjfont{Noto Sans CJK JP}
\RequirePackage[margin=20mm,top=29mm,bottom=23mm,headheight=25pt]{geometry}
\RequirePackage[table]{xcolor}
\definecolor{KTeal}{HTML}{16686A}
\definecolor{KSteel}{HTML}{424F59}
\RequirePackage{tikz,fancyhdr,longtable,array}
\AtBeginDocument{\color{KSteel}}
\pagestyle{fancy}\fancyhf{}
\fancyhead[L]{\color{KTeal}\begin{tikzpicture}[baseline=-2pt,scale=.14]\draw[line width=1.5pt] (0,2)--(0,0)--(3,0)--(3,2);\draw[KSteel] (.6,2)--(2.4,2);\draw[KSteel] (.6,1.7)--(.6,2.3) (2.4,1.7)--(2.4,2.3);\end{tikzpicture}\quad 金原産業精機株式会社}
\fancyfoot[L]{\small\color{KSteel}品質保証部}\fancyfoot[R]{\small\thepage}
\renewcommand{\headrulewidth}{.5pt}
\setlength{\parindent}{0pt}\setlength{\parskip}{6pt}
\newcommand{\QualityHeading}[5]{{\LARGE\bfseries\color{KTeal}#1}\par{\small 文書番号：#2\quad 対象：#3\par 実施日：#4\quad 記録者：#5}\par\medskip}
\newcommand{\QualitySection}[1]{\par\medskip{\large\bfseries\color{KTeal}#1}\par}
\newcommand{\QualityNote}[1]{\par{\small\color{KSteel}#1}\par}
\newenvironment{SelectionTable}{\renewcommand{\arraystretch}{1.5}\begin{longtable}{|p{20mm}|p{25mm}|p{23mm}|p{62mm}|p{18mm}|}\hline\rowcolor{KTeal}\color{white}照合単位&\color{white}確認数（個）&\color{white}判定&\color{white}隔離・引継ぎの確認&\color{white}確認者\\\hline\endfirsthead\hline\rowcolor{KTeal}\color{white}照合単位&\color{white}確認数（個）&\color{white}判定&\color{white}隔離・引継ぎの確認&\color{white}確認者\\\hline\endhead}{\end{longtable}}

\newcommand{\EffectCheckColumns}{%
  \rowcolor{KTeal}\color{white}製造ロット&
  \color{white}照合項目&
  \color{white}確認内容・対応記録&
  \color{white}確認者\\\hline
}
\newenvironment{EffectCheckTable}{%
  \renewcommand{\arraystretch}{1.6}%
  \begin{longtable}{|p{25mm}|p{28mm}|p{73mm}|p{22mm}|}%
  \hline\EffectCheckColumns\endfirsthead
  \hline\EffectCheckColumns\endhead
}{\end{longtable}}

\newcommand{\BurrCheckColumns}{%
  \rowcolor{KTeal}\color{white}履歴・現物番号&
  \color{white}観察箇所・加工面&
  \color{white}写真番号・向き&
  \color{white}照合結果・確認事項\\\hline
}
\newenvironment{BurrCheckTable}{%
  \renewcommand{\arraystretch}{1.6}%
  \begin{longtable}{|p{27mm}|p{43mm}|p{35mm}|p{45mm}|}%
  \hline\BurrCheckColumns\endfirsthead
  \hline\BurrCheckColumns\endhead
}{\end{longtable}}

\newcommand{\WithdrawalColumns}{%
  \rowcolor{KTeal}\color{white}指示書番号&
  \color{white}掲示先・保管場所&
  \color{white}回収した版&
  \color{white}差替後の版&
  \color{white}回収確認・確認者\\\hline
}
\newenvironment{WithdrawalTable}{%
  \renewcommand{\arraystretch}{1.6}%
  \begin{longtable}{|p{38mm}|p{35mm}|p{19mm}|p{19mm}|p{35mm}|}%
  \hline\WithdrawalColumns\endfirsthead
  \hline\WithdrawalColumns\endhead
}{\end{longtable}}
