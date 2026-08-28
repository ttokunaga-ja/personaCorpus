-- Orchid Cellworks synthetic laboratory table for OCW-LAB-2BIYQAB2.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab2biyqab2 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab2biyqab2 VALUES (1, 'OCW-A24-012', 7.227, 0.14, 96.1, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (2, 'OCW-A24-012', 7.234, 0.28, 97.6, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (3, 'OCW-A24-011', 7.191, 0.57, 96.8, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (4, 'OCW-A24-021', 6.877, 0.14, 92.5, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (5, 'OCW-A24-005', 7.150, 0.00, 93.2, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (6, 'OCW-A24-009', 6.820, 0.42, 95.6, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (7, 'OCW-A24-024', 6.680, 0.42, 97.7, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (8, 'OCW-A24-010', 7.045, 0.00, 97.9, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (9, 'OCW-A24-007', 7.038, 0.14, 94.5, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (10, 'OCW-A24-009', 6.415, 0.00, 94.5, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (11, 'OCW-A24-005', 7.150, 0.00, 96.3, 'PASS');
INSERT INTO assay_ab2biyqab2 VALUES (12, 'OCW-A24-008', 7.101, 0.28, 93.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab2biyqab2 WHERE status = 'PASS';
