-- Orchid Cellworks synthetic laboratory table for OCW-LAB-S3SDTXHV.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abs3sdtxhv (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abs3sdtxhv VALUES (1, 'OCW-B16-010', 6.731, 0.71, 92.7, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (2, 'OCW-B16-007', 7.017, 0.14, 92.1, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (3, 'OCW-B16-002', 6.982, 0.57, 98.6, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (4, 'OCW-B16-011', 6.856, 0.42, 92.1, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (5, 'OCW-B16-004', 6.933, 0.14, 99.1, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (6, 'OCW-B16-009', 7.045, 0.71, 95.1, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (7, 'OCW-B16-007', 6.905, 0.71, 97.2, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (8, 'OCW-B16-012', 6.703, 0.57, 93.0, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (9, 'OCW-B16-010', 6.996, 0.42, 93.5, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (10, 'OCW-B16-016', 7.220, 0.00, 99.8, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (11, 'OCW-B16-003', 6.968, 0.56, 98.9, 'PASS');
INSERT INTO assay_abs3sdtxhv VALUES (12, 'OCW-B16-011', 6.511, 0.56, 92.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abs3sdtxhv WHERE status = 'PASS';
