-- Orchid Cellworks synthetic laboratory table for OCW-LAB-NMAYOSLM.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abnmayoslm (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abnmayoslm VALUES (1, 'OCW-A24-005', 6.933, 0.14, 93.9, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (2, 'OCW-A24-004', 6.424, 0.57, 98.4, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (3, 'OCW-A24-023', 7.136, 0.28, 98.8, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (4, 'OCW-A24-016', 7.038, 0.56, 94.7, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (5, 'OCW-A24-009', 6.522, 0.71, 92.0, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (6, 'OCW-A24-003', 6.632, 0.14, 96.7, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (7, 'OCW-A24-005', 7.066, 0.42, 92.0, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (8, 'OCW-A24-022', 7.198, 0.43, 98.2, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (9, 'OCW-A24-002', 7.024, 0.43, 97.2, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (10, 'OCW-A24-023', 6.590, 0.00, 94.2, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (11, 'OCW-A24-010', 6.933, 0.14, 94.7, 'PASS');
INSERT INTO assay_abnmayoslm VALUES (12, 'OCW-A24-005', 6.821, 0.28, 100.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abnmayoslm WHERE status = 'PASS';
