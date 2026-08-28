-- Orchid Cellworks synthetic laboratory table for OCW-LAB-AHXYH4CI.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abahxyh4ci (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abahxyh4ci VALUES (1, 'OCW-A24-004', 6.800, 0.00, 96.4, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (2, 'OCW-A24-003', 6.667, 0.14, 92.7, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (3, 'OCW-A24-003', 6.588, 0.70, 93.5, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (4, 'OCW-A24-009', 6.869, 0.70, 92.2, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (5, 'OCW-A24-007', 6.660, 0.00, 96.3, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (6, 'OCW-A24-009', 6.504, 0.42, 94.3, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (7, 'OCW-A24-001', 6.688, 0.14, 99.2, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (8, 'OCW-A24-020', 6.849, 0.28, 95.6, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (9, 'OCW-A24-004', 6.389, 0.57, 97.5, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (10, 'OCW-A24-013', 6.612, 0.28, 97.2, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (11, 'OCW-A24-021', 7.101, 0.28, 93.2, 'PASS');
INSERT INTO assay_abahxyh4ci VALUES (12, 'OCW-A24-012', 6.668, 0.57, 96.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abahxyh4ci WHERE status = 'PASS';
