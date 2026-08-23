-- Synthetic Study Alpha import schema; no patient or real research data.
CREATE TABLE alpha_a24_raw_export (
 organization TEXT NOT NULL, study TEXT NOT NULL, reference_date DATE NOT NULL, cohort TEXT NOT NULL,
 synthetic_sample_id TEXT PRIMARY KEY, protocol_run TEXT NOT NULL, run_id TEXT NOT NULL, assay_ng_ml NUMERIC(5,2) NOT NULL,
 anchor_reference TEXT NOT NULL, data_status TEXT NOT NULL
);
CREATE INDEX idx_alpha_a24_run ON alpha_a24_raw_export(run_id);
