-- Orchid Cellworks synthetic laboratory table for OCW-LAB-IBASY74E.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abibasy74e (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abibasy74e VALUES (1, 'OCW-A24-001', 7.234, 0.28, 95.3, 'PASS');
INSERT INTO assay_abibasy74e VALUES (2, 'OCW-A24-020', 7.179, 0.56, 92.8, 'PASS');
INSERT INTO assay_abibasy74e VALUES (3, 'OCW-A24-024', 6.884, 0.28, 99.6, 'PASS');
INSERT INTO assay_abibasy74e VALUES (4, 'OCW-A24-019', 6.456, 0.14, 96.5, 'PASS');
INSERT INTO assay_abibasy74e VALUES (5, 'OCW-A24-001', 6.653, 0.14, 98.6, 'PASS');
INSERT INTO assay_abibasy74e VALUES (6, 'OCW-A24-019', 7.045, 0.00, 97.0, 'PASS');
INSERT INTO assay_abibasy74e VALUES (7, 'OCW-A24-011', 6.583, 0.14, 94.2, 'PASS');
INSERT INTO assay_abibasy74e VALUES (8, 'OCW-A24-023', 7.191, 0.57, 95.1, 'PASS');
INSERT INTO assay_abibasy74e VALUES (9, 'OCW-A24-015', 6.926, 0.28, 97.2, 'PASS');
INSERT INTO assay_abibasy74e VALUES (10, 'OCW-A24-008', 6.354, 0.57, 97.7, 'PASS');
INSERT INTO assay_abibasy74e VALUES (11, 'OCW-A24-009', 6.801, 0.71, 93.0, 'PASS');
INSERT INTO assay_abibasy74e VALUES (12, 'OCW-A24-008', 7.186, 0.70, 93.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abibasy74e WHERE status = 'PASS';
