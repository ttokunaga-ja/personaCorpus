-- Orchid Cellworks synthetic laboratory table for OCW-LAB-CDSW2QZR.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abcdsw2qzr (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abcdsw2qzr VALUES (1, 'OCW-A24-016', 6.675, 0.43, 99.0, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (2, 'OCW-A24-003', 6.517, 0.70, 98.1, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (3, 'OCW-A24-008', 7.010, 0.70, 92.4, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (4, 'OCW-A24-010', 7.031, 0.42, 98.7, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (5, 'OCW-A24-005', 7.221, 0.70, 92.3, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (6, 'OCW-A24-001', 6.487, 0.71, 98.4, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (7, 'OCW-A24-004', 6.409, 0.14, 96.0, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (8, 'OCW-A24-012', 6.961, 0.28, 93.5, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (9, 'OCW-A24-004', 6.863, 0.14, 95.9, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (10, 'OCW-A24-003', 6.459, 0.57, 97.7, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (11, 'OCW-A24-001', 7.024, 0.28, 96.8, 'PASS');
INSERT INTO assay_abcdsw2qzr VALUES (12, 'OCW-A24-004', 6.687, 0.56, 97.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abcdsw2qzr WHERE status = 'PASS';
