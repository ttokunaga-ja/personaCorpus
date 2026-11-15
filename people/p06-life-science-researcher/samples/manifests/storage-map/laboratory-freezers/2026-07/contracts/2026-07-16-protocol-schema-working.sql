-- Orchid Cellworks synthetic laboratory table for OCW-LAB-WGDA6LSO.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abwgda6lso (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abwgda6lso VALUES (1, 'OCW-A24-016', 6.367, 0.28, 92.3, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (2, 'OCW-A24-012', 6.640, 0.43, 94.7, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (3, 'OCW-A24-010', 6.773, 0.57, 97.4, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (4, 'OCW-A24-002', 6.996, 0.28, 92.0, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (5, 'OCW-A24-005', 6.652, 0.56, 92.6, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (6, 'OCW-A24-015', 6.843, 0.57, 98.2, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (7, 'OCW-A24-009', 6.926, 0.28, 93.1, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (8, 'OCW-A24-004', 6.511, 0.56, 99.7, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (9, 'OCW-A24-016', 7.206, 0.28, 94.8, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (10, 'OCW-A24-017', 6.575, 0.42, 94.6, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (11, 'OCW-A24-013', 6.821, 0.28, 95.6, 'PASS');
INSERT INTO assay_abwgda6lso VALUES (12, 'OCW-A24-015', 6.836, 0.71, 96.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abwgda6lso WHERE status = 'PASS';
