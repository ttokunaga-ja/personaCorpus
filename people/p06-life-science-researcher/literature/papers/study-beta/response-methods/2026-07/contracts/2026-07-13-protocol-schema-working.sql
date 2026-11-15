-- Orchid Cellworks synthetic laboratory table for OCW-LAB-D35CQ22D.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abd35cq22d (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abd35cq22d VALUES (1, 'OCW-B16-004', 7.157, 0.14, 96.3, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (2, 'OCW-B16-007', 7.045, 0.70, 99.7, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (3, 'OCW-B16-005', 6.912, 0.14, 92.1, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (4, 'OCW-B16-006', 6.653, 0.14, 97.0, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (5, 'OCW-B16-013', 7.101, 0.28, 95.1, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (6, 'OCW-B16-009', 6.807, 0.14, 95.0, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (7, 'OCW-B16-013', 6.428, 0.28, 94.0, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (8, 'OCW-B16-010', 6.522, 0.71, 96.5, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (9, 'OCW-B16-002', 7.163, 0.43, 99.4, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (10, 'OCW-B16-016', 6.491, 0.14, 93.0, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (11, 'OCW-B16-009', 6.581, 0.56, 93.6, 'PASS');
INSERT INTO assay_abd35cq22d VALUES (12, 'OCW-B16-016', 6.961, 0.42, 98.9, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abd35cq22d WHERE status = 'PASS';
