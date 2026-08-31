-- Synthetic Mosaic Retail Analytics FY26 Q3 MTR-09 contract
-- artifact: p05-full-006625
-- review cut: q3-forecast / all-channels / central / filters

WITH metric_input AS (
  SELECT 86.0::numeric AS gross_jpy_m, 1.8::numeric AS refunds_jpy_m, 80.0::numeric AS plan_jpy_m
)
SELECT 'MTR-09' AS metric_code, gross_jpy_m - refunds_jpy_m AS reported_gmv_jpy_m,
       (gross_jpy_m - refunds_jpy_m) - plan_jpy_m AS plan_variance_jpy_m
FROM metric_input;
