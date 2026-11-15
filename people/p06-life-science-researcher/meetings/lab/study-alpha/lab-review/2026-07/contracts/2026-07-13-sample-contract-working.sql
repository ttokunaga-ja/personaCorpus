-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ZRWPNEU6.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abzrwpneu6 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abzrwpneu6 VALUES (1, 'OCW-A24-008', 6.598, 0.57, 96.9, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (2, 'OCW-A24-018', 7.087, 0.57, 94.3, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (3, 'OCW-A24-001', 7.038, 0.56, 92.4, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (4, 'OCW-A24-001', 6.522, 0.71, 95.1, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (5, 'OCW-A24-018', 6.616, 0.56, 97.5, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (6, 'OCW-A24-003', 6.469, 0.42, 95.2, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (7, 'OCW-A24-020', 6.989, 0.43, 96.4, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (8, 'OCW-A24-021', 6.441, 0.56, 95.1, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (9, 'OCW-A24-001', 7.073, 0.14, 94.0, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (10, 'OCW-A24-001', 6.710, 0.43, 92.8, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (11, 'OCW-A24-004', 7.066, 0.42, 94.6, 'PASS');
INSERT INTO assay_abzrwpneu6 VALUES (12, 'OCW-A24-004', 6.568, 0.28, 94.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abzrwpneu6 WHERE status = 'PASS';
