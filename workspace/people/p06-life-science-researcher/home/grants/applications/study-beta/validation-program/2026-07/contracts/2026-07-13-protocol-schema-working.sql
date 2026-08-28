-- Orchid Cellworks synthetic laboratory table for OCW-LAB-NQMPR4L5.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abnqmpr4l5 (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abnqmpr4l5 VALUES (1, 'OCW-B16-012', 6.954, 0.28, 99.4, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (2, 'OCW-B16-011', 6.581, 0.56, 99.9, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (3, 'OCW-B16-002', 7.164, 0.28, 95.6, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (4, 'OCW-B16-011', 6.835, 0.00, 98.1, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (5, 'OCW-B16-016', 6.856, 0.28, 92.7, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (6, 'OCW-B16-013', 6.695, 0.00, 98.8, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (7, 'OCW-B16-009', 7.031, 0.42, 95.5, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (8, 'OCW-B16-001', 6.623, 0.70, 100.0, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (9, 'OCW-B16-006', 6.592, 0.71, 97.7, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (10, 'OCW-B16-016', 6.447, 0.70, 97.3, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (11, 'OCW-B16-002', 6.548, 0.14, 96.5, 'PASS');
INSERT INTO assay_abnqmpr4l5 VALUES (12, 'OCW-B16-015', 6.869, 0.70, 97.0, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abnqmpr4l5 WHERE status = 'PASS';
