-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ZLIMWGCU.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abzlimwgcu (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abzlimwgcu VALUES (1, 'OCW-B16-014', 7.184, 0.71, 94.1, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (2, 'OCW-B16-002', 6.954, 0.28, 99.6, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (3, 'OCW-B16-002', 7.249, 0.56, 94.0, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (4, 'OCW-B16-010', 7.115, 0.70, 93.0, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (5, 'OCW-B16-014', 6.708, 0.28, 98.8, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (6, 'OCW-B16-010', 6.610, 0.42, 97.2, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (7, 'OCW-B16-012', 6.799, 0.70, 99.3, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (8, 'OCW-B16-014', 6.751, 0.28, 93.1, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (9, 'OCW-B16-010', 6.466, 0.43, 95.3, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (10, 'OCW-B16-009', 7.151, 0.70, 99.1, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (11, 'OCW-B16-004', 6.834, 0.70, 92.7, 'PASS');
INSERT INTO assay_abzlimwgcu VALUES (12, 'OCW-B16-013', 6.469, 0.42, 99.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abzlimwgcu WHERE status = 'PASS';
