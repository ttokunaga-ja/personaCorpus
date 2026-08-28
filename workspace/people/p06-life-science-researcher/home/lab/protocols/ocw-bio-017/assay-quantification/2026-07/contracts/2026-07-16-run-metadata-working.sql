-- Orchid Cellworks synthetic laboratory table for OCW-LAB-KEJGAJDQ.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abkejgajdq (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abkejgajdq VALUES (1, 'OCW-A24-020', 6.919, 0.28, 98.0, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (2, 'OCW-A24-014', 6.354, 0.57, 96.0, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (3, 'OCW-A24-007', 6.513, 0.14, 97.4, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (4, 'OCW-A24-013', 7.045, 0.71, 93.4, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (5, 'OCW-A24-023', 6.393, 0.28, 97.9, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (6, 'OCW-A24-012', 7.185, 0.00, 92.1, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (7, 'OCW-A24-011', 7.129, 0.43, 92.3, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (8, 'OCW-A24-021', 6.548, 0.14, 97.8, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (9, 'OCW-A24-017', 6.533, 0.28, 99.1, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (10, 'OCW-A24-002', 6.786, 0.28, 95.6, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (11, 'OCW-A24-014', 7.171, 0.42, 98.6, 'PASS');
INSERT INTO assay_abkejgajdq VALUES (12, 'OCW-A24-012', 6.441, 0.56, 97.5, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abkejgajdq WHERE status = 'PASS';
