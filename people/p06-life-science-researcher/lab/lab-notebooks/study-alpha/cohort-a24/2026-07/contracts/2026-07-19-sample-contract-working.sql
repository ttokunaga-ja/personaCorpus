-- Orchid Cellworks synthetic laboratory table for OCW-LAB-HWE5ZTZN.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abhwe5ztzn (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abhwe5ztzn VALUES (1, 'OCW-A24-023', 7.136, 0.42, 98.9, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (2, 'OCW-A24-019', 7.115, 0.00, 98.7, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (3, 'OCW-A24-009', 6.562, 0.14, 92.7, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (4, 'OCW-A24-020', 7.179, 0.56, 97.3, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (5, 'OCW-A24-002', 6.905, 0.71, 95.1, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (6, 'OCW-A24-016', 7.038, 0.56, 92.1, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (7, 'OCW-A24-009', 6.402, 0.28, 98.2, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (8, 'OCW-A24-009', 6.625, 0.00, 92.8, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (9, 'OCW-A24-018', 6.583, 0.14, 97.4, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (10, 'OCW-A24-005', 6.431, 0.43, 96.7, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (11, 'OCW-A24-013', 7.163, 0.43, 97.0, 'PASS');
INSERT INTO assay_abhwe5ztzn VALUES (12, 'OCW-A24-014', 6.546, 0.56, 94.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abhwe5ztzn WHERE status = 'PASS';
