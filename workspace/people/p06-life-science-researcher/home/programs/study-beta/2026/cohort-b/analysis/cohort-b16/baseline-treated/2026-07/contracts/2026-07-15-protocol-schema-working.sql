-- Orchid Cellworks synthetic laboratory table for OCW-LAB-PKQU5QN7.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abpkqu5qn7 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abpkqu5qn7 VALUES (1, 'OCW-B16-012', 6.520, 0.00, 92.4, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (2, 'OCW-B16-015', 6.722, 0.56, 95.6, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (3, 'OCW-B16-014', 6.722, 0.56, 94.5, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (4, 'OCW-B16-010', 6.947, 0.57, 97.6, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (5, 'OCW-B16-016', 6.745, 0.43, 97.0, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (6, 'OCW-B16-003', 6.919, 0.43, 92.0, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (7, 'OCW-B16-013', 6.968, 0.14, 92.3, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (8, 'OCW-B16-001', 6.393, 0.28, 93.8, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (9, 'OCW-B16-015', 6.731, 0.71, 94.8, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (10, 'OCW-B16-014', 6.975, 0.00, 97.3, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (11, 'OCW-B16-014', 6.772, 0.14, 97.3, 'PASS');
INSERT INTO assay_abpkqu5qn7 VALUES (12, 'OCW-B16-014', 6.758, 0.14, 92.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abpkqu5qn7 WHERE status = 'PASS';
