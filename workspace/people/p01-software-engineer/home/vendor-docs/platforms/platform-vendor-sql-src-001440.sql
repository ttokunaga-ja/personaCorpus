-- Synthetic vendor evaluation VND-26Q3-001; no customer or production data.
CREATE VIEW vendor_evaluation_1440 AS
SELECT 'VND-26Q3-001' AS evaluation_id, 'Platform Helix' AS vendor, '2026-07-03' AS review_date,
       165 AS trial_p95_ms, 99.77 AS availability_pct, 90 AS evidence_coverage_pct,
       'advance to controlled trial' AS recommendation;
