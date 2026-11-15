-- Orchid Cellworks synthetic laboratory table for OCW-LAB-IVDIZFIX.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abivdizfix (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abivdizfix VALUES (1, 'OCW-A24-006', 7.150, 0.00, 94.9, 'PASS');
INSERT INTO assay_abivdizfix VALUES (2, 'OCW-A24-011', 6.814, 0.43, 95.4, 'PASS');
INSERT INTO assay_abivdizfix VALUES (3, 'OCW-A24-020', 7.206, 0.28, 94.1, 'PASS');
INSERT INTO assay_abivdizfix VALUES (4, 'OCW-A24-015', 6.662, 0.71, 98.6, 'PASS');
INSERT INTO assay_abivdizfix VALUES (5, 'OCW-A24-016', 7.191, 0.57, 97.5, 'PASS');
INSERT INTO assay_abivdizfix VALUES (6, 'OCW-A24-007', 6.814, 0.43, 94.1, 'PASS');
INSERT INTO assay_abivdizfix VALUES (7, 'OCW-A24-023', 6.708, 0.28, 96.9, 'PASS');
INSERT INTO assay_abivdizfix VALUES (8, 'OCW-A24-016', 6.933, 0.56, 95.1, 'PASS');
INSERT INTO assay_abivdizfix VALUES (9, 'OCW-A24-013', 6.682, 0.28, 99.4, 'PASS');
INSERT INTO assay_abivdizfix VALUES (10, 'OCW-A24-013', 7.178, 0.14, 96.4, 'PASS');
INSERT INTO assay_abivdizfix VALUES (11, 'OCW-A24-021', 7.010, 0.71, 97.5, 'PASS');
INSERT INTO assay_abivdizfix VALUES (12, 'OCW-A24-002', 7.199, 0.28, 99.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abivdizfix WHERE status = 'PASS';
