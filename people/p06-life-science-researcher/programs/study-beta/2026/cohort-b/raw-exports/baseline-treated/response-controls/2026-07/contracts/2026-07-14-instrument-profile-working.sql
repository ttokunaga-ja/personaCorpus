-- Orchid Cellworks synthetic laboratory table for OCW-LAB-TWJYPTG6.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abtwjyptg6 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abtwjyptg6 VALUES (1, 'OCW-B16-010', 6.564, 0.57, 94.0, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (2, 'OCW-B16-001', 7.024, 0.28, 97.0, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (3, 'OCW-B16-016', 6.821, 0.28, 93.5, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (4, 'OCW-B16-010', 6.785, 0.42, 98.6, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (5, 'OCW-B16-015', 7.031, 0.28, 93.5, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (6, 'OCW-B16-016', 6.431, 0.43, 94.2, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (7, 'OCW-B16-009', 7.108, 0.56, 97.4, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (8, 'OCW-B16-006', 6.786, 0.28, 93.9, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (9, 'OCW-B16-015', 7.003, 0.14, 95.5, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (10, 'OCW-B16-011', 7.136, 0.42, 95.2, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (11, 'OCW-B16-001', 6.843, 0.57, 93.9, 'PASS');
INSERT INTO assay_abtwjyptg6 VALUES (12, 'OCW-B16-016', 7.066, 0.42, 92.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abtwjyptg6 WHERE status = 'PASS';
