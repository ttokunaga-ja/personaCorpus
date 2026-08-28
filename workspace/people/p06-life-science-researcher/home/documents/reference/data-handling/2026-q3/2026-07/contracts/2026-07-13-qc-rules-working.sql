-- Orchid Cellworks synthetic laboratory table for OCW-LAB-BESG3FZK.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abbesg3fzk (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abbesg3fzk VALUES (1, 'OCW-A24-004', 6.891, 0.28, 95.8, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (2, 'OCW-A24-007', 6.616, 0.56, 93.8, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (3, 'OCW-A24-007', 7.080, 0.70, 98.0, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (4, 'OCW-A24-015', 6.463, 0.28, 95.7, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (5, 'OCW-A24-009', 6.842, 0.14, 94.1, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (6, 'OCW-A24-021', 7.045, 0.00, 94.1, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (7, 'OCW-A24-019', 6.687, 0.56, 98.9, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (8, 'OCW-A24-005', 6.905, 0.71, 94.8, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (9, 'OCW-A24-013', 6.476, 0.56, 97.7, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (10, 'OCW-A24-008', 6.434, 0.42, 98.8, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (11, 'OCW-A24-024', 7.080, 0.70, 95.5, 'PASS');
INSERT INTO assay_abbesg3fzk VALUES (12, 'OCW-A24-020', 7.164, 0.28, 97.3, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abbesg3fzk WHERE status = 'PASS';
