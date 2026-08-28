-- Orchid Cellworks synthetic laboratory table for OCW-LAB-UI6XDBAS.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abui6xdbas (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abui6xdbas VALUES (1, 'OCW-A24-001', 6.542, 0.28, 93.9, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (2, 'OCW-A24-014', 6.820, 0.42, 93.2, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (3, 'OCW-A24-008', 6.688, 0.14, 97.1, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (4, 'OCW-A24-006', 6.444, 0.14, 94.3, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (5, 'OCW-A24-017', 7.052, 0.14, 95.0, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (6, 'OCW-A24-017', 7.129, 0.43, 93.9, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (7, 'OCW-A24-019', 6.415, 0.00, 95.1, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (8, 'OCW-A24-007', 7.038, 0.56, 99.6, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (9, 'OCW-A24-022', 7.079, 0.71, 98.8, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (10, 'OCW-A24-007', 6.800, 0.00, 98.7, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (11, 'OCW-A24-001', 7.066, 0.42, 95.3, 'PASS');
INSERT INTO assay_abui6xdbas VALUES (12, 'OCW-A24-014', 6.469, 0.42, 99.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abui6xdbas WHERE status = 'PASS';
