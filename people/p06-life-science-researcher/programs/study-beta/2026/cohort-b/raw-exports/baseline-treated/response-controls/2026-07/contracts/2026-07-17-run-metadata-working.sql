-- Orchid Cellworks synthetic laboratory table for OCW-LAB-FWKAJTUE.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abfwkajtue (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abfwkajtue VALUES (1, 'OCW-B16-016', 6.660, 0.00, 97.0, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (2, 'OCW-B16-013', 6.750, 0.42, 97.5, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (3, 'OCW-B16-004', 6.447, 0.70, 92.8, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (4, 'OCW-B16-008', 6.546, 0.56, 97.2, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (5, 'OCW-B16-015', 6.431, 0.43, 93.0, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (6, 'OCW-B16-008', 7.129, 0.43, 100.0, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (7, 'OCW-B16-016', 6.982, 0.14, 94.7, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (8, 'OCW-B16-003', 6.996, 0.42, 98.8, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (9, 'OCW-B16-014', 6.773, 0.57, 95.1, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (10, 'OCW-B16-014', 6.540, 0.42, 95.1, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (11, 'OCW-B16-012', 6.764, 0.70, 96.3, 'PASS');
INSERT INTO assay_abfwkajtue VALUES (12, 'OCW-B16-015', 7.101, 0.28, 92.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abfwkajtue WHERE status = 'PASS';
