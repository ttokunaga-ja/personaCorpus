-- Orchid Cellworks synthetic laboratory table for OCW-LAB-65SIWIRX.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab65siwirx (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab65siwirx VALUES (1, 'OCW-A24-023', 6.479, 0.14, 92.2, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (2, 'OCW-A24-016', 6.399, 0.42, 93.7, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (3, 'OCW-A24-014', 6.633, 0.57, 96.2, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (4, 'OCW-A24-021', 6.834, 0.70, 99.9, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (5, 'OCW-A24-022', 7.184, 0.71, 98.0, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (6, 'OCW-A24-016', 6.877, 0.57, 95.8, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (7, 'OCW-A24-020', 6.693, 0.70, 93.8, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (8, 'OCW-A24-020', 7.151, 0.70, 93.5, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (9, 'OCW-A24-008', 6.989, 0.43, 93.2, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (10, 'OCW-A24-012', 6.491, 0.14, 94.4, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (11, 'OCW-A24-003', 6.466, 0.43, 94.0, 'PASS');
INSERT INTO assay_ab65siwirx VALUES (12, 'OCW-A24-011', 7.214, 0.56, 98.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab65siwirx WHERE status = 'PASS';
