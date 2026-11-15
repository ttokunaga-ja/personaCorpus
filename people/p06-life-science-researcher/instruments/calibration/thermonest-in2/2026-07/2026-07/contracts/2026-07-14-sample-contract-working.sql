-- Orchid Cellworks synthetic laboratory table for OCW-LAB-UA7WETRW.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abua7wetrw (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abua7wetrw VALUES (1, 'OCW-A24-007', 6.527, 0.14, 96.0, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (2, 'OCW-A24-015', 6.667, 0.14, 97.5, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (3, 'OCW-A24-018', 6.814, 0.28, 95.4, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (4, 'OCW-A24-006', 6.982, 0.14, 97.4, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (5, 'OCW-A24-012', 6.863, 0.14, 92.1, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (6, 'OCW-A24-017', 6.581, 0.56, 95.9, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (7, 'OCW-A24-006', 7.003, 0.14, 97.5, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (8, 'OCW-A24-006', 7.038, 0.14, 95.4, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (9, 'OCW-A24-023', 6.792, 0.56, 92.8, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (10, 'OCW-A24-023', 6.849, 0.28, 98.3, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (11, 'OCW-A24-001', 6.494, 0.57, 98.5, 'PASS');
INSERT INTO assay_abua7wetrw VALUES (12, 'OCW-A24-001', 6.542, 0.28, 96.5, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abua7wetrw WHERE status = 'PASS';
