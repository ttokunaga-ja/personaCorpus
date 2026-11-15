-- Orchid Cellworks synthetic laboratory table for OCW-LAB-VJ2XDOPT.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abvj2xdopt (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abvj2xdopt VALUES (1, 'OCW-B16-016', 6.738, 0.57, 97.6, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (2, 'OCW-B16-007', 6.919, 0.28, 94.6, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (3, 'OCW-B16-008', 6.399, 0.42, 99.9, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (4, 'OCW-B16-004', 6.975, 0.70, 98.1, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (5, 'OCW-B16-005', 7.249, 0.56, 92.2, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (6, 'OCW-B16-016', 7.052, 0.14, 93.1, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (7, 'OCW-B16-012', 6.354, 0.57, 95.8, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (8, 'OCW-B16-002', 7.031, 0.42, 98.8, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (9, 'OCW-B16-005', 6.466, 0.43, 92.9, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (10, 'OCW-B16-002', 7.115, 0.70, 98.3, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (11, 'OCW-B16-005', 6.447, 0.70, 97.9, 'PASS');
INSERT INTO assay_abvj2xdopt VALUES (12, 'OCW-B16-012', 7.136, 0.42, 92.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abvj2xdopt WHERE status = 'PASS';
