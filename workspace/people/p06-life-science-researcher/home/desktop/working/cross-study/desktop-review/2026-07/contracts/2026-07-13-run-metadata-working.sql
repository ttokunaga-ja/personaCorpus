-- Orchid Cellworks synthetic laboratory table for OCW-LAB-WLIPONHA.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abwliponha (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abwliponha VALUES (1, 'OCW-A24-007', 6.723, 0.14, 96.5, 'PASS');
INSERT INTO assay_abwliponha VALUES (2, 'OCW-A24-016', 6.447, 0.70, 99.4, 'PASS');
INSERT INTO assay_abwliponha VALUES (3, 'OCW-A24-004', 6.926, 0.42, 98.3, 'PASS');
INSERT INTO assay_abwliponha VALUES (4, 'OCW-A24-003', 6.982, 0.14, 92.3, 'PASS');
INSERT INTO assay_abwliponha VALUES (5, 'OCW-A24-023', 6.954, 0.28, 98.5, 'PASS');
INSERT INTO assay_abwliponha VALUES (6, 'OCW-A24-023', 6.745, 0.43, 92.3, 'PASS');
INSERT INTO assay_abwliponha VALUES (7, 'OCW-A24-024', 6.773, 0.57, 98.7, 'PASS');
INSERT INTO assay_abwliponha VALUES (8, 'OCW-A24-013', 7.087, 0.57, 93.3, 'PASS');
INSERT INTO assay_abwliponha VALUES (9, 'OCW-A24-007', 6.399, 0.42, 94.7, 'PASS');
INSERT INTO assay_abwliponha VALUES (10, 'OCW-A24-011', 6.498, 0.28, 92.2, 'PASS');
INSERT INTO assay_abwliponha VALUES (11, 'OCW-A24-024', 6.926, 0.28, 92.6, 'PASS');
INSERT INTO assay_abwliponha VALUES (12, 'OCW-A24-018', 6.533, 0.28, 96.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abwliponha WHERE status = 'PASS';
