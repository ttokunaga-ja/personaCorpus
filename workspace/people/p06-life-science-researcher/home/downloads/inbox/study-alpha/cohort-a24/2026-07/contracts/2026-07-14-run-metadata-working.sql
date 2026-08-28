-- Orchid Cellworks synthetic laboratory table for OCW-LAB-PQGCBEOY.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abpqgcbeoy (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abpqgcbeoy VALUES (1, 'OCW-A24-001', 6.708, 0.28, 93.5, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (2, 'OCW-A24-006', 6.820, 0.42, 92.0, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (3, 'OCW-A24-017', 6.592, 0.71, 96.9, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (4, 'OCW-A24-004', 6.842, 0.14, 93.0, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (5, 'OCW-A24-011', 6.605, 0.43, 92.6, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (6, 'OCW-A24-009', 6.511, 0.56, 94.6, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (7, 'OCW-A24-024', 6.758, 0.14, 98.9, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (8, 'OCW-A24-022', 6.961, 0.42, 92.3, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (9, 'OCW-A24-020', 6.546, 0.56, 92.9, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (10, 'OCW-A24-015', 6.975, 0.71, 97.0, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (11, 'OCW-A24-020', 7.024, 0.28, 95.3, 'PASS');
INSERT INTO assay_abpqgcbeoy VALUES (12, 'OCW-A24-018', 7.003, 0.14, 99.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abpqgcbeoy WHERE status = 'PASS';
