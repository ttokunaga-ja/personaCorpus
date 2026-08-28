-- Orchid Cellworks synthetic laboratory table for OCW-LAB-RRGKDLLD.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abrrgkdlld (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abrrgkdlld VALUES (1, 'OCW-B16-012', 6.415, 0.00, 95.5, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (2, 'OCW-B16-001', 6.437, 0.28, 93.2, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (3, 'OCW-B16-001', 6.367, 0.28, 93.9, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (4, 'OCW-B16-012', 6.491, 0.14, 93.8, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (5, 'OCW-B16-013', 6.961, 0.42, 93.1, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (6, 'OCW-B16-014', 6.553, 0.70, 96.6, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (7, 'OCW-B16-003', 6.623, 0.70, 98.9, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (8, 'OCW-B16-008', 6.710, 0.43, 98.5, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (9, 'OCW-B16-014', 6.424, 0.57, 94.5, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (10, 'OCW-B16-003', 6.444, 0.14, 92.8, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (11, 'OCW-B16-010', 6.715, 0.42, 96.3, 'PASS');
INSERT INTO assay_abrrgkdlld VALUES (12, 'OCW-B16-014', 7.186, 0.70, 95.1, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abrrgkdlld WHERE status = 'PASS';
