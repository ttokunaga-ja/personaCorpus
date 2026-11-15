-- Orchid Cellworks synthetic laboratory table for OCW-LAB-5O7PFXOG.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab5o7pfxog (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab5o7pfxog VALUES (1, 'OCW-A24-001', 6.884, 0.28, 99.2, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (2, 'OCW-A24-007', 7.136, 0.42, 94.3, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (3, 'OCW-A24-002', 7.045, 0.70, 98.5, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (4, 'OCW-A24-015', 6.975, 0.70, 94.4, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (5, 'OCW-A24-022', 7.079, 0.71, 99.8, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (6, 'OCW-A24-023', 6.570, 0.43, 94.5, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (7, 'OCW-A24-007', 6.996, 0.28, 98.9, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (8, 'OCW-A24-011', 6.668, 0.57, 97.9, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (9, 'OCW-A24-001', 6.533, 0.28, 94.4, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (10, 'OCW-A24-016', 6.745, 0.43, 93.0, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (11, 'OCW-A24-019', 6.450, 0.00, 98.9, 'PASS');
INSERT INTO assay_ab5o7pfxog VALUES (12, 'OCW-A24-014', 6.743, 0.28, 92.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab5o7pfxog WHERE status = 'PASS';
