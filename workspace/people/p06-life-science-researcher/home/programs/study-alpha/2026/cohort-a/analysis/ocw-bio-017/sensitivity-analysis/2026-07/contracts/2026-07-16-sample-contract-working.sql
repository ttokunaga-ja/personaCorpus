-- Orchid Cellworks synthetic laboratory table for OCW-LAB-GRJETTDE.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abgrjettde (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abgrjettde VALUES (1, 'OCW-A24-005', 6.863, 0.14, 92.2, 'PASS');
INSERT INTO assay_abgrjettde VALUES (2, 'OCW-A24-015', 6.583, 0.14, 92.7, 'PASS');
INSERT INTO assay_abgrjettde VALUES (3, 'OCW-A24-015', 7.179, 0.56, 94.3, 'PASS');
INSERT INTO assay_abgrjettde VALUES (4, 'OCW-A24-009', 6.374, 0.14, 95.0, 'PASS');
INSERT INTO assay_abgrjettde VALUES (5, 'OCW-A24-005', 7.010, 0.71, 93.4, 'PASS');
INSERT INTO assay_abgrjettde VALUES (6, 'OCW-A24-008', 6.849, 0.43, 99.2, 'PASS');
INSERT INTO assay_abgrjettde VALUES (7, 'OCW-A24-017', 6.482, 0.70, 94.8, 'PASS');
INSERT INTO assay_abgrjettde VALUES (8, 'OCW-A24-013', 6.575, 0.42, 96.8, 'PASS');
INSERT INTO assay_abgrjettde VALUES (9, 'OCW-A24-008', 6.564, 0.57, 93.2, 'PASS');
INSERT INTO assay_abgrjettde VALUES (10, 'OCW-A24-008', 6.975, 0.00, 99.9, 'PASS');
INSERT INTO assay_abgrjettde VALUES (11, 'OCW-A24-015', 6.418, 0.71, 99.4, 'PASS');
INSERT INTO assay_abgrjettde VALUES (12, 'OCW-A24-015', 6.447, 0.70, 98.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abgrjettde WHERE status = 'PASS';
