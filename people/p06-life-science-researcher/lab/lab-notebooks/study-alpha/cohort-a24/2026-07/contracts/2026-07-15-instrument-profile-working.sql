-- Orchid Cellworks synthetic laboratory table for OCW-LAB-Z24VHWHF.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abz24vhwhf (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abz24vhwhf VALUES (1, 'OCW-A24-006', 6.485, 0.00, 93.1, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (2, 'OCW-A24-015', 6.856, 0.28, 98.4, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (3, 'OCW-A24-005', 6.849, 0.28, 93.7, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (4, 'OCW-A24-023', 7.122, 0.14, 94.5, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (5, 'OCW-A24-012', 6.453, 0.71, 93.5, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (6, 'OCW-A24-024', 6.680, 0.42, 92.6, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (7, 'OCW-A24-012', 6.618, 0.14, 94.4, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (8, 'OCW-A24-024', 6.933, 0.56, 93.4, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (9, 'OCW-A24-002', 6.507, 0.28, 92.3, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (10, 'OCW-A24-008', 6.975, 0.71, 97.5, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (11, 'OCW-A24-001', 7.003, 0.14, 96.6, 'PASS');
INSERT INTO assay_abz24vhwhf VALUES (12, 'OCW-A24-013', 6.402, 0.28, 92.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abz24vhwhf WHERE status = 'PASS';
