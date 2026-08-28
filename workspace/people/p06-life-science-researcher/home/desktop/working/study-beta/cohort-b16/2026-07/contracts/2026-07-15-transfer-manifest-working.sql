-- Orchid Cellworks synthetic laboratory table for OCW-LAB-GYNSWHFC.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abgynswhfc (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abgynswhfc VALUES (1, 'OCW-B16-006', 6.954, 0.43, 92.4, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (2, 'OCW-B16-014', 6.434, 0.42, 96.0, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (3, 'OCW-B16-013', 7.094, 0.28, 97.4, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (4, 'OCW-B16-001', 6.947, 0.57, 92.2, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (5, 'OCW-B16-008', 7.192, 0.14, 95.5, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (6, 'OCW-B16-004', 6.399, 0.42, 95.3, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (7, 'OCW-B16-015', 6.618, 0.14, 97.8, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (8, 'OCW-B16-013', 6.708, 0.28, 93.3, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (9, 'OCW-B16-006', 6.588, 0.70, 97.0, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (10, 'OCW-B16-007', 7.206, 0.28, 97.5, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (11, 'OCW-B16-011', 6.482, 0.70, 93.2, 'PASS');
INSERT INTO assay_abgynswhfc VALUES (12, 'OCW-B16-005', 7.178, 0.14, 99.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abgynswhfc WHERE status = 'PASS';
