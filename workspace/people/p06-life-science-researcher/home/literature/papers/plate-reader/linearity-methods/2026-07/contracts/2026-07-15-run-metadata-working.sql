-- Orchid Cellworks synthetic laboratory table for OCW-LAB-WFFQ4DP6.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abwffq4dp6 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abwffq4dp6 VALUES (1, 'OCW-A24-014', 6.717, 0.28, 99.8, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (2, 'OCW-A24-013', 6.891, 0.42, 92.1, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (3, 'OCW-A24-012', 6.717, 0.28, 98.3, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (4, 'OCW-A24-011', 6.487, 0.71, 92.4, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (5, 'OCW-A24-016', 6.522, 0.71, 95.9, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (6, 'OCW-A24-013', 6.652, 0.56, 93.5, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (7, 'OCW-A24-021', 6.703, 0.57, 93.2, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (8, 'OCW-A24-002', 7.198, 0.43, 97.9, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (9, 'OCW-A24-004', 6.434, 0.42, 96.1, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (10, 'OCW-A24-013', 7.191, 0.57, 93.7, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (11, 'OCW-A24-005', 6.682, 0.28, 99.9, 'PASS');
INSERT INTO assay_abwffq4dp6 VALUES (12, 'OCW-A24-017', 7.143, 0.14, 93.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abwffq4dp6 WHERE status = 'PASS';
