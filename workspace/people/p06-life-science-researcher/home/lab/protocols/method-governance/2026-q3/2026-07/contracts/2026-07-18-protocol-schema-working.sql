-- Orchid Cellworks synthetic laboratory table for OCW-LAB-AK2OCJN5.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abak2ocjn5 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abak2ocjn5 VALUES (1, 'OCW-A24-024', 7.129, 0.28, 92.7, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (2, 'OCW-A24-010', 7.017, 0.57, 99.7, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (3, 'OCW-A24-020', 7.024, 0.28, 92.2, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (4, 'OCW-A24-019', 6.843, 0.57, 92.5, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (5, 'OCW-A24-016', 6.800, 0.00, 92.9, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (6, 'OCW-A24-004', 7.101, 0.28, 95.1, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (7, 'OCW-A24-001', 6.374, 0.14, 95.1, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (8, 'OCW-A24-005', 7.213, 0.14, 94.3, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (9, 'OCW-A24-016', 6.912, 0.14, 97.4, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (10, 'OCW-A24-004', 6.597, 0.14, 99.1, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (11, 'OCW-A24-010', 6.884, 0.43, 92.7, 'PASS');
INSERT INTO assay_abak2ocjn5 VALUES (12, 'OCW-A24-024', 6.605, 0.43, 93.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abak2ocjn5 WHERE status = 'PASS';
