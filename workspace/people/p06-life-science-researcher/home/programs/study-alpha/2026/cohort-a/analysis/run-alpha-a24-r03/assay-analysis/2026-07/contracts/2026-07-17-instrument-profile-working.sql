-- Orchid Cellworks synthetic laboratory table for OCW-LAB-6HPD6EB4.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab6hpd6eb4 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab6hpd6eb4 VALUES (1, 'OCW-A24-004', 6.633, 0.57, 98.3, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (2, 'OCW-A24-022', 6.856, 0.42, 92.9, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (3, 'OCW-A24-011', 6.870, 0.00, 92.3, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (4, 'OCW-A24-001', 7.003, 0.56, 95.1, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (5, 'OCW-A24-010', 6.703, 0.57, 97.1, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (6, 'OCW-A24-024', 6.568, 0.28, 98.9, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (7, 'OCW-A24-002', 6.434, 0.42, 95.9, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (8, 'OCW-A24-006', 7.249, 0.56, 93.2, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (9, 'OCW-A24-023', 6.348, 0.71, 94.2, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (10, 'OCW-A24-023', 7.249, 0.56, 98.2, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (11, 'OCW-A24-005', 6.393, 0.28, 99.4, 'PASS');
INSERT INTO assay_ab6hpd6eb4 VALUES (12, 'OCW-A24-017', 7.045, 0.71, 96.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab6hpd6eb4 WHERE status = 'PASS';
