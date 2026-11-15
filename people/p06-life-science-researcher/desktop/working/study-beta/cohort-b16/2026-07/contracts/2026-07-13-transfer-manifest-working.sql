-- Orchid Cellworks synthetic laboratory table for OCW-LAB-IIQ75W7J.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abiiq75w7j (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abiiq75w7j VALUES (1, 'OCW-B16-015', 6.975, 0.00, 93.5, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (2, 'OCW-B16-008', 6.940, 0.00, 92.6, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (3, 'OCW-B16-014', 7.024, 0.28, 94.3, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (4, 'OCW-B16-002', 6.940, 0.00, 93.0, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (5, 'OCW-B16-007', 7.242, 0.42, 94.2, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (6, 'OCW-B16-011', 7.206, 0.28, 94.6, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (7, 'OCW-B16-015', 7.031, 0.42, 95.9, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (8, 'OCW-B16-016', 6.842, 0.14, 94.7, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (9, 'OCW-B16-014', 6.660, 0.00, 94.5, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (10, 'OCW-B16-011', 6.383, 0.71, 98.6, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (11, 'OCW-B16-009', 6.399, 0.42, 97.4, 'PASS');
INSERT INTO assay_abiiq75w7j VALUES (12, 'OCW-B16-015', 6.750, 0.42, 99.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abiiq75w7j WHERE status = 'PASS';
