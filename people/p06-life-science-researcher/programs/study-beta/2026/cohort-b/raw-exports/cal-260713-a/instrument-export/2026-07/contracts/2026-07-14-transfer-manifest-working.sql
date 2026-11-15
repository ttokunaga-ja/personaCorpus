-- Orchid Cellworks synthetic laboratory table for OCW-LAB-TW4DWLRA.
-- No patient, participant, PHI, PII, credential, or unpublished data.
CREATE TABLE assay_abtw4dwlra (observation_id INTEGER PRIMARY KEY, sample_id TEXT NOT NULL, mean_ng_ml REAL NOT NULL, duplicate_cv_percent REAL NOT NULL CHECK (duplicate_cv_percent <= 10.0), recovery_percent REAL NOT NULL CHECK (recovery_percent BETWEEN 90.0 AND 110.0), status TEXT NOT NULL);
INSERT INTO assay_abtw4dwlra VALUES (1, 'OCW-B16-001', 6.843, 0.57, 94.6, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (2, 'OCW-B16-013', 6.814, 0.43, 96.1, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (3, 'OCW-B16-002', 6.696, 0.71, 95.3, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (4, 'OCW-B16-013', 6.773, 0.57, 98.2, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (5, 'OCW-B16-008', 6.444, 0.14, 92.0, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (6, 'OCW-B16-013', 7.017, 0.14, 94.5, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (7, 'OCW-B16-015', 6.479, 0.14, 98.0, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (8, 'OCW-B16-010', 6.399, 0.42, 94.2, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (9, 'OCW-B16-013', 6.996, 0.42, 97.3, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (10, 'OCW-B16-012', 6.374, 0.14, 98.6, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (11, 'OCW-B16-011', 6.680, 0.42, 98.9, 'PASS');
INSERT INTO assay_abtw4dwlra VALUES (12, 'OCW-B16-010', 6.428, 0.28, 96.7, 'PASS');
SELECT COUNT(*) AS accepted_rows FROM assay_abtw4dwlra WHERE status = 'PASS';
