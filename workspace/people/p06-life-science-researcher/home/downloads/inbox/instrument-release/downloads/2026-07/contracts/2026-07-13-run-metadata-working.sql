-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ACM7IGKJ.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abacm7igkj (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abacm7igkj VALUES (1, 'OCW-A24-004', 7.024, 0.43, 98.6, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (2, 'OCW-A24-015', 6.905, 0.71, 96.7, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (3, 'OCW-A24-022', 6.479, 0.14, 97.4, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (4, 'OCW-A24-015', 7.024, 0.28, 99.4, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (5, 'OCW-A24-017', 7.213, 0.14, 95.8, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (6, 'OCW-A24-010', 6.821, 0.28, 97.8, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (7, 'OCW-A24-004', 6.758, 0.14, 99.6, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (8, 'OCW-A24-015', 6.662, 0.71, 95.3, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (9, 'OCW-A24-012', 6.919, 0.28, 95.4, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (10, 'OCW-A24-017', 6.555, 0.00, 93.3, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (11, 'OCW-A24-020', 7.143, 0.56, 97.2, 'PASS');
INSERT INTO assay_abacm7igkj VALUES (12, 'OCW-A24-023', 6.786, 0.28, 94.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abacm7igkj WHERE status = 'PASS';
