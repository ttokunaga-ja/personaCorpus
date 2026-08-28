-- Orchid Cellworks synthetic laboratory table for OCW-LAB-G2YTGPEC.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abg2ytgpec (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abg2ytgpec VALUES (1, 'OCW-A24-001', 6.843, 0.57, 93.5, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (2, 'OCW-A24-015', 7.179, 0.56, 98.7, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (3, 'OCW-A24-006', 6.575, 0.42, 92.2, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (4, 'OCW-A24-012', 7.038, 0.56, 94.4, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (5, 'OCW-A24-020', 6.542, 0.28, 95.5, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (6, 'OCW-A24-008', 6.792, 0.56, 93.2, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (7, 'OCW-A24-002', 6.469, 0.42, 94.8, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (8, 'OCW-A24-021', 7.017, 0.57, 99.9, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (9, 'OCW-A24-006', 6.919, 0.43, 92.5, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (10, 'OCW-A24-007', 7.186, 0.70, 92.9, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (11, 'OCW-A24-002', 6.447, 0.70, 98.7, 'PASS');
INSERT INTO assay_abg2ytgpec VALUES (12, 'OCW-A24-001', 6.389, 0.57, 97.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abg2ytgpec WHERE status = 'PASS';
