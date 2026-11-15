-- Orchid Cellworks synthetic laboratory table for OCW-LAB-KD3YWZA6.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abkd3ywza6 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abkd3ywza6 VALUES (1, 'OCW-A24-006', 6.618, 0.14, 94.8, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (2, 'OCW-A24-024', 6.482, 0.70, 92.4, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (3, 'OCW-A24-007', 6.562, 0.14, 97.1, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (4, 'OCW-A24-001', 6.421, 0.14, 99.9, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (5, 'OCW-A24-006', 6.658, 0.70, 98.2, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (6, 'OCW-A24-012', 6.793, 0.14, 98.0, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (7, 'OCW-A24-015', 6.856, 0.28, 96.5, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (8, 'OCW-A24-003', 7.045, 0.00, 99.0, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (9, 'OCW-A24-012', 6.898, 0.14, 94.5, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (10, 'OCW-A24-016', 6.884, 0.43, 93.8, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (11, 'OCW-A24-006', 6.919, 0.28, 96.1, 'PASS');
INSERT INTO assay_abkd3ywza6 VALUES (12, 'OCW-A24-007', 6.500, 0.43, 97.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abkd3ywza6 WHERE status = 'PASS';
