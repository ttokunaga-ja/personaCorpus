Beacon Field Research
Study Alpha survey results
Study date: 2026-07-13
Session: UX-184
Synthetic sample: n=24

Purpose
This folder contains a synthetic post-task survey export and derived analysis files for Study Alpha. All participant identifiers are synthetic study IDs. No names, contact details, recordings, or other personal information are included.

Source and derivatives
00-study-alpha-survey-results.csv is the seed response export. 01-response-level-clean.csv normalizes task completion to 1/0 and retains the one unanswered trust item as blank. 02-question-summary.csv and 03-segment-summary.csv are calculated from the cleaned responses. 04-task-confidence.csv and 05-open-text-coding.csv are focused derivatives. 06-analysis-ready.csv joins normalized responses to the paraphrase coding.

Scales and missingness
Likert items use a 1 to 5 scale, where higher values are favorable. S08 did not answer trust_1_5. The blank is not imputed. Trust summaries use n=23; all other item summaries use n=24. scale_item_mean averages only observed ease, findability, clarity, and trust values.

Segment rules
Novice corresponds to First visit, Occasional to Monthly, and Frequent to Weekly or more. Each segment contains eight responses.

Reproducibility
Question means are arithmetic means. sample_sd uses the n-1 denominator. Top-two-box is the count and percentage of valid values equal to 4 or 5. Completion rate is n_completed divided by n_responses. Displayed values are rounded to three decimals after calculation.
