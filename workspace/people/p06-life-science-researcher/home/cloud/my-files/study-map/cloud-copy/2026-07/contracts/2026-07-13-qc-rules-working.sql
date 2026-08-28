-- Orchid Cellworks synthetic laboratory table for OCW-LAB-COPULIHI.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abcopulihi (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abcopulihi VALUES (1, 'OCW-A24-008', 6.723, 0.14, 92.1, 'PASS');
INSERT INTO assay_abcopulihi VALUES (2, 'OCW-A24-007', 6.380, 0.00, 99.2, 'PASS');
INSERT INTO assay_abcopulihi VALUES (3, 'OCW-A24-013', 6.703, 0.57, 99.8, 'PASS');
INSERT INTO assay_abcopulihi VALUES (4, 'OCW-A24-004', 7.214, 0.56, 93.3, 'PASS');
INSERT INTO assay_abcopulihi VALUES (5, 'OCW-A24-021', 6.612, 0.28, 99.3, 'PASS');
INSERT INTO assay_abcopulihi VALUES (6, 'OCW-A24-015', 7.122, 0.14, 99.6, 'PASS');
INSERT INTO assay_abcopulihi VALUES (7, 'OCW-A24-011', 7.115, 0.00, 92.2, 'PASS');
INSERT INTO assay_abcopulihi VALUES (8, 'OCW-A24-003', 7.256, 0.70, 95.5, 'PASS');
INSERT INTO assay_abcopulihi VALUES (9, 'OCW-A24-014', 6.652, 0.56, 99.6, 'PASS');
INSERT INTO assay_abcopulihi VALUES (10, 'OCW-A24-004', 6.562, 0.14, 95.9, 'PASS');
INSERT INTO assay_abcopulihi VALUES (11, 'OCW-A24-004', 7.213, 0.14, 99.4, 'PASS');
INSERT INTO assay_abcopulihi VALUES (12, 'OCW-A24-024', 6.498, 0.28, 96.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abcopulihi WHERE status = 'PASS';
