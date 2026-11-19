\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{tomorie-guide}[2025/04/24 Support guidance]
\LoadClass{tomorie-export}
\RequirePackage{graphicx}
\fancyhead[R]{\small サポート・サクセス\\ご利用案内}
\renewcommand{\headrule}{\hbox to\headwidth{\color{TSamber}\leaders\hrule height .6pt\hfill}}
\newcommand{\TSGuide}[5]{%
  {\color{TSviolet}\LARGE\bfseries #1}\par\vspace{3mm}%
  {\small 文書番号：#2\quad 版：#3}\par
  {\small 公開日：#4\quad 作成者：#5}\par\vspace{3mm}%
}
\endinput
