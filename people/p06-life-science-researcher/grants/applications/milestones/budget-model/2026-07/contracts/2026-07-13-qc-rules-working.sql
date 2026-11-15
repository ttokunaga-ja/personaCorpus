-- Orchid Cellworks synthetic laboratory table for OCW-LAB-3DR6NV7E.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab3dr6nv7e (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab3dr6nv7e VALUES (1, 'OCW-A24-009', 6.652, 0.56, 95.6, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (2, 'OCW-A24-022', 6.592, 0.71, 94.2, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (3, 'OCW-A24-016', 7.038, 0.56, 95.5, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (4, 'OCW-A24-023', 7.010, 0.70, 94.0, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (5, 'OCW-A24-011', 6.807, 0.14, 97.2, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (6, 'OCW-A24-005', 6.598, 0.57, 92.8, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (7, 'OCW-A24-001', 6.603, 0.28, 98.3, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (8, 'OCW-A24-004', 6.828, 0.14, 92.1, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (9, 'OCW-A24-008', 6.623, 0.70, 96.1, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (10, 'OCW-A24-005', 6.884, 0.43, 97.4, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (11, 'OCW-A24-019', 6.680, 0.42, 93.4, 'PASS');
INSERT INTO assay_ab3dr6nv7e VALUES (12, 'OCW-A24-015', 6.540, 0.42, 94.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab3dr6nv7e WHERE status = 'PASS';
