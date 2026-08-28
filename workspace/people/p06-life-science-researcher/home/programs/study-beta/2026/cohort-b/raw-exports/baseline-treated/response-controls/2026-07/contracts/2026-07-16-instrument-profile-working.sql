-- Orchid Cellworks synthetic laboratory table for OCW-LAB-PTYGIEOO.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abptygieoo (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abptygieoo VALUES (1, 'OCW-B16-002', 6.870, 0.71, 98.9, 'PASS');
INSERT INTO assay_abptygieoo VALUES (2, 'OCW-B16-008', 7.164, 0.28, 92.7, 'PASS');
INSERT INTO assay_abptygieoo VALUES (3, 'OCW-B16-015', 6.933, 0.14, 94.8, 'PASS');
INSERT INTO assay_abptygieoo VALUES (4, 'OCW-B16-014', 6.598, 0.57, 93.0, 'PASS');
INSERT INTO assay_abptygieoo VALUES (5, 'OCW-B16-011', 7.087, 0.14, 96.9, 'PASS');
INSERT INTO assay_abptygieoo VALUES (6, 'OCW-B16-016', 7.163, 0.43, 93.2, 'PASS');
INSERT INTO assay_abptygieoo VALUES (7, 'OCW-B16-005', 6.453, 0.71, 95.2, 'PASS');
INSERT INTO assay_abptygieoo VALUES (8, 'OCW-B16-016', 7.031, 0.42, 96.7, 'PASS');
INSERT INTO assay_abptygieoo VALUES (9, 'OCW-B16-009', 7.101, 0.28, 95.6, 'PASS');
INSERT INTO assay_abptygieoo VALUES (10, 'OCW-B16-011', 6.453, 0.71, 94.4, 'PASS');
INSERT INTO assay_abptygieoo VALUES (11, 'OCW-B16-005', 6.745, 0.43, 97.7, 'PASS');
INSERT INTO assay_abptygieoo VALUES (12, 'OCW-B16-014', 7.185, 0.00, 98.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abptygieoo WHERE status = 'PASS';
