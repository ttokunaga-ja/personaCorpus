\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{tomorie-quality-report}[2025/10/23 Support quality report]
\LoadClass{tomorie-export}

\fancyhead[R]{\small サポート・サクセス\\支援品質報告}
\renewcommand{\headrule}{\hbox to\headwidth{\color{TSamber}\leaders\hrule height .6pt\hfill}}

\newcommand{\TSQualityReport}[5]{%
  {\color{TSviolet}\LARGE\bfseries #1}\par\vspace{3mm}%
  {\small 文書番号：#2\quad 版：#3\par
  報告日：#4\quad 作成者：#5}\par\vspace{3mm}%
}
\endinput
