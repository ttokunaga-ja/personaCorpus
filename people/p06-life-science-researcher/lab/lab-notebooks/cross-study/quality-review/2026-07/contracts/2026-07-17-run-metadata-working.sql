-- Orchid Cellworks synthetic laboratory table for OCW-LAB-N4GVWNXU.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abn4gvwnxu (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abn4gvwnxu VALUES (1, 'OCW-A24-012', 7.066, 0.42, 93.1, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (2, 'OCW-A24-005', 6.947, 0.57, 92.8, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (3, 'OCW-A24-014', 6.982, 0.57, 98.1, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (4, 'OCW-A24-013', 6.675, 0.43, 95.8, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (5, 'OCW-A24-019', 6.590, 0.00, 98.0, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (6, 'OCW-A24-003', 6.522, 0.71, 92.8, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (7, 'OCW-A24-016', 6.703, 0.57, 98.5, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (8, 'OCW-A24-020', 6.476, 0.56, 93.9, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (9, 'OCW-A24-001', 7.206, 0.28, 93.1, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (10, 'OCW-A24-011', 6.954, 0.43, 92.1, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (11, 'OCW-A24-012', 6.675, 0.43, 94.6, 'PASS');
INSERT INTO assay_abn4gvwnxu VALUES (12, 'OCW-A24-015', 6.652, 0.56, 95.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abn4gvwnxu WHERE status = 'PASS';
