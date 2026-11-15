-- Orchid Cellworks synthetic laboratory table for OCW-LAB-B3RVPX2V.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abb3rvpx2v (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abb3rvpx2v VALUES (1, 'OCW-B16-002', 6.399, 0.42, 95.2, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (2, 'OCW-B16-012', 6.383, 0.71, 94.3, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (3, 'OCW-B16-005', 6.402, 0.28, 93.5, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (4, 'OCW-B16-006', 6.557, 0.71, 92.5, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (5, 'OCW-B16-003', 7.003, 0.56, 99.7, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (6, 'OCW-B16-012', 6.947, 0.14, 92.6, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (7, 'OCW-B16-013', 6.940, 0.70, 98.6, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (8, 'OCW-B16-006', 6.757, 0.56, 98.9, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (9, 'OCW-B16-006', 6.548, 0.14, 97.2, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (10, 'OCW-B16-010', 7.249, 0.56, 97.7, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (11, 'OCW-B16-001', 6.863, 0.14, 99.8, 'PASS');
INSERT INTO assay_abb3rvpx2v VALUES (12, 'OCW-B16-008', 6.527, 0.14, 92.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abb3rvpx2v WHERE status = 'PASS';
