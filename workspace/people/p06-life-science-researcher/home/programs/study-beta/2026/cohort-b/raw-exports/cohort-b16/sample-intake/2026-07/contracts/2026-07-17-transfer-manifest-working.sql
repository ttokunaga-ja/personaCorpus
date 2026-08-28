-- Orchid Cellworks synthetic laboratory table for OCW-LAB-GXGDV26X.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abgxgdv26x (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abgxgdv26x VALUES (1, 'OCW-B16-010', 6.625, 0.00, 95.9, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (2, 'OCW-B16-013', 7.206, 0.28, 92.2, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (3, 'OCW-B16-013', 6.612, 0.28, 96.4, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (4, 'OCW-B16-006', 6.529, 0.57, 92.9, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (5, 'OCW-B16-003', 6.527, 0.14, 99.7, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (6, 'OCW-B16-003', 6.877, 0.57, 95.6, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (7, 'OCW-B16-001', 6.912, 0.14, 97.8, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (8, 'OCW-B16-005', 6.562, 0.14, 93.4, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (9, 'OCW-B16-008', 6.863, 0.14, 96.6, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (10, 'OCW-B16-001', 7.073, 0.56, 99.6, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (11, 'OCW-B16-003', 7.207, 0.42, 96.1, 'PASS');
INSERT INTO assay_abgxgdv26x VALUES (12, 'OCW-B16-006', 6.954, 0.28, 98.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abgxgdv26x WHERE status = 'PASS';
