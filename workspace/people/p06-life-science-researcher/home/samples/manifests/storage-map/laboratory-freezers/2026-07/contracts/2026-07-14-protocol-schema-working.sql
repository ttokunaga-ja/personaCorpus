-- Orchid Cellworks synthetic laboratory table for OCW-LAB-DPPKLYBC.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abdppklybc (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abdppklybc VALUES (1, 'OCW-A24-015', 6.463, 0.28, 95.4, 'PASS');
INSERT INTO assay_abdppklybc VALUES (2, 'OCW-A24-015', 6.396, 0.43, 99.8, 'PASS');
INSERT INTO assay_abdppklybc VALUES (3, 'OCW-A24-012', 7.024, 0.28, 97.7, 'PASS');
INSERT INTO assay_abdppklybc VALUES (4, 'OCW-A24-019', 6.456, 0.14, 93.6, 'PASS');
INSERT INTO assay_abdppklybc VALUES (5, 'OCW-A24-024', 7.094, 0.43, 94.7, 'PASS');
INSERT INTO assay_abdppklybc VALUES (6, 'OCW-A24-021', 6.402, 0.28, 98.2, 'PASS');
INSERT INTO assay_abdppklybc VALUES (7, 'OCW-A24-016', 6.905, 0.00, 99.1, 'PASS');
INSERT INTO assay_abdppklybc VALUES (8, 'OCW-A24-014', 6.947, 0.57, 92.0, 'PASS');
INSERT INTO assay_abdppklybc VALUES (9, 'OCW-A24-024', 6.745, 0.43, 93.0, 'PASS');
INSERT INTO assay_abdppklybc VALUES (10, 'OCW-A24-014', 6.765, 0.00, 95.1, 'PASS');
INSERT INTO assay_abdppklybc VALUES (11, 'OCW-A24-002', 6.380, 0.00, 97.4, 'PASS');
INSERT INTO assay_abdppklybc VALUES (12, 'OCW-A24-001', 7.199, 0.28, 96.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abdppklybc WHERE status = 'PASS';
