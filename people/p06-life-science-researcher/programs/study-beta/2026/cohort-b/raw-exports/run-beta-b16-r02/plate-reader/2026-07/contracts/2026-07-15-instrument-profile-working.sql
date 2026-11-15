-- Orchid Cellworks synthetic laboratory table for OCW-LAB-ALW65P6F.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abalw65p6f (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abalw65p6f VALUES (1, 'OCW-B16-002', 6.418, 0.71, 92.5, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (2, 'OCW-B16-007', 7.192, 0.14, 99.6, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (3, 'OCW-B16-009', 6.695, 0.00, 98.6, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (4, 'OCW-B16-012', 7.052, 0.57, 94.8, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (5, 'OCW-B16-011', 6.612, 0.28, 95.8, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (6, 'OCW-B16-005', 7.101, 0.28, 92.0, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (7, 'OCW-B16-009', 7.010, 0.00, 94.9, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (8, 'OCW-B16-010', 6.919, 0.28, 93.3, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (9, 'OCW-B16-006', 6.891, 0.42, 98.3, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (10, 'OCW-B16-013', 7.227, 0.14, 100.0, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (11, 'OCW-B16-010', 6.415, 0.00, 98.2, 'PASS');
INSERT INTO assay_abalw65p6f VALUES (12, 'OCW-B16-014', 6.399, 0.42, 94.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abalw65p6f WHERE status = 'PASS';
