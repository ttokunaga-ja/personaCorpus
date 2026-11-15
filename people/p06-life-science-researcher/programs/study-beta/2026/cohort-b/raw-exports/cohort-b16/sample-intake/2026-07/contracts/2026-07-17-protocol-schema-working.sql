-- Orchid Cellworks synthetic laboratory table for OCW-LAB-6ZBELBPM.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab6zbelbpm (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab6zbelbpm VALUES (1, 'OCW-B16-016', 6.507, 0.28, 92.5, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (2, 'OCW-B16-003', 6.758, 0.14, 96.1, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (3, 'OCW-B16-008', 7.031, 0.28, 96.4, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (4, 'OCW-B16-013', 6.975, 0.70, 96.2, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (5, 'OCW-B16-015', 6.542, 0.28, 92.9, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (6, 'OCW-B16-014', 6.520, 0.00, 96.1, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (7, 'OCW-B16-010', 6.877, 0.14, 95.5, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (8, 'OCW-B16-004', 6.835, 0.00, 99.9, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (9, 'OCW-B16-013', 6.750, 0.42, 97.4, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (10, 'OCW-B16-012', 7.087, 0.14, 99.3, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (11, 'OCW-B16-008', 6.702, 0.14, 94.6, 'PASS');
INSERT INTO assay_ab6zbelbpm VALUES (12, 'OCW-B16-015', 6.494, 0.57, 99.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab6zbelbpm WHERE status = 'PASS';
