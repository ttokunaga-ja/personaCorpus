-- Orchid Cellworks synthetic laboratory table for OCW-LAB-G2O5QQRH.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abg2o5qqrh (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abg2o5qqrh VALUES (1, 'OCW-B16-011', 6.940, 0.71, 96.2, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (2, 'OCW-B16-015', 6.418, 0.71, 95.1, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (3, 'OCW-B16-008', 6.905, 0.71, 99.6, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (4, 'OCW-B16-002', 6.456, 0.14, 96.8, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (5, 'OCW-B16-015', 6.476, 0.56, 92.3, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (6, 'OCW-B16-016', 6.472, 0.28, 97.1, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (7, 'OCW-B16-007', 6.466, 0.43, 99.0, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (8, 'OCW-B16-008', 6.814, 0.43, 99.5, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (9, 'OCW-B16-009', 7.221, 0.70, 99.1, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (10, 'OCW-B16-011', 6.750, 0.42, 99.4, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (11, 'OCW-B16-011', 6.905, 0.71, 92.0, 'PASS');
INSERT INTO assay_abg2o5qqrh VALUES (12, 'OCW-B16-014', 6.548, 0.14, 93.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abg2o5qqrh WHERE status = 'PASS';
