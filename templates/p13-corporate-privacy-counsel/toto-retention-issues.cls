\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{toto-retention-issues}[2024/04/10 Retention issues handout]
\LoadClass{toto-review-sheet}

\ReviewMatter{保存目的の棚卸し}
\newenvironment{RetentionIssues}{%
  \renewcommand{\arraystretch}{1.3}%
  \begin{longtable}{@{}p{28mm}p{40mm}p{43mm}p{46mm}@{}}
  \hline\rowcolor{TDMain}\color{white}対象記録 & \color{white}参照業務・時点 &
  \color{white}原記録が要る根拠 & \color{white}照会先・確認事項 \\\hline\endhead
}{\end{longtable}}
