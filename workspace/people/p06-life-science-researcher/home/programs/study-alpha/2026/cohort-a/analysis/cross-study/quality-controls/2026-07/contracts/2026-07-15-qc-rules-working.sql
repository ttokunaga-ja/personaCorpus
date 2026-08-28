-- Orchid Cellworks synthetic laboratory table for OCW-LAB-LAC2AIAX.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ablac2aiax (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ablac2aiax VALUES (1, 'OCW-A24-014', 6.751, 0.28, 93.3, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (2, 'OCW-A24-011', 7.207, 0.42, 93.2, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (3, 'OCW-A24-010', 7.038, 0.14, 93.1, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (4, 'OCW-A24-003', 6.533, 0.28, 98.2, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (5, 'OCW-A24-001', 7.206, 0.28, 96.0, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (6, 'OCW-A24-001', 6.469, 0.42, 97.2, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (7, 'OCW-A24-013', 6.849, 0.43, 93.6, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (8, 'OCW-A24-020', 7.059, 0.43, 96.5, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (9, 'OCW-A24-004', 7.114, 0.71, 99.6, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (10, 'OCW-A24-012', 6.393, 0.28, 94.9, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (11, 'OCW-A24-006', 6.828, 0.14, 97.0, 'PASS');
INSERT INTO assay_ablac2aiax VALUES (12, 'OCW-A24-011', 7.185, 0.00, 98.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ablac2aiax WHERE status = 'PASS';
