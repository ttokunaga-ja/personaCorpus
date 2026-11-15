-- Orchid Cellworks synthetic laboratory table for OCW-LAB-CBXJQRKT.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abcbxjqrkt (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abcbxjqrkt VALUES (1, 'OCW-A24-006', 6.406, 0.56, 96.0, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (2, 'OCW-A24-020', 6.491, 0.14, 97.1, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (3, 'OCW-A24-005', 6.758, 0.14, 95.6, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (4, 'OCW-A24-016', 6.940, 0.00, 95.8, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (5, 'OCW-A24-002', 6.603, 0.28, 93.3, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (6, 'OCW-A24-017', 6.723, 0.14, 99.0, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (7, 'OCW-A24-005', 6.919, 0.28, 93.1, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (8, 'OCW-A24-002', 7.213, 0.14, 95.6, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (9, 'OCW-A24-003', 6.507, 0.28, 94.3, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (10, 'OCW-A24-022', 7.143, 0.56, 99.0, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (11, 'OCW-A24-008', 6.814, 0.43, 92.2, 'PASS');
INSERT INTO assay_abcbxjqrkt VALUES (12, 'OCW-A24-012', 6.856, 0.42, 92.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abcbxjqrkt WHERE status = 'PASS';
