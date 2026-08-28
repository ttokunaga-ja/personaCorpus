-- Orchid Cellworks synthetic laboratory table for OCW-LAB-BJ5VNUIO.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abbj5vnuio (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abbj5vnuio VALUES (1, 'OCW-A24-014', 7.059, 0.43, 99.2, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (2, 'OCW-A24-024', 6.800, 0.00, 97.8, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (3, 'OCW-A24-001', 7.249, 0.56, 96.1, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (4, 'OCW-A24-017', 6.947, 0.57, 95.7, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (5, 'OCW-A24-017', 7.191, 0.57, 99.3, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (6, 'OCW-A24-006', 6.745, 0.43, 95.5, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (7, 'OCW-A24-003', 6.482, 0.70, 96.9, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (8, 'OCW-A24-013', 6.696, 0.71, 92.3, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (9, 'OCW-A24-013', 6.529, 0.57, 96.9, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (10, 'OCW-A24-024', 6.750, 0.42, 92.3, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (11, 'OCW-A24-015', 7.059, 0.28, 92.0, 'PASS');
INSERT INTO assay_abbj5vnuio VALUES (12, 'OCW-A24-018', 6.396, 0.43, 95.4, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abbj5vnuio WHERE status = 'PASS';
