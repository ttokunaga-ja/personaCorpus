-- Fixed synthetic slice ledger for Model Beta B-07, run R184.
WITH metadata AS (
  SELECT 'R184' AS run_id, 'run R184' AS run_label, '2026-07-13' AS evaluation_date,
         'Model Beta B-07' AS model_name, 'HF-Eval-S1/S2 v3.2' AS dataset_name,
         2400 AS dataset_records, 0.846 AS macro_f1
), slices AS (
  SELECT 'split' AS dimension, 'S1' AS slice_value, 1600 AS records, 0.852 AS slice_macro_f1
  UNION ALL SELECT 'split', 'S2', 800, 0.834
  UNION ALL SELECT 'label', 'refund_request', 320, 0.816
  UNION ALL SELECT 'confidence_band', '0.80-1.00', 1634, 0.878
)
SELECT m.run_id, m.run_label, m.evaluation_date, m.model_name, m.dataset_name,
       m.dataset_records, m.macro_f1 AS overall_macro_f1, s.dimension, s.slice_value,
       s.records, s.slice_macro_f1, s.slice_macro_f1 - m.macro_f1 AS delta_from_overall
FROM metadata AS m CROSS JOIN slices AS s
WHERE s.records >= 100
ORDER BY s.dimension, s.slice_value;
