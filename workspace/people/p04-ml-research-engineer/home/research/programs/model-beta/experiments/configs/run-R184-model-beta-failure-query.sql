-- Synthetic failure triage candidates for Model Beta B-07; read-only CTE query.
WITH metadata AS (
  SELECT 'R184' AS run_id, 'run R184' AS run_label, '2026-07-13' AS evaluation_date,
         'Model Beta B-07' AS model_name, 'HF-Eval-S1/S2 v3.2' AS dataset_name,
         2400 AS dataset_records, 'macro_f1' AS metric_name, 0.846 AS metric_value
), failure_modes AS (
  SELECT 'refund_request' AS label, 'implicit_policy_exception' AS failure_mode, 71 AS examples, 0.804 AS recall
  UNION ALL SELECT 'plan_change', 'multi_intent_utterance', 53, 0.833
  UNION ALL SELECT 'technical_help', 'version_ambiguity', 39, 0.846
)
SELECT m.run_id, m.run_label, m.evaluation_date, m.model_name, m.dataset_name,
       m.dataset_records, m.metric_name, m.metric_value, f.label, f.failure_mode,
       f.examples, f.recall
FROM metadata AS m CROSS JOIN failure_modes AS f
WHERE f.examples >= 30
ORDER BY f.recall ASC, f.examples DESC;
