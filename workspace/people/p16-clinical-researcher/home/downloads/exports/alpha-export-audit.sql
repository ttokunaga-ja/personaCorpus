-- Sakura Clinical Study Alpha 2026: synthetic export audit
-- Snapshot: 2026-09-02; cohort: 48 (24 SC-A 2.5 mg, 24 matched control)
-- All identifiers and data below are synthetic; no PHI is present.

CREATE TABLE IF NOT EXISTS alpha_export_audit (
  export_id TEXT PRIMARY KEY,
  study_code TEXT NOT NULL,
  snapshot_date DATE NOT NULL,
  cohort_total INTEGER NOT NULL CHECK (cohort_total = 48),
  sc_a_2_5_mg_count INTEGER NOT NULL CHECK (sc_a_2_5_mg_count = 24),
  matched_control_count INTEGER NOT NULL CHECK (matched_control_count = 24),
  nonserious_event_count INTEGER NOT NULL CHECK (nonserious_event_count = 7),
  serious_event_count INTEGER NOT NULL CHECK (serious_event_count = 0),
  synthetic_only BOOLEAN NOT NULL
);

INSERT INTO alpha_export_audit (
  export_id, study_code, snapshot_date, cohort_total, sc_a_2_5_mg_count,
  matched_control_count, nonserious_event_count, serious_event_count, synthetic_only
) VALUES (
  'ALPHA-EXPORT-20260902', 'SCS-ALPHA-2026', DATE '2026-09-02', 48, 24, 24, 7, 0, TRUE
);

SELECT
  export_id, snapshot_date, cohort_total,
  sc_a_2_5_mg_count + matched_control_count AS reconciled_cohort,
  nonserious_event_count, serious_event_count
FROM alpha_export_audit
WHERE export_id = 'ALPHA-EXPORT-20260902'
  AND cohort_total = sc_a_2_5_mg_count + matched_control_count;
