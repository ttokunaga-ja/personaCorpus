-- Orchid Cellworks synthetic laboratory table for OCW-LAB-VFXLGAML.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abvfxlgaml (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abvfxlgaml VALUES (1, 'OCW-A24-013', 6.968, 0.14, 94.5, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (2, 'OCW-A24-020', 7.207, 0.42, 97.3, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (3, 'OCW-A24-011', 7.094, 0.43, 92.0, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (4, 'OCW-A24-012', 6.386, 0.14, 99.8, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (5, 'OCW-A24-023', 7.136, 0.42, 93.1, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (6, 'OCW-A24-003', 6.662, 0.71, 97.0, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (7, 'OCW-A24-014', 6.453, 0.71, 98.7, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (8, 'OCW-A24-009', 6.640, 0.43, 95.6, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (9, 'OCW-A24-005', 7.184, 0.71, 93.7, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (10, 'OCW-A24-005', 7.185, 0.00, 92.8, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (11, 'OCW-A24-011', 6.487, 0.71, 94.6, 'PASS');
INSERT INTO assay_abvfxlgaml VALUES (12, 'OCW-A24-008', 6.421, 0.14, 93.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abvfxlgaml WHERE status = 'PASS';
