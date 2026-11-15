-- Orchid Cellworks synthetic laboratory table for OCW-LAB-LKPHVRMG.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ablkphvrmg (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ablkphvrmg VALUES (1, 'OCW-B16-010', 7.220, 0.00, 93.9, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (2, 'OCW-B16-007', 6.912, 0.57, 99.4, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (3, 'OCW-B16-004', 7.108, 0.14, 95.2, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (4, 'OCW-B16-005', 6.482, 0.70, 98.7, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (5, 'OCW-B16-011', 7.080, 0.00, 95.2, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (6, 'OCW-B16-016', 6.396, 0.43, 97.2, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (7, 'OCW-B16-005', 6.835, 0.00, 99.0, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (8, 'OCW-B16-012', 6.870, 0.00, 94.2, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (9, 'OCW-B16-016', 7.256, 0.70, 94.7, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (10, 'OCW-B16-004', 6.687, 0.56, 98.7, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (11, 'OCW-B16-012', 6.884, 0.28, 98.6, 'PASS');
INSERT INTO assay_ablkphvrmg VALUES (12, 'OCW-B16-008', 6.354, 0.57, 99.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ablkphvrmg WHERE status = 'PASS';
