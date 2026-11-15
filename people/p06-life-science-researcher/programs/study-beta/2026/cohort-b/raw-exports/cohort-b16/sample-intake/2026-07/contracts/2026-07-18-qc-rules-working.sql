-- Orchid Cellworks synthetic laboratory table for OCW-LAB-E4RC3FZA.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abe4rc3fza (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abe4rc3fza VALUES (1, 'OCW-B16-005', 6.450, 0.00, 97.6, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (2, 'OCW-B16-010', 6.590, 0.00, 94.5, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (3, 'OCW-B16-007', 6.555, 0.00, 93.0, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (4, 'OCW-B16-006', 6.842, 0.14, 95.2, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (5, 'OCW-B16-006', 6.361, 0.43, 92.2, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (6, 'OCW-B16-010', 6.616, 0.56, 97.1, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (7, 'OCW-B16-012', 6.625, 0.00, 93.2, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (8, 'OCW-B16-015', 6.940, 0.71, 93.7, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (9, 'OCW-B16-010', 7.221, 0.70, 95.5, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (10, 'OCW-B16-016', 6.779, 0.28, 97.3, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (11, 'OCW-B16-003', 6.658, 0.70, 99.3, 'PASS');
INSERT INTO assay_abe4rc3fza VALUES (12, 'OCW-B16-012', 6.708, 0.28, 99.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abe4rc3fza WHERE status = 'PASS';
