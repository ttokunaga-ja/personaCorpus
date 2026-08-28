-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ZEA3BJBD.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abzea3bjbd (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abzea3bjbd VALUES (1, 'OCW-A24-015', 6.555, 0.00, 92.3, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (2, 'OCW-A24-002', 6.730, 0.00, 99.8, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (3, 'OCW-A24-015', 7.101, 0.28, 93.5, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (4, 'OCW-A24-021', 6.821, 0.28, 96.2, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (5, 'OCW-A24-012', 7.199, 0.28, 93.0, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (6, 'OCW-A24-023', 7.157, 0.14, 98.9, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (7, 'OCW-A24-002', 6.961, 0.28, 97.2, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (8, 'OCW-A24-013', 6.406, 0.56, 98.0, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (9, 'OCW-A24-021', 6.820, 0.42, 99.6, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (10, 'OCW-A24-022', 7.163, 0.43, 99.8, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (11, 'OCW-A24-012', 7.256, 0.70, 93.6, 'PASS');
INSERT INTO assay_abzea3bjbd VALUES (12, 'OCW-A24-004', 6.926, 0.28, 97.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abzea3bjbd WHERE status = 'PASS';
