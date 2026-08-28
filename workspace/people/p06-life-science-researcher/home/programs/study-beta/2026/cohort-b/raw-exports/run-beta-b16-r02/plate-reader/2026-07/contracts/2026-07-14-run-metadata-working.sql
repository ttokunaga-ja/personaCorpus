-- Orchid Cellworks synthetic laboratory table for OCW-LAB-UBIJKTZR.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abubijktzr (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abubijktzr VALUES (1, 'OCW-B16-006', 6.905, 0.00, 98.7, 'PASS');
INSERT INTO assay_abubijktzr VALUES (2, 'OCW-B16-006', 6.374, 0.14, 94.9, 'PASS');
INSERT INTO assay_abubijktzr VALUES (3, 'OCW-B16-015', 6.520, 0.00, 99.3, 'PASS');
INSERT INTO assay_abubijktzr VALUES (4, 'OCW-B16-008', 6.412, 0.70, 94.7, 'PASS');
INSERT INTO assay_abubijktzr VALUES (5, 'OCW-B16-013', 7.045, 0.00, 94.6, 'PASS');
INSERT INTO assay_abubijktzr VALUES (6, 'OCW-B16-009', 6.482, 0.70, 93.4, 'PASS');
INSERT INTO assay_abubijktzr VALUES (7, 'OCW-B16-014', 6.564, 0.57, 94.0, 'PASS');
INSERT INTO assay_abubijktzr VALUES (8, 'OCW-B16-009', 7.213, 0.14, 93.9, 'PASS');
INSERT INTO assay_abubijktzr VALUES (9, 'OCW-B16-004', 6.418, 0.71, 95.4, 'PASS');
INSERT INTO assay_abubijktzr VALUES (10, 'OCW-B16-003', 6.919, 0.43, 96.5, 'PASS');
INSERT INTO assay_abubijktzr VALUES (11, 'OCW-B16-003', 6.479, 0.14, 93.8, 'PASS');
INSERT INTO assay_abubijktzr VALUES (12, 'OCW-B16-003', 6.533, 0.28, 96.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abubijktzr WHERE status = 'PASS';
