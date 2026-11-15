-- Orchid Cellworks synthetic laboratory table for OCW-LAB-LVFUNHFM.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_ablvfunhfm (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_ablvfunhfm VALUES (1, 'OCW-B16-001', 6.708, 0.28, 99.2, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (2, 'OCW-B16-013', 6.527, 0.14, 93.2, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (3, 'OCW-B16-013', 7.073, 0.14, 93.3, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (4, 'OCW-B16-002', 6.926, 0.28, 94.4, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (5, 'OCW-B16-010', 6.849, 0.28, 98.1, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (6, 'OCW-B16-009', 7.184, 0.71, 99.9, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (7, 'OCW-B16-014', 7.045, 0.70, 92.8, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (8, 'OCW-B16-012', 6.640, 0.43, 97.9, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (9, 'OCW-B16-002', 7.122, 0.14, 95.3, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (10, 'OCW-B16-016', 7.151, 0.70, 94.0, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (11, 'OCW-B16-006', 7.192, 0.14, 97.5, 'PASS');
INSERT INTO assay_ablvfunhfm VALUES (12, 'OCW-B16-014', 7.151, 0.70, 99.8, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_ablvfunhfm WHERE status = 'PASS';
