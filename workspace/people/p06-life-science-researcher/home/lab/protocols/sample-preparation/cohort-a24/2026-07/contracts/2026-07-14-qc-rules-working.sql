-- Orchid Cellworks synthetic laboratory table for OCW-LAB-5RHLCB4L.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab5rhlcb4l (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab5rhlcb4l VALUES (1, 'OCW-A24-015', 6.653, 0.14, 95.2, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (2, 'OCW-A24-024', 6.588, 0.70, 99.9, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (3, 'OCW-A24-023', 6.482, 0.70, 92.5, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (4, 'OCW-A24-008', 6.598, 0.57, 96.6, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (5, 'OCW-A24-012', 6.919, 0.28, 98.2, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (6, 'OCW-A24-009', 7.108, 0.14, 96.3, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (7, 'OCW-A24-023', 6.482, 0.70, 92.8, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (8, 'OCW-A24-004', 7.094, 0.43, 93.3, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (9, 'OCW-A24-017', 6.441, 0.56, 95.9, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (10, 'OCW-A24-008', 6.807, 0.14, 92.9, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (11, 'OCW-A24-024', 6.522, 0.71, 93.6, 'PASS');
INSERT INTO assay_ab5rhlcb4l VALUES (12, 'OCW-A24-012', 6.568, 0.28, 94.6, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab5rhlcb4l WHERE status = 'PASS';
