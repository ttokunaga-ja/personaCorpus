-- Synthetic, offline reporting view for run R184.
WITH evaluation_metadata AS (
  SELECT 'R184' AS run_id, 'run R184' AS run_label, '2026-07-13' AS evaluation_date,
         'Model Alpha M-14' AS model_name, 'HF-Eval-S1/S2 v3.2' AS dataset_name,
         2400 AS dataset_records, 'macro_f1' AS metric_name, 0.873 AS metric_value,
         'F1 0.873' AS metric_display
), split_metrics AS (
  SELECT 'S1' AS split_name, 1600 AS records, 0.879 AS macro_f1
  UNION ALL SELECT 'S2', 800, 0.861
)
SELECT m.run_id, m.run_label, m.evaluation_date, m.model_name, m.dataset_name,
       m.dataset_records, m.metric_name, m.metric_value, m.metric_display,
       s.split_name, s.records AS split_records, s.macro_f1 AS split_macro_f1
FROM evaluation_metadata AS m CROSS JOIN split_metrics AS s;
