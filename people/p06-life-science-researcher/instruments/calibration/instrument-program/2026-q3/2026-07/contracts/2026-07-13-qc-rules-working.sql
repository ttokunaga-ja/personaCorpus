-- Orchid Cellworks synthetic laboratory table for OCW-LAB-CJ3PD65F.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abcj3pd65f (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abcj3pd65f VALUES (1, 'OCW-A24-024', 6.680, 0.42, 96.5, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (2, 'OCW-A24-020', 6.675, 0.43, 98.4, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (3, 'OCW-A24-003', 6.940, 0.00, 99.8, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (4, 'OCW-A24-007', 6.469, 0.42, 93.7, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (5, 'OCW-A24-013', 6.487, 0.71, 93.6, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (6, 'OCW-A24-024', 7.186, 0.70, 94.5, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (7, 'OCW-A24-013', 6.546, 0.56, 93.2, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (8, 'OCW-A24-023', 6.933, 0.56, 92.8, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (9, 'OCW-A24-016', 6.800, 0.00, 95.4, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (10, 'OCW-A24-007', 7.171, 0.28, 99.1, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (11, 'OCW-A24-021', 6.485, 0.00, 99.0, 'PASS');
INSERT INTO assay_abcj3pd65f VALUES (12, 'OCW-A24-001', 6.522, 0.71, 92.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abcj3pd65f WHERE status = 'PASS';
