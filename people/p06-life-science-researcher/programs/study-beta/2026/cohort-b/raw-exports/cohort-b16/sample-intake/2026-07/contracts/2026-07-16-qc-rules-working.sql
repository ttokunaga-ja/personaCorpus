-- Orchid Cellworks synthetic laboratory table for OCW-LAB-QADZGCRU.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abqadzgcru (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abqadzgcru VALUES (1, 'OCW-B16-010', 7.059, 0.43, 99.6, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (2, 'OCW-B16-007', 7.003, 0.14, 98.2, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (3, 'OCW-B16-013', 7.003, 0.14, 95.1, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (4, 'OCW-B16-012', 7.221, 0.70, 94.0, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (5, 'OCW-B16-016', 7.157, 0.14, 94.4, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (6, 'OCW-B16-007', 6.808, 0.57, 93.9, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (7, 'OCW-B16-007', 6.412, 0.70, 93.4, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (8, 'OCW-B16-014', 6.428, 0.28, 92.7, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (9, 'OCW-B16-015', 6.581, 0.56, 95.3, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (10, 'OCW-B16-006', 7.179, 0.56, 93.2, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (11, 'OCW-B16-013', 6.877, 0.14, 96.3, 'PASS');
INSERT INTO assay_abqadzgcru VALUES (12, 'OCW-B16-012', 6.389, 0.57, 98.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abqadzgcru WHERE status = 'PASS';
