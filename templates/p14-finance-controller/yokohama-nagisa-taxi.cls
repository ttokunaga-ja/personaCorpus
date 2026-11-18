\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{yokohama-nagisa-taxi}[Yokohama Nagisa Taxi receipt]
\LoadClass[10pt]{ltjsarticle}
\RequirePackage[paperwidth=80mm,paperheight=115mm,margin=8mm]{geometry}
\RequirePackage{luatexja-fontspec}
\setmainjfont{Noto Sans Mono CJK JP}
\pagestyle{empty}
\setlength{\parindent}{0pt}

% Pass the printed issue timestamp, receipt number, amount and description.
\newcommand{\NagisaReceipt}[4]{%
  \begin{center}
    {\large\bfseries 横浜なぎさタクシー\par}
    \vspace{3mm}
    {\Large 領 収 証\par}
  \end{center}
  \noindent\hrulefill\par
  発行日時\hfill #1\par
  領収証番号\hfill #2\par
  \medskip
  領収金額\hfill {\Large\bfseries #3}\par
  \medskip
  \noindent\hrulefill\par
  但し\hfill #4\par
  上記金額を領収いたしました。\par
}
