# Model Alpha M-14 limitations

## Scope boundary

Model Alpha M-14 was assessed only on HF-Eval-S1/S2 v3.2, a synthetic five-label intent task. Its run R184 macro F1 of 0.873 is not evidence of performance on real conversations, documents, or people.

## Material constraints

- The 2,400-record corpus may not capture natural ambiguity, language variation, or changing terminology.
- Macro averages can conceal a poor result for a particular label or phrasing pattern.
- The model can confuse revision and escalation when a prompt contains both an altered constraint and urgency language.
- The evaluation does not measure latency, cost, privacy exposure, safety, robustness, or fairness.

## Required interpretation

Use the score to compare fixed research candidates. Do not convert it into a service-level target, an individual classification decision, or a claim that the model is reliable outside the synthetic taxonomy.
