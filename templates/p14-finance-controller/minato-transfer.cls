\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{minato-transfer}[2025/01/01 Minato corporate transfer inquiry]
\LoadClass{minato-balance}
\RequirePackage{longtable}

\newcommand{\TransferHeading}[4]{%
  \begin{flushright}照会番号：#1\\出力日時：#2\end{flushright}%
  {\LARGE\bfseries 振込結果照会}\par
  ご契約者：#3\par
  振込指定日：#4\par\medskip
}
\newenvironment{TransferTable}{%
  \renewcommand{\arraystretch}{1.6}%
  \begin{longtable}{@{}p{44mm}p{43mm}rr@{}}%
  \toprule 振込先 & 受付番号 & 振込額（円） & 状態\\\midrule\endhead
}{\\\bottomrule\end{longtable}}
\newcommand{\TransferSummary}[4]{%
  \begin{flushright}\begin{tabular}{lr}
    実行日時 & #1\\
    実行／返却 & #2 件／#3 件\\
    振込金額合計 & #4 円
  \end{tabular}\end{flushright}%
}
