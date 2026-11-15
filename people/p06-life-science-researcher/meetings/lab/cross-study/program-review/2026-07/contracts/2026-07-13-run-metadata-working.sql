-- Orchid Cellworks synthetic laboratory table for OCW-LAB-4N33IXSS.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ab4n33ixss (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ab4n33ixss VALUES (1, 'OCW-A24-011', 6.870, 0.00, 92.9, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (2, 'OCW-A24-020', 6.504, 0.42, 97.1, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (3, 'OCW-A24-021', 6.618, 0.14, 97.9, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (4, 'OCW-A24-007', 6.402, 0.28, 99.2, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (5, 'OCW-A24-010', 6.494, 0.57, 98.6, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (6, 'OCW-A24-015', 7.149, 0.71, 99.5, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (7, 'OCW-A24-016', 6.588, 0.70, 95.3, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (8, 'OCW-A24-016', 6.968, 0.14, 94.5, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (9, 'OCW-A24-018', 6.444, 0.14, 94.4, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (10, 'OCW-A24-003', 7.045, 0.70, 97.7, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (11, 'OCW-A24-004', 7.108, 0.14, 98.2, 'PASS');
INSERT INTO assay_ab4n33ixss VALUES (12, 'OCW-A24-018', 6.947, 0.14, 92.2, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ab4n33ixss WHERE status = 'PASS';
