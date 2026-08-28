-- Orchid Cellworks synthetic laboratory table for OCW-LAB-B5AOYWLF.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abb5aoywlf (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abb5aoywlf VALUES (1, 'OCW-A24-017', 7.059, 0.43, 99.4, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (2, 'OCW-A24-016', 6.399, 0.42, 92.6, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (3, 'OCW-A24-022', 6.592, 0.71, 96.9, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (4, 'OCW-A24-020', 6.905, 0.00, 99.2, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (5, 'OCW-A24-005', 6.673, 0.28, 92.8, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (6, 'OCW-A24-009', 6.723, 0.14, 98.4, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (7, 'OCW-A24-023', 6.750, 0.42, 93.9, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (8, 'OCW-A24-001', 6.647, 0.28, 95.9, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (9, 'OCW-A24-009', 6.975, 0.00, 96.8, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (10, 'OCW-A24-009', 6.856, 0.28, 94.6, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (11, 'OCW-A24-011', 7.214, 0.56, 98.3, 'PASS');
INSERT INTO assay_abb5aoywlf VALUES (12, 'OCW-A24-010', 7.249, 0.56, 95.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abb5aoywlf WHERE status = 'PASS';
