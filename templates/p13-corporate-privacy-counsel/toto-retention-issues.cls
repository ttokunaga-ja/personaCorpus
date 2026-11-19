\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{toto-retention-issues}[2024/04/10 Retention issues handout]
\LoadClass{toto-review-sheet}

\ReviewMatter{保存目的の棚卸し}
\newenvironment{RetentionIssues}{%
  \renewcommand{\arraystretch}{1.3}%
  % Keep outer padding so colortbl's fill and the horizontal rules share edges.
  \begin{longtable}{p{\dimexpr28mm-\tabcolsep\relax}p{40mm}p{43mm}p{\dimexpr46mm-\tabcolsep\relax}}
  \hline\rowcolor{TDMain}\color{white}対象記録 & \color{white}参照業務・時点 &
  \color{white}原記録が要る根拠 & \color{white}照会先・確認事項 \\\hline\endhead
}{\end{longtable}}
