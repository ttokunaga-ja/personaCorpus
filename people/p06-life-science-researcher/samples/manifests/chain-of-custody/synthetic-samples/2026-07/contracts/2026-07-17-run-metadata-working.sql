-- Orchid Cellworks synthetic laboratory table for OCW-LAB-OLWGQXQX.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abolwgqxqx (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abolwgqxqx VALUES (1, 'OCW-A24-001', 6.570, 0.43, 93.4, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (2, 'OCW-A24-024', 6.849, 0.28, 92.1, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (3, 'OCW-A24-017', 6.383, 0.71, 94.2, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (4, 'OCW-A24-023', 7.186, 0.70, 99.9, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (5, 'OCW-A24-012', 7.052, 0.14, 98.9, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (6, 'OCW-A24-011', 6.570, 0.43, 96.4, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (7, 'OCW-A24-008', 7.024, 0.28, 93.0, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (8, 'OCW-A24-013', 7.214, 0.56, 92.6, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (9, 'OCW-A24-005', 6.766, 0.71, 93.1, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (10, 'OCW-A24-018', 6.772, 0.14, 97.4, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (11, 'OCW-A24-004', 6.603, 0.28, 99.2, 'PASS');
INSERT INTO assay_abolwgqxqx VALUES (12, 'OCW-A24-003', 6.731, 0.71, 92.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abolwgqxqx WHERE status = 'PASS';
