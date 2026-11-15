-- Orchid Cellworks synthetic laboratory table for OCW-LAB-QXAEBULP.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abqxaebulp (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abqxaebulp VALUES (1, 'OCW-A24-012', 6.764, 0.70, 92.5, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (2, 'OCW-A24-006', 6.418, 0.71, 93.2, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (3, 'OCW-A24-019', 6.463, 0.28, 95.3, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (4, 'OCW-A24-007', 6.469, 0.42, 93.9, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (5, 'OCW-A24-005', 6.996, 0.28, 93.3, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (6, 'OCW-A24-013', 6.710, 0.43, 98.5, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (7, 'OCW-A24-017', 6.632, 0.14, 93.8, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (8, 'OCW-A24-010', 6.546, 0.56, 94.2, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (9, 'OCW-A24-007', 6.469, 0.42, 96.7, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (10, 'OCW-A24-006', 7.136, 0.42, 95.5, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (11, 'OCW-A24-013', 7.052, 0.14, 99.4, 'PASS');
INSERT INTO assay_abqxaebulp VALUES (12, 'OCW-A24-019', 7.171, 0.28, 97.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abqxaebulp WHERE status = 'PASS';
