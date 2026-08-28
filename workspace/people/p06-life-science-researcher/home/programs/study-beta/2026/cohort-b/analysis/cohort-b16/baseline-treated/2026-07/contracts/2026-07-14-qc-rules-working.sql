-- Orchid Cellworks synthetic laboratory table for OCW-LAB-MV2LN6OT.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abmv2ln6ot (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abmv2ln6ot VALUES (1, 'OCW-B16-006', 6.905, 0.00, 96.4, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (2, 'OCW-B16-001', 6.968, 0.14, 98.5, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (3, 'OCW-B16-001', 7.108, 0.14, 92.8, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (4, 'OCW-B16-016', 7.186, 0.70, 97.0, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (5, 'OCW-B16-015', 6.640, 0.43, 93.9, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (6, 'OCW-B16-003', 6.476, 0.56, 92.0, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (7, 'OCW-B16-013', 7.079, 0.71, 94.7, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (8, 'OCW-B16-016', 7.186, 0.70, 93.5, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (9, 'OCW-B16-008', 7.207, 0.42, 97.4, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (10, 'OCW-B16-005', 6.996, 0.28, 95.7, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (11, 'OCW-B16-006', 6.905, 0.00, 97.0, 'PASS');
INSERT INTO assay_abmv2ln6ot VALUES (12, 'OCW-B16-008', 6.653, 0.14, 94.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abmv2ln6ot WHERE status = 'PASS';
