-- Orchid Cellworks synthetic reference queries (SQLite)
-- Read-only examples for fictional Alpha A-24 and Beta B-16 records.

SELECT cohort, concentration_value, concentration_unit, dilution_factor
FROM assay_reference_records
WHERE run_date = '2026-07-13'
  AND cohort IN ('A24', 'B16')
ORDER BY cohort, assay_lane;

SELECT program, cohort, assay_lane, concentration_value,
       concentration_unit, review_status
FROM assay_reference_records
WHERE record_type = 'REF'
  AND review_status <> 'archived'
ORDER BY program, cohort, assay_lane;

SELECT program, cohort, COUNT(*) AS unresolved_count
FROM assay_reference_records
WHERE review_status IN ('needs_review', 'unit_missing', 'label_mismatch')
GROUP BY program, cohort
ORDER BY unresolved_count DESC, program, cohort;

SELECT program, cohort, run_date, assay_lane,
       concentration_value AS source_value,
       concentration_unit AS source_unit,
       CASE
         WHEN concentration_unit = 'ng/mL' THEN concentration_value
         ELSE NULL
       END AS equivalent_pg_per_uL
FROM assay_reference_records
WHERE concentration_unit IN ('ng/mL', 'pg/uL')
ORDER BY run_date, program, cohort, assay_lane;
