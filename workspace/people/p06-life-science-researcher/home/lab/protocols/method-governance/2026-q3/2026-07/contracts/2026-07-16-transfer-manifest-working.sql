-- Orchid Cellworks synthetic laboratory table for OCW-LAB-V5S2XREI.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abv5s2xrei (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abv5s2xrei VALUES (1, 'OCW-A24-009', 6.919, 0.43, 95.1, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (2, 'OCW-A24-014', 7.052, 0.14, 96.7, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (3, 'OCW-A24-023', 6.504, 0.42, 92.5, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (4, 'OCW-A24-016', 7.079, 0.71, 97.2, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (5, 'OCW-A24-020', 6.954, 0.28, 96.0, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (6, 'OCW-A24-014', 6.693, 0.70, 95.5, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (7, 'OCW-A24-010', 6.780, 0.43, 97.7, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (8, 'OCW-A24-010', 6.961, 0.28, 95.4, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (9, 'OCW-A24-003', 6.827, 0.56, 92.7, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (10, 'OCW-A24-014', 6.399, 0.42, 96.4, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (11, 'OCW-A24-020', 7.094, 0.28, 93.2, 'PASS');
INSERT INTO assay_abv5s2xrei VALUES (12, 'OCW-A24-002', 7.052, 0.14, 97.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abv5s2xrei WHERE status = 'PASS';
