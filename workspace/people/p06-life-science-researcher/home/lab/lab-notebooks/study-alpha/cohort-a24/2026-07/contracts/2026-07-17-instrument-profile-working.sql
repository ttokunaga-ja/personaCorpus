-- Orchid Cellworks synthetic laboratory table for OCW-LAB-DAPOZBW4.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abdapozbw4 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abdapozbw4 VALUES (1, 'OCW-A24-020', 6.529, 0.57, 95.5, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (2, 'OCW-A24-017', 7.066, 0.42, 99.2, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (3, 'OCW-A24-001', 7.156, 0.57, 98.1, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (4, 'OCW-A24-004', 6.940, 0.70, 94.2, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (5, 'OCW-A24-019', 6.498, 0.28, 97.6, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (6, 'OCW-A24-002', 6.386, 0.14, 98.9, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (7, 'OCW-A24-001', 7.059, 0.43, 94.7, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (8, 'OCW-A24-017', 7.066, 0.28, 96.6, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (9, 'OCW-A24-021', 6.658, 0.70, 99.3, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (10, 'OCW-A24-019', 7.171, 0.42, 92.8, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (11, 'OCW-A24-006', 7.249, 0.56, 98.2, 'PASS');
INSERT INTO assay_abdapozbw4 VALUES (12, 'OCW-A24-024', 6.779, 0.28, 97.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abdapozbw4 WHERE status = 'PASS';
