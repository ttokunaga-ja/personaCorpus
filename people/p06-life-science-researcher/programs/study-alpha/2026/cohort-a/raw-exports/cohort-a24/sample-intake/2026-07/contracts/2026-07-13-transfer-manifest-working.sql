-- Orchid Cellworks synthetic laboratory table for OCW-LAB-BUL4R34R.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abbul4r34r (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abbul4r34r VALUES (1, 'OCW-A24-002', 7.213, 0.14, 92.4, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (2, 'OCW-A24-012', 6.897, 0.56, 93.0, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (3, 'OCW-A24-016', 6.590, 0.00, 99.4, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (4, 'OCW-A24-001', 6.638, 0.28, 98.1, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (5, 'OCW-A24-018', 6.354, 0.57, 96.2, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (6, 'OCW-A24-016', 6.773, 0.57, 98.8, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (7, 'OCW-A24-021', 7.178, 0.14, 98.9, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (8, 'OCW-A24-017', 6.919, 0.28, 97.2, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (9, 'OCW-A24-009', 6.856, 0.42, 93.4, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (10, 'OCW-A24-022', 6.799, 0.70, 95.6, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (11, 'OCW-A24-003', 6.710, 0.43, 93.6, 'PASS');
INSERT INTO assay_abbul4r34r VALUES (12, 'OCW-A24-022', 6.553, 0.70, 97.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abbul4r34r WHERE status = 'PASS';
