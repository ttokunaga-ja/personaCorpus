-- Orchid Cellworks synthetic laboratory table for OCW-LAB-6OBM7IOJ.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab6obm7ioj (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab6obm7ioj VALUES (1, 'OCW-A24-001', 6.374, 0.14, 92.0, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (2, 'OCW-A24-001', 7.108, 0.14, 97.5, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (3, 'OCW-A24-004', 6.758, 0.14, 92.9, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (4, 'OCW-A24-018', 6.745, 0.43, 96.4, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (5, 'OCW-A24-001', 6.529, 0.57, 97.9, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (6, 'OCW-A24-018', 6.687, 0.56, 94.4, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (7, 'OCW-A24-024', 7.136, 0.42, 99.8, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (8, 'OCW-A24-022', 6.418, 0.71, 92.0, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (9, 'OCW-A24-010', 6.862, 0.56, 98.9, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (10, 'OCW-A24-004', 6.731, 0.71, 99.5, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (11, 'OCW-A24-013', 7.213, 0.14, 98.2, 'PASS');
INSERT INTO assay_ab6obm7ioj VALUES (12, 'OCW-A24-005', 7.087, 0.14, 93.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab6obm7ioj WHERE status = 'PASS';
