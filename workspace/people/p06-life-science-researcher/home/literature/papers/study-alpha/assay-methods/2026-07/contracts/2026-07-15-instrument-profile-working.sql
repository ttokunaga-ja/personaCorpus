-- Orchid Cellworks synthetic laboratory table for OCW-LAB-J4Y2KZLP.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abj4y2kzlp (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abj4y2kzlp VALUES (1, 'OCW-A24-002', 6.540, 0.42, 95.2, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (2, 'OCW-A24-017', 7.094, 0.43, 98.7, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (3, 'OCW-A24-006', 6.513, 0.14, 94.8, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (4, 'OCW-A24-023', 6.912, 0.57, 95.9, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (5, 'OCW-A24-021', 6.968, 0.56, 94.5, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (6, 'OCW-A24-011', 6.583, 0.14, 94.4, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (7, 'OCW-A24-001', 6.658, 0.70, 98.9, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (8, 'OCW-A24-008', 6.424, 0.57, 96.5, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (9, 'OCW-A24-004', 6.737, 0.14, 97.6, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (10, 'OCW-A24-009', 7.017, 0.14, 93.6, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (11, 'OCW-A24-005', 6.463, 0.28, 94.3, 'PASS');
INSERT INTO assay_abj4y2kzlp VALUES (12, 'OCW-A24-003', 6.553, 0.70, 99.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abj4y2kzlp WHERE status = 'PASS';
